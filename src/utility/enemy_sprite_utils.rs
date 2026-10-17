use crate::constants::enemy_sprite::{ENEMY_SPRITE_MASTER, EnemySprite, enemy_pixel_rgb};
use crate::constants::monster::MONSTER_MASTER;

/// 画素データを除いたスプライトのメタ情報
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnemySpriteSummary {
    pub id: u8,
    pub name: &'static str,
    pub mirror: bool,
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
    pub part_count: usize,
}

impl From<&EnemySprite> for EnemySpriteSummary {
    fn from(sprite: &EnemySprite) -> Self {
        Self {
            id: sprite.id,
            name: monster_name(sprite.id),
            mirror: sprite.mirror,
            x: sprite.x,
            y: sprite.y,
            width: sprite.width,
            height: sprite.height,
            part_count: sprite.parts.len(),
        }
    }
}

/// 名前は MONSTER_MASTER に合わせる
pub fn monster_name(id: u8) -> &'static str {
    MONSTER_MASTER
        .get(id as usize)
        .map(|m| m.name)
        .unwrap_or_default()
}

pub fn find_enemy_sprite(id: u8) -> Option<&'static EnemySprite> {
    ENEMY_SPRITE_MASTER.iter().find(|s| s.id == id)
}

pub fn enemy_sprite_summaries() -> Vec<EnemySpriteSummary> {
    ENEMY_SPRITE_MASTER
        .iter()
        .map(EnemySpriteSummary::from)
        .collect()
}

/// RGBA (1画素4byte、透明は alpha 0) に変換する。Canvas の ImageData にそのまま使える
pub fn enemy_sprite_rgba(sprite: &EnemySprite) -> Vec<u8> {
    sprite
        .pixels
        .iter()
        .flat_map(|&color| match enemy_pixel_rgb(color) {
            Some((r, g, b)) => [r, g, b, 255],
            None => [0, 0, 0, 0],
        })
        .collect()
}

/// ターミナル表示用 (24bit カラーの ANSI エスケープ)。上下2画素を「▀」1文字で表す
pub fn enemy_sprite_ansi(sprite: &EnemySprite) -> String {
    let width = sprite.width as usize;
    let height = sprite.height as usize;
    let pixel = |x: usize, y: usize| {
        (y < height)
            .then(|| sprite.pixels[y * width + x])
            .and_then(enemy_pixel_rgb)
    };

    let mut out = String::new();
    for y in (0..height).step_by(2) {
        for x in 0..width {
            match (pixel(x, y), pixel(x, y + 1)) {
                (Some((r, g, b)), Some((r2, g2, b2))) => out.push_str(&format!(
                    "\x1b[38;2;{r};{g};{b}m\x1b[48;2;{r2};{g2};{b2}m▀\x1b[0m"
                )),
                (Some((r, g, b)), None) => out.push_str(&format!("\x1b[38;2;{r};{g};{b}m▀\x1b[0m")),
                (None, Some((r, g, b))) => out.push_str(&format!("\x1b[38;2;{r};{g};{b}m▄\x1b[0m")),
                (None, None) => out.push(' '),
            }
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::enemy_sprite::{TRANSPARENT, enemy_tile_pattern};

    #[test]
    fn test_sprite_ids_match_monster_master() {
        assert_eq!(ENEMY_SPRITE_MASTER.len(), MONSTER_MASTER.len());
        for (id, sprite) in ENEMY_SPRITE_MASTER.iter().enumerate() {
            assert_eq!(sprite.id as usize, id);
        }
    }

    #[test]
    fn test_pixels_length_matches_size() {
        for sprite in ENEMY_SPRITE_MASTER.iter() {
            assert_eq!(
                sprite.pixels.len(),
                sprite.width as usize * sprite.height as usize,
                "id {}",
                sprite.id
            );
        }
    }

    #[test]
    fn test_parts_reference_existing_patterns() {
        for sprite in ENEMY_SPRITE_MASTER.iter() {
            for part in sprite.parts {
                assert!(
                    enemy_tile_pattern(part.pattern).is_some(),
                    "id {} pattern {}",
                    sprite.id,
                    part.pattern
                );
            }
        }
    }

    #[test]
    fn test_summary_uses_monster_name() {
        let summary = EnemySpriteSummary::from(find_enemy_sprite(37).unwrap());
        assert_eq!(summary.name, "ダースドラゴン");
        assert!(find_enemy_sprite(40).is_none());
    }

    #[test]
    fn test_rgba_conversion() {
        let sprite = find_enemy_sprite(0).unwrap();
        let rgba = enemy_sprite_rgba(sprite);
        assert_eq!(rgba.len(), sprite.pixels.len() * 4);
        for (i, &color) in sprite.pixels.iter().enumerate() {
            let alpha = rgba[i * 4 + 3];
            assert_eq!(alpha == 0, color == TRANSPARENT);
        }
    }

    #[test]
    fn test_ansi_line_count() {
        let sprite = find_enemy_sprite(0).unwrap();
        let lines = enemy_sprite_ansi(sprite).lines().count();
        assert_eq!(lines, (sprite.height as usize).div_ceil(2));
    }
}
