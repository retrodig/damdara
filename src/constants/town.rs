// このファイルは rom解析/tools/gen_rust.py で自動生成したもの。直接編集しないこと。
// データ出典: ファミコン版『ドラゴンクエスト』ROM。町の名前は ROM に格納されていないため慣用名。

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Town {
    pub map_id: u8,
    pub name: &'static str,
    pub name_en: &'static str,
    pub kind: &'static str,               // "town" | "castle" | "shrine"
    pub world_entrance: Option<(u8, u8)>, // フィールド上の入口 (x, y)
    pub npc_count: u8,
    pub dialogue_ids: &'static [u16], // DIALOGUE_MASTER の添字
}

pub const TOWN_MASTER: [Town; 9] = [
    Town {
        map_id: 3,
        name: "ドムドーラ",
        name_en: "Hauksness",
        kind: "town",
        world_entrance: Some((25, 89)),
        npc_count: 0,
        dialogue_ids: &[],
    },
    Town {
        map_id: 4,
        name: "ラダトームのしろ",
        name_en: "Tantegel Castle",
        kind: "castle",
        world_entrance: Some((43, 43)),
        npc_count: 20,
        dialogue_ids: &[69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81],
    },
    Town {
        map_id: 7,
        name: "マイラ",
        name_en: "Kol",
        kind: "town",
        world_entrance: Some((104, 10)),
        npc_count: 16,
        dialogue_ids: &[93, 95, 97, 98, 99, 100, 101, 102, 103, 104, 141, 142],
    },
    Town {
        map_id: 8,
        name: "ラダトーム",
        name_en: "Brecconary",
        kind: "town",
        world_entrance: Some((48, 41)),
        npc_count: 20,
        dialogue_ids: &[84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 140],
    },
    Town {
        map_id: 9,
        name: "ガライ",
        name_en: "Garinham",
        kind: "town",
        world_entrance: Some((2, 2)),
        npc_count: 15,
        dialogue_ids: &[105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115],
    },
    Town {
        map_id: 10,
        name: "メルキド",
        name_en: "Cantlin",
        kind: "town",
        world_entrance: Some((73, 102)),
        npc_count: 20,
        dialogue_ids: &[116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 143],
    },
    Town {
        map_id: 11,
        name: "リムルダール",
        name_en: "Rimuldar",
        kind: "town",
        world_entrance: Some((102, 72)),
        npc_count: 20,
        dialogue_ids: &[
            95, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 144,
        ],
    },
    Town {
        map_id: 13,
        name: "あまぐものつえのほこら",
        name_en: "Staff of Rain Shrine",
        kind: "shrine",
        world_entrance: Some((81, 1)),
        npc_count: 1,
        dialogue_ids: &[],
    },
    Town {
        map_id: 14,
        name: "にじのしずくのほこら",
        name_en: "Rainbow Drop Shrine",
        kind: "shrine",
        world_entrance: Some((108, 109)),
        npc_count: 1,
        dialogue_ids: &[],
    },
];
