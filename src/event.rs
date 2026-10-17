use crate::constants::event::{
    Action, Condition, Step, dragonlord_defeated_steps, find_event_script,
};
use crate::constants::item_weapon::ITEM_MASTER;
use crate::constants::status::STATUS_TABLE;
use crate::constants::talk::{TalkKind, find_talk};
use crate::player::Player;
use crate::utility::dialogue_utils::get_dialogue;
use serde::{Deserialize, Serialize};

const RING: u8 = 6; // せんしのゆびわ
const CURSED_BELT: u8 = 9; // のろいのベルト
const CURSED_NECKLACE: u8 = 11; // しのくびかざり
const MAX_LEVEL: u8 = 30;

/// ふっかつのじゅもんに残らない、物語の進み具合
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct StoryState {
    pub carrying_princess: bool,   // ローラ姫を抱いている
    pub princess_delivered: bool,  // ローラ姫を王様に送り届けた
    pub left_throne_room: bool,    // 王の間を一度でも出た
    pub dragonlord_defeated: bool, // 竜王を倒した
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventStatus {
    Finished,
    /// はい / いいえ の答えが足りず、最後のセリフで止まっている
    WaitingAnswer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventLine {
    pub dialogue_id: u16,
    pub text: String, // {player} {word} {number} を置き換え済み
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventResult {
    pub lines: Vec<EventLine>,
    pub actions: Vec<Action>,
    pub status: EventStatus,
}

/// 条件の判定に使うプレイヤーの状態。イベント中の Action はこちらに反映して、後の条件に効かせる
#[derive(Debug, Clone)]
struct Subject {
    name: String,
    exp: u16,
    weapon: u8,
    items: [u8; 8],
    ring_equipped: bool,
    cursed: bool,
    story: StoryState,
}

impl Subject {
    fn new(player: &Player, story: &StoryState) -> Self {
        Self {
            name: player.name.trim_end_matches('　').to_string(),
            exp: player.exp,
            weapon: player.weapon,
            items: player.items,
            ring_equipped: player.flags.has_warrior_ring,
            cursed: player.is_curse_belt || player.is_curse_necklace,
            story: story.clone(),
        }
    }

    fn level(&self) -> u8 {
        crate::utility::status_utils::get_level_by_exp(self.exp)
    }

    fn has_item(&self, item: u8) -> bool {
        item != 0 && self.items.contains(&item)
    }

    fn check(&self, condition: Condition) -> bool {
        match condition {
            Condition::PrincessRescued => {
                self.story.carrying_princess || self.story.princess_delivered
            }
            Condition::CarryingPrincess => self.story.carrying_princess,
            Condition::LeftThroneRoom => self.story.left_throne_room,
            Condition::HasItem(item) => self.has_item(item),
            Condition::HasAnyItem(items) => items.iter().any(|&i| self.has_item(i)),
            Condition::HasAllItems(items) => items.iter().all(|&i| self.has_item(i)),
            Condition::WeaponEquipped(weapon) => self.weapon == weapon,
            Condition::RingEquipped => self.ring_equipped && self.has_item(RING),
            Condition::Cursed => self.cursed,
            Condition::InventoryFull => !self.items.contains(&0),
            Condition::MaxLevel => self.level() >= MAX_LEVEL,
        }
    }

    fn take_item(&mut self, item: u8) {
        if let Some(slot) = self.items.iter_mut().find(|i| **i == item) {
            *slot = 0;
        }
    }

    fn give_item(&mut self, item: u8) {
        if let Some(slot) = self.items.iter_mut().find(|i| **i == 0) {
            *slot = item;
        }
    }
}

/// Condition を Player と StoryState から判定する
pub fn check_condition(condition: Condition, player: &Player, story: &StoryState) -> bool {
    Subject::new(player, story).check(condition)
}

/// 次のレベルまでの経験値。レベル 30 は 0
pub fn exp_to_next_level(exp: u16) -> u16 {
    STATUS_TABLE
        .iter()
        .find(|s| s.required_exp > exp)
        .map_or(0, |s| s.required_exp - exp)
}

struct Runner<'a> {
    subject: Subject,
    answers: std::slice::Iter<'a, bool>,
    seed: u32,
    word: String,
    number: u16,
    lines: Vec<EventLine>,
    actions: Vec<Action>,
}

/// 答えが足りずに止まったことを表す
struct Waiting;

impl Runner<'_> {
    fn say(&mut self, dialogue_id: u16) {
        let text = get_dialogue(dialogue_id as usize)
            .unwrap_or_default()
            .replace("{player}", &self.subject.name)
            .replace("{word}", &self.word)
            .replace("{number}", &self.number.to_string());
        self.lines.push(EventLine { dialogue_id, text });
    }

    fn answer(&mut self) -> Result<bool, Waiting> {
        self.answers.next().copied().ok_or(Waiting)
    }

    /// xorshift32。Random の選択に使う
    fn random(&mut self) -> u32 {
        let mut x = self.seed.max(1);
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.seed = x;
        x
    }

    fn run(&mut self, steps: &[Step]) -> Result<(), Waiting> {
        for step in steps {
            match *step {
                Step::Say(id) => self.say(id),
                Step::Ask {
                    dialogue_id,
                    yes,
                    no,
                } => {
                    self.say(dialogue_id);
                    let next = if self.answer()? { yes } else { no };
                    self.run(next)?;
                }
                Step::AskUntilYes {
                    dialogue_id,
                    yes,
                    no,
                } => loop {
                    self.say(dialogue_id);
                    if self.answer()? {
                        self.run(yes)?;
                        break;
                    }
                    self.run(no)?;
                },
                Step::If {
                    condition,
                    then,
                    otherwise,
                } => {
                    let next = if self.subject.check(condition) {
                        then
                    } else {
                        otherwise
                    };
                    self.run(next)?;
                }
                Step::Random(choices) => {
                    let total: u32 = choices.iter().map(|&(w, _)| w as u32).sum();
                    let mut pick = self.random() % total.max(1);
                    for &(weight, next) in choices {
                        if pick < weight as u32 {
                            self.run(next)?;
                            break;
                        }
                        pick -= weight as u32;
                    }
                }
                Step::Do(action) => self.act(action),
            }
        }
        Ok(())
    }

    fn act(&mut self, action: Action) {
        let subject = &mut self.subject;
        match action {
            Action::RemoveCurses => {
                subject.take_item(CURSED_BELT);
                subject.take_item(CURSED_NECKLACE);
                subject.cursed = false;
            }
            Action::TakeItem(item) => subject.take_item(item),
            Action::GiveItem(item) => subject.give_item(item),
            Action::PrincessTakesItem(candidates) => {
                if let Some(&item) = candidates.iter().find(|&&i| subject.has_item(i)) {
                    self.word = ITEM_MASTER[item as usize].name.to_string();
                    subject.take_item(item);
                }
            }
            Action::DeliverPrincess => {
                subject.story.carrying_princess = false;
                subject.story.princess_delivered = true;
            }
            Action::ExpToNextLevel => self.number = exp_to_next_level(subject.exp),
            Action::ResetPlayer => {
                subject.items = [0; 8];
                subject.weapon = 0;
                subject.cursed = false;
            }
            _ => {}
        }
        self.actions.push(action);
    }
}

/// 話しかけたときのセリフと Action の列を返す。
/// はい / いいえ は answers から順に使い、足りなければ WaitingAnswer で止まる。Random は seed で選ぶ。
/// お店・宿屋など、流れのない talk_id は None
pub fn run_event(
    talk_id: u8,
    map_id: u8,
    player: &Player,
    story: &StoryState,
    answers: &[bool],
    seed: u32,
) -> Option<EventResult> {
    let mut runner = Runner {
        subject: Subject::new(player, story),
        answers: answers.iter(),
        seed,
        word: String::new(),
        number: 0,
        lines: Vec::new(),
        actions: Vec::new(),
    };

    let result = if story.dragonlord_defeated {
        runner.run(dragonlord_defeated_steps(map_id, talk_id))
    } else if let Some(script) = find_event_script(talk_id) {
        runner.run(script.steps)
    } else {
        let talk = find_talk(talk_id)?;
        match (talk.kind, talk.dialogue_id) {
            (TalkKind::Message, Some(id)) => {
                runner.say(id);
                Ok(())
            }
            (TalkKind::Question, Some(id)) => {
                runner.say(id);
                runner.answer().map(|yes| {
                    let next = if yes {
                        talk.yes_dialogue_id
                    } else {
                        talk.no_dialogue_id
                    };
                    if let Some(next) = next {
                        runner.say(next);
                    }
                })
            }
            _ => return None,
        }
    };

    Some(EventResult {
        lines: runner.lines,
        actions: runner.actions,
        status: match result {
            Ok(()) => EventStatus::Finished,
            Err(Waiting) => EventStatus::WaitingAnswer,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::PlayerArgs;

    const KING: u8 = 110;
    const DRAGONLORD: u8 = 112;
    const RAINBOW_SAGE: u8 = 109;
    const THRONE_ROOM: u8 = 5;

    fn player(items: [u8; 8], exp: u16) -> Player {
        Player::new_with(PlayerArgs {
            name: Some("ゆうてい".to_string()),
            exp: Some(exp),
            items: Some(items),
            ..Default::default()
        })
    }

    fn ids(result: &EventResult) -> Vec<u16> {
        result.lines.iter().map(|l| l.dialogue_id).collect()
    }

    fn run(talk_id: u8, player: &Player, story: &StoryState, answers: &[bool]) -> EventResult {
        run_event(talk_id, THRONE_ROOM, player, story, answers, 1).unwrap()
    }

    #[test]
    fn test_king_with_princess() {
        let story = StoryState {
            carrying_princess: true,
            left_throne_room: true,
            ..Default::default()
        };
        let result = run(KING, &player([0; 8], 0), &story, &[]);
        assert_eq!(ids(&result), vec![185, 187, 188, 194, 195, 196]);
        assert_eq!(result.status, EventStatus::Finished);
        assert!(result.actions.contains(&Action::DeliverPrincess));
    }

    #[test]
    fn test_king_before_leaving_throne_room() {
        let result = run(KING, &player([0; 8], 0), &StoryState::default(), &[]);
        assert_eq!(ids(&result), vec![191]);
    }

    #[test]
    fn test_king_normal() {
        let story = StoryState {
            left_throne_room: true,
            ..Default::default()
        };
        let result = run(KING, &player([0; 8], 0), &story, &[]);
        assert_eq!(ids(&result), vec![192, 193, 194, 195, 196]);
        // レベル 1 → 2 は 7 ポイント
        assert!(
            result.lines[1]
                .text
                .contains(&exp_to_next_level(0).to_string())
        );
        assert!(result.lines[0].text.contains("ゆうてい"));

        let result = run(KING, &player([0; 8], 65535), &story, &[]);
        assert_eq!(ids(&result), vec![192, 2, 194, 195, 196]);
    }

    #[test]
    fn test_dragonlord() {
        let p = player([0; 8], 0);
        let story = StoryState::default();

        let result = run(DRAGONLORD, &p, &story, &[true, true]);
        assert_eq!(ids(&result), vec![199, 164, 200, 278, 202, 194, 195, 203]);
        assert_eq!(result.actions.last(), Some(&Action::GameOver));

        let result = run(DRAGONLORD, &p, &story, &[false]);
        assert_eq!(ids(&result), vec![199, 164, 200, 201]);
        assert_eq!(result.actions.last(), Some(&Action::Battle(38)));

        // 答えがないと最初の質問で止まる
        let result = run(DRAGONLORD, &p, &story, &[]);
        assert_eq!(ids(&result), vec![199, 164, 200]);
        assert_eq!(result.status, EventStatus::WaitingAnswer);
    }

    #[test]
    fn test_rainbow_sage() {
        let story = StoryState::default();

        let result = run(RAINBOW_SAGE, &player([0; 8], 0), &story, &[]);
        assert_eq!(ids(&result), vec![179]);
        assert_eq!(result.actions.last(), Some(&Action::LeaveMap));

        // ロトのしるし・たいようのいし・あまぐものつえ
        let result = run(
            RAINBOW_SAGE,
            &player([7, 12, 13, 0, 0, 0, 0, 0], 0),
            &story,
            &[],
        );
        assert_eq!(ids(&result), vec![180]);
        assert!(result.actions.contains(&Action::GiveItem(14)));
    }

    #[test]
    fn test_every_npc_talk_runs() {
        use crate::constants::map::MAP_MASTER;
        let p = player([0; 8], 0);
        for map in MAP_MASTER.iter() {
            for npc in map.npcs {
                let talk = find_talk(npc.talk_id).unwrap();
                let result = run_event(npc.talk_id, map.id, &p, &StoryState::default(), &[], 1);
                let is_shop = matches!(
                    talk.kind,
                    TalkKind::WeaponShop
                        | TalkKind::ItemShop
                        | TalkKind::KeyShop
                        | TalkKind::FairyWaterShop
                        | TalkKind::Inn
                );
                if is_shop && find_event_script(npc.talk_id).is_none() {
                    assert!(result.is_none(), "talk_id {}", npc.talk_id);
                } else {
                    let result = result.unwrap_or_else(|| panic!("talk_id {}", npc.talk_id));
                    assert!(!result.lines.is_empty(), "talk_id {}", npc.talk_id);
                }
            }
        }
    }

    #[test]
    fn test_after_dragonlord_defeated() {
        let story = StoryState {
            dragonlord_defeated: true,
            ..Default::default()
        };
        let result = run_event(KING, 4, &player([0; 8], 0), &story, &[], 1).unwrap();
        assert_eq!(ids(&result), vec![289]);
        let result = run_event(100, 8, &player([0; 8], 0), &story, &[], 1).unwrap();
        assert_eq!(ids(&result), vec![277]);
    }
}
