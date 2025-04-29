use crate::constants::status::{DEFAULT_STATUS, STATUS_TABLE, Status};
use crate::constants::text::KANA_TABLE;
use crate::utility::status_utils::get_status_by_level;
use crate::utility::string_utils::name_normalize;
use std::collections::HashMap;

// 「゛」と「゜」のみ別
pub fn calculate_growth_name_total(name: &str) -> u16 {
    let mut char_to_value = HashMap::new();
    for (i, &c) in KANA_TABLE.iter().enumerate() {
        char_to_value.insert(c, (i % 16) as u8);
    }
    // 無効文字を除去 → 濁音分解 → 4文字整形
    let normalized = name_normalize(name);
    let sum: u32 = normalized
        .chars()
        .map(|c| match c {
            '゛' => Some(3),
            '゜' => Some(4),
            _ => char_to_value.get(&c).copied(),
        })
        .flatten()
        .map(|v| v as u32)
        .sum();

    (sum % 16) as u16
}

#[derive(Debug)]
pub struct GrowthModifiers {
    pub a: u16, // bonus
    pub b: u16, // HP/AGI 補正用
    pub c: u16, // STR/MP 補正用
}

pub fn calculate_abc(total: u16) -> GrowthModifiers {
    GrowthModifiers {
        a: (total / 4) % 4,
        b: (total / 2) % 2,
        c: total % 2,
    }
}

pub fn get_adjusted_status_list(name: &str) -> Vec<Status> {
    let abc = calculate_abc(calculate_growth_name_total(&name));
    STATUS_TABLE
        .iter()
        .map(|base| base.apply_abc_modifiers(&abc))
        .collect()
}

pub fn get_adjusted_status_by_name_lv(name: &str, lv: u8) -> Status {
    let abc = calculate_abc(calculate_growth_name_total(&name));
    let base = get_status_by_level(lv).unwrap_or(DEFAULT_STATUS.clone());
    base.apply_abc_modifiers(&abc)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Growth Name Table
    // 0	1	2	3	4	5	6	7	8	9	10	11	12	13	14	15
    // ０	１	２	３	４	５	６	７	８	９	あ	い	う	え	お	か
    // き	く	け	こ	さ	し	す	せ	そ	た	ち	つ	て	と	な	に
    // ぬ	ね	の	は	ひ	ふ	へ	ほ	ま	み	む	め	も	や	ゆ	よ
    // ら	り	る	れ	ろ	わ	を	ん	っ	ゃ	ゅ	ょ	 	 	－
    //              ゛  ゜

    #[test]
    fn test_normal_name() {
        // ゆ=14, う=12, て=12, い=11 → 合計 49 → %16 = 1
        assert_eq!(calculate_growth_name_total("ゆうてい"), 1);
        // み=9, や=13, お=14, う=12 → 合計 48 → %16 = 0
        assert_eq!(calculate_growth_name_total("みやおう"), 0);
    }

    #[test]
    fn test_dakuten_handakuten() {
        // た=9, ゛=3, い=11, 空白=15 → 合計 38 → %16 = 6
        assert_eq!(calculate_growth_name_total("だい"), 6);
        // ひ=4, ゜=4, ひ=4, ゜=4 → 合計 16 → %16 = 0
        assert_eq!(calculate_growth_name_total("ぴぴ"), 0);
        // は=3, ゛=3, は=3, ゛=3 → 合計 12 → %16 = 12
        assert_eq!(calculate_growth_name_total("ばば"), 12);
    }

    #[test]
    fn test_zero_one() {
        // ０=0, き=0, ぬ=0, ら=0 → 合計 0 → %16 = 0
        assert_eq!(calculate_growth_name_total("０きぬら"), 0);
        // き=0, く=1, ぬ=0, ら=0 → 合計 1 → %16 = 1
        assert_eq!(calculate_growth_name_total("きくぬら"), 1);
    }

    #[test]
    fn test_more_than_5_characters() {
        // あ=10, い=11, う=12, え=13 → 合計46 → %16 = 14（おは無視される）
        assert_eq!(calculate_growth_name_total("あいうえお"), 14);
    }

    #[test]
    fn test_contains_invalid_characters() {
        // 有効: か=15, な=14 → 15+14=29
        // 残り2文字分は空白（15）→ 合計 = 29 + 15 + 15 = 59 → %16 = 11
        assert_eq!(calculate_growth_name_total("かXなY"), 11);
    }

    #[test]
    fn test_moji_lenght() {
        // 1 length
        // う = 12, 空白=15×3 → 合計 = 12 + 15*3 = 57 → %16 = 9
        assert_eq!(calculate_growth_name_total("う"), 9);
        // 2 length
        // ゆ=14, う=12, 空白=15×2 → 合計 = 14 + 12 + 15 + 15 = 56 → %16 = 8
        assert_eq!(calculate_growth_name_total("ゆう"), 8);
        // 3 length
        // や=13, お=14, う=12, 空白=15 → 合計 = 13 + 14 + 12 + 15 = 54 → %16 = 6
        assert_eq!(calculate_growth_name_total("やおう"), 6);
    }
}
