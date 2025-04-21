pub struct SaveData {
    pub name: String, // 4文字ひらがな
    pub experience: u16,
    pub gold: u16,
    pub weapon: u8,     // 0–7
    pub armor: u8,      // 0–7
    pub shield: u8,     // 0–3
    pub items: [u8; 8], // 各 0–15
    pub herbs: u8,      // 0–15
    pub keys: u8,       // 0–15
    pub has_dragon_scale: bool,
    pub has_warrior_ring: bool,
    pub has_cursed_necklace: bool,
    pub defeated_dragon: bool,
    pub defeated_golem: bool,
    pub pattern: u8, // 0–7
}
