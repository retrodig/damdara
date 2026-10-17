pub mod battle;
pub mod constants;
#[cfg(not(target_arch = "wasm32"))]
pub mod explore;
pub mod field;
pub mod growth_type;
pub mod input;
pub mod load;
pub mod message;
pub mod monster;
pub mod output;
pub mod player;
pub mod save;
pub mod traits;
pub mod utility;

// WASM module (only compiled for wasm32 target)
#[cfg(target_arch = "wasm32")]
pub mod wasm;

use battle::Battle;
pub use constants::config::Cli;
use constants::battle_background::BATTLE_BACKGROUND;
use constants::config::{DISPLAY_CATEGORY_LIST, Mode};
use constants::dialogue::DIALOGUE_MASTER;
use constants::field_sprite::{CHARACTER_SPRITE_MASTER, map_tile_sprite};
use constants::item_weapon::{ARMOR_MASTER, ITEM_MASTER, SHIELD_MASTER, WEAPON_MASTER};
use constants::map::{TILE_MASTER, find_map};
use constants::monster::MONSTER_MASTER;
use constants::spell::SPELL_INFO_LIST;
use constants::status::STATUS_TABLE;
use constants::town::TOWN_MASTER;
use field::{FIELD_MAP_ID, Field};
use input::cli_input::CliInput;
use monster::Monster;
use output::cli_output::CliOutput;
use player::Player;
use std::collections::HashSet;
use utility::battle_background_utils::{battle_background_pixels, battle_scene};
use utility::dialogue_utils::render_dialogue;
use utility::enemy_sprite_utils::{
    EnemySpriteSummary, enemy_sprite_ansi, enemy_sprite_summaries, find_enemy_sprite,
};
use utility::field_sprite_utils::{color_pixels_ansi, field_pixels_ansi, find_character_sprite};
use utility::map_utils::{find_tile, map_summaries};
use utility::status_utils::{get_status_by_level, get_status_list};

/// りゅうおうと戦う「りゅうおうのしろ さいかそう」
const FINAL_BOSS_MAP_ID: u8 = 6;

pub fn run_from_args(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let views: HashSet<String> = args
        .view
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|s| s.to_lowercase())
        .collect();
    let mut player = Player::new_with(args.to_player_args());
    if args.option.iter().any(|opt| opt == "max") {
        player.maximize();
    }

    let category = views
        .iter()
        .find(|s| DISPLAY_CATEGORY_LIST.contains(&s.as_str()))
        .map(|s| s.as_str());
    let is_list_mode = views.contains("list");
    let index = args
        .view
        .clone()
        .unwrap_or_default()
        .iter()
        .find_map(|s| s.parse::<usize>().ok());

    let mode = args.mode();
    match mode {
        Mode::Start => {
            println!("player name: {}", player.name);
            println!("summary: {:?}", player.summary());
            println!("strength_status: {:?}", player.strength_status());
            println!("item: {:?}", player.item_list());
        }
        Mode::Save => {
            println!("password: {}", player.to_password_string()?);
        }
        Mode::Load => {
            let new_player = Player::from_password_string(&args.password)?;
            println!("new_player from Password");
            println!("player name: {}", new_player.name);
            println!("summary: {:?}", new_player.summary());
            println!("strength_status: {:?}", new_player.strength_status());
        }
        Mode::Status => {
            if is_list_mode {
                // todo: 強化params
                println!("{:?}", get_status_list());
            } else {
                println!("status: {:?}", get_status_by_level(player.level()));
            }
        }
        Mode::Display => match category {
            Some("item") => print_list_or_index(&ITEM_MASTER, is_list_mode, index),
            Some("weapon") => print_list_or_index(&WEAPON_MASTER, is_list_mode, index),
            Some("armor") => print_list_or_index(&ARMOR_MASTER, is_list_mode, index),
            Some("shield") => print_list_or_index(&SHIELD_MASTER, is_list_mode, index),
            Some("status") => print_list_or_index(&STATUS_TABLE, is_list_mode, index),
            Some("spell") => print_list_or_index(&SPELL_INFO_LIST, is_list_mode, index),
            Some("monster") => print_list_or_index(&MONSTER_MASTER, is_list_mode, index),
            Some("town") => print_list_or_index(&TOWN_MASTER, is_list_mode, index),
            Some("dialogue") => match index {
                Some(i) if !is_list_mode => {
                    match render_dialogue(i, player.name.trim_end_matches('　')) {
                        Some(text) => println!("{}", text),
                        None => println!("Dialogue not found: {}", i),
                    }
                }
                _ => println!("{:?}", DIALOGUE_MASTER),
            },
            Some("map") => match index {
                // 番号はマップID（1: フィールド）
                Some(id) if !is_list_mode => match u8::try_from(id).ok().and_then(find_map) {
                    Some(map) => println!("{:?}", map),
                    None => println!("Map not found: {}", id),
                },
                _ => println!("{:?}", map_summaries()),
            },
            Some("tile") => print_list_or_index(&TILE_MASTER, is_list_mode, index),
            Some("sprite") => match index {
                // 番号はモンスターID
                Some(id) if !is_list_mode => {
                    match u8::try_from(id).ok().and_then(find_enemy_sprite) {
                        Some(sprite) => {
                            println!("{:?}", EnemySpriteSummary::from(sprite));
                            print!("{}", enemy_sprite_ansi(sprite));
                        }
                        None => println!("Sprite not found: {}", id),
                    }
                }
                _ => println!("{:?}", enemy_sprite_summaries()),
            },
            Some("tile_sprite") => match index {
                // 番号はタイルID。フィールド・街・ダンジョンの色を並べて表示する
                Some(id) if !is_list_mode => {
                    match u8::try_from(id).ok().and_then(map_tile_sprite) {
                        Some(tile) => {
                            println!("{:?}", find_tile(tile.id));
                            print!("{}", field_pixels_ansi(&tile.pixels));
                        }
                        None => println!("Tile sprite not found: {}", id),
                    }
                }
                _ => println!("{:?}", TILE_MASTER),
            },
            Some("character") => match index {
                // 歩行アニメーションの2コマを並べて表示する
                Some(id) if !is_list_mode => {
                    match u8::try_from(id).ok().and_then(find_character_sprite) {
                        Some(sprite) => {
                            println!("{}: {} ({})", sprite.id, sprite.name, sprite.name_en);
                            let frames = sprite.frames.each_ref().map(|f| f.pixels);
                            print!("{}", field_pixels_ansi(&frames));
                        }
                        None => println!("Character not found: {}", id),
                    }
                }
                _ => {
                    for sprite in CHARACTER_SPRITE_MASTER.iter() {
                        println!("{}: {} ({})", sprite.id, sprite.name, sprite.name_en);
                    }
                }
            },
            Some("battle_bg") => match index {
                // 番号はモンスターID。フィールドの背景に重ねて表示する
                Some(id) if !is_list_mode => {
                    match u8::try_from(id).ok().and_then(|id| battle_scene(id, FIELD_MAP_ID)) {
                        Some(scene) => print!("{}", color_pixels_ansi(&scene.pixels, scene.width as usize, 1)),
                        None => println!("Monster not found: {}", id),
                    }
                }
                _ => {
                    let pixels = battle_background_pixels(FIELD_MAP_ID, None);
                    print!("{}", color_pixels_ansi(&pixels, BATTLE_BACKGROUND.width as usize, 1));
                }
            },
            None => println!("Category not specified"),
            _ => {}
        },
        Mode::Battle => {
            let mut output = CliOutput;
            let mut input = CliInput;
            let monster = Monster::new(index.unwrap_or(0));
            // りゅうおうとの戦闘は城の中なので背景なし
            let map_id = if monster.is_final_boss() {
                FINAL_BOSS_MAP_ID
            } else {
                FIELD_MAP_ID
            };
            if let Some(scene) = battle_scene(monster.id, map_id) {
                print!("{}", color_pixels_ansi(&scene.pixels, scene.width as usize, 2));
            }
            let mut battle = Battle::new(player, monster, &mut input, &mut output);
            battle.start();
        }
        Mode::Explore => {
            // --view <マップID> <x> <y> で開始位置を指定できる
            let numbers: Vec<u8> = args
                .view
                .clone()
                .unwrap_or_default()
                .iter()
                .filter_map(|s| s.parse().ok())
                .collect();
            let field = match numbers[..] {
                [map_id, x, y, ..] if find_map(map_id).is_some() => Field::new(map_id, x, y),
                _ => Field::default(),
            };
            #[cfg(not(target_arch = "wasm32"))]
            explore::run_explore(&mut player, field)?;
            #[cfg(target_arch = "wasm32")]
            let _ = field;
        }
    }
    Ok(())
}

fn print_list_or_index<T: std::fmt::Debug>(list: &[T], is_list_mode: bool, index: Option<usize>) {
    if is_list_mode {
        println!("{:?}", list);
    } else if let Some(i) = index {
        let safe_index = i.min(list.len().saturating_sub(1));
        println!("{:?}", list[safe_index]);
    } else {
        println!("{:?}", list);
    }
}
