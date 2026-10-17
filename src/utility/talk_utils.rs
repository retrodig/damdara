use crate::constants::item_weapon::{
    ARMOR_MASTER, Equipment, ITEM_MASTER, SHIELD_MASTER, WEAPON_MASTER,
};
use crate::constants::talk::ShopItem;

/// 品物の種類 ("weapon" | "armor" | "shield" | "item" | "herb")
pub fn shop_item_kind(item: ShopItem) -> &'static str {
    match item {
        ShopItem::Weapon(_) => "weapon",
        ShopItem::Armor(_) => "armor",
        ShopItem::Shield(_) => "shield",
        ShopItem::Item(_) => "item",
        ShopItem::Herb => "herb",
    }
}

/// 各マスターの添字。やくそうはマスターにないため None
pub fn shop_item_index(item: ShopItem) -> Option<u8> {
    match item {
        ShopItem::Weapon(i) | ShopItem::Armor(i) | ShopItem::Shield(i) | ShopItem::Item(i) => {
            Some(i)
        }
        ShopItem::Herb => None,
    }
}

/// 品物に対応するマスターのデータ。やくそうは None
pub fn shop_item_equipment(item: ShopItem) -> Option<&'static Equipment> {
    let master: &[Equipment] = match item {
        ShopItem::Weapon(_) => &WEAPON_MASTER,
        ShopItem::Armor(_) => &ARMOR_MASTER,
        ShopItem::Shield(_) => &SHIELD_MASTER,
        ShopItem::Item(_) => &ITEM_MASTER,
        ShopItem::Herb => return None,
    };
    master.get(shop_item_index(item)? as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::dialogue::DIALOGUE_MASTER;
    use crate::constants::map::MAP_MASTER;
    use crate::constants::talk::{TALK_MASTER, TalkKind, find_talk};

    #[test]
    fn test_every_npc_has_talk() {
        for map in MAP_MASTER.iter() {
            for npc in map.npcs {
                let talk = find_talk(npc.talk_id)
                    .unwrap_or_else(|| panic!("map {} talk_id {}", map.id, npc.talk_id));
                assert_eq!(talk.dialogue_id, npc.dialogue_id, "talk_id {}", npc.talk_id);
            }
        }
    }

    #[test]
    fn test_shop_items_match_masters() {
        for talk in TALK_MASTER.iter() {
            for entry in talk.items {
                match shop_item_equipment(entry.item) {
                    Some(equipment) => {
                        assert_eq!(equipment.name, entry.name, "talk_id {}", talk.talk_id);
                        assert_eq!(equipment.price, entry.price, "talk_id {}", talk.talk_id);
                    }
                    None => assert_eq!(entry.item, ShopItem::Herb, "talk_id {}", talk.talk_id),
                }
                assert_ne!(shop_item_index(entry.item), Some(0), "「なし」は売らない");
            }
        }
    }

    #[test]
    fn test_talk_fields_by_kind() {
        for talk in TALK_MASTER.iter() {
            let is_shop = matches!(talk.kind, TalkKind::WeaponShop | TalkKind::ItemShop);
            assert_eq!(!talk.items.is_empty(), is_shop, "talk_id {}", talk.talk_id);

            let has_price = matches!(
                talk.kind,
                TalkKind::KeyShop | TalkKind::FairyWaterShop | TalkKind::Inn
            );
            assert_eq!(talk.price.is_some(), has_price, "talk_id {}", talk.talk_id);
            assert_eq!(
                talk.role.is_some(),
                talk.kind == TalkKind::Event,
                "talk_id {}",
                talk.talk_id
            );

            let ids = talk
                .dialogue_id
                .iter()
                .chain(&talk.yes_dialogue_id)
                .chain(&talk.no_dialogue_id)
                .chain(talk.related_dialogue_ids);
            for &id in ids {
                assert!(
                    (id as usize) < DIALOGUE_MASTER.len(),
                    "talk_id {}",
                    talk.talk_id
                );
            }
        }
    }

    #[test]
    fn test_chimera_wing_price() {
        let wing = ITEM_MASTER
            .iter()
            .find(|i| i.name == "キメラのつばさ")
            .unwrap();
        assert_eq!((wing.price, wing.sell), (70, 35));
    }
}
