use std::collections::HashMap;
use crate::constants::{KANA_TABLE, NAME_MAX_LENGTH};

pub fn calculate_growth_type(name: &str) -> u8 {
    let mut char_to_value = HashMap::new();
    for (i, &c) in KANA_TABLE.iter().enumerate() {
        char_to_value.insert(c, (i % 16) as u8);
    }

    // 有効文字だけを抽出
    let mut valid_chars: Vec<char> = name
        .chars()
        .filter(|c| char_to_value.contains_key(c))
        .collect();

    // 足りない分は空白で補完
    while valid_chars.len() < NAME_MAX_LENGTH {
        valid_chars.push('　');
    }

    let sum: u32 = valid_chars
        .iter()
        .take(NAME_MAX_LENGTH)
        .filter_map(|c| char_to_value.get(c).copied())
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
    fn test_１文字() {
        // う = 12, 空白=15×3 → 合計 = 12 + 15*3 = 57 → %16 = 9
        assert_eq!(calculate_growth_type("う"), 9);
    }

    #[test]
    fn test_２文字() {
        // ゆ=14, う=12, 空白=15×2 → 合計 = 14 + 12 + 15 + 15 = 56 → %16 = 8
        assert_eq!(calculate_growth_type("ゆう"), 8);
    }

    #[test]
    fn test_３文字() {
        // や=13, お=14, う=12, 空白=15 → 合計 = 13 + 14 + 12 + 15 = 54 → %16 = 6
        assert_eq!(calculate_growth_type("やおう"), 6);
    }

}