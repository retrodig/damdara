#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SpellInfo {
    pub spell: Spell,
    pub learn_level: u8,
    pub mp_cost: u8,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Spell {
    Hoimi,
    Gira,
    Rarirho,
    Remira,
    Mahoton,
    Riremito,
    Rura,
    Toheros,
    Behoimi,
    Begirama,
}

impl Spell {
    pub fn as_str(&self) -> &'static str {
        match self {
            Spell::Hoimi => "ホイミ",
            Spell::Gira => "ギラ",
            Spell::Rarirho => "ラリホー",
            Spell::Remira => "レミーラ",
            Spell::Mahoton => "マホトーン",
            Spell::Riremito => "リレミト",
            Spell::Rura => "ルーラ",
            Spell::Toheros => "トヘロス",
            Spell::Behoimi => "ベホイミ",
            Spell::Begirama => "ベギラマ",
        }
    }
}

impl std::str::FromStr for Spell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ホイミ" => Ok(Spell::Hoimi),
            "ギラ" => Ok(Spell::Gira),
            "ラリホー" => Ok(Spell::Rarirho),
            "レミーラ" => Ok(Spell::Remira),
            "マホトーン" => Ok(Spell::Mahoton),
            "リレミト" => Ok(Spell::Riremito),
            "ルーラ" => Ok(Spell::Rura),
            "トヘロス" => Ok(Spell::Toheros),
            "ベホイミ" => Ok(Spell::Behoimi),
            "ベギラマ" => Ok(Spell::Begirama),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpellResistance {
    pub gira: u8,
    pub rariho: u8,
    pub mahoton: u8,
}

pub const SPELL_INFO_LIST: &[SpellInfo] = &[
    SpellInfo {
        spell: Spell::Hoimi,
        learn_level: 3,
        mp_cost: 4,
        description: "自分のHPを10～17回復",
    },
    SpellInfo {
        spell: Spell::Gira,
        learn_level: 4,
        mp_cost: 2,
        description: "戦闘で、相手に5～12の火の玉ダメージ",
    },
    SpellInfo {
        spell: Spell::Rarirho,
        learn_level: 7,
        mp_cost: 2,
        description: "戦闘で、相手を眠らせる",
    },
    SpellInfo {
        spell: Spell::Remira,
        learn_level: 9,
        mp_cost: 3,
        description: "ダンジョンにいる時、周囲7×7マスを照らす。移動するうちに照らせる範囲が小さくなり、最後は暗くなる",
    },
    SpellInfo {
        spell: Spell::Mahoton,
        learn_level: 10,
        mp_cost: 2,
        description: "戦闘で、相手の呪文を封じる",
    },
    SpellInfo {
        spell: Spell::Riremito,
        learn_level: 12,
        mp_cost: 6,
        description: "ダンジョンにいる時、ダンジョンを脱出する",
    },
    SpellInfo {
        spell: Spell::Rura,
        learn_level: 13,
        mp_cost: 8,
        description: "ダンジョンにいない時、ラダトームの城にワープ。ダンジョンにいる時、MP消費するだけ",
    },
    SpellInfo {
        spell: Spell::Toheros,
        learn_level: 15,
        mp_cost: 2,
        description: "フィールドにいる時、自分の守備力以下の力のモンスターと遭遇しなくなる。ダンジョンにいる時は遭遇する",
    },
    SpellInfo {
        spell: Spell::Behoimi,
        learn_level: 17,
        mp_cost: 10,
        description: "自分のHPを85～100回復",
    },
    SpellInfo {
        spell: Spell::Begirama,
        learn_level: 19,
        mp_cost: 5,
        description: "戦闘で、相手に58～65の炎ダメージ",
    },
];
