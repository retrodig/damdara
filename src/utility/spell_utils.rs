use crate::constants::spell::{SPELL_INFO_LIST, Spell, SpellInfo};
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
