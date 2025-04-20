use std::collections::HashMap;
use crate::constants::text::{KANA_TABLE, NAME_MAX_LENGTH, DAKUTEN_PAIRS, HANDAKUTEN_PAIRS};

pub fn filter_valid_chars(input: &str) -> String {
    let valid_set: std::collections::HashSet<char> = KANA_TABLE.iter().copied().collect();

    input.chars()
        .filter(|c| valid_set.contains(c))
        .collect()
}

pub fn build_dakuten_map() -> HashMap<char, (char, Option<char>)> {
    let mut map = HashMap::new();

    for &(voiced, base) in DAKUTEN_PAIRS {
        map.insert(voiced, (base, Some('゛')));
    }
    for &(voiced, base) in HANDAKUTEN_PAIRS {
        map.insert(voiced, (base, Some('゜')));
    }
    map
}

pub fn split_dakuten(input: &str) -> String {
    let dakuten_map = build_dakuten_map();

    let mut result = String::new();
    for c in input.chars() {
        if let Some((base, mark)) = dakuten_map.get(&c) {
            result.push(*base);
            if let Some(m) = mark {
                result.push(*m);
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn normalize_to_4_chars(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.truncate(NAME_MAX_LENGTH);

    while chars.len() < NAME_MAX_LENGTH {
        chars.push('　');
    }
    chars.into_iter().collect()
}

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

    #[test]
    fn test_split_dakuten_simple() {
        assert_eq!(split_dakuten("だい"), "た゛い");
        assert_eq!(split_dakuten("がぎぐげご"), "か゛き゛く゛け゛こ゛");
        assert_eq!(split_dakuten("ぱぴぷぺぽ"), "は゜ひ゜ふ゜へ゜ほ゜");
    }

    #[test]
    fn test_split_dakuten_mixed() {
        assert_eq!(split_dakuten("だいがく"), "た゛いか゛く");
        assert_eq!(split_dakuten("おはよう"), "おはよう"); // 濁点なし → そのまま
        assert_eq!(split_dakuten("ばななとぱいん"), "は゛ななとは゜いん");
    }

    #[test]
    fn test_split_dakuten_empty() {
        assert_eq!(split_dakuten(""), "");
    }

    #[test]
    fn test_exactly_4_chars() {
        assert_eq!(normalize_to_4_chars("あいうえ"), "あいうえ");
    }

    #[test]
    fn test_less_than_4_chars() {
        assert_eq!(normalize_to_4_chars("あい"), "あい　　"); // 全角スペース2つ補完
        assert_eq!(normalize_to_4_chars("た"), "た　　　"); // 全角スペース3つ補完
        assert_eq!(normalize_to_4_chars(""), "　　　　");  // 全角スペース4つ補完
    }

    #[test]
    fn test_more_than_4_chars() {
        assert_eq!(normalize_to_4_chars("あいうえお"), "あいうえ");
        assert_eq!(normalize_to_4_chars("だいがくせい"), "だいがく");
    }

    #[test]
    fn test_combined_with_dakuten() {
        // た゛い → 3文字（'た', '゛', 'い'）なので補完1文字必要
        assert_eq!(normalize_to_4_chars("た゛い"), "た゛い　");
        // た゛いた゛い → 6文字 → 先頭4文字のみ
        assert_eq!(normalize_to_4_chars("た゛いた゛い"), "た゛いた");
    }
}