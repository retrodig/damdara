use crate::constants::monster::{ActionType, MonsterAction};
use crate::constants::spell::Spell;
use crate::monster::Monster;
use crate::player::Player;
use crate::utility::monster_utils::choose_action;
use crate::utility::random_utils::{
    check_escape_success, get_escape_rand_max_by_monster_index, random_success_by_percent,
    random_success_by_ratio,
};
use crate::utility::spell_utils::{monster_action_effect, player_spell_effect};
use rand::Rng;
use std::io::{self, Write};

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
        println!("{}があらわれた！", self.monster.stats.name);
        self.display_status();

        while self.is_battle_continue() {
            self.player_turn();

            if !self.player_state.escaped {
                if self.monster.is_alive() {
                    self.monster_turn();
                }
                self.display_status();
            }
        }

        if self.player_state.escaped {
            println!();
        } else if !self.monster_state.escaped {
            if self.player.is_alive() {
                let gold = self.monster.get_gold();
                println!("{} をたおした！", self.monster.name());

                println!();
                println!("けいけんち {}ポイントかくとく", self.monster.stats.exp);
                println!("{}ゴールドを てにいれた！", gold);
            } else {
                println!("あなたは しにました");
            }
        }
    }

    pub fn display_status(&self) {
        println!();
        println!(
            "{} HP: {:?} MP: {:?}",
            self.player.name, self.player.hp, self.player.mp
        );
        println!("{} HP: {:?}", self.monster.stats.name, self.monster.hp);
        println!();
    }

    pub fn display_enemy_special_skill_message(&self, name: &str, damage: u8) {
        print!(" {}は ", self.monster.stats.name);
        if name.contains("ほのお") {
            print!(" ほのおをはいた!");
        } else {
            print!(" {}を使った！", name);
        }
        println!();
        println!(" {}は {}ポイントの", self.player.name, damage);
        println!(" ダメージを うけた");
    }

    pub fn display_use_spell(&self, spell: &Spell) {
        println!("{}は {}の", self.player.name, spell.as_str());
        println!("じゅもんを となえた！");
    }

    pub fn display_monster_damage(&self, damage: u8) {
        println!("{}に {}ポイントの", self.monster.name(), damage);
        println!("ダメージを あたえた！");
    }

    pub fn display_monster_spell(&self, spell: &Spell) {
        println!(" {}は {}の", self.monster.name(), spell.as_str());
        println!(" じゅもんを となえた！");
    }

    pub fn display_monster_not_spell(&self) {
        println!("しかし じゅもんは");
        println!("ふうじこまれている！");
        return;
    }

    pub fn is_battle_continue(&self) -> bool {
        (self.player.is_alive() && !self.player_state.escaped)
            && (self.monster.is_alive() && !self.monster_state.escaped)
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

    pub fn monster_turn(&mut self) {
        if self.monster_state.sleep {
            let is_wakeup = random_success_by_percent(33.33);
            if is_wakeup {
                println!("{}は めをさました！", self.monster.name());
                self.monster_state.sleep = false;
            } else {
                println!("{}は ねむっている⋯⋯⋯", self.monster.name());
                return;
            }
        }

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
                println!("{} は にげだした！", self.monster.name());
                self.monster_state.escaped = true;
            }
        }
    }

    fn handle_enemy_normal_attack(&mut self) {
        let damage = self.monster.normal_damage(&self.player) as i16;

        if damage > 0 {
            println!(" {} のこうげき！", self.monster.stats.name);
            println!(" {}は {}ポイントの", self.player.name, damage);
            println!(" ダメージを うけた！",);

            self.player.adjust_hp(-damage);
        } else {
            println!(" {} のこうげき！", self.monster.stats.name);
            println!(" ミス");
        }
    }

    /// 敵: ホイミ、ベホイミ
    fn handle_enemy_heal_spell(&mut self, spell: &Spell, monster_action: &MonsterAction) {
        let heal = monster_action_effect(&monster_action.action);
        self.display_monster_spell(spell);

        if self.monster_state.seal {
            return self.display_monster_not_spell();
        }
        println!(" {}は きずが", self.monster.name(),);
        println!(" かいふくした！");
        self.monster.adjust_hp(heal as i16);
    }

    /// 敵: ギラ、ベギラマ
    fn handle_enemy_attack_spell(&mut self, spell: &Spell, monster_action: &MonsterAction) {
        let damage = monster_action_effect(&monster_action.action);
        self.display_monster_spell(spell);

        if self.monster_state.seal {
            return self.display_monster_not_spell();
        }
        println!(" {}は {}ポイントの", self.player.name, damage);
        println!(" ダメージを うけた！",);
        self.player.adjust_hp(-(damage as i16));
    }

    /// 敵: ラリホー（100%）
    fn handle_enemy_sleep_spell(&mut self, spell: &Spell) {
        self.display_monster_spell(spell);

        if self.monster_state.seal {
            return self.display_monster_not_spell();
        }
        println!("{}は ねむってしまった！", self.player.name);
        self.player_state.sleep = true;
    }

    /// 敵: マホトーン（50%）
    fn handle_enemy_seal_spell(&mut self, spell: &Spell) {
        self.display_monster_spell(spell);
        if self.monster_state.seal {
            return self.display_monster_not_spell();
        }

        let success = random_success_by_percent(50.0);
        if success && !self.player.is_max_armor() {
            println!("{}は じゅもんを", self.player.name);
            println!("ふうじこめられた！");
            self.player_state.seal = true;
        } else {
            println!("しかし じゅもんは きかなかった！");
        }
    }

    /// 敵: ほのお(弱)、ほのお(強)
    fn handle_enemy_special_skill(&mut self, name: &str, monster_action: &MonsterAction) {
        let mut damage = monster_action_effect(&monster_action.action);

        if name.contains("ほのお") {
            damage = self.player.reduce_fire_damage(damage);
        } else {
            damage = self.player.reduce_spell_damage(damage);
        }

        self.display_enemy_special_skill_message(name, damage);
        self.player.adjust_hp(-(damage as i16));
    }

    pub fn display_command(&self) {
        println!("\n--- {}のターン ---", self.player.name);
        println!("コマンド？");
        println!("1: たたかう");
        println!("2: じゅもん");
        println!("3: どうぐ");
        println!("4: にげる");
    }

    pub fn player_turn(&mut self) {
        if self.player_state.sleep {
            let is_wakeup = random_success_by_percent(33.33);
            if is_wakeup {
                println!("{}は めをさました！", self.player.name);
                self.player_state.sleep = false;
            } else {
                println!("{}は ねむっている⋯⋯⋯", self.player.name);
                return;
            }
        }
        self.display_command();
        // Receive input
        self.commands();
    }

    pub fn player_battle_attack_damage(&self) -> u8 {
        // 回避
        let is_evade = random_success_by_percent(self.monster.behavior.evade_rate as f64);
        if is_evade {
            return 0;
        }
        // かいしんのいちげき
        let is_critical = random_success_by_ratio(32);
        if is_critical && !self.monster.is_final_boss() {
            println!("かいしんの いちげき！！");
            self.player.critical_damage()
        } else {
            self.player.normal_damage(&self.monster)
        }
    }

    // プレイヤー: こうげき
    pub fn player_action_attack(&mut self) {
        println!("{} のこうげき！", self.player.name);
        let damage = self.player_battle_attack_damage();
        if damage == 0 {
            println!("ミス");
        } else {
            self.display_monster_damage(damage);
            self.monster.adjust_hp(-(damage as i16));
        }
    }

    // プレイヤー: じゅもん
    pub fn player_action_spell(&mut self) {
        if self.player.is_empty_spell_list() {
            println!("{}は まだ じゅもんを", self.player.name);
            println!("つかえない。");
            return self.commands_cancel();
        }

        let spell_list = self.player.spell_list();
        let spell_len = spell_list.len();

        println!("--- じゅもん ---");
        println!("0: もどる");
        for (i, spell_info) in spell_list.into_iter().enumerate() {
            println!("{}: {}", i + 1, spell_info.spell.as_str());
        }

        let spell_index = self.get_player_input(spell_len + 1);
        if spell_index == 0 {
            return self.commands_cancel();
        }
        let selected_spell = self.player.select_spell(spell_index - 1);
        if self.player.mp < selected_spell.mp_cost {
            println!("MP が たりません。");
            return self.commands_cancel();
        }

        self.player.consume_mp(selected_spell);
        self.display_use_spell(&selected_spell.spell);
        if self.player_state.seal {
            println!("しかし じゅもんは ふうじこまれている！");
            return;
        }

        match selected_spell.spell {
            Spell::Hoimi | Spell::Behoimi => {
                let heal = player_spell_effect(selected_spell.spell);
                self.player.adjust_hp(heal as i16);
            }
            Spell::Gira | Spell::Begirama => {
                let spell_invalid =
                    random_success_by_percent(self.monster.behavior.resist.gira as f64);

                if spell_invalid {
                    println!("しかし じゅもんは きかなかった！\n");
                } else {
                    let damage = player_spell_effect(selected_spell.spell);
                    self.display_monster_damage(damage);
                    self.monster.adjust_hp(-(damage as i16));
                }
            }
            Spell::Rarirho => {
                let spell_invalid =
                    random_success_by_percent(self.monster.behavior.resist.rariho as f64);

                if spell_invalid {
                    println!("しかし じゅもんは きかなかった！\n");
                } else {
                    println!("{}を ねむらせた!", self.monster.name());
                    self.monster_state.sleep = true;
                }
            }
            Spell::Mahoton => {
                let spell_invalid =
                    random_success_by_percent(self.monster.behavior.resist.mahoton as f64);

                if spell_invalid {
                    println!("しかし じゅもんは きかなかった！\n");
                } else {
                    println!("{}の じゅもんを", self.monster.name());
                    println!("ふうじこめた！");
                    self.monster_state.seal = true;
                }
            }
            _ => {
                println!("それは たたかいに つかえない！");
                self.commands_cancel()
            }
        }
    }

    // プレイヤー: どうぐ
    pub fn player_action_item(&mut self) {
        println!("{} は道具を使おうとした！", self.player.name);

        // let items = self.player.items();
        // if items.is_empty() {
        //     println!("持っている道具がない！");
        //     return;
        // }
        //
        // println!("持っている道具一覧:");
        // for (i, item) in items.iter().enumerate() {
        //     println!("{}: {}", i, item.to_string());
        // }
        //
        // let item_index = get_player_input(items.len());
        // let chosen_item = items[item_index];
        //
        // // TODO: アイテムごとの効果処理
        // println!("{} を使った！", chosen_item.to_string());
        // 例: HP回復とか
    }

    // プレイヤー: にげる
    pub fn player_action_escape(&mut self) {
        println!("{}は にげだした！", self.player.name);
        let is_escape = self.is_escape();
        if is_escape {
            self.player_state.escaped = true;
        } else {
            println!("しかし まわりこまれてしまった！ \n");
        }
    }

    pub fn get_player_input(&mut self, max: usize) -> usize {
        loop {
            print!("番号を選んでください (0-{}): ", max);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if let Ok(_) = io::stdin().read_line(&mut input) {
                if let Ok(num) = input.trim().parse::<usize>() {
                    if num <= max {
                        return num;
                    }
                }
            }
            println!("無効な入力です。もう一度入力してください。");
        }
    }

    pub fn commands(&mut self) {
        let action = self.get_player_action();
        match action {
            PlayerAction::Attack => self.player_action_attack(),
            PlayerAction::Spell => self.player_action_spell(),
            PlayerAction::Item => self.player_action_item(),
            PlayerAction::Escape => self.player_action_escape(),
        }
    }

    pub fn commands_cancel(&mut self) {
        self.display_command();
        self.commands();
        return;
    }

    fn get_player_action(&self) -> PlayerAction {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => PlayerAction::Attack,
            "2" => PlayerAction::Spell,
            "3" => PlayerAction::Item,
            "4" => PlayerAction::Escape,
            _ => {
                println!("無効な入力です。もう一度選んでください。");
                self.display_command();
                self.get_player_action() // 再帰的にリトライ
            }
        }
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

    // fn setup_battle_for_action(enemy_action: EnemyAction) -> Battle {
    //     let player = Player::new("ゆうてい");
    //     let monster = Monster::new(0);
    //     let mut battle = Battle::new(player, monster);
    //     battle.monster_state.escaped = false;
    //     battle
    // }

    // #[test]
    // fn test_monster_turn_escape() {
    //     let mut battle = setup_battle_for_action(EnemyAction::Escape);
    //     battle.monster_state.escaped = true;
    //     battle.monster_turn();
    //     // assert!(battle.monster_state.escaped, "Monster should have escaped");
    // }
}
