use crate::constants::save_data::SaveData;
use crate::constants::status::Flags;
use crate::constants::text::KANA_TABLE;
use crate::constants::text::build_password_map;
use crate::utility::binary_utils::{validate_6bit_array, validate_120bit};

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

/// 累積加算を逆に剥がして元の120bit値に戻す
pub fn undo_password_addition(values: &[u8]) -> Result<Vec<u8>, String> {
    validate_6bit_array(values)?;

    let mut result = vec![0u8; 20];
    for i in 0..20 {
        result[i] = if i == 0 {
            (values[0] + 64 - 4) % 64
        } else {
            // signedにして安全に演算し、modで戻す
            let current = values[i] as i16;
            let prev = values[i - 1] as i16;
            ((current - 4 - prev + 64 * 2) % 64) as u8
        };
    }

    Ok(result)
}

/// 6bit×20の並び順をブロック単位で復元する
pub fn reorder_blocks_back(bits: &[u8]) -> Result<Vec<u8>, String> {
    validate_6bit_array(bits)?;

    let mut result = Vec::with_capacity(20);

    // 6bit × 4 = 24bit × 5ブロック
    for block_i in 0..5 {
        let block_6bit = &bits[block_i * 4..(block_i + 1) * 4];

        // Step4で並びを逆順にされた → 元の順に戻す
        let reversed_6bit = block_6bit.iter().rev().cloned().collect::<Vec<_>>();

        // 6bit × 4 → 24bit に復元（ビット列結合）
        let mut combined: u32 = 0;
        for (i, &b) in reversed_6bit.iter().enumerate() {
            combined |= (b as u32) << (18 - i * 6);
        }

        // 24bit → 8bit × 3（反転された順） → 正しい順に戻す
        let byte1 = ((combined >> 16) & 0xFF) as u8;
        let byte2 = ((combined >> 8) & 0xFF) as u8;
        let byte3 = (combined & 0xFF) as u8;

        // 左右反転を戻す
        result.push(byte3);
        result.push(byte2);
        result.push(byte1);
    }

    Ok(result)
}

pub fn extract_name_from_bits(bits: &[u8]) -> Result<String, String> {
    validate_120bit(bits)?;
    let mut indices = [0u8; 4];
    // name[0] = bits[5] >> 2
    indices[0] = (bits[5] >> 2) & 0b0011_1111;
    // name[1] = (bits[13] >> 1) & 0b0011_1111;
    indices[1] = (bits[13] >> 1) & 0b0011_1111;
    // name[2] = bits[2] & 0b0011_1111;
    indices[2] = bits[2] & 0b0011_1111;
    // name[3] = bits[7] & 0b0011_1111;
    indices[3] = bits[7] & 0b0011_1111;

    let name = indices
        .iter()
        .map(|&i| KANA_TABLE.get(i as usize).copied().unwrap_or('　'))
        .collect();

    Ok(name)
}

pub fn extract_experience_from_bits(bits: &[u8]) -> Result<u16, String> {
    validate_120bit(bits)?;
    let lower = bits[1] as u16;
    let upper = bits[12] as u16;

    Ok((upper << 8) | lower)
}

pub fn extract_gold_from_bits(bits: &[u8]) -> Result<u16, String> {
    validate_120bit(bits)?;

    let lower = bits[4] as u16;
    let upper = bits[9] as u16;

    Ok((upper << 8) | lower)
}

pub fn extract_weapon_from_bits(bits: &[u8]) -> Result<u8, String> {
    validate_120bit(bits)?;

    Ok((bits[8] >> 5) & 0b0000_0111)
}

pub fn extract_armor_from_bits(bits: &[u8]) -> Result<u8, String> {
    validate_120bit(bits)?;
    Ok((bits[8] >> 2) & 0b0000_0111)
}

pub fn extract_shield_from_bits(bits: &[u8]) -> Result<u8, String> {
    validate_120bit(bits)?;
    Ok(bits[8] & 0b0000_0011)
}

pub fn extract_items_from_bits(bits: &[u8]) -> Result<[u8; 8], String> {
    validate_120bit(bits)?;
    Ok([
        bits[14] & 0x0F,
        (bits[14] >> 4) & 0x0F,
        bits[3] & 0x0F,
        (bits[3] >> 4) & 0x0F,
        bits[11] & 0x0F,
        (bits[11] >> 4) & 0x0F,
        bits[6] & 0x0F,
        (bits[6] >> 4) & 0x0F,
    ])
}

pub fn extract_flags_from_bits(bits: &[u8]) -> Result<Flags, String> {
    validate_120bit(bits)?;
    Ok(Flags {
        has_dragon_scale: ((bits[13] >> 7) & 1) == 1,
        has_warrior_ring: (bits[13] & 1) == 1,
        has_cursed_necklace: ((bits[2] >> 6) & 1) == 1,
        defeated_dragon: ((bits[7] >> 6) & 1) == 1,
        defeated_golem: ((bits[5] >> 1) & 1) == 1,
    })
}

pub fn extract_pattern_from_bits(bits: &[u8]) -> Result<u8, String> {
    validate_120bit(bits)?;
    let b0 = ((bits[7] >> 7) & 1) << 0;
    let b1 = (bits[5] & 1) << 1;
    let b2 = ((bits[2] >> 7) & 1) << 2;
    Ok(b0 | b1 | b2)
}

pub fn extract_herbs_and_keys_from_bits(bits: &[u8]) -> Result<(u8, u8), String> {
    validate_120bit(bits)?;

    let herbs = bits[10] & 0x0F;
    let keys = (bits[10] >> 4) & 0x0F;

    Ok((herbs, keys))
}

pub fn parse_bitstring_to_save_data(bits: &[u8]) -> Result<SaveData, String> {
    validate_120bit(bits)?;

    let flags = extract_flags_from_bits(&bits)?;
    let (herbs, keys) = extract_herbs_and_keys_from_bits(&bits)?;

    Ok(SaveData {
        name: extract_name_from_bits(&bits)?,
        experience: extract_experience_from_bits(&bits)?,
        gold: extract_gold_from_bits(&bits)?,
        weapon: extract_weapon_from_bits(&bits)?,
        armor: extract_armor_from_bits(&bits)?,
        shield: extract_shield_from_bits(&bits)?,
        items: extract_items_from_bits(&bits)?,
        herbs,
        keys,
        flags,
        pattern: extract_pattern_from_bits(&bits)?,
    })
}

pub fn decode_from_password_string(password: &str) -> Result<SaveData, String> {
    let encoded = decode_password_string(password)?;
    let raw = undo_password_addition(&encoded)?;
    let bit_block = reorder_blocks_back(&raw)?;
    parse_bitstring_to_save_data(&bit_block)
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

    #[test]
    fn test_undo_password_addition_normal_case() {
        // 仮の元データ（復号前の値）を Step5 でエンコードして、
        // その結果を元に逆演算 → 元に戻るかを確認
        let raw = vec![
            10, 20, 30, 40, 50, 12, 18, 24, 30, 36, 15, 23, 31, 39, 47, 5, 10, 20, 30, 40,
        ];

        // Step5風にエンコード
        let mut encoded = vec![0u8; 20];
        for i in 0..20 {
            encoded[i] = (raw[i] + 4 + if i > 0 { encoded[i - 1] } else { 0 }) % 64;
        }

        let decoded = undo_password_addition(&encoded).unwrap();
        assert_eq!(decoded, raw);
    }

    #[test]
    fn test_undo_password_addition_invalid_length() {
        let too_short = vec![1, 2, 3]; // 20未満

        let result = undo_password_addition(&too_short);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("20個のビット列"));
    }

    #[test]
    fn test_reorder_blocks_back_roundtrip() {
        // 仮のデータ：8bit × 15 = 120bit
        let original_bytes: Vec<u8> = (0..15).collect();

        // Step4: 保存処理側の再現（save.rsと同じ処理でエンコード）
        let mut encoded_bits = Vec::with_capacity(20);

        for chunk in original_bytes.chunks(3) {
            let byte1 = chunk[0];
            let byte2 = chunk[1];
            let byte3 = chunk[2];

            // 左右反転
            let block = ((byte3 as u32) << 16) | ((byte2 as u32) << 8) | (byte1 as u32);

            // 24bitを6bit×4に分解し、順序逆転
            let b1 = ((block >> 18) & 0x3F) as u8;
            let b2 = ((block >> 12) & 0x3F) as u8;
            let b3 = ((block >> 6) & 0x3F) as u8;
            let b4 = (block & 0x3F) as u8;

            encoded_bits.push(b4);
            encoded_bits.push(b3);
            encoded_bits.push(b2);
            encoded_bits.push(b1);
        }

        let decoded = reorder_blocks_back(&encoded_bits).unwrap();
        assert_eq!(decoded, original_bytes);
    }

    #[test]
    fn test_reorder_blocks_back_invalid_length() {
        let invalid_bits = vec![1, 2, 3]; // 不足している
        let result = reorder_blocks_back(&invalid_bits);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("20"));
    }
}
