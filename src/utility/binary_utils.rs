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

pub fn get_bits(bit_string: &str, from: usize, to: usize) -> Result<u32, String> {
    let slice = &bit_string
        .get(from..to)
        .ok_or_else(|| format!("bit range out of bounds: {}..{}", from, to))?;
    u32::from_str_radix(slice, 2).map_err(|e| format!("bit parse error: {}", e))
}

/// 120bit = 15バイトであるかをチェックする共通関数
pub fn validate_120bit(bits: &[u8]) -> Result<(), String> {
    if bits.len() != 15 {
        Err("ビット列は120bit（15バイト）である必要があります".to_string())
    } else {
        Ok(())
    }
}

/// 6bit × 20（= 20個の要素）であることを検証する関数
pub fn validate_6bit_array(bits: &[u8]) -> Result<(), String> {
    if bits.len() != 20 {
        Err("6bit × 20個のビット列が必要です（長さ: 20）".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_bits_simple() {
        let byte = combine_bits(&[(0b101, 3), (0b011, 3), (0b10, 2)]).unwrap();
        assert_eq!(byte, 0b10101110);
    }

    #[test]
    fn test_get_bits_valid_range() {
        let bit_string = "1101010101110001"; // 16bit
        let value = get_bits(&bit_string, 4, 8).unwrap(); // "0101"
        assert_eq!(value, 5);
    }

    #[test]
    fn test_get_bits_out_of_bounds() {
        let bit_string = "1010"; // only 4 bits
        let result = get_bits(&bit_string, 0, 8); // 超えてる
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("bit range out of bounds"));
    }

    #[test]
    fn test_get_bits_parse_error() {
        let bit_string = "1010XYZ101"; // 不正な文字
        let result = get_bits(&bit_string, 4, 7); // "XYZ"
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("bit parse error"));
    }
}
