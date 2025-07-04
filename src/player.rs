use crate::constants::item_weapon::{
    ARMOR_MASTER, Equipment, ITEM_MASTER, SHIELD_MASTER, WEAPON_MASTER,
};
use crate::constants::save_data::{SaveData, SaveDataArgs};
use crate::constants::spell::SpellInfo;
use crate::constants::status::{DEFAULT_STATUS, Flags, PlayerSummary, Status, StrengthStatus};
use crate::constants::text::DEFAULT_NAME;
use crate::growth_type::{
    GrowthModifiers, calculate_abc, calculate_growth_name_total, get_adjusted_status_by_name_lv,
};
use crate::load::decode_from_password_string;
use crate::monster::Monster;
use crate::utility::random_utils::{generate_in_range, random_value};
use crate::utility::spell_utils::spells_learned_by_level;
use crate::utility::status_utils::{get_level_by_exp, get_status_by_level, resolve_experience};
use crate::utility::string_utils::name_normalize;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hp: u8,
    pub mp: u8,
    pub exp: u16,
    pub gold: u16,
    pub weapon: u8,
    pub armor: u8,
    pub shield: u8,
    pub items: [u8; 8],
    pub herbs: u8,
    pub keys: u8,
    pub is_curse_belt: bool,
    pub is_curse_necklace: bool,
    pub flags: Flags,
}

#[derive(Default)]
pub struct PlayerArgs {
    pub name: Option<String>,
    pub level: Option<u8>,
    pub exp: Option<u16>,
    pub gold: Option<u16>,
    pub weapon: Option<u8>,
    pub armor: Option<u8>,
    pub shield: Option<u8>,
    pub items: Option<[u8; 8]>,
    pub herbs: Option<u8>,
    pub keys: Option<u8>,
    pub flags: Option<Flags>,
}

impl PlayerArgs {
    pub fn from_save_data(save: &SaveData) -> Self {
        Self {
            name: Some(save.name.clone()),
            exp: Some(save.experience),
            gold: Some(save.gold),
            weapon: Some(save.weapon),
            armor: Some(save.armor),
            shield: Some(save.shield),
            items: Some(save.items),
            herbs: Some(save.herbs),
            keys: Some(save.keys),
            flags: Some(Flags {
                has_dragon_scale: save.flags.has_dragon_scale,
                has_warrior_ring: save.flags.has_warrior_ring,
                has_cursed_necklace: save.flags.has_cursed_necklace,
                defeated_dragon: save.flags.defeated_dragon,
                defeated_golem: save.flags.defeated_golem,
            })
            .into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnifiedItem {
    pub id: u8,
    pub name: &'static str,
    pub count: u8,
    pub kind: ItemKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemKind {
    Herb,
    Key,
    Equipment,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self::new_with(PlayerArgs {
            name: Some(name.to_string()),
            ..Default::default()
        })
    }

    pub fn new_with(args: PlayerArgs) -> Self {
        let name = name_normalize(&args.name.unwrap_or_else(|| DEFAULT_NAME.to_string()));
        let base_level = args.level.unwrap_or(1);
        let final_exp = resolve_experience(base_level, args.exp);
        let level = get_level_by_exp(final_exp);
        let adjusted = get_adjusted_status_by_name_lv(&name, level);

        Self {
            name,
            hp: adjusted.max_hp,
            mp: adjusted.max_mp,
            exp: final_exp,
            gold: args.gold.unwrap_or(0),
            weapon: args.weapon.unwrap_or(0),
            armor: args.armor.unwrap_or(0),
            shield: args.shield.unwrap_or(0),
            items: args.items.unwrap_or([0; 8]),
            herbs: args.herbs.unwrap_or(0),
            keys: args.keys.unwrap_or(0),
            is_curse_belt: false,
            is_curse_necklace: false,
            flags: args.flags.unwrap_or_default(),
        }
    }

    pub fn from_password_string(s: &str) -> Result<Self, String> {
        let save = decode_from_password_string(s)?;
        Ok(Self::from_save_data(&save))
    }

    pub fn from_save_data(save: &SaveData) -> Self {
        Self::new_with(PlayerArgs::from_save_data(save))
    }

    pub fn level(&self) -> u8 {
        get_level_by_exp(self.exp)
    }

    pub fn growth_name_total(&self) -> u16 {
        calculate_growth_name_total(&self.name)
    }

    pub fn strength(&self) -> u8 {
        self.status().map(|s| s.strength).unwrap_or(0)
    }

    pub fn agility(&self) -> u8 {
        self.status().map(|s| s.agility).unwrap_or(0)
    }

    pub fn max_hp(&self) -> u8 {
        self.status().map(|s| s.max_hp).unwrap_or(0)
    }

    pub fn max_mp(&self) -> u8 {
        self.status().map(|s| s.max_mp).unwrap_or(0)
    }

    pub fn is_mp_cast(&self, spell_info: &SpellInfo) -> bool {
        self.mp >= spell_info.mp_cost
    }

    pub fn abc(&self) -> GrowthModifiers {
        calculate_abc(self.growth_name_total())
    }

    pub fn base_status(&self) -> Option<Status> {
        get_status_by_level(self.level())
    }

    pub fn status(&self) -> Option<Status> {
        Some(get_adjusted_status_by_name_lv(&self.name, self.level()))
    }

    pub fn get_weapon(&self) -> &Equipment {
        WEAPON_MASTER
            .get(self.weapon as usize)
            .unwrap_or(&WEAPON_MASTER[0])
    }

    pub fn get_armor(&self) -> &Equipment {
        ARMOR_MASTER
            .get(self.armor as usize)
            .unwrap_or(&ARMOR_MASTER[0])
    }

    pub fn get_shield(&self) -> &Equipment {
        SHIELD_MASTER
            .get(self.shield as usize)
            .unwrap_or(&SHIELD_MASTER[0])
    }

    pub fn attack_power(&self) -> u8 {
        let ring_bonus = if self.flags.has_warrior_ring { 2 } else { 0 };
        self.status()
            .map(|s| s.strength + self.get_weapon().attack + ring_bonus)
            .unwrap_or(self.get_weapon().attack + ring_bonus)
    }

    pub fn defense_power(&self) -> u8 {
        let armor = self.get_armor();
        let shield = self.get_shield();
        let scale_bonus = if self.flags.has_dragon_scale { 2 } else { 0 };
        self.status()
            .map(|s| s.agility / 2 + armor.defense + shield.defense + scale_bonus)
            .unwrap_or(armor.defense + shield.defense + scale_bonus)
    }

    pub fn adjust_hp(&mut self, amount: i16) {
        if amount >= 0 {
            self.hp = (self.hp as i16 + amount).min(self.max_hp() as i16) as u8;
        } else {
            let damage = (-amount) as u8;
            self.hp = self.hp.saturating_sub(damage);
        }
    }

    pub fn item_list(&self) -> Vec<&'static str> {
        self.items
            .iter()
            .map(|&id| {
                ITEM_MASTER
                    .get(id as usize)
                    .map(|i| i.name)
                    .unwrap_or("なし")
            })
            .collect()
    }

    pub fn unified_item_list(&self) -> Vec<UnifiedItem> {
        let mut result = Vec::new();

        if self.herbs > 0 {
            result.push(UnifiedItem {
                id: 101,
                name: "やくそう",
                count: self.herbs,
                kind: ItemKind::Herb,
            });
        }
        if self.keys > 0 {
            result.push(UnifiedItem {
                id: 102,
                name: "かぎ",
                count: self.keys,
                kind: ItemKind::Key,
            });
        }
        for &item_id in &self.items {
            if let Some(item) = ITEM_MASTER.get(item_id as usize) {
                if item.name != "なし" {
                    result.push(UnifiedItem {
                        id: item_id,
                        name: item.name,
                        count: 1,
                        kind: ItemKind::Equipment,
                    });
                }
            }
        }
        result
    }

    pub fn is_unified_item_list(&self) -> bool {
        !self.unified_item_list().is_empty()
    }

    pub fn spell_list(&self) -> Vec<&'static SpellInfo> {
        spells_learned_by_level(self.level())
    }

    pub fn select_spell(&self, index: usize) -> &'static SpellInfo {
        self.spell_list()[index]
    }

    pub fn is_empty_spell_list(&self) -> bool {
        self.spell_list().is_empty()
    }

    pub fn consume_mp(&mut self, spell_info: &SpellInfo) {
        self.mp = self.mp - spell_info.mp_cost;
    }

    pub fn to_password_string(&self) -> Result<String, String> {
        let save = SaveData::new_with(SaveDataArgs {
            name: Some(self.name.clone()),
            experience: Some(self.exp),
            gold: Some(self.gold),
            weapon: Some(self.weapon),
            armor: Some(self.armor),
            shield: Some(self.shield),
            items: Some(self.items),
            herbs: Some(self.herbs),
            keys: Some(self.keys),
            flags: Some(self.flags.clone()),
            pattern: None,
            ..Default::default()
        });
        save.to_password_string()
    }

    pub fn hp_maximize(&mut self) {
        self.hp = self.max_hp()
    }

    pub fn mp_maximize(&mut self) {
        self.mp = self.max_mp()
    }

    pub fn maximize(&mut self) {
        self.exp = 65535;
        self.gold = 65535;
        self.weapon = 7;
        self.armor = 7;
        self.shield = 3;
        self.items = [4, 6, 7, 8, 10, 12, 13, 14]; // 要調整
        self.herbs = 6;
        self.keys = 6;
        self.flags = Flags {
            has_dragon_scale: true,
            has_warrior_ring: true,
            has_cursed_necklace: true,
            defeated_dragon: true,
            defeated_golem: true,
        };

        self.hp_maximize();
        self.mp_maximize();
    }

    pub fn summary(&self) -> PlayerSummary {
        PlayerSummary {
            name: self.name.clone(),
            level: self.level(),
            hp: self.hp,
            mp: self.mp,
            gold: self.gold,
            experience: self.exp,
        }
    }

    pub fn strength_status(&self) -> StrengthStatus {
        let status = self.status().unwrap_or(DEFAULT_STATUS.clone());
        let weapon = self.get_weapon();
        let armor = self.get_armor();
        let shield = self.get_shield();

        StrengthStatus {
            level: self.level(),
            strength: status.strength,
            agility: status.agility,
            max_hp: status.max_hp,
            max_mp: status.max_mp,
            attack_power: self.attack_power(),
            defense_power: self.defense_power(),
            weapon: weapon.name.to_string(),
            armor: armor.name.to_string(),
            shield: shield.name.to_string(),
        }
    }

    pub fn normal_damage(&self, monster: &Monster) -> u8 {
        let attack = self.attack_power();
        let defense = monster.stats.defense;
        let effective_defense = defense / 2;
        let base = attack.saturating_sub(effective_defense);
        let base_plus = base.saturating_add(1);
        let rand_val = random_value(255) as u32;
        let damage = ((rand_val * base_plus as u32) / 256 + base as u32) / 4;
        damage.min(255) as u8
    }

    pub fn critical_damage(&self) -> u8 {
        let attack = self.attack_power();
        let rand_val = random_value(255) as u32;
        let damage = attack as u32 - (attack as u32 / 2 * rand_val) / 256;
        damage.min(255) as u8
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn is_max_armor(&self) -> bool {
        self.armor == 7
    }

    pub fn is_magic_armor(&self) -> bool {
        self.armor == 6
    }

    /// Spell Damage Correction
    pub fn reduce_spell_damage(&self, base_damage: u8) -> u8 {
        if self.is_magic_armor() || self.is_max_armor() {
            ((base_damage as f32) * (2.0 / 3.0)).floor() as u8
        } else {
            base_damage
        }
    }

    /// Fire Damage Compensation
    pub fn reduce_fire_damage(&self, base_damage: u8) -> u8 {
        if self.is_max_armor() {
            ((base_damage as f32) * (2.0 / 3.0)).floor() as u8
        } else {
            base_damage
        }
    }

    pub fn use_herbs(&mut self) {
        let heal = generate_in_range(23, 30);
        self.adjust_hp(heal as i16);
        self.herbs -= 1;
    }

    pub fn equip_warrior_ring(&mut self) {
        self.flags.has_warrior_ring = true;
    }

    pub fn equip_cursed_belt(&mut self) {
        self.flags.has_cursed_necklace = true;
        self.is_curse_belt = true;
    }

    pub fn equip_cursed_necklace(&mut self) {
        self.flags.has_cursed_necklace = true;
        self.is_curse_necklace = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::status::DEFAULT_STATUS;

    #[test]
    fn test_player_default_level() {
        let player = Player::new_with(PlayerArgs {
            name: Some(DEFAULT_NAME.to_string()),
            ..Default::default()
        });

        assert_eq!(player.name, DEFAULT_NAME);
        assert_eq!(player.level(), 1);
        assert_eq!(player.exp, 0);
        assert!(player.hp > 0);
    }

    #[test]
    fn test_player_with_level() {
        let player = Player::new_with(PlayerArgs {
            name: Some("みやおう".to_string()),
            level: Some(10),
            ..Default::default()
        });

        assert_eq!(player.level(), 10);
        assert_eq!(player.exp, get_status_by_level(10).unwrap().required_exp);
    }

    #[test]
    fn test_player_with_exp() {
        let player = Player::new_with(PlayerArgs {
            name: Some("たけし".to_string()),
            exp: Some(8000),
            ..Default::default()
        });

        let expected_level = get_level_by_exp(8000);
        assert_eq!(player.level(), expected_level);
        assert_eq!(player.exp, 8000);
    }

    #[test]
    fn test_player_with_level_and_exp() {
        let player = Player::new_with(PlayerArgs {
            name: Some("こういち".to_string()),
            level: Some(5),
            exp: Some(10000),
            gold: Some(20000),
            ..Default::default()
        });

        assert!(player.level() >= 5);
        assert_eq!(player.exp, 10000);
        assert_eq!(player.gold, 20000);
    }

    #[test]
    fn test_player_maximize_password() {
        let mut player = Player::new("だい");
        player.maximize();
        let password = player.to_password_string().unwrap();
        assert_eq!(password, "へへみぞあうぞてえきいおくらちきこぜくゆ");
    }

    #[test]
    fn test_player_status_parameter() {
        let mut player_1 = Player::new("だい");
        let player_1_status = player_1.status().unwrap_or(DEFAULT_STATUS.clone());
        assert_eq!(player_1_status.strength, 4);
        assert_eq!(player_1_status.agility, 4);
        assert_eq!(player_1_status.max_hp, 14);
        assert_eq!(player_1_status.max_mp, 0);

        // maximize
        player_1.maximize();
        let player_1_max_status = player_1.status().unwrap_or(DEFAULT_STATUS.clone());
        assert_eq!(player_1_max_status.strength, 127);
        assert_eq!(player_1_max_status.agility, 130);
        assert_eq!(player_1_max_status.max_hp, 190);
        assert_eq!(player_1_max_status.max_mp, 200);

        let mut player_2 = Player::new("ゆうてい");
        let player_2_status = player_2.status().unwrap_or(DEFAULT_STATUS.clone());
        assert_eq!(player_2_status.strength, 4);
        assert_eq!(player_2_status.agility, 3);
        assert_eq!(player_2_status.max_hp, 15);
        assert_eq!(player_2_status.max_mp, 0);

        // maximize
        player_2.maximize();
        let player_2_max_status = player_2.status().unwrap_or(DEFAULT_STATUS.clone());
        assert_eq!(player_2_max_status.strength, 140);
        assert_eq!(player_2_max_status.agility, 117);
        assert_eq!(player_2_max_status.max_hp, 210);
        assert_eq!(player_2_max_status.max_mp, 180);
    }
}
