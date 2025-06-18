use crate::constants::battle::PlayerAction;

pub trait PlayerInput {
    fn get_player_action(&mut self) -> PlayerAction;
}
