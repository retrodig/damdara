use std::collections::HashMap;

use crate::constants::text::KANA_TABLE;
use crate::string_utils::{filter_valid_chars, normalize_to_4_chars, split_dakuten};

pub fn calculate_growth_type(name: &str) -> u8 {
    let mut char_to_value = HashMap::new();
    for (i, &c) in KANA_TABLE.iter().enumerate() {
        char_to_value.insert(c, (i % 16) as u8);
    }

    // 無効文字を除去 → 濁音分解 → 4文字整形
    let cleaned = filter_valid_chars(name);
    let normalized = normalize_to_4_chars(&split_dakuten(&cleaned));

    // 各文字を合計
    let sum: u32 = normalized
        .chars()
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