use crate::constants::{
    battle::{BattleState, EnemyAction, PlayerAction},
    monster::{ActionType, MonsterAction},
    spell::Spell,
};
use crate::message::BattleMessages;
use crate::monster::Monster;
use crate::player::{ItemKind, Player, UnifiedItem};
use crate::traits::message_output::MessageOutput;
use crate::traits::player_input::PlayerInput;
use crate::utility::monster_utils::choose_action;
use crate::utility::random_utils::{
    check_escape_success, get_escape_rand_max_by_monster_index, random_success_by_percent,
    random_success_by_ratio,
};
use crate::utility::spell_utils::{monster_action_effect, player_spell_effect};
use rand::Rng;

pub struct Battle<'a> {
    pub player: Player,
    pub monster: Monster,
    pub player_state: BattleState,
    pub monster_state: BattleState,
    pub messages: BattleMessages<'a>,
    pub input: &'a mut dyn PlayerInput,
}

impl<'a> Battle<'a> {
    pub fn new(
        player: Player,
        monster: Monster,
        input: &'a mut dyn PlayerInput,
        output: &'a mut dyn MessageOutput,
    ) -> Self {
        let player_name = player.name.clone();
        let monster_name = monster.stats.name.to_string();
        Self {
            player,
            monster,
            player_state: BattleState::default(),
            monster_state: BattleState::default(),
            messages: BattleMessages::new(player_name, monster_name, output),
            input,
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
                self.messages
                    .add_defeat_monster(self.monster.stats.exp.into(), gold.into());
            } else {
                self.messages.add_player_death();
            }
            self.messages.display();
        }
    }

    pub fn update_status(&mut self) {
        self.messages
            .add_status(self.player.hp, self.player.mp, self.monster.hp);
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
                self.messages.monster_wake_up();
                self.monster_state.sleep = false;
            } else {
                self.messages.monster_still_asleep();
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
                self.messages.monster_escaped();
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
        self.messages.monster_heal();
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
        self.messages.fall_asleep();
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
            self.messages.spells_sealed();
            self.player_state.seal = true;
        } else {
            self.messages.spell_resisted();
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

        self.messages.enemy_special_skill(name, damage);
        self.player.adjust_hp(-(damage as i16));
    }

    pub fn display_command(&mut self) {
        self.messages.display_command();
        self.messages.display();
    }

    pub fn player_turn(&mut self) {
        if self.player_state.sleep {
            let is_wakeup = random_success_by_percent(33.33);
            if is_wakeup {
                self.messages.wake_up();
                self.player_state.sleep = false;
            } else {
                self.messages.still_asleep();
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
            self.messages.critical_damage();
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
            self.messages.can_not_use_spell();
            self.messages.display();
            return self.commands_cancel();
        }

        let spell_list = self.player.spell_list();
        let spell_len = spell_list.len();

        self.messages.push("--- じゅもん ---".to_string());
        self.messages.push("0: もどる".to_string());
        for (i, spell_info) in spell_list.into_iter().enumerate() {
            self.messages
                .push(format!("{}: {}", i + 1, spell_info.spell.as_str()));
        }
        self.messages.display();
        self.messages.clear();

        let spell_index = self.input.get_player_input(spell_len);
        if spell_index == 0 {
            return self.commands_cancel();
        }
        let selected_spell = self.player.select_spell(spell_index - 1);
        if self.player.mp < selected_spell.mp_cost {
            self.messages.mp_not_enough();
            self.messages.display();
            self.messages.clear();
            return self.commands_cancel();
        }

        self.player.consume_mp(selected_spell);
        self.add_use_spell(&selected_spell.spell);
        if self.player_state.seal {
            self.messages.spell_sealed();
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
                    self.messages.spell_resisted();
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
                    self.messages.spell_resisted();
                } else {
                    self.messages.monster_fall_asleep();
                    self.monster_state.sleep = true;
                }
            }
            Spell::Mahoton => {
                let spell_invalid =
                    random_success_by_percent(self.monster.behavior.resist.mahoton as f64);

                if spell_invalid {
                    self.messages.spell_resisted();
                } else {
                    self.messages.seal_monster_spell();
                    self.monster_state.seal = true;
                }
            }
            _ => {
                self.messages.can_not_use_by_battle();
                self.messages.display();
                self.messages.clear();
                self.commands_cancel()
            }
        }
    }

    // プレイヤー: どうぐ
    pub fn player_action_item(&mut self) {
        if !self.player.is_unified_item_list() {
            self.messages.no_usable_items();
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
                self.messages
                    .push(format!("{}: {} （{}）", i + 1, item.name, item.count));
            } else {
                self.messages.push(format!("{}: {}", i + 1, item.name));
            }
        }
        self.messages.display();
        self.messages.clear();

        let item_index = self.input.get_player_input(unified_item_list.len());
        if item_index == 0 {
            return self.commands_cancel();
        }
        self.use_item(unified_item_list[item_index - 1].clone());
    }

    pub fn use_item(&mut self, item: UnifiedItem) {
        match item.kind {
            ItemKind::Herb => {
                self.messages.used_item_herbs();
                self.player.use_herbs();
            }
            ItemKind::Key => {
                self.messages.can_not_use_by_battle();
                self.messages.display();
                self.messages.clear();
                self.commands_cancel();
                return;
            }
            ItemKind::Equipment => match item.id {
                4 => {
                    if self.player.flags.has_dragon_scale {
                        self.messages.already_has_dragon_scale();
                    } else {
                        self.messages.used_dragon_scale();
                        self.player.flags.has_dragon_scale = true;
                    }
                }
                5 => {
                    self.messages.used_flute();
                    if self.monster.id == 32 {
                        self.messages.used_flute_monster_fell_asleep();
                        self.monster_state.sleep = true;
                    } else {
                        self.messages.nothing_happened();
                    }
                }
                6 => {
                    if self.player.flags.has_warrior_ring {
                        self.messages.reequipped_warrior_ring();
                    } else {
                        self.messages.used_warrior_ring();
                        self.player.equip_warrior_ring();
                    }
                }
                9 => {
                    if self.player.is_curse_belt {
                        self.messages.cursed_belt_constricting();
                    } else {
                        self.messages.used_cursed_belt();
                        self.messages.cursed_belt_activated();
                        self.player.equip_cursed_belt();
                    }
                }
                10 => {
                    self.messages.used_lyre_and_monster_rejoiced();
                }
                11 => {
                    if self.player.is_curse_necklace {
                        self.messages.cursed_necklace_constricting();
                    } else {
                        self.messages.used_cursed_necklace();
                        self.messages.cursed_necklace_activated();
                        self.player.equip_cursed_necklace();
                    }
                }
                _ => {
                    self.messages.can_not_use_by_battle();
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
        self.messages.player_escaped();
        let is_escape = self.is_escape();
        if is_escape {
            self.player_state.escaped = true;
        } else {
            self.messages.escape_blocked();
        }
    }

    pub fn commands(&mut self) {
        let action = self.input.get_player_action(&mut || {
            self.messages.clear();
            self.messages.display_command();
            self.messages.display();
        });
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monster::Monster;
    use crate::player::{Player, PlayerArgs};

    struct DummyOutput;
    struct DummyInput {
        pub predefined_input: Vec<PlayerAction>,
        pub cursor: usize,
    }

    impl MessageOutput for DummyOutput {
        fn output(&mut self, _message: &str) {
            todo!()
        }
    }

    impl DummyInput {
        pub fn new(predefined_input: Vec<PlayerAction>) -> Self {
            Self {
                predefined_input,
                cursor: 0,
            }
        }
    }

    impl PlayerInput for DummyInput {
        fn get_player_input(&mut self, _max: usize) -> usize {
            0
        }

        fn get_player_action(&mut self, _display_commands: &mut dyn FnMut()) -> PlayerAction {
            let action = self
                .predefined_input
                .get(self.cursor)
                .cloned()
                .unwrap_or(PlayerAction::Attack); // デフォルト行動
            self.cursor += 1;
            action
        }
    }

    #[test]
    fn test_real_player_high_agility() {
        let player = Player::new_with(PlayerArgs {
            name: Some("ゆうてい".to_string()),
            exp: Some(3000),
            ..Default::default()
        });
        let monster = Monster::new(0);
        let mut dummy_output = DummyOutput;
        let mut dummy_input = DummyInput::new(vec![PlayerAction::Spell, PlayerAction::Escape]);
        let battle = Battle::new(player, monster, &mut dummy_input, &mut dummy_output);
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
            let mut dummy_output = DummyOutput;
            let mut dummy_input = DummyInput::new(vec![PlayerAction::Spell, PlayerAction::Escape]);
            let battle = Battle::new(player, monster, &mut dummy_input, &mut dummy_output);

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
