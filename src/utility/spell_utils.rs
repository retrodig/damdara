use crate::constants::monster::ActionType;
use crate::constants::spell::{SPELL_INFO_LIST, Spell, SpellInfo};
use crate::utility::random_utils::generate_in_range;
use std::str::FromStr;

pub fn spells_learned_by_level(level: u8) -> Vec<&'static SpellInfo> {
    SPELL_INFO_LIST
        .iter()
        .filter(|info| info.learn_level <= level)
        .collect()
}

pub fn get_spell_info(spell: Spell) -> Option<&'static SpellInfo> {
    SPELL_INFO_LIST.iter().find(|info| info.spell == spell)
}

pub fn get_spell_info_by_name(name: &str) -> Option<&'static SpellInfo> {
    if let Ok(spell) = Spell::from_str(name) {
        get_spell_info(spell)
    } else {
        None
    }
}

pub fn player_spell_effect(spell: Spell) -> u8 {
    match spell {
        Spell::Hoimi => generate_in_range(10, 17),
        Spell::Gira => generate_in_range(5, 12),
        Spell::Behoimi => generate_in_range(85, 100),
        Spell::Begirama => generate_in_range(58, 65),
        _ => 0,
    }
}

pub fn monster_action_effect(action: &ActionType) -> u8 {
    match action {
        ActionType::Spell(spell) => match spell {
            Spell::Hoimi => generate_in_range(20, 27),
            Spell::Behoimi => generate_in_range(85, 100),
            Spell::Gira => generate_in_range(3, 10),
            Spell::Begirama => generate_in_range(30, 45),
            _ => 0,
        },
        ActionType::Special(name) => match *name {
            "ほのお(弱)" => generate_in_range(16, 23),
            "ほのお(強)" => generate_in_range(65, 72),
            _ => 0,
        },
    }
}
