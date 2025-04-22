use crate::constants::save_data::{SaveData, SaveDataArgs};
use crate::constants::status::Flags;
use crate::constants::status::{DEFAULT_STATUS, Status, get_level_by_exp, get_status_by_level};
use crate::constants::text::DEFAULT_NAME;
use crate::growth_type::{GrowthModifiers, calculate_abc, calculate_name_total};
use crate::utility::string_utils::name_normalize;

pub struct Player {
    pub name: String,
    pub hp: u16,
    pub mp: u16,
    pub exp: u16,
    pub gold: u16,
    pub weapon: u8,
    pub armor: u8,
    pub shield: u8,
    pub items: [u8; 8],
    pub herbs: u8,
    pub keys: u8,
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
        let gold = args.gold.unwrap_or(0);

        let base_status = get_status_by_level(base_level).unwrap_or(&DEFAULT_STATUS);
        let abc = calculate_abc(calculate_name_total(&name));

        let mut final_exp = args.exp.unwrap_or(base_status.required_exp);
        if let Some(user_exp) = args.exp {
            final_exp = final_exp.max(user_exp);
        }

        let level = get_level_by_exp(final_exp);
        let status = get_status_by_level(level).unwrap_or(&DEFAULT_STATUS);
        let adjusted = status.apply_abc_modifiers(&abc);

        Self {
            name,
            hp: adjusted.max_hp,
            mp: adjusted.max_mp,
            exp: final_exp,
            gold,
            weapon: args.weapon.unwrap_or(0),
            armor: args.armor.unwrap_or(0),
            shield: args.shield.unwrap_or(0),
            items: args.items.unwrap_or([0; 8]),
            herbs: args.herbs.unwrap_or(0),
            keys: args.keys.unwrap_or(0),
            flags: args.flags.unwrap_or_default(),
        }
    }

    pub fn level(&self) -> u8 {
        get_level_by_exp(self.exp)
    }

    pub fn get_status_by_level(&self, level: u8) -> Option<Status> {
        get_status_by_level(level).map(|base| base.apply_abc_modifiers(&self.abc()))
    }

    pub fn name_total(&self) -> u16 {
        calculate_name_total(&self.name)
    }

    pub fn strength(&self) -> Option<u16> {
        self.adjusted_status().map(|s| s.strength)
    }

    pub fn agility(&self) -> Option<u16> {
        self.adjusted_status().map(|s| s.agility)
    }

    pub fn abc(&self) -> GrowthModifiers {
        calculate_abc(self.name_total())
    }

    pub fn base_status(&self) -> Option<&Status> {
        get_status_by_level(self.level())
    }

    pub fn adjusted_status(&self) -> Option<Status> {
        self.base_status()
            .map(|s| s.apply_abc_modifiers(&self.abc()))
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
            has_dragon_scale: Some(self.flags.has_dragon_scale),
            has_warrior_ring: Some(self.flags.has_warrior_ring),
            has_cursed_necklace: Some(self.flags.has_cursed_necklace),
            defeated_dragon: Some(self.flags.defeated_dragon),
            defeated_golem: Some(self.flags.defeated_golem),
            pattern: None,
            ..Default::default()
        });
        save.to_password_string()
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
