use crate::constants::battle::PlayerAction;
use crate::traits::player_input::PlayerInput;
pub struct CliInput;
use std::io::{self, Write};

impl PlayerInput for CliInput {
    fn get_player_input(&mut self, max: usize) -> usize {
        loop {
            print!("番号を選んでください (0-{}): ", max);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if let Ok(_) = io::stdin().read_line(&mut input) {
                if let Ok(num) = input.trim().parse::<usize>() {
                    if num <= max {
                        return num;
                    }
                }
            }
            println!("無効な入力です。もう一度入力してください。");
        }
    }

    fn get_player_action(&mut self, display_commands: &mut dyn FnMut()) -> PlayerAction {
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => return PlayerAction::Attack,
                "2" => return PlayerAction::Spell,
                "3" => return PlayerAction::Item,
                "4" => return PlayerAction::Escape,
                _ => {
                    println!("無効な入力です。もう一度選んでください。");
                    display_commands();
                    continue;
                    // self.get_player_action(display_commands) // 再帰的にリトライ
                }
            }
        }
    }
}
