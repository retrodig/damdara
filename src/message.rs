use crate::traits::message_output::MessageOutput;

#[derive(Debug, Default)]
pub struct Messages {
    messages: Vec<String>,
}

impl Messages {
    pub fn push(&mut self, msg: impl Into<String>) {
        self.messages.push(msg.into());
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn all(&self) -> &[String] {
        &self.messages
    }
}

pub struct BattleMessages<'a> {
    pub player_name: String,
    pub monster_name: String,
    pub messages: Vec<String>,
    pub output: &'a mut dyn MessageOutput,
}

impl<'a> BattleMessages<'a> {
    pub fn new(
        player_name: String,
        monster_name: String,
        output: &'a mut dyn MessageOutput,
    ) -> Self {
        Self {
            player_name,
            monster_name,
            messages: Vec::new(),
            output,
        }
    }

    pub fn push(&mut self, msg: impl Into<String>) {
        self.messages.push(msg.into());
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn all(&self) -> &[String] {
        &self.messages
    }

    pub fn display(&mut self) {
        for message in &self.messages {
            self.output.output(message);
        }
    }

    pub fn add_monster_appears(&mut self) {
        self.push(format!("{}があらわれた！", self.monster_name));
    }

    pub fn add_status(&mut self, player_hp: u8, player_mp: u8, monster_hp: u8) {
        self.push("".to_string());
        self.push(format!(
            "{} HP: {:?} MP: {:?}",
            self.player_name, player_hp, player_mp
        ));
        self.push(format!("{} HP: {:?}", self.monster_name, monster_hp));
        self.push("".to_string());
    }

    pub fn add_player_attack(&mut self, player_name: &str) {
        self.push(format!("{} のこうげき！", player_name));
    }

    pub fn add_monster_damage(&mut self, damage: u8) {
        self.push(format!("{}に {}ポイントの", self.monster_name, damage));
        self.push("ダメージを あたえた！".to_string());
    }

    pub fn add_player_damage(&mut self, damage: u8) {
        self.push(format!(" {}は {}ポイントの", self.player_name, damage));
        self.push(" ダメージを うけた！".to_string());
    }

    pub fn add_miss(&mut self) {
        self.push("ミス".to_string());
    }

    pub fn add_monster_attack(&mut self) {
        self.push(format!(" {} のこうげき！", self.monster_name));
    }

    pub fn add_use_spell(&mut self, spell_name: &str) {
        self.push(format!("{}は {}の", self.player_name, spell_name));
        self.push("じゅもんを となえた！".to_string());
    }

    pub fn add_monster_spell(&mut self, spell_name: &str) {
        self.push(format!(" {}は {}の", self.monster_name, spell_name));
        self.push(" じゅもんを となえた！".to_string());
    }

    pub fn add_spell_sealed(&mut self) {
        self.push("しかし じゅもんは".to_string());
        self.push("ふうじこまれている！".to_string());
    }

    pub fn add_empty_line(&mut self) {
        self.push("".to_string());
    }

    pub fn add_defeat_monster(&mut self, exp: u16, gold: u32) {
        self.push(format!("{} をたおした！", self.monster_name));
        self.push("".to_string());
        self.push(format!("けいけんち {}ポイントかくとく", exp));
        self.push(format!("{}ゴールドを てにいれた！", gold));
    }

    pub fn add_player_death(&mut self) {
        self.push("あなたは しにました".to_string());
    }

    pub fn display_command(&mut self) {
        self.push(format!("\n--- {}のターン ---", self.player_name));
        self.push("コマンド？".to_string());
        self.push("1: たたかう".to_string());
        self.push("2: じゅもん".to_string());
        self.push("3: どうぐ".to_string());
        self.push("4: にげる".to_string());
    }

    pub fn spells_sealed(&mut self) {
        self.push(format!("{}は じゅもんを", self.player_name));
        self.push("ふうじこめられた！".to_string());
    }

    pub fn can_not_use_spell(&mut self) {
        self.push(format!("{}は まだ じゅもんを", self.player_name));
        self.push("つかえない。".to_string());
    }

    pub fn can_not_use_by_battle(&mut self) {
        self.push("それは たたかいに つかえない！".to_string());
    }

    pub fn critical_damage(&mut self) {
        self.push("かいしんの いちげき！！".to_string());
    }

    pub fn monster_heal(&mut self) {
        self.push(format!(" {}は きずが", self.monster_name));
        self.push(" かいふくした！".to_string());
    }

    pub fn fall_asleep(&mut self) {
        self.push(format!("{}は ねむってしまった！", self.player_name));
    }

    pub fn wake_up(&mut self) {
        self.push(format!("{}は めをさました！", self.player_name));
    }

    pub fn still_asleep(&mut self) {
        self.push(format!("{}は ねむっている⋯⋯⋯", self.player_name));
    }

    pub fn monster_fall_asleep(&mut self) {
        self.push(format!("{}を ねむらせた!", self.monster_name));
    }

    pub fn monster_wake_up(&mut self) {
        self.push(format!("{}は めをさました！", self.monster_name));
    }

    pub fn monster_still_asleep(&mut self) {
        self.push(format!("{}は ねむっている⋯⋯⋯", self.monster_name));
    }

    pub fn enemy_special_skill(&mut self, name: &str, damage: u8) {
        self.push(format!(" {}は ", self.monster_name));
        if name.contains("ほのお") {
            self.push(" ほのおをはいた!".to_string());
        } else {
            self.push(format!(" {}を使った！", name));
        }
        self.add_empty_line();
        self.add_player_damage(damage);
    }

    pub fn spell_resisted(&mut self) {
        self.messages
            .push("しかし じゅもんは きかなかった！\n".to_string());
    }

    pub fn spell_sealed(&mut self) {
        self.messages
            .push("しかし じゅもんは ふうじこまれている！".to_string());
    }

    pub fn mp_not_enough(&mut self) {
        self.messages.push("MP が たりません。".to_string());
    }

    pub fn monster_escaped(&mut self) {
        self.push(format!("{} は にげだした！", self.monster_name));
    }

    pub fn seal_monster_spell(&mut self) {
        self.messages
            .push(format!("{}の じゅもんを", self.monster_name));
        self.messages.push("ふうじこめた！".to_string());
    }

    pub fn no_usable_items(&mut self) {
        self.messages.push("つかえるものを まだ".to_string());
        self.messages.push("もっていない。".to_string());
    }

    pub fn used_item_herbs(&mut self) {
        self.messages
            .push(format!("{}は やくそうを つかった！", self.player_name));
    }

    pub fn already_has_dragon_scale(&mut self) {
        self.messages.push("りゅうのうろこは すでに".to_string());
        self.messages.push("みにつけています。".to_string());
    }

    pub fn used_dragon_scale(&mut self) {
        self.messages
            .push(format!("{}は りゅうのうろこを", self.player_name));
        self.messages.push("みにつけた。".to_string());
    }

    pub fn used_flute(&mut self) {
        self.messages
            .push(format!("{}は ふえをふいた。", self.player_name));
    }

    pub fn used_flute_monster_fell_asleep(&mut self) {
        self.messages
            .push(format!("{}は しずかに めをとじる⋯", self.monster_name));
        self.messages.push("ねむってしまった！".to_string());
    }

    pub fn nothing_happened(&mut self) {
        self.messages
            .push("しかし なにも おきなかった。".to_string());
    }

    pub fn used_warrior_ring(&mut self) {
        self.messages
            .push(format!("{}は せんしのゆびわを", self.player_name));
        self.messages.push("はめた。".to_string());
    }

    pub fn reequipped_warrior_ring(&mut self) {
        self.messages
            .push(format!("{}は せんしのゆびわを", self.player_name));
        self.messages.push("はめなおした。".to_string());
    }

    pub fn used_cursed_belt(&mut self) {
        self.messages
            .push(format!("{}は のろいのベルトを", self.player_name));
        self.messages.push("みにつけた。".to_string());
    }

    pub fn cursed_belt_constricting(&mut self) {
        self.messages.push("のろいのベルトが あなたの".to_string());
        self.messages.push("からだを しめつけている。".to_string());
    }

    pub fn cursed_belt_activated(&mut self) {
        self.messages.push("のろいのベルトが あなたの".to_string());
        self.messages.push("からだを しめつける。".to_string());
        self.messages
            .push("あなたは のろわれてしまった。".to_string());
    }

    pub fn used_lyre_and_monster_rejoiced(&mut self) {
        self.messages
            .push(format!("{}は たてごとを かなでた。", self.player_name));
        self.messages
            .push(format!("{}は うれしそうだ!", self.monster_name));
    }

    pub fn used_cursed_necklace(&mut self) {
        self.messages
            .push(format!("{}は しのくびかざりを", self.player_name));
        self.messages.push("みにつけた。".to_string());
    }

    pub fn cursed_necklace_constricting(&mut self) {
        self.messages.push("しのくびかざりが あなたの".to_string());
        self.messages.push("からだを しめつけている。".to_string());
    }

    pub fn cursed_necklace_activated(&mut self) {
        self.messages.push("しのくびかざりが あなたの".to_string());
        self.messages.push("からだを しめつける。".to_string());
        self.messages
            .push("あなたは のろわれてしまった。".to_string());
    }

    pub fn player_escaped(&mut self) {
        self.messages
            .push(format!("{}は にげだした！", self.player_name));
    }

    pub fn escape_blocked(&mut self) {
        self.messages
            .push("しかし まわりこまれてしまった！ \n".to_string());
    }
}
