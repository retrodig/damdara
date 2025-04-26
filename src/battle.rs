use crate::constants::monster::{ActionType, MonsterAction};
use crate::constants::spell::Spell;
use crate::monster::Monster;
use crate::player::Player;
use crate::utility::monster_utils::choose_action;
use rand::Rng;

pub struct Battle {
    pub player: Player,
    pub monster: Monster,
    pub player_state: BattleState,
    pub monster_state: BattleState,
}

#[derive(Default, Debug, Clone)]
pub struct BattleState {
    pub sleep: bool,
    pub seal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnemyAction {
    Escape,
    Attack,
    Special(MonsterAction),
}

impl Battle {
    pub fn new(player: Player, monster: Monster) -> Self {
        Self {
            player,
            monster,
            player_state: BattleState::default(),
            monster_state: BattleState::default(),
        }
    }

    pub fn start(&mut self) {
        println!(
            "バトル開始！ {} vs {}",
            self.player.name, self.monster.stats.name
        );

        while self.player.is_alive() && self.monster.is_alive() {
            self.player_turn();
            if self.monster.is_alive() {
                self.monster_turn();
            }
        }

        if self.player.is_alive() {
            println!(
                "{} は {} を倒した！",
                self.player.name, self.monster.stats.name
            );
        } else {
            println!("{} は やられてしまった...", self.player.name);
        }
    }

    pub fn player_goes_first(&self) -> bool {
        let player_agility = self.player.agility() as u32;
        let monster_defense = self.monster.stats.defense as u32;

        let mut rng = rand::rng();
        let player_value = player_agility * rng.random_range(0..=255);
        let monster_value = monster_defense * rng.random_range(0..=63);

        player_value >= monster_value
    }

    pub fn decide_enemy_action(&self) -> EnemyAction {
        let monster_attack = self.monster.stats.attack;
        let player_strength = self.player.strength();

        if player_strength / 2 >= monster_attack {
            let mut rng = rand::rng();
            if rng.random_bool(0.25) {
                return EnemyAction::Escape;
            }
        }
        // それ以外 → 通常攻撃 or 特技選択
        self.decide_monster_support_magic_action()
    }

    pub fn decide_monster_support_magic_action(&self) -> EnemyAction {
        if self.monster.has_support_magic() {
            let candidates = self.monster.support_spells_actions();

            if let Some(monster_action) = choose_action(&candidates) {
                match monster_action.action {
                    ActionType::Spell(Spell::Hoimi) | ActionType::Spell(Spell::Behoimi)
                        if self.monster.is_low_hp() =>
                    {
                        return EnemyAction::Special(monster_action.clone());
                    }
                    ActionType::Spell(Spell::Rarirho) if !self.player_state.sleep => {
                        return EnemyAction::Special(monster_action.clone());
                    }
                    ActionType::Spell(Spell::Mahoton) if !self.player_state.seal => {
                        return EnemyAction::Special(monster_action.clone());
                    }
                    _ => {}
                }
            }
        }
        self.decide_monster_attack_magic_action()
    }

    pub fn decide_monster_attack_magic_action(&self) -> EnemyAction {
        if self.monster.has_attack_skill() {
            let candidates = self.monster.attack_spells_actions();

            if let Some(monster_action) = choose_action(&candidates) {
                match &monster_action.action {
                    ActionType::Spell(Spell::Gira)
                    | ActionType::Spell(Spell::Begirama)
                    | ActionType::Special("ほのお(弱)")
                    | ActionType::Special("ほのお(強)") => {
                        return EnemyAction::Special(monster_action.clone());
                    }
                    _ => {}
                }
            }
        }
        EnemyAction::Attack
    }

    fn player_turn(&mut self) {
        // println!("{} の攻撃！", self.player.name);
        // let damage = self.player.attack_damage();
        // println!("{} に {}ダメージ！", self.monster.stats.name, damage);
        // self.monster.take_damage(damage);
    }

    fn monster_turn(&mut self) {
        // println!("{} の攻撃！", self.monster.stats.name);
        // let damage = self.monster.attack_damage();
        // println!("{} に {}ダメージ！", self.player.name, damage);
        // self.player.take_damage(damage);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monster::Monster;
    use crate::player::{Player, PlayerArgs};

    #[test]
    fn test_real_player_high_agility() {
        let player = Player::new_with(PlayerArgs {
            name: Some("ゆうてい".to_string()),
            exp: Some(3000),
            ..Default::default()
        });
        let monster = Monster::new(0);
        let battle = Battle::new(player, monster);
        let mut player_first = 0;
        for _ in 0..1000 {
            if battle.player_goes_first() {
                player_first += 1;
            }
        }

        println!("プレイヤー先制率: {}%", player_first / 10);
        assert!(player_first > 700);
    }
}
