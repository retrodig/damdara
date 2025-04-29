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
    player_name: &str,
    monster_name: &str,
    messages: Vec<String>,
}

impl BattleMessages {
    pub fn push(&mut self, msg: impl Into<String>) {
        self.messages.push(msg.into());
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn all(&self) -> &[String] {
        &self.messages
    }

    // pub fn display_status(&self) {
    //     println!();
    //     println!(
    //         "{} HP: {:?} MP: {:?}",
    //         self.player_name, self.player.hp, self.player.mp
    //     );
    //     println!("{} HP: {:?}", self.monster_name, self.monster.hp);
    //     println!();
    // }
}
