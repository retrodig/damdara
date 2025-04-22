use crate::constants::text::build_password_map;

pub fn decode_password_string(s: &str) -> Result<Vec<u8>, String> {
    if s.chars().count() != 20 {
        return Err("ふっかつのじゅもんは20文字である必要があります".to_string());
    }

    let password_map = build_password_map();
    let mut bits = Vec::with_capacity(20);

    for c in s.chars() {
        match password_map.get(&c) {
            Some(&v) => bits.push(v),
            None => return Err(format!("未対応の文字が含まれています: {}", c)),
        }
    }

    Ok(bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password_string() {
        let kana = "あいうえおかきくけこさしすせそたちつてと";
        let result = decode_password_string(kana).unwrap();

        assert_eq!(result.len(), 20);
        assert_eq!(result[0], 0); // 'あ'
        assert_eq!(result[1], 1); // 'い'
        assert_eq!(result[4], 4); // 'お'
        assert_eq!(result[15], 15); // 'た'
    }

    #[test]
    fn test_password_too_short() {
        let short = "あいうえお"; // 5文字
        let err = decode_password_string(short).unwrap_err();
        assert!(err.contains("20文字"));
    }

    #[test]
    fn test_invalid_character() {
        let bad = "あいうえおかきくけこさしすせそたちつて💥"; // 最後がemoji
        let err = decode_password_string(bad).unwrap_err();
        assert!(err.contains("未対応の文字"));
    }
}
