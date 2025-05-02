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

#[derive(Debug, Default)]
pub struct BattleMessages {
    pub player_name: String,
    pub monster_name: String,
    pub messages: Vec<String>,
}

impl BattleMessages {
    pub fn new(player_name: String, monster_name: String) -> Self {
        Self {
            player_name,
            monster_name,
            messages: Vec::new(),
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

    pub fn display(&self) {
        for message in &self.messages {
            println!("{}", message);
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
}

impl std::fmt::Display for BattleMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for message in &self.messages {
            writeln!(f, "{}", message)?;
        }
        Ok(())
    }
}
