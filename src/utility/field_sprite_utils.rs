use crate::constants::field_sprite::{
    CHARACTER_SPRITE_MASTER, CharacterSprite, PaletteSet, map_tile_pixels, pixel_rgb,
};
use crate::constants::map::find_map;

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
    fn test_ansi_line_count() {
        let sprite = find_character_sprite(0).unwrap();
        let ansi = field_pixels_ansi(&[sprite.frames[0].pixels, sprite.frames[1].pixels]);
        assert_eq!(ansi.lines().count(), FIELD_SPRITE_SIZE / 2);
    }
}
