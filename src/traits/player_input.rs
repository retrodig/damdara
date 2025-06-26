use crate::constants::battle::PlayerAction;

pub trait PlayerInput {
    fn get_player_input(&mut self, max: usize) -> usize;
    fn get_player_action(&mut self, display_commands: &mut dyn FnMut()) -> PlayerAction;
}
