use crate::constants::text::{DAKUTEN_PAIRS, HANDAKUTEN_PAIRS, KANA_TABLE, NAME_MAX_LENGTH};
use std::collections::HashMap;

/// 無効文字を除去
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

/// 濁音分解
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

/// 4文字整形
pub fn normalize_to_4_chars(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.truncate(NAME_MAX_LENGTH);

    while chars.len() < NAME_MAX_LENGTH {
        chars.push('　');
    }
    chars.into_iter().collect()
}

/// 無効文字を除去 → 濁音分解 → 4文字整形
pub fn name_normalize(name: &str) -> String {
    let cleaned = filter_valid_chars(name);
    normalize_to_4_chars(&split_dakuten(&cleaned))
}

pub fn build_kana_map() -> HashMap<char, u8> {
    KANA_TABLE
        .iter()
        .enumerate()
        .map(|(i, &kana)| (kana, i as u8))
        .collect()
}

pub fn kana_index(c: char) -> Result<u8, String> {
    KANA_TABLE
        .iter()
        .position(|&k| k == c)
        .map(|i| i as u8)
        .ok_or_else(|| format!("文字 '{}' はKANA_TABLEに存在しません", c))
}

/// 指定された文字列の position（1始まり）番目の文字を返す
pub fn nth_char(s: &str, position: usize) -> Result<char, String> {
    if position == 0 {
        return Err("位置は1以上である必要があります".to_string());
    }

    s.chars()
        .nth(position - 1)
        .ok_or_else(|| format!("{}文字目が見つかりません（文字列: {}）", position, s))
}

/// 任意のビット列の (値, ビット幅) タプルを受け取り、上位から順に結合して u8 を返す
pub fn combine_bits(bits: &[(u8, u8)]) -> Result<u8, String> {
    let total_bits: u8 = bits.iter().map(|&(_, width)| width).sum();
    if total_bits > 8 {
        return Err(format!("合計ビット数が8を超えています: {}", total_bits));
    }

    let mut result: u8 = 0;
    let mut shift = total_bits;

    for &(value, width) in bits {
        shift -= width;
        result |= (value & ((1 << width) - 1)) << shift;
    }

    Ok(result)
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

    #[test]
    fn test_nth_char() {
        assert_eq!(nth_char("あいうえ", 1), Ok('あ'));
        assert_eq!(nth_char("あいうえ", 4), Ok('え'));
        assert!(nth_char("あい", 5).is_err());
        assert!(nth_char("うえ", 0).is_err());
    }

    #[test]
    fn test_combine_bits_simple() {
        let byte = combine_bits(&[(0b101, 3), (0b011, 3), (0b10, 2)]).unwrap();
        assert_eq!(byte, 0b10101110);
    }
}
