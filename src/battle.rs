use crate::constants::monster::{ActionType, MonsterAction};
use crate::constants::spell::Spell;
use crate::message::BattleMessages;
use crate::monster::Monster;
use crate::player::{ItemKind, Player, UnifiedItem};
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
    pub messages: BattleMessages,
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
        let player_name = player.name.clone();
        let monster_name = monster.stats.name.to_string();
        Self {
            player,
            monster,
            player_state: BattleState::default(),
            monster_state: BattleState::default(),
            messages: BattleMessages::new(player_name, monster_name),
        }
    }

    pub fn start(&mut self) {
        self.messages.add_monster_appears();
        self.update_status();
        self.messages.display();
        self.messages.clear();
        
        while self.is_battle_continue() {
            self.player_turn();
            
            if !self.player_state.escaped {
                if self.monster.is_alive() {
                    self.monster_turn();
                }
                self.update_status();
            }
            
            self.messages.display();
            self.messages.clear();
        }
        
        if self.player_state.escaped {
            self.messages.add_empty_line();
            self.messages.display();
        } else if !self.monster_state.escaped {
            if self.player.is_alive() {
                let gold = self.monster.get_gold();
                self.messages.add_defeat_monster(self.monster.stats.exp.into(), gold.into());
            } else {
                self.messages.add_player_death();
            }
            self.messages.display();
        }
    }

    pub fn update_status(&mut self) {
        self.messages.add_status(self.player.hp, self.player.mp, self.monster.hp);
    }

    pub fn add_enemy_special_skill_message(&mut self, name: &str, damage: u8) {
        self.messages.push(format!(" {}は ", self.monster.stats.name));
        if name.contains("ほのお") {
            self.messages.push(" ほのおをはいた!".to_string());
        } else {
            self.messages.push(format!(" {}を使った！", name));
        }
        self.messages.add_empty_line();
        self.messages.add_player_damage(damage);
    }

    pub fn add_use_spell(&mut self, spell: &Spell) {
        self.messages.add_use_spell(spell.as_str());
    }

    pub fn add_monster_damage(&mut self, damage: u8) {
        self.messages.add_monster_damage(damage);
    }

    pub fn add_monster_spell(&mut self, spell: &Spell) {
        self.messages.add_monster_spell(spell.as_str());
    }

    pub fn add_monster_spell_sealed(&mut self) {
        self.messages.add_spell_sealed();
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
                self.messages.push(format!("{}は めをさました！", self.monster.name()));
                self.monster_state.sleep = false;
            } else {
                self.messages.push(format!("{}は ねむっている⋯⋯⋯", self.monster.name()));
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
                self.messages.push(format!("{} は にげだした！", self.monster.name()));
                self.monster_state.escaped = true;
            }
        }
    }

    fn handle_enemy_normal_attack(&mut self) {
        let damage = self.monster.normal_damage(&self.player) as i16;
        
        if damage > 0 {
            self.messages.add_monster_attack();
            self.messages.add_player_damage(damage as u8);
            
            self.player.adjust_hp(-damage);
        } else {
            self.messages.add_monster_attack();
            self.messages.add_miss();
        }
    }

    /// 敵: ホイミ、ベホイミ
    fn handle_enemy_heal_spell(&mut self, spell: &Spell, monster_action: &MonsterAction) {
        let heal = monster_action_effect(&monster_action.action);
        self.add_monster_spell(spell);

        if self.monster_state.seal {
            return self.add_monster_spell_sealed();
        }
        self.messages.push(format!(" {}は きずが", self.monster.name()));
        self.messages.push(" かいふくした！".to_string());
        self.monster.adjust_hp(heal as i16);
    }

    /// 敵: ギラ、ベギラマ
    fn handle_enemy_attack_spell(&mut self, spell: &Spell, monster_action: &MonsterAction) {
        let damage = monster_action_effect(&monster_action.action);
        self.add_monster_spell(spell);

        if self.monster_state.seal {
            return self.add_monster_spell_sealed();
        }
        self.messages.add_player_damage(damage);
        self.player.adjust_hp(-(damage as i16));
    }

    /// 敵: ラリホー（100%）
    fn handle_enemy_sleep_spell(&mut self, spell: &Spell) {
        self.add_monster_spell(spell);

        if self.monster_state.seal {
            return self.add_monster_spell_sealed();
        }
        self.messages.push(format!("{}は ねむってしまった！", self.player.name));
        self.player_state.sleep = true;
    }

    /// 敵: マホトーン（50%）
    fn handle_enemy_seal_spell(&mut self, spell: &Spell) {
        self.add_monster_spell(spell);
        if self.monster_state.seal {
            return self.add_monster_spell_sealed();
        }

        let success = random_success_by_percent(50.0);
        if success && !self.player.is_max_armor() {
            self.messages.push(format!("{}は じゅもんを", self.player.name));
            self.messages.push("ふうじこめられた！".to_string());
            self.player_state.seal = true;
        } else {
            self.messages.push("しかし じゅもんは きかなかった！".to_string());
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

        self.add_enemy_special_skill_message(name, damage);
        self.player.adjust_hp(-(damage as i16));
    }

    pub fn display_command(&mut self) {
        self.messages.push(format!("\n--- {}のターン ---", self.player.name));
        self.messages.push("コマンド？".to_string());
        self.messages.push("1: たたかう".to_string());
        self.messages.push("2: じゅもん".to_string());
        self.messages.push("3: どうぐ".to_string());
        self.messages.push("4: にげる".to_string());
        self.messages.display();
    }

    pub fn player_turn(&mut self) {
        if self.player_state.sleep {
            let is_wakeup = random_success_by_percent(33.33);
            if is_wakeup {
                self.messages.push(format!("{}は めをさました！", self.player.name));
                self.player_state.sleep = false;
            } else {
                self.messages.push(format!("{}は ねむっている⋯⋯⋯", self.player.name));
                self.messages.display();
                return;
            }
        }
        self.display_command();
        // Receive input
        self.commands();
    }

    pub fn player_battle_attack_damage(&mut self) -> u8 {
        // 回避
        let is_evade = random_success_by_percent(self.monster.behavior.evade_rate as f64);
        if is_evade {
            return 0;
        }
        // かいしんのいちげき
        let is_critical = random_success_by_ratio(32);
        if is_critical && !self.monster.is_final_boss() {
            self.messages.push("かいしんの いちげき！！".to_string());
            self.player.critical_damage()
        } else {
            self.player.normal_damage(&self.monster)
        }
    }

    // プレイヤー: こうげき
    pub fn player_action_attack(&mut self) {
        self.messages.add_player_attack(&self.player.name);
        let damage = self.player_battle_attack_damage();
        if damage == 0 {
            self.messages.add_miss();
        } else {
            self.add_monster_damage(damage);
            self.monster.adjust_hp(-(damage as i16));
        }
    }

    // プレイヤー: じゅもん
    pub fn player_action_spell(&mut self) {
        if self.player.is_empty_spell_list() {
            self.messages.push(format!("{}は まだ じゅもんを", self.player.name));
            self.messages.push("つかえない。".to_string());
            self.messages.display();
            return self.commands_cancel();
        }

        let spell_list = self.player.spell_list();
        let spell_len = spell_list.len();

        self.messages.push("--- じゅもん ---".to_string());
        self.messages.push("0: もどる".to_string());
        for (i, spell_info) in spell_list.into_iter().enumerate() {
            self.messages.push(format!("{}: {}", i + 1, spell_info.spell.as_str()));
        }
        self.messages.display();
        self.messages.clear();

        let spell_index = self.get_player_input(spell_len);
        if spell_index == 0 {
            return self.commands_cancel();
        }
        let selected_spell = self.player.select_spell(spell_index - 1);
        if self.player.mp < selected_spell.mp_cost {
            self.messages.push("MP が たりません。".to_string());
            self.messages.display();
            self.messages.clear();
            return self.commands_cancel();
        }

        self.player.consume_mp(selected_spell);
        self.add_use_spell(&selected_spell.spell);
        if self.player_state.seal {
            self.messages.push("しかし じゅもんは ふうじこまれている！".to_string());
            self.messages.display();
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
                    self.messages.push("しかし じゅもんは きかなかった！\n".to_string());
                } else {
                    let damage = player_spell_effect(selected_spell.spell);
                    self.add_monster_damage(damage);
                    self.monster.adjust_hp(-(damage as i16));
                }
            }
            Spell::Rarirho => {
                let spell_invalid =
                    random_success_by_percent(self.monster.behavior.resist.rariho as f64);

                if spell_invalid {
                    self.messages.push("しかし じゅもんは きかなかった！\n".to_string());
                } else {
                    self.messages.push(format!("{}を ねむらせた!", self.monster.name()));
                    self.monster_state.sleep = true;
                }
            }
            Spell::Mahoton => {
                let spell_invalid =
                    random_success_by_percent(self.monster.behavior.resist.mahoton as f64);

                if spell_invalid {
                    self.messages.push("しかし じゅもんは きかなかった！\n".to_string());
                } else {
                    self.messages.push(format!("{}の じゅもんを", self.monster.name()));
                    self.messages.push("ふうじこめた！".to_string());
                    self.monster_state.seal = true;
                }
            }
            _ => {
                self.messages.push("それは たたかいに つかえない！".to_string());
                self.messages.display();
                self.messages.clear();
                self.commands_cancel()
            }
        }
    }

    // プレイヤー: どうぐ
    pub fn player_action_item(&mut self) {
        if !self.player.is_unified_item_list() {
            self.messages.push("つかえるものを まだ".to_string());
            self.messages.push("もっていない。".to_string());
            self.messages.display();
            self.messages.clear();
            self.commands_cancel();
            return;
        }

        self.messages.push("--- どうぐ ---".to_string());
        self.messages.push("0: もどる".to_string());
        let unified_item_list = self.player.unified_item_list();
        for (i, item) in unified_item_list.iter().enumerate() {
            if item.kind == ItemKind::Herb || item.kind == ItemKind::Key {
                self.messages.push(format!("{}: {} （{}）", i + 1, item.name, item.count));
            } else {
                self.messages.push(format!("{}: {}", i + 1, item.name));
            }
        }
        self.messages.display();
        self.messages.clear();

        let item_index = self.get_player_input(unified_item_list.len());
        if item_index == 0 {
            return self.commands_cancel();
        }
        self.use_item(unified_item_list[item_index - 1].clone());
    }

    pub fn use_item(&mut self, item: UnifiedItem) {
        match item.kind {
            ItemKind::Herb => {
                self.messages.push(format!("{}は やくそうを つかった！", self.player.name));
                self.player.use_herbs();
            }
            ItemKind::Key => {
                self.messages.push("それは たたかいに つかえない！".to_string());
                self.messages.display();
                self.messages.clear();
                self.commands_cancel();
                return;
            }
            ItemKind::Equipment => match item.id {
                4 => {
                    if self.player.flags.has_dragon_scale {
                        self.messages.push("りゅうのうろこは すでに".to_string());
                        self.messages.push("みにつけています。".to_string());
                    } else {
                        self.messages.push(format!("{}は りゅうのうろこを", self.player.name));
                        self.messages.push("みにつけた。".to_string());
                        self.player.flags.has_dragon_scale = true;
                    }
                }
                5 => {
                    self.messages.push(format!("{}は ふえをふいた。", self.player.name));
                    if self.monster.id == 32 {
                        self.messages.push(format!("{}は しずかに めをとじる⋯", self.monster.name()));
                        self.messages.push("ねむってしまった！".to_string());
                        self.monster_state.sleep = true;
                    } else {
                        self.messages.push("しかし なにも おきなかった。".to_string());
                    }
                }
                6 => {
                    self.messages.push(format!("{}は せんしのゆびわを", self.player.name));
                    if self.player.flags.has_warrior_ring {
                        self.messages.push("はめなおした。".to_string());
                    } else {
                        self.messages.push("はめた。".to_string());
                    }
                }
                9 => {
                    if self.player.is_curse_belt {
                        self.messages.push("のろいのベルトが あなたの".to_string());
                        self.messages.push("からだを しめつけている。".to_string());
                    } else {
                        self.messages.push(format!("{}は のろいのベルトを", self.player.name));
                        self.messages.push("みにつけた。".to_string());

                        self.messages.push("のろいのベルトが あなたの".to_string());
                        self.messages.push("からだを しめつける。".to_string());
                        self.messages.push("あなたは のろわれてしまった。".to_string());

                        self.player.is_curse_belt = true;
                    }
                }
                10 => {
                    self.messages.push(format!("{}は たてごとを かなでた。", self.player.name));
                    self.messages.push(format!("{}は うれしそうだ!", self.monster.name()));
                }
                11 => {
                    if self.player.is_curse_necklace {
                        self.messages.push("しのくびかざりが あなたの".to_string());
                        self.messages.push("からだを しめつけている。".to_string());
                    } else {
                        self.messages.push(format!("{}は しのくびかざりを", self.player.name));
                        self.messages.push("みにつけた。".to_string());

                        self.messages.push("しのくびかざりが あなたの".to_string());
                        self.messages.push("からだを しめつける。".to_string());
                        self.messages.push("あなたは のろわれてしまった。".to_string());

                        self.player.is_curse_necklace = true;
                    }
                }
                _ => {
                    self.messages.push("それは たたかいに つかえない！".to_string());
                    self.messages.display();
                    self.messages.clear();
                    self.commands_cancel();
                    return;
                }
            },
        }
    }

    // プレイヤー: にげる
    pub fn player_action_escape(&mut self) {
        self.messages.push(format!("{}は にげだした！", self.player.name));
        let is_escape = self.is_escape();
        if is_escape {
            self.player_state.escaped = true;
        } else {
            self.messages.push("しかし まわりこまれてしまった！ \n".to_string());
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

    fn get_player_action(&mut self) -> PlayerAction {
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
