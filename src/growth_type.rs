use std::collections::HashMap;
use crate::constants::{KANA_TABLE, NAME_MAX_LENGTH};

pub fn calculate_growth_type(name: &str) -> u8 {
    let mut char_to_value = HashMap::new();
    for (i, &c) in KANA_TABLE.iter().enumerate() {
        char_to_value.insert(c, (i % 16) as u8);
    }

    let sum: u32 = name.chars()
        .take(NAME_MAX_LENGTH)
        .filter_map(|c| char_to_value.get(&c).copied())
        .map(|v| v as u32)
        .sum();

    (sum % 16) as u8
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
        // 文字「X」「Y」は kana_table に含まれない → 無視される
        // か=15, な=14 → 15+14 = 29 → 29 % 16 = 13
        assert_eq!(calculate_growth_type("かXなY"), 13);
    }

    #[test]
    fn test_空文字列() {
        // 何も入力されない → 合計 0 → 0 % 16 = 0
        assert_eq!(calculate_growth_type(""), 0);
    }

    #[test]
    fn test_全角スペース() {
        // 「　」＝全角スペース → 対応表では 15
        assert_eq!(calculate_growth_type("　"), 15);
    }
}