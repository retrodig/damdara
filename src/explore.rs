use crate::constants::field_sprite::hero_character_id;
use crate::field::{Direction, Field, MoveEvent};
use crate::player::Player;
use crate::utility::field_sprite_utils::{FIELD_SPRITE_SIZE, color_pixels_ansi, field_view_pixels};
use crate::utility::map_utils::find_tile;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{cursor, execute, terminal};
use std::io::{self, Write};

/// 1/2 に縮小して表示する (1タイル = 横8文字 x 縦4行)
const SCALE: usize = 2;
const MAX_COLS: u16 = 15;
const MAX_ROWS: u16 = 11;
/// 表示の上下に使う行数
const TEXT_LINES: u16 = 4;

/// 終了時 (panic を含む) にターミナルを元に戻す
struct TerminalGuard;

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

/// 上下左右のキーでマップを歩く。q / Esc で終了
pub fn run_explore(player: &mut Player, mut field: Field) -> io::Result<()> {
    let _guard = TerminalGuard::new()?;
    let hero_id = hero_character_id(player.weapon > 0, player.shield > 0, false);
    let mut message = String::new();

    loop {
        draw(&field, hero_id, player.keys, &message)?;

        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        let direction = match key.code {
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => Direction::Up,
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => Direction::Down,
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => Direction::Left,
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => Direction::Right,
            KeyCode::Char('q') | KeyCode::Esc => break,
            _ => continue,
        };

        message = match field.try_move(direction, &mut player.keys) {
            MoveEvent::Warped { .. } => format!("{}　に　はいった", field.map().name),
            MoveEvent::DoorOpened => "とびらを　あけた".to_string(),
            MoveEvent::DoorLocked => "とびらには　かぎが　かかっている".to_string(),
            MoveEvent::Moved | MoveEvent::Blocked => String::new(),
        };
    }
    Ok(())
}

/// 端末の大きさに収まる奇数のタイル数 (勇者を中央に置くため)
fn view_size() -> (u8, u8) {
    let (width, height) = terminal::size().unwrap_or((80, 24));
    let tile_width = (FIELD_SPRITE_SIZE / SCALE) as u16;
    let tile_height = (FIELD_SPRITE_SIZE / SCALE / 2) as u16;
    let odd = |n: u16, max: u16| {
        let n = n.clamp(1, max);
        (if n % 2 == 0 { n - 1 } else { n }) as u8
    };
    (
        odd(width / tile_width, MAX_COLS),
        odd(height.saturating_sub(TEXT_LINES) / tile_height, MAX_ROWS),
    )
}

fn draw(field: &Field, hero_id: u8, keys: u8, message: &str) -> io::Result<()> {
    let (cols, rows) = view_size();
    let pixels = field_view_pixels(field, cols, rows, hero_id, field.steps as usize);
    let view = color_pixels_ansi(&pixels, cols as usize * FIELD_SPRITE_SIZE, SCALE);
    let map = field.map();
    let tile = find_tile(field.tile(field.x as i32, field.y as i32)).map_or("", |t| t.name);

    let mut out = io::stdout().lock();
    execute!(
        out,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )?;
    // raw mode では改行で行頭に戻らないため \r を足す
    write!(
        out,
        "{} ({}, {})  {}  かぎ: {}\r\n{}{}\r\n←↑↓→ / WASD / HJKL: いどう  q: おわる\r\n",
        map.name,
        field.x,
        field.y,
        tile,
        keys,
        view.replace('\n', "\r\n"),
        message,
    )?;
    out.flush()
}
