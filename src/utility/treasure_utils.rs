use crate::constants::item_weapon::{ARMOR_MASTER, ITEM_MASTER, WEAPON_MASTER};
use crate::constants::treasure::{TreasureContent, TreasureKind};

/// 宝箱 / しらべる ("chest" | "hidden")
pub fn treasure_kind_str(kind: TreasureKind) -> &'static str {
    match kind {
        TreasureKind::Chest => "chest",
        TreasureKind::Hidden => "hidden",
    }
}

/// 中身に対応するマスターの名前。やくそう・かぎ・ゴールドなどマスターにないものは None
pub fn treasure_content_master_name(content: TreasureContent) -> Option<&'static str> {
    let equipment = match content {
        TreasureContent::Item(i) => ITEM_MASTER.get(i as usize),
        TreasureContent::Weapon(i) => WEAPON_MASTER.get(i as usize),
        TreasureContent::Armor(i) => ARMOR_MASTER.get(i as usize),
        _ => None,
    };
    equipment.map(|e| e.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::map::{MAP_MASTER, find_map};
    use crate::constants::treasure::{TREASURE_MASTER, treasure_at, treasures_in_map};

    const TILE_CHEST: u8 = 12;

    fn tile(map_id: u8, x: u8, y: u8) -> u8 {
        let map = find_map(map_id).unwrap();
        map.tiles[y as usize * map.width as usize + x as usize]
    }

    #[test]
    fn test_chests_are_on_chest_tiles() {
        for t in TREASURE_MASTER
            .iter()
            .filter(|t| t.kind == TreasureKind::Chest)
        {
            assert_eq!(tile(t.map_id, t.x, t.y), TILE_CHEST, "{:?}", t);
        }
    }

    #[test]
    fn test_chest_count_matches_chest_tiles() {
        for map in MAP_MASTER.iter() {
            let tiles = map.tiles.iter().filter(|&&t| t == TILE_CHEST).count();
            let chests = treasures_in_map(map.id)
                .filter(|t| t.kind == TreasureKind::Chest)
                .count();
            assert_eq!(tiles, chests, "map {}", map.id);
        }
    }

    #[test]
    fn test_contents_match_masters() {
        for t in TREASURE_MASTER.iter() {
            let item = match t.content {
                TreasureContent::ItemOrGold { item, .. } => Some(TreasureContent::Item(item)),
                TreasureContent::Item(_)
                | TreasureContent::Weapon(_)
                | TreasureContent::Armor(_) => Some(t.content),
                _ => None,
            };
            let Some(item) = item else { continue };
            let name = treasure_content_master_name(item).unwrap_or_else(|| panic!("{:?}", t));
            assert_ne!(name, "なし", "{:?}", t);
            assert_eq!(t.name, Some(name), "{:?}", t);
        }
    }

    #[test]
    fn test_positions_are_unique_and_in_range() {
        for t in TREASURE_MASTER.iter() {
            let map = find_map(t.map_id).unwrap();
            assert!(t.x < map.width && t.y < map.height, "{:?}", t);
            assert_eq!(treasure_at(t.map_id, t.x, t.y), Some(t));
        }
    }
}
