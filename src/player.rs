use crate::constants::status::{DEFAULT_STATUS, Status, get_status_by_level};
use crate::growth_type::{GrowthModifiers, calculate_abc, calculate_name_total};
use crate::save::{SaveData, SaveDataArgs};
use crate::string_utils::name_normalize;

pub struct Player {
    pub name: String,
    pub level: u8,
    pub hp: u16,
    pub mp: u16,
    pub exp: u16,
}

impl Player {
    pub fn new(name: &str, level: Option<u8>) -> Self {
        let name = name_normalize(name);
        let level = level.unwrap_or(1);
        println!("{:?}", calculate_name_total(&name));

        let abc = calculate_abc(calculate_name_total(&name));
        println!("{:?}", abc);
        let base = get_status_by_level(level).unwrap_or(&DEFAULT_STATUS);
        let adjusted = base.apply_abc_modifiers(&abc);

        Self {
            name,
            level,
            hp: adjusted.max_hp,
            mp: adjusted.max_mp,
            exp: adjusted.required_exp,
        }
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
        get_status_by_level(self.level)
    }

    pub fn adjusted_status(&self) -> Option<Status> {
        self.base_status()
            .map(|s| s.apply_abc_modifiers(&self.abc()))
    }

    pub fn to_password_string(&self) -> Result<String, String> {
        let save = SaveData::new_with(SaveDataArgs {
            name: Some(self.name.clone()),
            experience: Some(self.exp),
            ..Default::default()
        });
        save.to_password_string()
    }
}
