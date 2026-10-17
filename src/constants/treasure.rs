// 品物は ITEM_MASTER、武器は WEAPON_MASTER、鎧は ARMOR_MASTER の添字。セリフ番号は DIALOGUE_MASTER の添字。
// 宝箱は取ると床になるが、フィールドに出ると元に戻る (中身も再び取れる)。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreasureKind {
    Chest,  // 宝箱 (とる)
    Hidden, // しらべると見つかる
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreasureContent {
    Item(u8),
    Herb,
    Key,
    Weapon(u8),
    Armor(u8),
    Gold {
        min: u16,
        max: u16,
    },
    // まだ一度も手に入れていなければ 1/one_in で item、それ以外は gold_min..=gold_max ゴールド
    ItemOrGold {
        item: u8,
        one_in: u8,
        gold_min: u16,
        gold_max: u16,
    },
    Tablet,       // 石板。読むと CHEST_TABLET のセリフ
    HiddenStairs, // 隠し階段が現れる
    Hint(u16),    // しらべるとセリフだけ出る
    Decoration,   // 飾りの宝箱 (とっても CHEST_NOT_A_TREASURE)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmptyIf {
    HerbsFull, // やくそう 6 個
    KeysFull,  // かぎ 6 個
    HasItem(u8),
    HasAnyItem(&'static [u8]),
    WeaponEquipped(u8),
    ArmorEquipped(u8),
    HiddenStairsFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Treasure {
    pub map_id: u8,
    pub x: u8,
    pub y: u8,
    pub kind: TreasureKind,
    pub content: TreasureContent,
    pub name: Option<&'static str>,
    pub empty_if: Option<EmptyIf>, // この条件のときは中身がない
}

pub fn treasure_at(map_id: u8, x: u8, y: u8) -> Option<&'static Treasure> {
    TREASURE_MASTER
        .iter()
        .find(|t| t.map_id == map_id && t.x == x && t.y == y)
}

pub fn treasures_in_map(map_id: u8) -> impl Iterator<Item = &'static Treasure> {
    TREASURE_MASTER.iter().filter(move |t| t.map_id == map_id)
}

pub const CHEST_FOUND_ITEM_DIALOGUE: u16 = 217;
pub const CHEST_FOUND_GOLD_DIALOGUE: u16 = 216;
pub const CHEST_EMPTY_DIALOGUE: u16 = 218;
pub const CHEST_NOT_A_TREASURE_DIALOGUE: u16 = 215;
pub const CHEST_TABLET_DIALOGUE: u16 = 259;
pub const CHEST_INVENTORY_FULL_DIALOGUE: u16 = 204;
pub const SEARCH_SEARCH_DIALOGUE: u16 = 210;
pub const SEARCH_FOUND_DIALOGUE: u16 = 213;
pub const SEARCH_NOTHING_DIALOGUE: u16 = 211;
pub const SEARCH_CHEST_HERE_DIALOGUE: u16 = 212;

pub const TREASURE_MASTER: [Treasure; 43] = [
    Treasure {
        map_id: 4,
        x: 1,
        y: 13,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 6, max: 13 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 4,
        x: 1,
        y: 15,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 6, max: 13 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 4,
        x: 2,
        y: 14,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 6, max: 13 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 4,
        x: 3,
        y: 15,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 6, max: 13 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 5,
        x: 4,
        y: 4,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 120, max: 120 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 5,
        x: 5,
        y: 4,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(1),
        name: Some("たいまつ"),
        empty_if: None,
    },
    Treasure {
        map_id: 5,
        x: 6,
        y: 1,
        kind: TreasureKind::Chest,
        content: TreasureContent::Key,
        name: Some("かぎ"),
        empty_if: Some(EmptyIf::KeysFull),
    },
    Treasure {
        map_id: 11,
        x: 24,
        y: 23,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(3),
        name: Some("キメラのつばさ"),
        empty_if: None,
    },
    Treasure {
        map_id: 9,
        x: 8,
        y: 5,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 10, max: 17 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 9,
        x: 8,
        y: 6,
        kind: TreasureKind::Chest,
        content: TreasureContent::Herb,
        name: Some("やくそう"),
        empty_if: Some(EmptyIf::HerbsFull),
    },
    Treasure {
        map_id: 9,
        x: 9,
        y: 5,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(1),
        name: Some("たいまつ"),
        empty_if: None,
    },
    Treasure {
        map_id: 6,
        x: 11,
        y: 11,
        kind: TreasureKind::Chest,
        content: TreasureContent::Herb,
        name: Some("やくそう"),
        empty_if: Some(EmptyIf::HerbsFull),
    },
    Treasure {
        map_id: 6,
        x: 11,
        y: 12,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 500, max: 755 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 6,
        x: 11,
        y: 13,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(3),
        name: Some("キメラのつばさ"),
        empty_if: None,
    },
    Treasure {
        map_id: 6,
        x: 12,
        y: 12,
        kind: TreasureKind::Chest,
        content: TreasureContent::Key,
        name: Some("かぎ"),
        empty_if: Some(EmptyIf::KeysFull),
    },
    Treasure {
        map_id: 6,
        x: 12,
        y: 13,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(9),
        name: Some("のろいのベルト"),
        empty_if: None,
    },
    Treasure {
        map_id: 6,
        x: 13,
        y: 13,
        kind: TreasureKind::Chest,
        content: TreasureContent::Herb,
        name: Some("やくそう"),
        empty_if: Some(EmptyIf::HerbsFull),
    },
    Treasure {
        map_id: 12,
        x: 4,
        y: 5,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(12),
        name: Some("たいようのいし"),
        empty_if: Some(EmptyIf::HasAnyItem(&[12, 14])),
    },
    Treasure {
        map_id: 13,
        x: 3,
        y: 4,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(13),
        name: Some("あまぐものつえ"),
        empty_if: None,
    },
    Treasure {
        map_id: 24,
        x: 11,
        y: 0,
        kind: TreasureKind::Chest,
        content: TreasureContent::Herb,
        name: Some("やくそう"),
        empty_if: Some(EmptyIf::HerbsFull),
    },
    Treasure {
        map_id: 24,
        x: 12,
        y: 0,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 5, max: 20 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 24,
        x: 13,
        y: 0,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 6, max: 13 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 26,
        x: 1,
        y: 1,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(9),
        name: Some("のろいのベルト"),
        empty_if: None,
    },
    Treasure {
        map_id: 26,
        x: 13,
        y: 6,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(10),
        name: Some("ぎんのたてごと"),
        empty_if: Some(EmptyIf::HasAnyItem(&[10, 13, 14])),
    },
    Treasure {
        map_id: 16,
        x: 5,
        y: 5,
        kind: TreasureKind::Chest,
        content: TreasureContent::Weapon(7),
        name: Some("ロトのつるぎ"),
        empty_if: Some(EmptyIf::WeaponEquipped(7)),
    },
    Treasure {
        map_id: 23,
        x: 1,
        y: 6,
        kind: TreasureKind::Chest,
        content: TreasureContent::ItemOrGold {
            item: 11,
            one_in: 32,
            gold_min: 100,
            gold_max: 131,
        },
        name: Some("しのくびかざり"),
        empty_if: None,
    },
    Treasure {
        map_id: 23,
        x: 3,
        y: 2,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(1),
        name: Some("たいまつ"),
        empty_if: None,
    },
    Treasure {
        map_id: 23,
        x: 2,
        y: 2,
        kind: TreasureKind::Chest,
        content: TreasureContent::Item(6),
        name: Some("せんしのゆびわ"),
        empty_if: None,
    },
    Treasure {
        map_id: 23,
        x: 10,
        y: 9,
        kind: TreasureKind::Chest,
        content: TreasureContent::Gold { min: 10, max: 17 },
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 22,
        x: 13,
        y: 5,
        kind: TreasureKind::Chest,
        content: TreasureContent::Herb,
        name: Some("やくそう"),
        empty_if: Some(EmptyIf::HerbsFull),
    },
    Treasure {
        map_id: 29,
        x: 9,
        y: 3,
        kind: TreasureKind::Chest,
        content: TreasureContent::Tablet,
        name: Some("ロトのせきばん"),
        empty_if: None,
    },
    Treasure {
        map_id: 7,
        x: 15,
        y: 20,
        kind: TreasureKind::Chest,
        content: TreasureContent::Decoration,
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 7,
        x: 15,
        y: 21,
        kind: TreasureKind::Chest,
        content: TreasureContent::Decoration,
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 7,
        x: 15,
        y: 22,
        kind: TreasureKind::Chest,
        content: TreasureContent::Decoration,
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 10,
        x: 27,
        y: 25,
        kind: TreasureKind::Chest,
        content: TreasureContent::Decoration,
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 10,
        x: 2,
        y: 27,
        kind: TreasureKind::Chest,
        content: TreasureContent::Decoration,
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 10,
        x: 27,
        y: 27,
        kind: TreasureKind::Chest,
        content: TreasureContent::Decoration,
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 14,
        x: 5,
        y: 5,
        kind: TreasureKind::Chest,
        content: TreasureContent::Decoration,
        name: None,
        empty_if: None,
    },
    Treasure {
        map_id: 1,
        x: 83,
        y: 113,
        kind: TreasureKind::Hidden,
        content: TreasureContent::Item(7),
        name: Some("ロトのしるし"),
        empty_if: Some(EmptyIf::HasItem(7)),
    },
    Treasure {
        map_id: 7,
        x: 9,
        y: 6,
        kind: TreasureKind::Hidden,
        content: TreasureContent::Item(5),
        name: Some("ようせいのふえ"),
        empty_if: Some(EmptyIf::HasItem(5)),
    },
    Treasure {
        map_id: 3,
        x: 18,
        y: 12,
        kind: TreasureKind::Hidden,
        content: TreasureContent::Armor(7),
        name: Some("ロトのよろい"),
        empty_if: Some(EmptyIf::ArmorEquipped(7)),
    },
    Treasure {
        map_id: 2,
        x: 10,
        y: 1,
        kind: TreasureKind::Hidden,
        content: TreasureContent::HiddenStairs,
        name: Some("かいだん"),
        empty_if: Some(EmptyIf::HiddenStairsFound),
    },
    Treasure {
        map_id: 2,
        x: 10,
        y: 3,
        kind: TreasureKind::Hidden,
        content: TreasureContent::Hint(214),
        name: None,
        empty_if: None,
    },
];
