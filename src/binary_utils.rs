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
    fn test_combine_bits_simple() {
        let byte = combine_bits(&[(0b101, 3), (0b011, 3), (0b10, 2)]).unwrap();
        assert_eq!(byte, 0b10101110);
    }
}
