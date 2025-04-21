use crate::constants::text::DEFAULT_NAME;
use crate::string_utils::{build_kana_map, normalize_to_4_chars};
use std::collections::HashMap;

pub struct SaveData {
    pub name: String,
    pub experience: u16, // 0–65535
    pub gold: u16,       // 0–65535
    pub weapon: u8,      // 0–7
    pub armor: u8,       // 0–7
    pub shield: u8,      // 0–3
    pub items: [u8; 8],  // 各 0–15
    pub herbs: u8,       // 0–15
    pub keys: u8,        // 0–15
    pub has_dragon_scale: bool,
    pub has_warrior_ring: bool,
    pub has_cursed_necklace: bool,
    pub defeated_dragon: bool,
    pub defeated_golem: bool,
    pub pattern: u8, // 0–7
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
            self.check_code(),
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
}
