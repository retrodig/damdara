use crate::binary_utils::combine_bits;
use crate::constants::save_data::{SaveData, SaveDataArgs};
use crate::constants::text::{DEFAULT_NAME, PASSWORD_TABLE};
use crate::string_utils::{build_kana_map, kana_index, name_normalize, nth_char};
use std::collections::HashMap;

impl SaveData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with(args: SaveDataArgs) -> Self {
        let name = name_normalize(&args.name.unwrap_or_else(|| DEFAULT_NAME.to_string()));

        Self {
            name,
            experience: args.experience.unwrap_or(0),
            gold: args.gold.unwrap_or(0),
            weapon: args.weapon.unwrap_or(0),
            armor: args.armor.unwrap_or(0),
            shield: args.shield.unwrap_or(0),
            items: args.items.unwrap_or([0; 8]),
            herbs: args.herbs.unwrap_or(0),
            keys: args.keys.unwrap_or(0),
            has_dragon_scale: args.has_dragon_scale.unwrap_or(false),
            has_warrior_ring: args.has_warrior_ring.unwrap_or(false),
            has_cursed_necklace: args.has_cursed_necklace.unwrap_or(false),
            defeated_dragon: args.defeated_dragon.unwrap_or(false),
            defeated_golem: args.defeated_golem.unwrap_or(false),
            pattern: args.pattern.unwrap_or(0),
        }
    }

    pub fn full_bitstring(&self) -> Result<String, String> {
        Ok([
            self.check_code(),
            self.name_as_binary(),
            self.ex_as_binary(),
            self.gold_as_binary(),
            self.weapon_as_binary(),
            self.armor_as_binary(),
            self.shield_as_binary(),
            self.items_as_binary(),
            self.herbs_as_binary(),
            self.keys_as_binary(),
            self.flags_as_binary(),
            self.pattern_as_binary(),
        ]
        .join(""))
    }

    pub fn encode_name_to_bits(&self) -> Result<u32, String> {
        let kana_map: HashMap<char, u8> = build_kana_map();
        let chars: Vec<char> = self.name.chars().collect();

        let mut result: u32 = 0;
        for (i, c) in chars.iter().enumerate() {
            let index = kana_map
                .get(c)
                .ok_or_else(|| format!("未対応の文字が含まれています: {}", c))?;
            let shift = (3 - i) * 6;
            result |= (*index as u32) << shift;
        }

        Ok(result)
    }

    pub fn name_as_binary(&self) -> String {
        let result = self.encode_name_to_bits().unwrap();
        format!("{:024b}", result)
    }

    pub fn ex_as_binary(&self) -> String {
        format!("{:016b}", self.experience)
    }

    pub fn gold_as_binary(&self) -> String {
        format!("{:016b}", self.gold)
    }

    pub fn weapon_as_binary(&self) -> String {
        format!("{:03b}", self.weapon & 0b111)
    }

    pub fn armor_as_binary(&self) -> String {
        format!("{:03b}", self.armor & 0b111)
    }

    pub fn shield_as_binary(&self) -> String {
        format!("{:02b}", self.shield & 0b11)
    }

    pub fn items_as_binary(&self) -> String {
        self.items
            .iter()
            .map(|item| format!("{:04b}", item & 0b1111))
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn herbs_as_binary(&self) -> String {
        format!("{:04b}", self.herbs & 0b1111)
    }

    pub fn keys_as_binary(&self) -> String {
        format!("{:04b}", self.keys & 0b1111)
    }

    pub fn flags_as_binary(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.has_dragon_scale as u8,
            self.has_warrior_ring as u8,
            self.has_cursed_necklace as u8,
            self.defeated_dragon as u8,
            self.defeated_golem as u8
        )
    }

    pub fn pattern_as_binary(&self) -> String {
        format!("{:03b}", self.pattern & 0b111)
    }

    pub fn check_code(&self) -> String {
        format!("{:08b}", 0b00000000)
    }

    /// 経験値の下位8bit（後ろ半分）
    pub fn experience_low_byte(&self) -> u8 {
        (self.experience & 0x00FF) as u8
    }

    /// 経験値の上位8bit（前半分）
    pub fn experience_high_byte(&self) -> u8 {
        (self.experience >> 8) as u8
    }

    /// 所持金の下位8bit（後ろ半分）
    pub fn gold_low_byte(&self) -> u8 {
        (self.gold & 0x00FF) as u8
    }

    /// 所持金の上位8bit（前半分）
    pub fn gold_high_byte(&self) -> u8 {
        (self.gold >> 8) as u8
    }

    pub fn get_name_char_index(&self, position: usize) -> Result<u8, String> {
        if !(1..=4).contains(&position) {
            return Err(format!("position must be 1 to 4, got {}", position));
        }

        let c = self
            .name
            .chars()
            .nth(position - 1)
            .ok_or_else(|| format!("name is too short: {}", self.name))?;

        kana_index(c)
    }

    pub fn get_name_char_binary(&self, position: usize) -> String {
        let index = self.get_name_char_index(position).unwrap();
        format!("{:06b}", index)
    }

    pub fn pattern_bit_index(&self, bit: u8) -> Result<u8, String> {
        match bit {
            1 => Ok((self.pattern >> 0) & 1), // 1bit目（右端）
            2 => Ok((self.pattern >> 1) & 1), // 2bit目（中央）
            3 => Ok((self.pattern >> 2) & 1), // 3bit目（左端）
            _ => Err(format!("無効なbit位置: {}。1〜3を指定してください", bit)),
        }
    }

    fn get_item_value(&self, position: usize) -> Result<u8, String> {
        if position == 0 || position > self.items.len() {
            return Err(format!("無効なアイテム位置: {}", position));
        }
        Ok(self.items[position - 1] & 0x0F)
    }

    /// N番目のアイテム（1始まり）を取得（4bit想定）
    pub fn item_index_binary(&self, position: usize) -> Result<String, String> {
        let value = self.get_item_value(position)?;
        Ok(format!("{:04b}", value))
    }

    /// 指定された2つのアイテム位置の4bit値を結合して8bitの2進文字列を返す
    pub fn sum_item_index_binary(&self, pos1: usize, pos2: usize) -> Result<String, String> {
        let high = self.get_item_value(pos1)?;
        let low = self.get_item_value(pos2)?;
        Ok(format!("{:08b}", (high << 4) | low))
    }

    /// パターン3bit目 + has_cursed_necklace + 名前3文字目 の8bit合成
    pub fn cursed_check_code(&self) -> Result<u8, String> {
        let pattern_bit = self.pattern_bit_index(3)?; // 1bit
        let cursed_bit = if self.has_cursed_necklace { 1 } else { 0 };
        let kana_char = nth_char(&self.name, 3)?;
        let kana_index = kana_index(kana_char)?; // 6bit
        combine_bits(&[(pattern_bit, 1), (cursed_bit, 1), (kana_index, 6)])
    }

    /// 名前の1文字目 ＋ ゴーレム倒した？ ＋ パターンの2bit目（6bit + 1bit + 1bit）
    pub fn golem_check_code(&self) -> Result<u8, String> {
        let kana_char = nth_char(&self.name, 1)?;
        let kana_index = kana_index(kana_char)?; // 6bit
        let golem_bit = if self.defeated_golem { 1 } else { 0 };
        let pattern_bit = self.pattern_bit_index(2)?; // 1bit
        combine_bits(&[(kana_index, 6), (golem_bit, 1), (pattern_bit, 1)])
    }

    /// パターンの1bit目 ＋ ドラゴン倒した？ ＋ 名前の4文字目（1bit + 1bit + 6bit）
    pub fn dragon_check_code(&self) -> Result<u8, String> {
        let pattern_bit = self.pattern_bit_index(1)?; // 1bit
        let dragon_bit = if self.defeated_dragon { 1 } else { 0 };
        let kana_char = nth_char(&self.name, 4)?;
        let kana_index = kana_index(kana_char)?; // 6bit
        combine_bits(&[(pattern_bit, 1), (dragon_bit, 1), (kana_index, 6)])
    }

    /// りゅうのうろこ装備した？ ＋ 名前の2文字目 ＋ せんしのゆびわ装備した？（1bit + 6bit + 1bit）
    pub fn dragon_scale_check_code(&self) -> Result<u8, String> {
        let dragon_scale_bit = if self.has_dragon_scale { 1 } else { 0 };
        let kana_char = nth_char(&self.name, 2)?;
        let kana_index = kana_index(kana_char)?; // 6bit
        let warrior_ring_bit = if self.has_warrior_ring { 1 } else { 0 };
        combine_bits(&[
            (dragon_scale_bit, 1),
            (kana_index, 6),
            (warrior_ring_bit, 1),
        ])
    }

    // チェックコード（8bit）
    // 経験値の後ろ半分（8bit）
    // パターンの3bit目 ＋ しのくびかざり装備した？ ＋ 名前の3文字目（1bit + 1bit + 6bit）
    //
    // 4つ目のアイテム ＋ 3つ目のアイテム（4bit + 4bit）
    // ゴールドの後ろ半分（8bit）
    // 名前の1文字目 ＋ ゴーレム倒した？ ＋ パターンの2bit目（6bit + 1bit + 1bit）
    //
    // 8つ目のアイテム ＋ 7つ目のアイテム（4bit + 4bit）
    // パターンの1bit目 ＋ ドラゴン倒した？ ＋ 名前の4文字目（1bit + 1bit + 6bit）
    // ぶき ＋ よろい ＋ たて（3bit + 3bit + 2bit）
    //
    // ゴールドの前半分（8bit）
    // かぎの数 ＋ やくそうの数（4bit + 4bit）
    // 6つ目のアイテム ＋ 5つ目のアイテム（4bit + 4bit）
    //
    // 経験値の前半分（8bit）
    // りゅうのうろこ装備した？ ＋ 名前の2文字目 ＋ せんしのゆびわ装備した？（1bit + 6bit + 1bit）
    // 2つ目のアイテム ＋ 1つ目のアイテム（4bit + 4bit）
    pub fn build_password_base(&self) -> Result<Vec<String>, String> {
        Ok(vec![
            "00000000".to_string(),                        // チェックコード（仮）
            format!("{:08b}", self.experience_low_byte()), // 経験値下位8bit
            format!("{:08b}", self.cursed_check_code()?), // パターン3bit目 + くびかざり + 名前3文字目
            self.sum_item_index_binary(4, 3)?,            // アイテム4 + 3
            format!("{:08b}", self.gold_low_byte()),      // ゴールド下位8bit
            format!("{:08b}", self.golem_check_code()?),  // 名前1 + ゴーレム + パターン2bit目
            self.sum_item_index_binary(8, 7)?,            // アイテム8 + 7
            format!("{:08b}", self.dragon_check_code()?), // パターン1bit目 + ドラゴン + 名前4
            format!(
                "{:08b}",
                combine_bits(&[(self.weapon, 3), (self.armor, 3), (self.shield, 2),])?
            ), // 装備：武器 + 鎧 + 盾
            format!("{:08b}", self.gold_high_byte()),     // ゴールド上位8bit
            format!("{:08b}", combine_bits(&[(self.keys, 4), (self.herbs, 4),])?), // かぎ + やくそう
            self.sum_item_index_binary(6, 5)?,                                     // アイテム6 + 5
            format!("{:08b}", self.experience_high_byte()),                        // 経験値上位8bit
            format!("{:08b}", self.dragon_scale_check_code()?), // りゅうのうろこ + 名前2 + 戦士の指輪
            self.sum_item_index_binary(2, 1)?,                  // アイテム2 + 1
        ])
    }

    pub fn build_password_bits(&self) -> Result<Vec<String>, String> {
        let mut bits = self.build_password_base()?;
        let checksum = calculate_crc_from_bits(&bits)?;
        bits[0] = checksum; // 先頭に反映
        Ok(bits)
    }

    pub fn build_password_bitstring(&self) -> Result<String, String> {
        let bits = self.build_password_bits()?;
        Ok(bits.concat())
    }

    pub fn to_password_string(&self) -> Result<String, String> {
        let bitstring = self.build_password_bitstring()?; // Step3
        let reordered = reorder_password_bits(&bitstring)?; // Step4
        let kana_indices = apply_password_offsets(&reordered)?; // Step5
        let password = indices_to_password_kana(&kana_indices)?; // Step6
        Ok(password)
    }
}

pub fn calculate_crc_from_bits(bits: &[String]) -> Result<String, String> {
    if bits.len() != 15 {
        return Err(format!(
            "ビット列は15個必要です（実際は {} 個）",
            bits.len()
        ));
    }

    let mut crc: u16 = 0;
    for i in 1..15 {
        let mut octet =
            u8::from_str_radix(&bits[i], 2).map_err(|_| format!("無効な2進数: {}", bits[i]))?;

        for _ in 0..8 {
            let carry_bit = (((crc >> 8) as u8) ^ octet) & 0x80 != 0;
            crc = (crc << 1) & 0xffff;
            octet = (octet << 1) & 0xff;
            if carry_bit {
                crc ^= 0x1021;
            }
        }
    }
    Ok(format!("{:08b}", crc & 0xff)) // 下位8bit（8文字の2進文字列）
}

pub fn reorder_password_bits(bitstring: &str) -> Result<Vec<String>, String> {
    if bitstring.len() != 120 {
        return Err(format!(
            "ビット列は120bit必要です（現在: {}bit）",
            bitstring.len()
        ));
    }

    let mut result = Vec::new();

    for chunk in bitstring.as_bytes().chunks(24) {
        let chunk_str = std::str::from_utf8(chunk).unwrap(); // 安全: ASCIIのみ

        // 8bit × 3 に分ける
        let a = &chunk_str[0..8];
        let b = &chunk_str[8..16];
        let c = &chunk_str[16..24];

        // 左右反転: C, B, A
        let reversed = format!("{}{}{}", c, b, a);

        // 6bit × 4 に分割
        let s1 = &reversed[0..6];
        let s2 = &reversed[6..12];
        let s3 = &reversed[12..18];
        let s4 = &reversed[18..24];

        // 後ろから順に並べる
        result.extend_from_slice(&[
            s4.to_string(),
            s3.to_string(),
            s2.to_string(),
            s1.to_string(),
        ]);
    }

    Ok(result)
}

pub fn apply_password_offsets(base: &[String]) -> Result<Vec<u8>, String> {
    if base.len() != 20 {
        return Err(format!(
            "6bit文字列は20個必要です（現在: {}個）",
            base.len()
        ));
    }

    let mut result = Vec::with_capacity(20);
    let mut previous = 0u8;

    for bin in base {
        let mut value = u8::from_str_radix(bin, 2).map_err(|_| format!("無効な2進数: {}", bin))?;

        value = value.wrapping_add(4).wrapping_add(previous) & 0b111111;

        result.push(value);
        previous = value;
    }

    Ok(result)
}

pub fn indices_to_password_kana(indices: &[u8]) -> Result<String, String> {
    indices
        .iter()
        .map(|&i| {
            PASSWORD_TABLE
                .get(i as usize)
                .copied()
                .ok_or_else(|| format!("無効なインデックス: {}", i))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_name_to_bits_exact_value() {
        let save = SaveData::new();
        let result = save.encode_name_to_bits().unwrap();

        assert_eq!(result, 12109579, "bit pattern for 'ゆうてい' is incorrect");
        assert_eq!(format!("{:024b}", result), "101110001100011100001011");
    }

    #[test]
    fn test_name_as_binary() {
        let save = SaveData::new();
        let binary = save.name_as_binary();
        assert_eq!(binary, "101110001100011100001011");
    }

    #[test]
    fn test_full_bitstring_length_is_120() {
        let save = SaveData::new();
        let bitstring = save.full_bitstring().unwrap();
        assert_eq!(bitstring.len(), 120, "bitstring length is not 120");
    }

    #[test]
    fn test_experience_bytes() {
        let save = SaveData {
            experience: 43981, // 0xABCD - 1010101111001101
            ..Default::default()
        };

        assert_eq!(save.experience_low_byte(), 0xCD);
        assert_eq!(save.experience_high_byte(), 0xAB);
    }

    #[test]
    fn test_gold_bytes() {
        let save = SaveData {
            gold: 64206, // 0xFACE - 1111101011001110
            ..Default::default()
        };

        assert_eq!(save.gold_low_byte(), 0xCE); // 下位: 11001110
        assert_eq!(save.gold_high_byte(), 0xFA); // 上位: 11111010
    }

    #[test]
    fn test_get_name_char_index() {
        let save = SaveData::new();

        assert_eq!(save.get_name_char_index(1), Ok(46)); // ゆ
        assert_eq!(save.get_name_char_index(2), Ok(12)); // う
        assert_eq!(save.get_name_char_index(3), Ok(28)); // て
        assert_eq!(save.get_name_char_index(4), Ok(11)); // い
        assert!(save.get_name_char_index(0).is_err());
        assert!(save.get_name_char_index(5).is_err());
    }

    #[test]
    fn test_get_name_char_binary() {
        let save = SaveData::new();

        assert_eq!(save.get_name_char_binary(1), "101110"); // ゆ = 46 → 101110
        assert_eq!(save.get_name_char_binary(2), "001100"); // う = 12 → 001100
        assert_eq!(save.get_name_char_binary(3), "011100"); // て = 28 → 011100
        assert_eq!(save.get_name_char_binary(4), "001011"); // い = 11 → 001011
    }

    #[test]
    fn test_pattern_bit_index() {
        let save = SaveData {
            pattern: 0b101,
            ..Default::default()
        };

        assert_eq!(save.pattern_bit_index(1), Ok(1));
        assert_eq!(save.pattern_bit_index(2), Ok(0));
        assert_eq!(save.pattern_bit_index(3), Ok(1));
        assert!(save.pattern_bit_index(0).is_err());
        assert!(save.pattern_bit_index(4).is_err());
    }

    #[test]
    fn test_cursed_check_code() {
        let save = SaveData {
            pattern: 0b100, // pattern_bit_3 = 1
            has_cursed_necklace: true,
            ..Default::default()
        };

        let result = save.cursed_check_code().unwrap();
        // P3 = 1, cursed = 1, index(て) = 28 = 0b011100
        // => 1_1_011100 = 0b11011100 = 0xDC = 220
        assert_eq!(result, 0b11011100);
    }

    #[test]
    fn test_item_index_binary() {
        let save = SaveData {
            items: [1, 2, 3, 4, 0, 0, 0, 0],
            ..Default::default()
        };

        assert_eq!(save.item_index_binary(4), Ok("0100".to_string()));
        assert_eq!(save.item_index_binary(3), Ok("0011".to_string()));
        assert!(save.item_index_binary(0).is_err());
        assert!(save.item_index_binary(9).is_err());
    }

    #[test]
    fn test_sum_item_index_binary() {
        let save = SaveData {
            items: [1, 2, 3, 4, 0, 0, 0, 0],
            ..Default::default()
        };

        // 4 = 0100, 3 = 0011 → 01000011
        assert_eq!(save.sum_item_index_binary(4, 3), Ok("01000011".to_string()));
        // 1 = 0001, 2 = 0010 → 00010010
        assert_eq!(save.sum_item_index_binary(1, 2), Ok("00010010".to_string()));
        // 無効位置
        assert!(save.sum_item_index_binary(0, 3).is_err());
        assert!(save.sum_item_index_binary(3, 9).is_err());
    }

    #[test]
    fn test_build_password_base_bit_length() {
        let save = SaveData::new();
        let result = save.build_password_base().unwrap();

        assert_eq!(result.len(), 15, "出力の要素数が15ではありません");

        let total_len: usize = result.iter().map(|s| s.len()).sum();
        assert_eq!(total_len, 120, "合計ビット長が120ではありません");

        for (i, s) in result.iter().enumerate() {
            assert_eq!(s.len(), 8, "インデックス {} の長さが8bitではありません", i);
            assert!(
                s.chars().all(|c| c == '0' || c == '1'),
                "インデックス {} に無効な文字があります",
                i
            );
        }
    }

    #[test]
    fn test_calculate_crc_from_bits() {
        let dummy = vec![
            "00000000", "01010010", "00001101", "10010111", "00100111", "00010010", "01011100",
            "10011001", "01001100", "00111011", "00010100", "10100010", "00001011", "11001001",
            "01010000",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

        let crc = calculate_crc_from_bits(&dummy).unwrap();
        assert_eq!(crc.len(), 8);
        assert!(crc.chars().all(|c| c == '0' || c == '1'));
    }

    #[test]
    fn test_reorder_block() {
        let input = "111011110011110100010001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let reordered = reorder_password_bits(input).unwrap();

        assert_eq!(reordered[0], "101111");
        assert_eq!(reordered[1], "110111");
        assert_eq!(reordered[2], "010011");
        assert_eq!(reordered[3], "000100");
        assert_eq!(reordered.len(), 20);
    }

    #[test]
    fn test_to_password_string_is_20_chars() {
        let save = SaveData::new();
        let password = save.to_password_string().unwrap();
        assert_eq!(password.chars().count(), 20);
    }
}
