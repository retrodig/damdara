use crate::constants::status::{DEFAULT_STATUS, STATUS_TABLE, Status};
use crate::growth_type::GrowthModifiers;

impl Status {
    pub fn pretty_string(&self) -> String {
        format!(
            "LV: {}\nちから: {}\nはやさ: {}\n最大HP: {}\n最大MP: {}\n必要EXP: {}\n呪文: {}",
            self.level,
            self.strength,
            self.agility,
            self.max_hp,
            self.max_mp,
            self.required_exp,
            self.spell.unwrap_or("なし")
        )
    }

    pub fn apply_abc_modifiers(&self, abc: &GrowthModifiers) -> Status {
        Status {
            level: self.level,
            strength: apply_modifier(self.strength, abc.c == 1, abc.a),
            agility: apply_modifier(self.agility, abc.b == 1, abc.a),
            max_hp: apply_modifier(self.max_hp, abc.b == 0, abc.a),
            max_mp: apply_modifier(self.max_mp, abc.c == 0, abc.a),
            required_exp: self.required_exp,
            spell: self.spell,
        }
    }
}

fn apply_modifier(base: u16, keep_base: bool, a: u16) -> u16 {
    if keep_base {
        base
    } else {
        ((base as f32 * 0.9).floor() as u16) + a
    }
}

pub fn get_status_by_level(level: u8) -> Option<&'static Status> {
    if (1..=30).contains(&level) {
        Some(&STATUS_TABLE[(level - 1) as usize])
    } else {
        None
    }
}

pub fn get_status_list() -> [Status; 30] {
    STATUS_TABLE
}

pub fn get_level_by_exp(exp: u16) -> u8 {
    for status in STATUS_TABLE.iter().rev() {
        if exp >= status.required_exp {
            return status.level;
        }
    }
    1
}

/// レベルに基づいた required_exp と、与えられた exp を比較し、高い方を返す
pub fn resolve_experience(level: u8, exp: Option<u16>) -> u16 {
    let base_required_exp = get_status_by_level(level)
        .unwrap_or(&DEFAULT_STATUS)
        .required_exp;

    exp.map(|e| e.max(base_required_exp))
        .unwrap_or(base_required_exp)
}

#[test]
fn test_get_status_by_level() {
    let s = get_status_by_level(3).unwrap();
    assert_eq!(s.level, 3);
    assert_eq!(s.strength, 7);
    assert_eq!(s.spell, Some("ホイミ"));

    assert!(get_status_by_level(0).is_none());
    assert!(get_status_by_level(31).is_none());
}
