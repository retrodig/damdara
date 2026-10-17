use crate::constants::dialogue::DIALOGUE_MASTER;

pub fn get_dialogue(id: usize) -> Option<&'static str> {
    DIALOGUE_MASTER.get(id).copied()
}

/// セリフ中の {player} を主人公の名前に置き換える
/// {word} {number} {wait} はそのまま残す
pub fn render_dialogue(id: usize, player_name: &str) -> Option<String> {
    get_dialogue(id).map(|text| text.replace("{player}", player_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dialogue_existing() {
        assert_eq!(get_dialogue(1), Some("あなたは　しにました"));
    }

    #[test]
    fn test_get_dialogue_out_of_range() {
        assert!(get_dialogue(DIALOGUE_MASTER.len()).is_none());
    }

    #[test]
    fn test_render_dialogue_replaces_player() {
        let text = render_dialogue(4, "だい").unwrap();
        assert!(text.starts_with("だいは　にじのしずくを"));
        assert!(!text.contains("{player}"));
    }

    #[test]
    fn test_render_dialogue_keeps_other_variables() {
        let text = render_dialogue(11, "だい").unwrap();
        assert!(text.contains("{number}"));
    }
}
