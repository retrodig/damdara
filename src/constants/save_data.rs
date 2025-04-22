use crate::constants::text::DEFAULT_NAME;

// bits[0]  = Checksum
// bits[1]  = experience lower 8 bits
// bits[2]  = pattern bit 2 (1), cursed necklace (1), name[2] index (6)
// bits[3]  = item[3] (4), item[2] (4)
// bits[4]  = gold lower 8 bits
// bits[5]  = name[0] (6), golem (1), pattern bit 1 (1)
// bits[6]  = item[7] (4), item[6] (4)
// bits[7]  = pattern bit 0 (1), defeated_dragon (1), name[3] (6)
// bits[8]  = weapon (3), armor (3), shield (2)
// bits[9]  = gold upper 8 bits
// bits[10] = keys (4), herbs (4)
// bits[11] = item[5] (4), item[4] (4)
// bits[12] = experience upper 8 bits
// bits[13] = has_dragon_scale (1), name[1] (6), has_warrior_ring (1)
// bits[14] = item[1] (4), item[0] (4)

#[derive(Debug)]
pub struct SaveData {
    pub name: String,              // 名前: 6bit×4文字
    pub experience: u16,           // 経験値: 0–65535 （16bit）
    pub gold: u16,                 // ゴールド: 0–65535 （16bit）
    pub weapon: u8,                // ぶき: 0–7 （3bit）
    pub armor: u8,                 // よろい: 0–7 （3bit）
    pub shield: u8,                // たて: 0–3 （2bit）
    pub items: [u8; 8],            // アイテム: 各 0–15（4bit×8）
    pub herbs: u8,                 // やくそうの数: 0–6（4bit）
    pub keys: u8,                  // かぎの数: 0–6（4bit）
    pub has_dragon_scale: bool,    // りゅうのうろこを装備したか
    pub has_warrior_ring: bool,    // せんしのゆびわを装備したか
    pub has_cursed_necklace: bool, // しのくびかざりを入手したか
    pub defeated_dragon: bool,     // ドラゴンを倒したか
    pub defeated_golem: bool,      // ゴーレムを倒したか
    pub pattern: u8,               // パターン: 0–7
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            name: DEFAULT_NAME.to_string(),
            experience: 0,
            gold: 0,
            weapon: 0,
            armor: 0,
            shield: 0,
            items: [0; 8],
            herbs: 0,
            keys: 0,
            has_dragon_scale: false,
            has_warrior_ring: false,
            has_cursed_necklace: false,
            defeated_dragon: false,
            defeated_golem: false,
            pattern: 0,
        }
    }
}

pub struct SaveDataArgs {
    pub name: Option<String>,
    pub experience: Option<u16>,
    pub gold: Option<u16>,
    pub weapon: Option<u8>,
    pub armor: Option<u8>,
    pub shield: Option<u8>,
    pub items: Option<[u8; 8]>,
    pub herbs: Option<u8>,
    pub keys: Option<u8>,
    pub has_dragon_scale: Option<bool>,
    pub has_warrior_ring: Option<bool>,
    pub has_cursed_necklace: Option<bool>,
    pub defeated_dragon: Option<bool>,
    pub defeated_golem: Option<bool>,
    pub pattern: Option<u8>,
}

impl Default for SaveDataArgs {
    fn default() -> Self {
        Self {
            name: None,
            experience: None,
            gold: None,
            weapon: None,
            armor: None,
            shield: None,
            items: None,
            herbs: None,
            keys: None,
            has_dragon_scale: None,
            has_warrior_ring: None,
            has_cursed_necklace: None,
            defeated_dragon: None,
            defeated_golem: None,
            pattern: None,
        }
    }
}
