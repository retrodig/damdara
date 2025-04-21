use crate::constants::text::DEFAULT_NAME;
use crate::string_utils::{build_kana_map, kana_index, normalize_to_4_chars};
use std::collections::HashMap;

pub struct SaveData {
    pub name: String,              // 名前: 6bit×4文字
    pub experience: u16,           // 経験値: 0–65535 （16bit）
    pub gold: u16,                 // ゴールド: 0–65535 （16bit）
    pub weapon: u8,                // ぶき: 0–7 （3bit）
    pub armor: u8,                 // よろい: 0–7 （3bit）
    pub shield: u8,                // たて: 0–3 （2bit）
    pub items: [u8; 8],            // アイテム: 各 0–15（4bit×8）
    pub herbs: u8,                 // やくそうの数: 0–6（3bit）
    pub keys: u8,                  // かぎの数: 0–6（3bit）
    pub has_dragon_scale: bool,    // りゅうのうろこを装備したか
    pub has_warrior_ring: bool,    // せんしのゆびわを装備したか
    pub has_cursed_necklace: bool, // しのくびかざりを入手したか
    pub defeated_dragon: bool,     // ドラゴンを倒したか
    pub defeated_golem: bool,      // ゴーレムを倒したか
    pub pattern: u8,               // パターン: 0–7
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            name: DEFAULT_NAME.to_string(),
            experience: 0,
            gold: 0,
            weapon: 0,
            armor: 0,
            shield: 0,
            items: [0; 8],
            herbs: 0,
            keys: 0,
            has_dragon_scale: false,
            has_warrior_ring: false,
            has_cursed_necklace: false,
            defeated_dragon: false,
            defeated_golem: false,
            pattern: 0,
        }
    }
}

pub struct SaveDataArgs {
    pub name: Option<String>,
    pub experience: Option<u16>,
    pub gold: Option<u16>,
    pub weapon: Option<u8>,
    pub armor: Option<u8>,
    pub shield: Option<u8>,
    pub items: Option<[u8; 8]>,
    pub herbs: Option<u8>,
    pub keys: Option<u8>,
    pub has_dragon_scale: Option<bool>,
    pub has_warrior_ring: Option<bool>,
    pub has_cursed_necklace: Option<bool>,
    pub defeated_dragon: Option<bool>,
    pub defeated_golem: Option<bool>,
    pub pattern: Option<u8>,
}

impl Default for SaveDataArgs {
    fn default() -> Self {
        Self {
            name: None,
            experience: None,
            gold: None,
            weapon: None,
            armor: None,
            shield: None,
            items: None,
            herbs: None,
            keys: None,
            has_dragon_scale: None,
            has_warrior_ring: None,
            has_cursed_necklace: None,
            defeated_dragon: None,
            defeated_golem: None,
            pattern: None,
        }
    }
}

impl SaveData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with(args: SaveDataArgs) -> Self {
        Self {
            name: normalize_to_4_chars(&args.name.unwrap_or_else(|| DEFAULT_NAME.to_string())),
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

    /// パターン3bit目 + has_cursed_necklace + 名前3文字目 の8bit合成
    pub fn cursed_check_code(&self) -> Result<u8, String> {
        let pattern_bit = self.pattern_bit_index(3)?; // ← 最上位ビット
        let cursed_bit = if self.has_cursed_necklace { 1 } else { 0 };
        let kana_char = self
            .name
            .chars()
            .nth(2)
            .ok_or_else(|| "名前が3文字未満です".to_string())?;
        let kana_index = kana_index(kana_char)?; // 6bit値

        Ok((pattern_bit << 7) | (cursed_bit << 6) | kana_index)
    }
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
}
