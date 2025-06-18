use crate::constants::monster::MonsterAction;

#[derive(Default, Debug, Clone)]
pub struct BattleState {
    pub sleep: bool,
    pub seal: bool,
    pub escaped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerAction {
    Attack,
    Spell,
    Item,
    Escape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnemyAction {
    Escape,
    Attack,
    Special(MonsterAction),
}
