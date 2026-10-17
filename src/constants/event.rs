// セリフ番号は DIALOGUE_MASTER の添字。品物は ITEM_MASTER、武器は WEAPON_MASTER、敵は MONSTER_MASTER の添字。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    PrincessRescued,  // ローラ姫を助けた (抱いている、または王様に送り届けた)
    CarryingPrincess, // ローラ姫を抱いている
    LeftThroneRoom,   // 王の間を一度でも出た
    HasItem(u8),
    HasAnyItem(&'static [u8]),
    HasAllItems(&'static [u8]),
    WeaponEquipped(u8),
    RingEquipped,  // せんしのゆびわ を持っていて装備している
    Cursed,        // のろいのベルト か しのくびかざり を身につけている
    InventoryFull, // 持ち物がいっぱいで受け取れない
    MaxLevel,      // レベル 30
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    RemoveCurses,                     // のろいのベルト・しのくびかざり を外して捨てる
    RestoreMp,                        // MP を最大まで回復
    Flash,                            // 画面が光る演出
    TakeItem(u8),                     // 持ち物から取り除く
    GiveItem(u8),                     // 持ち物に加える
    PrincessTakesItem(&'static [u8]), // 候補を先頭から探し、持っている最初の 1 つを渡す ({word} はその名前)
    NpcDisappears,                    // 話しかけた人が消える (後ろの宝箱が取れるようになる)
    LeaveMap,                         // 建物の外 (フィールド) へ出される
    DeliverPrincess,                  // 姫を王様に送り届ける (以後、王の間に姫が立つ)
    TellPassword,                     // ふっかつのじゅもん を表示
    ExpToNextLevel,                   // 次のレベルまでの経験値を {number} に入れる
    ResetPlayer,                      // レベル・ゴールド・持ち物・装備などを初期化
    GameOver,                         // ゲームが止まる
    Battle(u8),                       // 戦闘になる
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Say(u16),
    Ask {
        dialogue_id: u16, // このセリフの後に はい / いいえ
        yes: &'static [Step],
        no: &'static [Step],
    },
    AskUntilYes {
        dialogue_id: u16, // いいえ なら no の後にもう一度聞く
        yes: &'static [Step],
        no: &'static [Step],
    },
    If {
        condition: Condition,
        then: &'static [Step],
        otherwise: &'static [Step],
    },
    Random(&'static [(u8, &'static [Step])]), // (重み, 流れ)
    Do(Action),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventScript {
    pub talk_id: u8,
    pub steps: &'static [Step],
}

pub fn find_event_script(talk_id: u8) -> Option<&'static EventScript> {
    EVENT_SCRIPT_MASTER.iter().find(|e| e.talk_id == talk_id)
}

// 竜王を倒した後は talk_id に関係なくこちらになる
pub fn dragonlord_defeated_steps(map_id: u8, talk_id: u8) -> &'static [Step] {
    match (map_id, talk_id) {
        (4, _) => &[Step::Say(289)],
        (_, 100) => &[Step::Say(277)],
        _ => &[Step::Random(&[
            (1, &[Step::Say(287)]),
            (1, &[Step::Say(288)]),
        ])],
    }
}

pub const EVENT_SCRIPT_MASTER: [EventScript; 22] = [
    EventScript {
        talk_id: 26,
        steps: &[Step::Say(73), Step::Say(176)],
    },
    EventScript {
        talk_id: 35,
        steps: &[Step::If {
            condition: Condition::LeftThroneRoom,
            then: &[Step::Say(82)],
            otherwise: &[Step::Say(257)],
        }],
    },
    EventScript {
        talk_id: 36,
        steps: &[Step::If {
            condition: Condition::LeftThroneRoom,
            then: &[Step::Say(83)],
            otherwise: &[Step::Say(256)],
        }],
    },
    EventScript {
        talk_id: 98,
        steps: &[Step::If {
            condition: Condition::PrincessRescued,
            then: &[Step::Say(156)],
            otherwise: &[Step::Say(155)],
        }],
    },
    EventScript {
        talk_id: 99,
        steps: &[Step::If {
            condition: Condition::PrincessRescued,
            then: &[Step::Say(157)],
            otherwise: &[Step::Say(155)],
        }],
    },
    EventScript {
        talk_id: 100,
        steps: &[Step::If {
            condition: Condition::PrincessRescued,
            then: &[Step::Say(159)],
            otherwise: &[Step::Say(158)],
        }],
    },
    EventScript {
        talk_id: 101,
        steps: &[Step::If {
            condition: Condition::PrincessRescued,
            then: &[Step::Say(163)],
            otherwise: &[
                Step::Ask {
                    dialogue_id: 160,
                    yes: &[],
                    no: &[Step::Say(161)],
                },
                Step::Say(162),
            ],
        }],
    },
    EventScript {
        talk_id: 102,
        steps: &[Step::If {
            condition: Condition::HasAnyItem(&[12, 14]),
            then: &[Step::Say(165)],
            otherwise: &[Step::Say(164), Step::Say(198)],
        }],
    },
    EventScript {
        talk_id: 103,
        steps: &[Step::If {
            condition: Condition::Cursed,
            then: &[
                Step::Say(167),
                Step::Do(Action::RemoveCurses),
                Step::Say(168),
            ],
            otherwise: &[Step::Say(166)],
        }],
    },
    EventScript {
        talk_id: 104,
        steps: &[Step::If {
            condition: Condition::WeaponEquipped(7),
            then: &[Step::Say(170)],
            otherwise: &[Step::Say(169)],
        }],
    },
    EventScript {
        talk_id: 105,
        steps: &[Step::If {
            condition: Condition::RingEquipped,
            then: &[Step::Say(171)],
            otherwise: &[Step::Say(172)],
        }],
    },
    EventScript {
        talk_id: 106,
        steps: &[
            Step::Say(173),
            Step::Do(Action::Flash),
            Step::Do(Action::RestoreMp),
        ],
    },
    EventScript {
        talk_id: 107,
        steps: &[Step::Say(76), Step::Say(174), Step::Say(175)],
    },
    EventScript {
        talk_id: 108,
        steps: &[Step::If {
            condition: Condition::HasAnyItem(&[13, 14]),
            then: &[Step::Say(165)],
            otherwise: &[Step::If {
                condition: Condition::HasItem(10),
                then: &[
                    Step::Say(178),
                    Step::Say(164),
                    Step::Say(198),
                    Step::Do(Action::TakeItem(10)),
                    Step::Do(Action::NpcDisappears),
                ],
                otherwise: &[Step::Say(177)],
            }],
        }],
    },
    EventScript {
        talk_id: 109,
        steps: &[Step::If {
            condition: Condition::HasItem(14),
            then: &[Step::Say(165)],
            otherwise: &[Step::If {
                condition: Condition::HasItem(7),
                then: &[Step::If {
                    condition: Condition::HasAllItems(&[12, 13]),
                    then: &[
                        Step::Say(180),
                        Step::Do(Action::TakeItem(12)),
                        Step::Do(Action::TakeItem(13)),
                        Step::Do(Action::GiveItem(14)),
                        Step::Do(Action::Flash),
                    ],
                    otherwise: &[Step::Say(73), Step::Say(174)],
                }],
                otherwise: &[
                    Step::Say(179),
                    Step::Do(Action::Flash),
                    Step::Do(Action::LeaveMap),
                ],
            }],
        }],
    },
    EventScript {
        talk_id: 110,
        steps: &[Step::If {
            condition: Condition::CarryingPrincess,
            then: &[
                Step::Say(185),
                Step::If {
                    condition: Condition::InventoryFull,
                    then: &[Step::If {
                        condition: Condition::HasAnyItem(&[1, 4, 6, 2, 3, 9, 13]),
                        then: &[
                            Step::Do(Action::PrincessTakesItem(&[1, 4, 6, 2, 3, 9, 13])),
                            Step::Do(Action::GiveItem(8)),
                            Step::Say(186),
                        ],
                        otherwise: &[],
                    }],
                    otherwise: &[Step::Do(Action::GiveItem(8))],
                },
                Step::Say(187),
                Step::Say(188),
                Step::Do(Action::DeliverPrincess),
                Step::Say(194),
                Step::Do(Action::TellPassword),
                Step::Say(195),
                Step::Say(196),
            ],
            otherwise: &[Step::If {
                condition: Condition::LeftThroneRoom,
                then: &[
                    Step::Say(192),
                    Step::If {
                        condition: Condition::MaxLevel,
                        then: &[Step::Say(2)],
                        otherwise: &[Step::Do(Action::ExpToNextLevel), Step::Say(193)],
                    },
                    Step::Say(194),
                    Step::Do(Action::TellPassword),
                    Step::Say(195),
                    Step::Say(196),
                ],
                otherwise: &[Step::Say(191)],
            }],
        }],
    },
    EventScript {
        talk_id: 111,
        steps: &[Step::Random(&[
            (1, &[Step::Say(187)]),
            (1, &[Step::Say(189)]),
            (
                2,
                &[Step::AskUntilYes {
                    dialogue_id: 190,
                    yes: &[Step::Say(184)],
                    no: &[Step::Say(182)],
                }],
            ),
        ])],
    },
    EventScript {
        talk_id: 112,
        steps: &[
            Step::Say(199),
            Step::Say(164),
            Step::Ask {
                dialogue_id: 200,
                yes: &[Step::Ask {
                    dialogue_id: 278,
                    yes: &[
                        Step::Say(202),
                        Step::Do(Action::ResetPlayer),
                        Step::Say(194),
                        Step::Do(Action::TellPassword),
                        Step::Say(195),
                        Step::Say(203),
                        Step::Do(Action::GameOver),
                    ],
                    no: &[Step::Say(201), Step::Do(Action::Battle(38))],
                }],
                no: &[Step::Say(201), Step::Do(Action::Battle(38))],
            },
        ],
    },
    EventScript {
        talk_id: 113,
        steps: &[Step::Say(3)],
    },
    EventScript {
        talk_id: 114,
        steps: &[Step::Say(145)],
    },
    EventScript {
        talk_id: 253,
        steps: &[Step::Say(289)],
    },
    EventScript {
        talk_id: 254,
        steps: &[Step::Say(289)],
    },
];
