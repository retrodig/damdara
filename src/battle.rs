use crate::constants::monster::{ActionType, MonsterAction};
use crate::constants::spell::Spell;
use crate::monster::Monster;
use crate::player::Player;
use crate::utility::monster_utils::choose_action;
use crate::utility::random_utils::{
    check_escape_success, check_success_by_percent, get_escape_rand_max_by_monster_index,
};
use crate::utility::spell_utils::monster_action_effect;
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
    pub escaped: bool,
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
        let player_agility = self.player.agility() as u16;
        let monster_defense = self.monster.stats.defense as u16;
        check_escape_success(player_agility, monster_defense, 63)
    }

    pub fn is_escape(&self) -> bool {
        let player_agility = self.player.agility() as u16;
        let monster_defense = self.monster.stats.defense as u16;
        let index = self.monster.behavior.index;
        let rand_max = get_escape_rand_max_by_monster_index(index);
        check_escape_success(player_agility, monster_defense, rand_max)
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

    pub fn monster_turn(&mut self) {
        let action = self.decide_enemy_action();

        match action {
            EnemyAction::Attack => self.handle_enemy_normal_attack(),
            EnemyAction::Special(monster_action) => match &monster_action.action {
                ActionType::Spell(spell) => match spell {
                    Spell::Hoimi | Spell::Behoimi => {
                        self.handle_enemy_heal_spell(spell, &monster_action)
                    }
                    Spell::Gira | Spell::Begirama => {
                        self.handle_enemy_attack_spell(spell, &monster_action)
                    }
                    Spell::Rarirho => self.handle_enemy_sleep_spell(spell),
                    Spell::Mahoton => self.handle_enemy_seal_spell(spell),
                    _ => {}
                },
                ActionType::Special(name) => {
                    self.handle_enemy_special_skill(name, &monster_action);
                }
            },
            EnemyAction::Escape => {
                println!("{} は逃げ出した！", self.monster.stats.name);
                self.monster_state.escaped = true;
            }
        }
    }

    fn handle_enemy_normal_attack(&mut self) {
        let damage = self.monster.normal_damage(&self.player) as i16;
        println!(
            "{} の攻撃！{} に {}ダメージ！",
            self.monster.stats.name, self.player.name, damage
        );
        self.player.adjust_hp(-damage);
    }

    fn handle_enemy_heal_spell(&mut self, spell: &Spell, monster_action: &MonsterAction) {
        let heal = monster_action_effect(&monster_action.action);
        println!(
            "{} は {} を唱えた！自分のHPが {} 回復！",
            self.monster.stats.name,
            spell.as_str(),
            heal
        );
        self.monster.adjust_hp(heal as i16);
    }

    fn handle_enemy_attack_spell(&mut self, spell: &Spell, monster_action: &MonsterAction) {
        let damage = monster_action_effect(&monster_action.action);
        println!(
            "{} は {} を唱えた！{} に {}ダメージ！",
            self.monster.stats.name,
            spell.as_str(),
            self.player.name,
            damage
        );
        self.player.adjust_hp(-(damage as i16));
    }

    fn handle_enemy_sleep_spell(&mut self, spell: &Spell) {
        println!(
            "{} は {} を唱えた！",
            self.monster.stats.name,
            spell.as_str(),
        );
        self.player_state.sleep = true;
    }

    fn handle_enemy_seal_spell(&mut self, spell: &Spell) {
        let success = check_success_by_percent(50);
        if success && !self.player.is_max_armor() {
            println!(
                "{} は {} を唱えた！{} は呪文を封じられた！",
                self.monster.stats.name,
                spell.as_str(),
                self.player.name
            );
            self.player_state.seal = true;
        } else {
            println!(
                "{} の {} は失敗した！",
                self.monster.stats.name,
                spell.as_str(),
            );
        }
    }

    fn handle_enemy_special_skill(&mut self, name: &str, monster_action: &MonsterAction) {
        let damage = monster_action_effect(&monster_action.action);
        println!(
            "{} は {} を使った！{} に {}ダメージ！",
            self.monster.stats.name, name, self.player.name, damage
        );
        self.player.adjust_hp(-(damage as i16));
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
        // println!("Player Preemption Rate: {}%", player_first / 10);
        assert!(player_first > 700);
    }

    #[test]
    fn test_decide_enemy_action_for_all_monsters() {
        for index in 0..40 {
            let monster = Monster::new(index);
            let player = Player::new("ゆうてい");
            let battle = Battle::new(player, monster);

            let action = battle.decide_enemy_action();
            // Test that EnemyAction always returns
            match action {
                EnemyAction::Attack | EnemyAction::Escape | EnemyAction::Special(_) => {
                    // OK
                }
            }
        }
    }
}
