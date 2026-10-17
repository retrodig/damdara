use crate::constants::dialogue::DIALOGUE_MASTER;
use crate::constants::town::{TOWN_MASTER, Town};

pub fn find_town_by_map_id(map_id: u8) -> Option<&'static Town> {
    TOWN_MASTER.iter().find(|t| t.map_id == map_id)
}

pub fn list_town_names() -> Vec<&'static str> {
    TOWN_MASTER.iter().map(|t| t.name).collect()
}

pub fn town_dialogues(town: &Town) -> Vec<&'static str> {
    town.dialogue_ids
        .iter()
        .filter_map(|&id| DIALOGUE_MASTER.get(id as usize).copied())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_town_by_map_id_existing() {
        let town = find_town_by_map_id(4).unwrap();
        assert_eq!(town.name, "ラダトームのしろ");
        assert_eq!(town.kind, "castle");
    }

    #[test]
    fn test_find_town_by_map_id_non_existing() {
        assert!(find_town_by_map_id(1).is_none());
    }

    #[test]
    fn test_list_town_names_count() {
        assert_eq!(list_town_names().len(), TOWN_MASTER.len());
    }

    #[test]
    fn test_town_dialogue_ids_in_range() {
        for town in TOWN_MASTER.iter() {
            assert_eq!(town_dialogues(town).len(), town.dialogue_ids.len());
        }
    }
}
