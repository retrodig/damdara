use std::collections::HashMap;

use crate::constants::text::{GROWTH_VALUE_TABLE, KANA_TABLE};
use crate::string_utils::name_normalize;

// pub enum GrowthType {
//     A,
//     B,
//     C,
//     D,
// }
//
// pub struct GrowthPattern {
//     pub growth_type: GrowthType,
//     pub bonus: u8, // 0〜3
// }

// pub fn determine_growth_pattern(total: u8) -> GrowthPattern {
//     let pattern = total % 16;
//     let growth_type = match pattern % 4 {
//         0 => GrowthType::A,
//         1 => GrowthType::B,
//         2 => GrowthType::C,
//         3 => GrowthType::D,
//         _ => unreachable!(),
//     };
//     let bonus = pattern / 4;
//     GrowthPattern { growth_type, bonus }
// }

pub fn growth_table_index(c: char) -> Option<u8> {
    GROWTH_VALUE_TABLE
        .iter()
        .find(|&&(ch, _)| ch == c)
        .map(|&(_, val)| val)
}

pub fn calculate_name_total(name: &str) -> u16 {
    let normalized = name_normalize(name);
    normalized
        .chars()
        .filter_map(growth_table_index)
        .map(u16::from)
        .sum::<u16>()
}

pub fn calculate_growth_type(name: &str) -> u8 {
    let mut char_to_value = HashMap::new();
    for (i, &c) in KANA_TABLE.iter().enumerate() {
        char_to_value.insert(c, (i % 16) as u8);
    }
    // 無効文字を除去 → 濁音分解 → 4文字整形
    let normalized = name_normalize(name);
    // 各文字を合計
    let sum: u32 = normalized
        .chars()
        .filter_map(|c| char_to_value.get(&c).copied())
        .map(|v| v as u32)
        .sum();

    (sum % 16) as u8
}

#[derive(Debug)]
pub struct GrowthModifiers {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

pub fn calculate_abc(total: u16) -> GrowthModifiers {
    GrowthModifiers {
        a: (total / 4) % 4,
        b: (total / 2) % 2,
        c: total % 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ゆうてい() {
        // ゆ=14, う=12, て=12, い=11 → 合計 49 → %16 = 1
        assert_eq!(calculate_growth_type("ゆうてい"), 1);
    }

    #[test]
    fn test_みやおう() {
        // み=9, や=13, お=14, う=12 → 合計 48 → %16 = 0
        assert_eq!(calculate_growth_type("みやおう"), 0);
    }

    #[test]
    fn test_5文字以上() {
        // あ=10, い=11, う=12, え=13 → 合計46 → %16 = 14（おは無視される）
        assert_eq!(calculate_growth_type("あいうえお"), 14);
    }

    #[test]
    fn test_無効文字含む() {
        // 有効: か=15, な=14 → 15+14=29
        // 残り2文字分は空白（15）→ 合計 = 29 + 15 + 15 = 59 → %16 = 11
        assert_eq!(calculate_growth_type("かXなY"), 11);
    }

    #[test]
    fn test_1_moji() {
        // う = 12, 空白=15×3 → 合計 = 12 + 15*3 = 57 → %16 = 9
        assert_eq!(calculate_growth_type("う"), 9);
    }

    #[test]
    fn test_2_moji() {
        // ゆ=14, う=12, 空白=15×2 → 合計 = 14 + 12 + 15 + 15 = 56 → %16 = 8
        assert_eq!(calculate_growth_type("ゆう"), 8);
    }

    #[test]
    fn test_3_moji() {
        // や=13, お=14, う=12, 空白=15 → 合計 = 13 + 14 + 12 + 15 = 54 → %16 = 6
        assert_eq!(calculate_growth_type("やおう"), 6);
    }
}
