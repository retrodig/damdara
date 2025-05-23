use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Default, Debug, Clone)]
pub struct Flags {
    pub has_dragon_scale: bool,    // りゅうのうろこを装備したか
    pub has_warrior_ring: bool,    // せんしのゆびわを装備したか
    pub has_cursed_necklace: bool, // しのくびかざりを入手したか
    pub defeated_dragon: bool,     // ドラゴンを倒したか
    pub defeated_golem: bool,      // ゴーレムを倒したか
}

impl FromStr for Flags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err("フラグは5文字の01文字列で指定してください".into());
        }

        let chars: Vec<char> = s.chars().collect();
        Ok(Self {
            has_dragon_scale: chars[0] == '1',
            has_warrior_ring: chars[1] == '1',
            has_cursed_necklace: chars[2] == '1',
            defeated_dragon: chars[3] == '1',
            defeated_golem: chars[4] == '1',
        })
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // "1" or "0" を順番に並べて文字列化
        write!(
            f,
            "{}{}{}{}{}",
            self.has_dragon_scale as u8,
            self.has_warrior_ring as u8,
            self.has_cursed_necklace as u8,
            self.defeated_dragon as u8,
            self.defeated_golem as u8,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Status {
    pub level: u8,
    pub strength: u8,
    pub agility: u8,
    pub max_hp: u8,
    pub max_mp: u8,
    pub required_exp: u16,
    pub spell: Option<&'static str>,
}

#[derive(Debug)]
pub struct PlayerSummary {
    pub name: String,
    pub level: u8,
    pub hp: u8,
    pub mp: u8,
    pub gold: u16,
    pub experience: u16,
}

#[derive(Debug)]
pub struct StrengthStatus {
    pub level: u8,
    pub strength: u8,
    pub agility: u8,
    pub max_hp: u8,
    pub max_mp: u8,
    pub attack_power: u8,
    pub defense_power: u8,
    pub weapon: String,
    pub armor: String,
    pub shield: String,
}

pub static DEFAULT_STATUS: Status = Status {
    level: 1,
    strength: 0,
    agility: 0,
    max_hp: 0,
    max_mp: 0,
    required_exp: 0,
    spell: None,
};

pub const STATUS_TABLE: [Status; 30] = [
    Status {
        level: 1,
        strength: 4,
        agility: 4,
        max_hp: 15,
        max_mp: 0,
        required_exp: 0,
        spell: None,
    },
    Status {
        level: 2,
        strength: 5,
        agility: 4,
        max_hp: 22,
        max_mp: 0,
        required_exp: 7,
        spell: None,
    },
    Status {
        level: 3,
        strength: 7,
        agility: 6,
        max_hp: 24,
        max_mp: 5,
        required_exp: 23,
        spell: Some("ホイミ"),
    },
    Status {
        level: 4,
        strength: 7,
        agility: 8,
        max_hp: 31,
        max_mp: 16,
        required_exp: 47,
        spell: Some("ギラ"),
    },
    Status {
        level: 5,
        strength: 12,
        agility: 10,
        max_hp: 35,
        max_mp: 20,
        required_exp: 110,
        spell: None,
    },
    Status {
        level: 6,
        strength: 16,
        agility: 10,
        max_hp: 38,
        max_mp: 24,
        required_exp: 220,
        spell: None,
    },
    Status {
        level: 7,
        strength: 18,
        agility: 17,
        max_hp: 40,
        max_mp: 26,
        required_exp: 450,
        spell: Some("ラリホー"),
    },
    Status {
        level: 8,
        strength: 22,
        agility: 20,
        max_hp: 46,
        max_mp: 29,
        required_exp: 800,
        spell: None,
    },
    Status {
        level: 9,
        strength: 30,
        agility: 22,
        max_hp: 50,
        max_mp: 36,
        required_exp: 1300,
        spell: Some("レミーラ"),
    },
    Status {
        level: 10,
        strength: 35,
        agility: 31,
        max_hp: 54,
        max_mp: 40,
        required_exp: 2000,
        spell: Some("マホトーン"),
    },
    Status {
        level: 11,
        strength: 40,
        agility: 35,
        max_hp: 62,
        max_mp: 50,
        required_exp: 2900,
        spell: None,
    },
    Status {
        level: 12,
        strength: 48,
        agility: 40,
        max_hp: 63,
        max_mp: 58,
        required_exp: 4000,
        spell: Some("リレミト"),
    },
    Status {
        level: 13,
        strength: 52,
        agility: 48,
        max_hp: 70,
        max_mp: 64,
        required_exp: 5500,
        spell: Some("ルーラ"),
    },
    Status {
        level: 14,
        strength: 60,
        agility: 55,
        max_hp: 78,
        max_mp: 70,
        required_exp: 7500,
        spell: None,
    },
    Status {
        level: 15,
        strength: 68,
        agility: 64,
        max_hp: 86,
        max_mp: 72,
        required_exp: 10000,
        spell: Some("トヘロス"),
    },
    Status {
        level: 16,
        strength: 72,
        agility: 70,
        max_hp: 92,
        max_mp: 95,
        required_exp: 13000,
        spell: None,
    },
    Status {
        level: 17,
        strength: 72,
        agility: 78,
        max_hp: 100,
        max_mp: 100,
        required_exp: 17000,
        spell: Some("ベホイミ"),
    },
    Status {
        level: 18,
        strength: 85,
        agility: 84,
        max_hp: 115,
        max_mp: 108,
        required_exp: 21000,
        spell: None,
    },
    Status {
        level: 19,
        strength: 87,
        agility: 86,
        max_hp: 130,
        max_mp: 115,
        required_exp: 25000,
        spell: Some("ベギラマ"),
    },
    Status {
        level: 20,
        strength: 92,
        agility: 88,
        max_hp: 138,
        max_mp: 128,
        required_exp: 29000,
        spell: None,
    },
    Status {
        level: 21,
        strength: 95,
        agility: 90,
        max_hp: 149,
        max_mp: 135,
        required_exp: 33000,
        spell: None,
    },
    Status {
        level: 22,
        strength: 97,
        agility: 90,
        max_hp: 158,
        max_mp: 146,
        required_exp: 37000,
        spell: None,
    },
    Status {
        level: 23,
        strength: 99,
        agility: 94,
        max_hp: 165,
        max_mp: 153,
        required_exp: 41000,
        spell: None,
    },
    Status {
        level: 24,
        strength: 103,
        agility: 98,
        max_hp: 170,
        max_mp: 161,
        required_exp: 45000,
        spell: None,
    },
    Status {
        level: 25,
        strength: 113,
        agility: 100,
        max_hp: 174,
        max_mp: 161,
        required_exp: 49000,
        spell: None,
    },
    Status {
        level: 26,
        strength: 117,
        agility: 105,
        max_hp: 180,
        max_mp: 168,
        required_exp: 53000,
        spell: None,
    },
    Status {
        level: 27,
        strength: 125,
        agility: 107,
        max_hp: 189,
        max_mp: 175,
        required_exp: 57000,
        spell: None,
    },
    Status {
        level: 28,
        strength: 130,
        agility: 115,
        max_hp: 195,
        max_mp: 180,
        required_exp: 61000,
        spell: None,
    },
    Status {
        level: 29,
        strength: 135,
        agility: 120,
        max_hp: 200,
        max_mp: 190,
        required_exp: 65000,
        spell: None,
    },
    Status {
        level: 30,
        strength: 140,
        agility: 130,
        max_hp: 210,
        max_mp: 200,
        required_exp: 65535,
        spell: None,
    },
];
