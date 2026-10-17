use crate::constants::field_sprite::{
    CHARACTER_SPRITE_MASTER, CharacterSprite, PaletteSet, TRANSPARENT, map_tile_pixels,
    npc_character_id, pixel_rgb,
};
use crate::constants::map::find_map;
use crate::field::Field;

/// マップチップ・キャラクターの1辺の画素数
pub const FIELD_SPRITE_SIZE: usize = 16;

pub fn find_character_sprite(id: u8) -> Option<&'static CharacterSprite> {
    CHARACTER_SPRITE_MASTER.iter().find(|s| s.id == id)
}

/// 色番号の画素を RGBA (1画素4byte、透明は alpha 0) に変換する
pub fn field_pixels_rgba(pixels: &[u8]) -> Vec<u8> {
    pixels
        .iter()
        .flat_map(|&color| match pixel_rgb(color) {
            Some((r, g, b)) => [r, g, b, 255],
            None => [0, 0, 0, 0],
        })
        .collect()
}

/// マップの指定範囲 (タイル単位) を RGBA で描画する。範囲外は border_tile で埋める
pub fn render_map_rgba(map_id: u8, x: i32, y: i32, width: u32, height: u32) -> Option<Vec<u8>> {
    let map = find_map(map_id)?;
    let width = width as usize;
    let height = height as usize;
    let row_len = width * FIELD_SPRITE_SIZE * 4;
    let mut out = vec![0; row_len * height * FIELD_SPRITE_SIZE];

    for ty in 0..height {
        for tx in 0..width {
            let (mx, my) = (x + tx as i32, y + ty as i32);
            let in_range =
                (0..map.width as i32).contains(&mx) && (0..map.height as i32).contains(&my);
            let tile = if in_range {
                map.tiles[my as usize * map.width as usize + mx as usize]
            } else {
                map.border_tile
            };
            let Some(pixels) = map_tile_pixels(tile, map_id) else {
                continue;
            };
            let rgba = field_pixels_rgba(pixels);
            for py in 0..FIELD_SPRITE_SIZE {
                let src = py * FIELD_SPRITE_SIZE * 4;
                let dst = (ty * FIELD_SPRITE_SIZE + py) * row_len + tx * FIELD_SPRITE_SIZE * 4;
                out[dst..dst + FIELD_SPRITE_SIZE * 4]
                    .copy_from_slice(&rgba[src..src + FIELD_SPRITE_SIZE * 4]);
            }
        }
    }
    Some(out)
}

/// 16x16 の画素を横に並べてターミナル表示用 (24bit カラーの ANSI エスケープ) にする。上下2画素を「▀」1文字で表す
pub fn field_pixels_ansi(images: &[&[u8; 256]]) -> String {
    let pixel = |image: &[u8; 256], x: usize, y: usize| pixel_rgb(image[y * FIELD_SPRITE_SIZE + x]);

    let mut out = String::new();
    for y in (0..FIELD_SPRITE_SIZE).step_by(2) {
        for image in images {
            for x in 0..FIELD_SPRITE_SIZE {
                match (pixel(image, x, y), pixel(image, x, y + 1)) {
                    (Some((r, g, b)), Some((r2, g2, b2))) => out.push_str(&format!(
                        "\x1b[38;2;{r};{g};{b}m\x1b[48;2;{r2};{g2};{b2}m▀\x1b[0m"
                    )),
                    (Some((r, g, b)), None) => {
                        out.push_str(&format!("\x1b[38;2;{r};{g};{b}m▀\x1b[0m"))
                    }
                    (None, Some((r, g, b))) => {
                        out.push_str(&format!("\x1b[38;2;{r};{g};{b}m▄\x1b[0m"))
                    }
                    (None, None) => out.push(' '),
                }
            }
            out.push(' ');
        }
        out.push('\n');
    }
    out
}

/// 現在地を中心に cols x rows タイルを色番号の画素で描く (NPC と勇者を重ねる)。幅は cols * 16
pub fn field_view_pixels(field: &Field, cols: u8, rows: u8, hero_id: u8, frame: usize) -> Vec<u8> {
    let (cols, rows) = (cols as i32, rows as i32);
    let width = cols as usize * FIELD_SPRITE_SIZE;
    let mut out = vec![TRANSPARENT; width * rows as usize * FIELD_SPRITE_SIZE];
    let (left, top) = (field.x as i32 - cols / 2, field.y as i32 - rows / 2);

    let mut draw = |pixels: &[u8; 256], tx: i32, ty: i32| {
        for (i, &color) in pixels.iter().enumerate() {
            if color == TRANSPARENT {
                continue;
            }
            let px = tx as usize * FIELD_SPRITE_SIZE + i % FIELD_SPRITE_SIZE;
            let py = ty as usize * FIELD_SPRITE_SIZE + i / FIELD_SPRITE_SIZE;
            out[py * width + px] = color;
        }
    };

    for ty in 0..rows {
        for tx in 0..cols {
            let tile = field.tile(left + tx, top + ty);
            if let Some(pixels) = map_tile_pixels(tile, field.map_id) {
                draw(pixels, tx, ty);
            }
        }
    }

    let mut characters: Vec<(u8, i32, i32)> = field
        .map()
        .npcs
        .iter()
        .map(|npc| {
            let id = npc_character_id(npc.sprite, field.map_id, false);
            (id, npc.x as i32, npc.y as i32)
        })
        .collect();
    characters.push((hero_id, field.x as i32, field.y as i32));

    for (id, x, y) in characters {
        let (tx, ty) = (x - left, y - top);
        if !(0..cols).contains(&tx) || !(0..rows).contains(&ty) {
            continue;
        }
        if let Some(sprite) = find_character_sprite(id) {
            draw(sprite.frames[frame % 2].pixels, tx, ty);
        }
    }
    out
}

/// 色番号の画素をターミナル表示用 (24bit カラーの ANSI エスケープ) にする。
/// scale 画素ごとに間引き、上下2画素を「▀」1文字で表す
pub fn color_pixels_ansi(pixels: &[u8], width: usize, scale: usize) -> String {
    let height = pixels.len() / width;
    let pixel = |x: usize, y: usize| {
        (y < height)
            .then(|| pixels[y * width + x])
            .and_then(pixel_rgb)
    };

    // 色は変わったときだけ出力して、1画面の文字数を抑える
    let (mut fg, mut bg) = (None, None);
    let mut out = String::new();
    for y in (0..height).step_by(scale * 2) {
        for x in (0..width).step_by(scale) {
            let (top, bottom) = (pixel(x, y), pixel(x, y + scale));
            let (next_fg, next_bg, ch) = match (top, bottom) {
                (Some(_), _) => (top, bottom, '▀'),
                (None, Some(_)) => (bottom, None, '▄'),
                (None, None) => (fg, None, ' '),
            };
            if next_fg != fg {
                if let Some((r, g, b)) = next_fg {
                    out.push_str(&format!("\x1b[38;2;{r};{g};{b}m"));
                }
                fg = next_fg;
            }
            if next_bg != bg {
                match next_bg {
                    Some((r, g, b)) => out.push_str(&format!("\x1b[48;2;{r};{g};{b}m")),
                    None => out.push_str("\x1b[49m"),
                }
                bg = next_bg;
            }
            out.push(ch);
        }
        out.push_str("\x1b[0m\n");
        (fg, bg) = (None, None);
    }
    out
}

/// PaletteSet を文字列にする ("field" | "town" | "dungeon")
pub fn palette_set_name(set: PaletteSet) -> &'static str {
    match set {
        PaletteSet::Field => "field",
        PaletteSet::Town => "town",
        PaletteSet::Dungeon => "dungeon",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::field_sprite::{
        BG_TILE_PATTERNS, MAP_TILE_SPRITE_MASTER, SPRITE_TILE_PATTERNS, hero_character_id,
        npc_character_id,
    };
    use crate::constants::map::{MAP_MASTER, TILE_MASTER};

    #[test]
    fn test_map_tile_ids_match_tile_master() {
        assert_eq!(MAP_TILE_SPRITE_MASTER.len(), TILE_MASTER.len());
        for (id, tile) in MAP_TILE_SPRITE_MASTER.iter().enumerate() {
            assert_eq!(tile.id as usize, id);
            for key in tile.tiles {
                assert!(
                    BG_TILE_PATTERNS.iter().any(|(k, _)| *k == key),
                    "tile {} key {}",
                    id,
                    key
                );
            }
        }
    }

    #[test]
    fn test_character_ids_and_parts() {
        for (id, sprite) in CHARACTER_SPRITE_MASTER.iter().enumerate() {
            assert_eq!(sprite.id as usize, id);
            for frame in &sprite.frames {
                for part in frame.parts {
                    assert!(
                        SPRITE_TILE_PATTERNS.iter().any(|(k, _)| *k == part.tile),
                        "character {} tile {}",
                        id,
                        part.tile
                    );
                }
            }
        }
    }

    #[test]
    fn test_all_map_tiles_and_npcs_have_sprites() {
        for map in MAP_MASTER.iter() {
            for &tile in map.tiles {
                assert!(
                    map_tile_pixels(tile, map.id).is_some(),
                    "map {} tile {}",
                    map.id,
                    tile
                );
            }
            for npc in map.npcs {
                let id = npc_character_id(npc.sprite, map.id, false);
                assert!(
                    find_character_sprite(id).is_some(),
                    "map {} sprite {}",
                    map.id,
                    npc.sprite
                );
            }
        }
    }

    #[test]
    fn test_hero_character_id() {
        assert_eq!(
            find_character_sprite(hero_character_id(false, false, false))
                .unwrap()
                .name,
            "ゆうしゃ"
        );
        assert_eq!(
            find_character_sprite(hero_character_id(true, true, false))
                .unwrap()
                .name,
            "ゆうしゃ（ぶき・たて）"
        );
        assert_eq!(
            find_character_sprite(hero_character_id(true, true, true))
                .unwrap()
                .name,
            "ゆうしゃ（ローラひめ）"
        );
    }

    #[test]
    fn test_render_map_rgba() {
        let rgba = render_map_rgba(1, -1, 0, 3, 2).unwrap();
        assert_eq!(rgba.len(), 3 * 2 * 256 * 4);
        // 範囲外は border_tile
        let border = find_map(1).unwrap().border_tile;
        let expected = field_pixels_rgba(map_tile_pixels(border, 1).unwrap());
        assert_eq!(&rgba[0..64], &expected[0..64]);
        assert!(render_map_rgba(0, 0, 0, 1, 1).is_none());
    }

    #[test]
    fn test_field_view_pixels() {
        let field = Field::default();
        let pixels = field_view_pixels(&field, 5, 3, 8, 0);
        assert_eq!(pixels.len(), 5 * 3 * 256);
        // 中央に勇者が描かれる
        let hero = find_character_sprite(8).unwrap().frames[0].pixels;
        let (cx, cy) = (2 * 16, 16);
        for (i, &color) in hero.iter().enumerate() {
            if color != TRANSPARENT {
                assert_eq!(pixels[(cy + i / 16) * 80 + cx + i % 16], color);
            }
        }
        let ansi = color_pixels_ansi(&pixels, 80, 2);
        assert_eq!(ansi.lines().count(), 3 * 16 / 4);
    }

    #[test]
    fn test_ansi_line_count() {
        let sprite = find_character_sprite(0).unwrap();
        let ansi = field_pixels_ansi(&[sprite.frames[0].pixels, sprite.frames[1].pixels]);
        assert_eq!(ansi.lines().count(), FIELD_SPRITE_SIZE / 2);
    }
}
