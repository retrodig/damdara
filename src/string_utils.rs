use crate::constants::text::{DAKUTEN_PAIRS, HANDAKUTEN_PAIRS, KANA_TABLE, NAME_MAX_LENGTH};
use std::collections::HashMap;

pub fn filter_valid_chars(input: &str) -> String {
    let valid_set: std::collections::HashSet<char> = KANA_TABLE.iter().copied().collect();

    input.chars().filter(|c| valid_set.contains(c)).collect()
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

pub fn build_kana_map() -> HashMap<char, u8> {
    KANA_TABLE
        .iter()
        .enumerate()
        .map(|(i, &kana)| (kana, i as u8))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(normalize_to_4_chars(""), "　　　　"); // 全角スペース4つ補完
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
