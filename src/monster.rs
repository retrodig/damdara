use crate::constants::config::BIT_8_MAX;
use crate::constants::monster::{
    ActionType, MONSTER_BEHAVIORS, MONSTER_MASTER, MonsterAction, MonsterBehavior, MonsterStats,
};
use crate::constants::spell::Spell;
use crate::player::Player;
use crate::utility::random_utils::random_value;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Monster {
    pub hp: u8,
    pub stats: MonsterStats,
    pub behavior: MonsterBehavior,
}

impl Monster {
    pub fn new(index: usize) -> Self {
        let stats = MONSTER_MASTER.get(index).unwrap_or(&MONSTER_MASTER[0]);
        let behavior = MONSTER_BEHAVIORS
            .get(index)
            .unwrap_or(&MONSTER_BEHAVIORS[0]);

        let mut rng = rand::rng();
        let random_value: u16 = rng.random_range(0..=BIT_8_MAX as u16);
        let reduction = (stats.hp as u16 * random_value) / 1024;
        let initial_hp = stats.hp.saturating_sub(reduction as u8);

        Self {
            hp: initial_hp,
            stats: stats.clone(),
            behavior: behavior.clone(),
        }
    }

    pub fn max_hp(&self) -> u8 {
        self.stats.hp
    }

    pub fn is_low_hp(&self) -> bool {
        (self.hp as f32) <= (self.stats.hp as f32 / 4.0)
    }

    pub fn adjust_hp(&mut self, amount: i16) {
        let new_hp = (self.hp as i16 + amount).clamp(0, self.max_hp() as i16);
        self.hp = new_hp as u8;
    }

    pub fn has_support_magic(&self) -> bool {
        self.behavior.actions.iter().any(|action| {
            matches!(
                action.action,
                ActionType::Spell(Spell::Hoimi)
                    | ActionType::Spell(Spell::Behoimi)
                    | ActionType::Spell(Spell::Rarirho)
                    | ActionType::Spell(Spell::Mahoton)
            )
        })
    }

    pub fn support_spells_actions(&self) -> Vec<MonsterAction> {
        self.behavior
            .actions
            .iter()
            .filter(|action| {
                matches!(
                    action.action,
                    ActionType::Spell(Spell::Hoimi)
                        | ActionType::Spell(Spell::Behoimi)
                        | ActionType::Spell(Spell::Rarirho)
                        | ActionType::Spell(Spell::Mahoton)
                )
            })
            .cloned()
            .collect()
    }

    pub fn attack_spells_actions(&self) -> Vec<MonsterAction> {
        self.behavior
            .actions
            .iter()
            .filter(|action| {
                matches!(
                    action.action,
                    ActionType::Spell(Spell::Gira)
                        | ActionType::Spell(Spell::Begirama)
                        | ActionType::Special("ほのお(弱)")
                        | ActionType::Special("ほのお(強)")
                )
            })
            .cloned()
            .collect()
    }

    pub fn support_spells(&self) -> Vec<Spell> {
        self.behavior
            .actions
            .iter()
            .filter_map(|action| {
                if let ActionType::Spell(spell) = action.action {
                    Some(spell)
                } else {
                    None
                }
            })
            .filter(|spell| {
                matches!(
                    spell,
                    Spell::Hoimi | Spell::Behoimi | Spell::Rarirho | Spell::Mahoton
                )
            })
            .collect()
    }

    pub fn has_attack_skill(&self) -> bool {
        self.behavior.actions.iter().any(|action| {
            matches!(
                action.action,
                ActionType::Spell(Spell::Gira)
                    | ActionType::Spell(Spell::Begirama)
                    | ActionType::Special("ほのお(弱)")
                    | ActionType::Special("ほのお(強)")
            )
        })
    }

    // pub fn attack_skills(&self) -> Vec<ActionType> {
    //     self.behavior.actions.iter()
    //         .filter(|action| match action.action {
    //             ActionType::Spell(Spell::Gira)
    //             | ActionType::Spell(Spell::Begirama)
    //             | ActionType::Special("ほのお(弱)")
    //             | ActionType::Special("ほのお(強)") => true,
    //             _ => false,
    //         })
    //         .cloned() // ActionTypeをコピー
    //         .collect()
    // }

    pub fn is_alive(&self) -> bool {
        self.stats.hp > 0
    }

    pub fn is_final_boss(&self) -> bool {
        self.behavior.index == 38 || self.behavior.index == 39
    }

    pub fn correction_damage(&self, player: &Player) -> u8 {
        let monster_strength = self.stats.attack as i32;
        let player_defense = player.defense_power() as i32;

        let base_damage = (monster_strength - (player_defense / 2) + 2).max(0) / 4;

        let mut rng = rand::rng();
        let random_bonus = rng.random_range(0..=(monster_strength / 4).max(0));

        let mut damage = base_damage + random_bonus;
        if damage <= 0 {
            damage = 0;
        }

        damage.min(255) as u8
    }

    pub fn normal_damage(&self, player: &Player) -> u8 {
        let monster_strength = self.stats.attack as i32;
        let player_defense = player.defense_power() as i32;
        let diff = monster_strength - (player_defense / 2);
        let rand_val = random_value(BIT_8_MAX) as i32;
        let damage = (rand_val * (diff + 1) / 256 + diff) / 4;
        damage.max(0).min(BIT_8_MAX as i32) as u8
    }

    pub fn battle_attack(&self, player: &Player) -> u8 {
        let monster_strength = self.stats.attack as i32;
        let player_defense = player.defense_power() as i32;
        let diff = monster_strength - (player_defense / 2);

        if diff >= (monster_strength / 2 + 1) {
            self.normal_damage(player)
        } else {
            self.correction_damage(player)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monster_no_support_magic() {
        let monster = Monster::new(0);
        assert!(
            !monster.has_support_magic(),
            "Monster has no auxiliary magic."
        );
    }

    #[test]
    fn test_monster_with_support_magic() {
        let monster = Monster::new(26);
        assert!(
            monster.has_support_magic(),
            "Monster has auxiliary magic (Rariho)"
        );
    }

    #[test]
    fn test_monster_has_no_attack_spells() {
        let slime = Monster::new(0);
        let attacks = slime.attack_spells_actions();
        assert!(
            attacks.is_empty(),
            "Slime should have no offensive specialties."
        );
    }

    #[test]
    fn test_monster_has_attack_spells() {
        let dragonlord = Monster::new(39);
        let attacks = dragonlord.attack_spells_actions();
        assert!(
            !attacks.is_empty(),
            "Ryuoh should have an attack special (fire)"
        );
        let found = attacks.iter().any(|action| match &action.action {
            ActionType::Special(name) => *name == "ほのお(強)",
            _ => false,
        });
        assert!(found, "Ryuoh should have “Ho-oh (strong)”.");
    }

    // #[test]
    // fn test_player_max_damage() {
    //     let mut player = Player::new("だい");
    //     player.maximize();
    //     let monster = Monster::new(30);
    //     println!("{}", monster.stats.name);
    //     for _ in 0..50 {
    //         let damage = monster.battle_attack(&player);
    //         println!("{}", damage);
    //     }
    //     assert!(player.level() >= 5);
    // }
}
