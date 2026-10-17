use crate::constants::battle_background::{
    BATTLE_BACKDROP, BATTLE_BACKGROUND, BattleBackground, battle_background,
};
use crate::constants::enemy_sprite::{TRANSPARENT, enemy_pixel_rgb};
use crate::utility::enemy_sprite_utils::find_enemy_sprite;

/// 背景を 2x2 タイル (16x16) 単位で表示するときの1辺の画素数
const BLOCK_SIZE: usize = 16;

/// 戦闘画面 (背景 + 敵) の画素。座標は画面座標
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleScene {
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
    pub pixels: Vec<u8>, // 色番号、行優先 (index = y * width + x)
}

/// 背景の画素。reveal_blocks を指定すると reveal_order の先頭からその数のブロックだけ描き、残りは BATTLE_BACKDROP にする
/// (戦闘開始時に中央から渦巻き状に表示される演出用)。背景のないマップは全面 BATTLE_BACKDROP
pub fn battle_background_pixels(map_id: u8, reveal_blocks: Option<usize>) -> Vec<u8> {
    let Some(background) = battle_background(map_id) else {
        return vec![BATTLE_BACKDROP; background_size(&BATTLE_BACKGROUND)];
    };
    let Some(count) = reveal_blocks else {
        return background.pixels.to_vec();
    };

    let width = background.width as usize;
    let mut out = vec![BATTLE_BACKDROP; background_size(background)];
    for &(bx, by) in background.reveal_order.iter().take(count) {
        for py in 0..BLOCK_SIZE {
            let start = (by as usize * BLOCK_SIZE + py) * width + bx as usize * BLOCK_SIZE;
            out[start..start + BLOCK_SIZE]
                .copy_from_slice(&background.pixels[start..start + BLOCK_SIZE]);
        }
    }
    out
}

fn background_size(background: &BattleBackground) -> usize {
    background.width as usize * background.height as usize
}

/// 背景に敵を重ねた戦闘画面。敵が背景からはみ出す場合は、その分だけ広げて BATTLE_BACKDROP で埋める
pub fn battle_scene(monster_id: u8, map_id: u8) -> Option<BattleScene> {
    let sprite = find_enemy_sprite(monster_id)?;
    let bg = &BATTLE_BACKGROUND;

    let left = bg.x.min(sprite.x);
    let top = bg.y.min(sprite.y);
    let right = (bg.x as usize + bg.width as usize).max(sprite.x as usize + sprite.width as usize);
    let bottom =
        (bg.y as usize + bg.height as usize).max(sprite.y as usize + sprite.height as usize);
    let (width, height) = (right - left as usize, bottom - top as usize);

    let mut pixels = vec![BATTLE_BACKDROP; width * height];
    let mut draw = |src: &[u8], x: u8, y: u8, w: u8| {
        let (ox, oy) = ((x - left) as usize, (y - top) as usize);
        for (i, &color) in src.iter().enumerate() {
            if color != TRANSPARENT {
                pixels[(oy + i / w as usize) * width + ox + i % w as usize] = color;
            }
        }
    };
    draw(
        &battle_background_pixels(map_id, None),
        bg.x,
        bg.y,
        bg.width,
    );
    draw(sprite.pixels, sprite.x, sprite.y, sprite.width);

    Some(BattleScene {
        x: left,
        y: top,
        width: width as u8,
        height: height as u8,
        pixels,
    })
}

/// 色番号の画素を RGBA (1画素4byte) に変換する
pub fn battle_pixels_rgba(pixels: &[u8]) -> Vec<u8> {
    pixels
        .iter()
        .flat_map(|&color| match enemy_pixel_rgb(color) {
            Some((r, g, b)) => [r, g, b, 255],
            None => [0, 0, 0, 0],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::battle_background::BATTLE_BACKGROUND_TILE_PATTERNS;
    use crate::constants::enemy_sprite::ENEMY_SPRITE_MASTER;

    #[test]
    fn test_background_data() {
        let bg = &BATTLE_BACKGROUND;
        assert_eq!(bg.pixels.len(), background_size(bg));
        assert_eq!(
            bg.tiles.len(),
            (bg.width as usize / 8) * (bg.height as usize / 8)
        );
        for key in bg.tiles {
            assert!(
                BATTLE_BACKGROUND_TILE_PATTERNS
                    .iter()
                    .any(|(k, _)| k == key)
            );
        }
        // reveal_order は全ブロックを1回ずつ含む
        let blocks = bg.width as usize / BLOCK_SIZE * (bg.height as usize / BLOCK_SIZE);
        let mut order = bg.reveal_order.to_vec();
        order.sort();
        order.dedup();
        assert_eq!(order.len(), blocks);
        assert_eq!(bg.block_palettes.len(), blocks);
    }

    #[test]
    fn test_background_only_on_field() {
        assert!(battle_background(1).is_some());
        assert!(battle_background(15).is_none());
        let dungeon = battle_background_pixels(15, None);
        assert!(dungeon.iter().all(|&c| c == BATTLE_BACKDROP));
        assert_eq!(dungeon.len(), background_size(&BATTLE_BACKGROUND));
    }

    #[test]
    fn test_reveal_blocks() {
        assert!(
            battle_background_pixels(1, Some(0))
                .iter()
                .all(|&c| c == BATTLE_BACKDROP)
        );
        let all = BATTLE_BACKGROUND.reveal_order.len();
        assert_eq!(
            battle_background_pixels(1, Some(all)),
            BATTLE_BACKGROUND.pixels
        );

        // 最初のブロックだけ描かれる
        let first = battle_background_pixels(1, Some(1));
        let (bx, by) = BATTLE_BACKGROUND.reveal_order[0];
        let width = BATTLE_BACKGROUND.width as usize;
        for (i, &color) in first.iter().enumerate() {
            let in_block =
                i % width / BLOCK_SIZE == bx as usize && i / width / BLOCK_SIZE == by as usize;
            let expected = if in_block {
                BATTLE_BACKGROUND.pixels[i]
            } else {
                BATTLE_BACKDROP
            };
            assert_eq!(color, expected);
        }
    }

    #[test]
    fn test_scene_contains_every_monster() {
        for sprite in ENEMY_SPRITE_MASTER.iter() {
            let scene = battle_scene(sprite.id, 1).unwrap();
            assert_eq!(
                scene.pixels.len(),
                scene.width as usize * scene.height as usize
            );
            assert!(
                scene.x <= sprite.x && scene.y <= sprite.y,
                "id {}",
                sprite.id
            );
            assert!(scene.x <= BATTLE_BACKGROUND.x && scene.y <= BATTLE_BACKGROUND.y);
            assert!(!scene.pixels.contains(&TRANSPARENT));
        }
        assert!(battle_scene(40, 1).is_none());
    }

    #[test]
    fn test_scene_draws_monster_over_background() {
        let sprite = find_enemy_sprite(0).unwrap();
        let scene = battle_scene(0, 1).unwrap();
        for (i, &color) in sprite.pixels.iter().enumerate() {
            if color == TRANSPARENT {
                continue;
            }
            let x = (sprite.x - scene.x) as usize + i % sprite.width as usize;
            let y = (sprite.y - scene.y) as usize + i / sprite.width as usize;
            assert_eq!(scene.pixels[y * scene.width as usize + x], color);
        }
        assert_eq!(
            battle_pixels_rgba(&scene.pixels).len(),
            scene.pixels.len() * 4
        );
    }
}
