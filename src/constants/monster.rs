use crate::constants::spell::{Spell, SpellResistance};

#[derive(Debug, Clone)]
pub struct Monster {
    pub stats: MonsterStats,
    pub behavior: MonsterBehavior,
}

impl Monster {
    pub fn new(index: usize) -> Self {
        let stats = MONSTER_MASTER.get(index).unwrap_or(&MONSTER_MASTER[0]);
        let behavior = MONSTER_BEHAVIORS
            .get(index)
            .unwrap_or(&MONSTER_BEHAVIORS[0]);

        Self {
            stats: stats.clone(),
            behavior: behavior.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonsterStats {
    pub name: &'static str,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub exp: u8,
    pub gold: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Spell(Spell),
    Special(&'static str),
}

#[derive(Debug, Clone)]
pub struct MonsterAction {
    pub ab_type: &'static str,
    pub action: ActionType,
    pub rate: u8, // 発動率（0, 25, 50, 75%）
}

#[derive(Debug, Clone)]
pub struct MonsterBehavior {
    pub index: usize,
    pub resist: SpellResistance,
    pub evade_rate: f32,
    pub actions: &'static [MonsterAction],
}

pub const MONSTER_MASTER: [MonsterStats; 40] = [
    MonsterStats {
        name: "スライム",
        hp: 3,
        attack: 5,
        defense: 3,
        exp: 1,
        gold: 2,
    },
    MonsterStats {
        name: "スライムベス",
        hp: 4,
        attack: 7,
        defense: 3,
        exp: 1,
        gold: 2,
    },
    MonsterStats {
        name: "ドラキー",
        hp: 6,
        attack: 9,
        defense: 6,
        exp: 2,
        gold: 2,
    },
    MonsterStats {
        name: "ゴースト",
        hp: 7,
        attack: 11,
        defense: 8,
        exp: 3,
        gold: 4,
    },
    MonsterStats {
        name: "まほうつかい",
        hp: 13,
        attack: 11,
        defense: 12,
        exp: 4,
        gold: 11,
    },
    MonsterStats {
        name: "メイジドラキー",
        hp: 15,
        attack: 14,
        defense: 14,
        exp: 5,
        gold: 11,
    },
    MonsterStats {
        name: "おおさそり",
        hp: 20,
        attack: 18,
        defense: 16,
        exp: 6,
        gold: 15,
    },
    MonsterStats {
        name: "がいこつ",
        hp: 30,
        attack: 28,
        defense: 22,
        exp: 11,
        gold: 29,
    },
    MonsterStats {
        name: "メーダ",
        hp: 22,
        attack: 20,
        defense: 18,
        exp: 7,
        gold: 15,
    },
    MonsterStats {
        name: "メトロゴースト",
        hp: 23,
        attack: 18,
        defense: 20,
        exp: 8,
        gold: 17,
    },
    MonsterStats {
        name: "ドロル",
        hp: 25,
        attack: 24,
        defense: 24,
        exp: 10,
        gold: 24,
    },
    MonsterStats {
        name: "ドラキーマ",
        hp: 20,
        attack: 22,
        defense: 26,
        exp: 11,
        gold: 19,
    },
    MonsterStats {
        name: "まどうし",
        hp: 30,
        attack: 28,
        defense: 22,
        exp: 13,
        gold: 34,
    },
    MonsterStats {
        name: "てつのさそり",
        hp: 22,
        attack: 36,
        defense: 42,
        exp: 14,
        gold: 39,
    },
    MonsterStats {
        name: "リカント",
        hp: 34,
        attack: 40,
        defense: 30,
        exp: 16,
        gold: 49,
    },
    MonsterStats {
        name: "しりょう",
        hp: 36,
        attack: 44,
        defense: 34,
        exp: 17,
        gold: 59,
    },
    MonsterStats {
        name: "リカントマムル",
        hp: 38,
        attack: 50,
        defense: 36,
        exp: 20,
        gold: 79,
    },
    MonsterStats {
        name: "キメラ",
        hp: 42,
        attack: 56,
        defense: 48,
        exp: 24,
        gold: 99,
    },
    MonsterStats {
        name: "ゴールドマン",
        hp: 50,
        attack: 48,
        defense: 40,
        exp: 6,
        gold: 199,
    },
    MonsterStats {
        name: "ヘルゴースト",
        hp: 36,
        attack: 40,
        defense: 38,
        exp: 18,
        gold: 69,
    },
    MonsterStats {
        name: "メーダロード",
        hp: 35,
        attack: 47,
        defense: 40,
        exp: 20,
        gold: 84,
    },
    MonsterStats {
        name: "ドロルメイジ",
        hp: 38,
        attack: 52,
        defense: 50,
        exp: 22,
        gold: 89,
    },
    MonsterStats {
        name: "しりょうのきし",
        hp: 46,
        attack: 68,
        defense: 56,
        exp: 28,
        gold: 119,
    },
    MonsterStats {
        name: "しのさそり",
        hp: 35,
        attack: 60,
        defense: 90,
        exp: 26,
        gold: 109,
    },
    MonsterStats {
        name: "よろいのきし",
        hp: 55,
        attack: 76,
        defense: 78,
        exp: 33,
        gold: 129,
    },
    MonsterStats {
        name: "かげのきし",
        hp: 50,
        attack: 79,
        defense: 64,
        exp: 37,
        gold: 149,
    },
    MonsterStats {
        name: "メイジキメラ",
        hp: 58,
        attack: 78,
        defense: 68,
        exp: 34,
        gold: 139,
    },
    MonsterStats {
        name: "メタルスライム",
        hp: 4,
        attack: 10,
        defense: 255,
        exp: 115,
        gold: 5,
    },
    MonsterStats {
        name: "キラーリカント",
        hp: 60,
        attack: 86,
        defense: 70,
        exp: 40,
        gold: 154,
    },
    MonsterStats {
        name: "スターキメラ",
        hp: 65,
        attack: 86,
        defense: 80,
        exp: 43,
        gold: 159,
    },
    MonsterStats {
        name: "ドラゴン",
        hp: 65,
        attack: 88,
        defense: 74,
        exp: 45,
        gold: 159,
    },
    MonsterStats {
        name: "だいまどう",
        hp: 65,
        attack: 80,
        defense: 70,
        exp: 50,
        gold: 164,
    },
    MonsterStats {
        name: "ゴーレム",
        hp: 70,
        attack: 120,
        defense: 60,
        exp: 5,
        gold: 9,
    },
    MonsterStats {
        name: "あくまのきし",
        hp: 70,
        attack: 94,
        defense: 82,
        exp: 54,
        gold: 164,
    },
    MonsterStats {
        name: "キースドラゴン",
        hp: 70,
        attack: 98,
        defense: 84,
        exp: 60,
        gold: 149,
    },
    MonsterStats {
        name: "ストーンマン",
        hp: 160,
        attack: 100,
        defense: 40,
        exp: 65,
        gold: 139,
    },
    MonsterStats {
        name: "しにがみのきし",
        hp: 90,
        attack: 105,
        defense: 86,
        exp: 70,
        gold: 139,
    },
    MonsterStats {
        name: "ダースドラゴン",
        hp: 100,
        attack: 120,
        defense: 90,
        exp: 100,
        gold: 139,
    },
    MonsterStats {
        name: "りゅうおう",
        hp: 100,
        attack: 90,
        defense: 75,
        exp: 0,
        gold: 0,
    },
    MonsterStats {
        name: "りゅうおう",
        hp: 130,
        attack: 140,
        defense: 200,
        exp: 0,
        gold: 0,
    },
];

pub static MONSTER_BEHAVIORS: [MonsterBehavior; 40] = [
    MonsterBehavior {
        index: 0,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 94,
        },
        evade_rate: 1.6,
        actions: &[],
    },
    MonsterBehavior {
        index: 1,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 94,
        },
        evade_rate: 1.6,
        actions: &[],
    },
    MonsterBehavior {
        index: 2,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 94,
        },
        evade_rate: 1.6,
        actions: &[],
    },
    MonsterBehavior {
        index: 3,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 94,
        },
        evade_rate: 6.2,
        actions: &[],
    },
    MonsterBehavior {
        index: 4,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 0,
        },
        evade_rate: 1.6,
        actions: &[MonsterAction {
            ab_type: "B",
            action: ActionType::Spell(Spell::Gira),
            rate: 50,
        }],
    },
    MonsterBehavior {
        index: 5,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 0,
        },
        evade_rate: 1.6,
        actions: &[MonsterAction {
            ab_type: "B",
            action: ActionType::Spell(Spell::Gira),
            rate: 50,
        }],
    },
    MonsterBehavior {
        index: 6,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 94,
        },
        evade_rate: 1.6,
        actions: &[],
    },
    MonsterBehavior {
        index: 7,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 94,
        },
        evade_rate: 6.2,
        actions: &[],
    },
    MonsterBehavior {
        index: 8,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 94,
        },
        evade_rate: 3.1,
        actions: &[],
    },
    MonsterBehavior {
        index: 9,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 0,
        },
        evade_rate: 9.4,
        actions: &[MonsterAction {
            ab_type: "B",
            action: ActionType::Spell(Spell::Gira),
            rate: 75,
        }],
    },
    MonsterBehavior {
        index: 10,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 88,
        },
        evade_rate: 3.1,
        actions: &[],
    },
    MonsterBehavior {
        index: 11,
        resist: SpellResistance {
            gira: 0,
            rariho: 12,
            mahoton: 0,
        },
        evade_rate: 9.4,
        actions: &[
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Hoimi),
                rate: 25,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Spell(Spell::Gira),
                rate: 50,
            },
        ],
    },
    MonsterBehavior {
        index: 12,
        resist: SpellResistance {
            gira: 0,
            rariho: 19,
            mahoton: 6,
        },
        evade_rate: 3.1,
        actions: &[
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Rarirho),
                rate: 25,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Spell(Spell::Gira),
                rate: 50,
            },
        ],
    },
    MonsterBehavior {
        index: 13,
        resist: SpellResistance {
            gira: 0,
            rariho: 0,
            mahoton: 94,
        },
        evade_rate: 3.1,
        actions: &[],
    },
    MonsterBehavior {
        index: 14,
        resist: SpellResistance {
            gira: 0,
            rariho: 6,
            mahoton: 94,
        },
        evade_rate: 3.1,
        actions: &[],
    },
    MonsterBehavior {
        index: 15,
        resist: SpellResistance {
            gira: 0,
            rariho: 44,
            mahoton: 0,
        },
        evade_rate: 6.2,
        actions: &[MonsterAction {
            ab_type: "A",
            action: ActionType::Spell(Spell::Hoimi),
            rate: 25,
        }],
    },
    MonsterBehavior {
        index: 16,
        resist: SpellResistance {
            gira: 0,
            rariho: 25,
            mahoton: 44,
        },
        evade_rate: 3.1,
        actions: &[MonsterAction {
            ab_type: "A",
            action: ActionType::Spell(Spell::Mahoton),
            rate: 50,
        }],
    },
    MonsterBehavior {
        index: 17,
        resist: SpellResistance {
            gira: 0,
            rariho: 25,
            mahoton: 94,
        },
        evade_rate: 3.1,
        actions: &[],
    },
    MonsterBehavior {
        index: 18,
        resist: SpellResistance {
            gira: 0,
            rariho: 81,
            mahoton: 94,
        },
        evade_rate: 1.6,
        actions: &[],
    },
    MonsterBehavior {
        index: 19,
        resist: SpellResistance {
            gira: 0,
            rariho: 19,
            mahoton: 6,
        },
        evade_rate: 6.2,
        actions: &[
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Rarirho),
                rate: 25,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Spell(Spell::Gira),
                rate: 75,
            },
        ],
    },
    MonsterBehavior {
        index: 20,
        resist: SpellResistance {
            gira: 0,
            rariho: 94,
            mahoton: 0,
        },
        evade_rate: 6.2,
        actions: &[
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Hoimi),
                rate: 75,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Spell(Spell::Gira),
                rate: 25,
            },
        ],
    },
    MonsterBehavior {
        index: 21,
        resist: SpellResistance {
            gira: 0,
            rariho: 12,
            mahoton: 12,
        },
        evade_rate: 1.6,
        actions: &[MonsterAction {
            ab_type: "A",
            action: ActionType::Spell(Spell::Mahoton),
            rate: 50,
        }],
    },
    MonsterBehavior {
        index: 22,
        resist: SpellResistance {
            gira: 19,
            rariho: 31,
            mahoton: 0,
        },
        evade_rate: 6.2,
        actions: &[MonsterAction {
            ab_type: "A",
            action: ActionType::Spell(Spell::Hoimi),
            rate: 75,
        }],
    },
    MonsterBehavior {
        index: 23,
        resist: SpellResistance {
            gira: 0,
            rariho: 44,
            mahoton: 94,
        },
        evade_rate: 3.1,
        actions: &[],
    },
    MonsterBehavior {
        index: 24,
        resist: SpellResistance {
            gira: 0,
            rariho: 38,
            mahoton: 44,
        },
        evade_rate: 1.6,
        actions: &[MonsterAction {
            ab_type: "A",
            action: ActionType::Spell(Spell::Mahoton),
            rate: 50,
        }],
    },
    MonsterBehavior {
        index: 25,
        resist: SpellResistance {
            gira: 94,
            rariho: 94,
            mahoton: 94,
        },
        evade_rate: 23.4,
        actions: &[],
    },
    MonsterBehavior {
        index: 26,
        resist: SpellResistance {
            gira: 0,
            rariho: 12,
            mahoton: 0,
        },
        evade_rate: 3.1,
        actions: &[MonsterAction {
            ab_type: "A",
            action: ActionType::Spell(Spell::Rarirho),
            rate: 50,
        }],
    },
    MonsterBehavior {
        index: 27,
        resist: SpellResistance {
            gira: 94,
            rariho: 94,
            mahoton: 94,
        },
        evade_rate: 1.6,
        actions: &[MonsterAction {
            ab_type: "B",
            action: ActionType::Spell(Spell::Gira),
            rate: 75,
        }],
    },
    MonsterBehavior {
        index: 28,
        resist: SpellResistance {
            gira: 0,
            rariho: 44,
            mahoton: 94,
        },
        evade_rate: 10.9,
        actions: &[],
    },
    MonsterBehavior {
        index: 29,
        resist: SpellResistance {
            gira: 6,
            rariho: 50,
            mahoton: 0,
        },
        evade_rate: 3.1,
        actions: &[
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Behoimi),
                rate: 75,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Special("ほのお(弱)"),
                rate: 25,
            },
        ],
    },
    MonsterBehavior {
        index: 30,
        resist: SpellResistance {
            gira: 12,
            rariho: 44,
            mahoton: 94,
        },
        evade_rate: 3.1,
        actions: &[MonsterAction {
            ab_type: "B",
            action: ActionType::Special("ほのお(弱)"),
            rate: 25,
        }],
    },
    MonsterBehavior {
        index: 31,
        resist: SpellResistance {
            gira: 94,
            rariho: 94,
            mahoton: 44,
        },
        evade_rate: 3.1,
        actions: &[MonsterAction {
            ab_type: "B",
            action: ActionType::Spell(Spell::Begirama),
            rate: 50,
        }],
    },
    MonsterBehavior {
        index: 32,
        resist: SpellResistance {
            gira: 94,
            rariho: 94,
            mahoton: 94,
        },
        evade_rate: 0.0,
        actions: &[],
    },
    MonsterBehavior {
        index: 33,
        resist: SpellResistance {
            gira: 6,
            rariho: 94,
            mahoton: 19,
        },
        evade_rate: 1.6,
        actions: &[MonsterAction {
            ab_type: "A",
            action: ActionType::Spell(Spell::Rarirho),
            rate: 25,
        }],
    },
    MonsterBehavior {
        index: 34,
        resist: SpellResistance {
            gira: 44,
            rariho: 94,
            mahoton: 94,
        },
        evade_rate: 3.1,
        actions: &[MonsterAction {
            ab_type: "B",
            action: ActionType::Special("ほのお(弱)"),
            rate: 25,
        }],
    },
    MonsterBehavior {
        index: 35,
        resist: SpellResistance {
            gira: 44,
            rariho: 12,
            mahoton: 94,
        },
        evade_rate: 1.6,
        actions: &[],
    },
    MonsterBehavior {
        index: 36,
        resist: SpellResistance {
            gira: 6,
            rariho: 94,
            mahoton: 44,
        },
        evade_rate: 3.1,
        actions: &[
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Behoimi),
                rate: 75,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Spell(Spell::Begirama),
                rate: 25,
            },
        ],
    },
    MonsterBehavior {
        index: 37,
        resist: SpellResistance {
            gira: 94,
            rariho: 94,
            mahoton: 44,
        },
        evade_rate: 3.1,
        actions: &[
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Rarirho),
                rate: 25,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Special("ほのお(弱)"),
                rate: 25,
            },
        ],
    },
    MonsterBehavior {
        index: 38,
        resist: SpellResistance {
            gira: 94,
            rariho: 94,
            mahoton: 94,
        },
        evade_rate: 0.0,
        actions: &[
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Mahoton),
                rate: 25,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Spell(Spell::Begirama),
                rate: 75,
            },
        ],
    },
    MonsterBehavior {
        index: 39,
        resist: SpellResistance {
            gira: 94,
            rariho: 94,
            mahoton: 94,
        },
        evade_rate: 0.0,
        actions: &[MonsterAction {
            ab_type: "B",
            action: ActionType::Special("ほのお(強)"),
            rate: 50,
        }],
    },
];
