use crate::constants::monster::{MONSTER_MASTER, MonsterAction};
use crate::monster::Monster;
use rand::Rng;
use std::collections::HashMap;

pub fn create_all_monsters() -> Vec<Monster> {
    (0..MONSTER_MASTER.len()).map(Monster::new).collect()
}

pub fn build_monster_name_index_map() -> HashMap<&'static str, usize> {
    let mut map = HashMap::new();
    for (index, monster) in MONSTER_MASTER.iter().enumerate() {
        map.insert(monster.name, index);
    }
    map
}

pub fn get_monster_by_name(name: &str) -> Option<Monster> {
    let name_map = build_monster_name_index_map();
    let index = name_map.get(name)?;
    Some(Monster::new(*index))
}

pub fn list_monster_names() -> Vec<&'static str> {
    MONSTER_MASTER.iter().map(|m| m.name).collect()
}

pub fn choose_action<'a>(candidates: &'a [MonsterAction]) -> Option<&'a MonsterAction> {
    let mut rng = rand::rng();
    candidates
        .iter()
        .find(|action| rng.random_bool(action.rate as f64 / 100.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::monster::ActionType;
    use crate::constants::spell::Spell;

    #[test]
    fn test_monster_new_valid_index() {
        let monster = Monster::new(0);
        assert_eq!(monster.stats.name, "スライム");
    }

    #[test]
    fn test_monster_new_invalid_index_defaults_to_slime() {
        let monster = Monster::new(999);
        assert_eq!(monster.stats.name, "スライム");
    }

    #[test]
    fn test_create_all_monsters_length() {
        let monsters = create_all_monsters();
        assert_eq!(monsters.len(), MONSTER_MASTER.len());
    }

    #[test]
    fn test_build_monster_name_index_map_contains_slime() {
        let map = build_monster_name_index_map();
        assert!(map.contains_key("スライム"));
    }

    #[test]
    fn test_get_monster_by_name_existing() {
        let monster = get_monster_by_name("スライム");
        assert!(monster.is_some());
        assert_eq!(monster.unwrap().stats.name, "スライム");
    }

    #[test]
    fn test_get_monster_by_name_non_existing() {
        let monster = get_monster_by_name("ファントム");
        assert!(monster.is_none());
    }

    #[test]
    fn test_list_monster_names_count() {
        let names = list_monster_names();
        assert_eq!(names.len(), MONSTER_MASTER.len());
    }

    #[test]
    fn test_list_monster_names_contains_slime() {
        let names = list_monster_names();
        assert!(names.contains(&"スライム"));
    }

    #[test]
    fn test_choose_action_always_selects_high_rate() {
        let candidates = vec![
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Hoimi),
                rate: 100,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Spell(Spell::Mahoton),
                rate: 0,
            },
        ];
        for _ in 0..10 {
            let result = choose_action(&candidates);
            assert!(result.is_some());
            assert_eq!(result.unwrap().ab_type, "A");
        }
    }

    #[test]
    fn test_choose_action_none_when_all_zero() {
        let candidates = vec![
            MonsterAction {
                ab_type: "A",
                action: ActionType::Spell(Spell::Hoimi),
                rate: 0,
            },
            MonsterAction {
                ab_type: "B",
                action: ActionType::Spell(Spell::Mahoton),
                rate: 0,
            },
        ];
        for _ in 0..10 {
            let result = choose_action(&candidates);
            assert!(result.is_none());
        }
    }
}
