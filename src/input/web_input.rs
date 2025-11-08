use crate::constants::battle::PlayerAction;
use crate::traits::player_input::PlayerInput;
use std::collections::VecDeque;

/// Web-based input handler that uses action queues
/// Actions are pushed from JavaScript and consumed by the game logic
#[derive(Default)]
pub struct WebInput {
    /// Queue for numeric inputs (menu selections)
    input_queue: VecDeque<usize>,
    /// Queue for player actions (battle commands)
    action_queue: VecDeque<PlayerAction>,
}

impl WebInput {
    /// Create a new WebInput instance
    pub fn new() -> Self {
        Self {
            input_queue: VecDeque::new(),
            action_queue: VecDeque::new(),
        }
    }

    /// Push a numeric input to the queue
    pub fn push_input(&mut self, value: usize) {
        self.input_queue.push_back(value);
    }

    /// Push a player action to the queue
    pub fn push_action(&mut self, action: PlayerAction) {
        self.action_queue.push_back(action);
    }

    /// Check if there are pending inputs
    pub fn has_pending_input(&self) -> bool {
        !self.input_queue.is_empty()
    }

    /// Check if there are pending actions
    pub fn has_pending_action(&self) -> bool {
        !self.action_queue.is_empty()
    }

    /// Clear all queues
    pub fn clear(&mut self) {
        self.input_queue.clear();
        self.action_queue.clear();
    }
}

impl PlayerInput for WebInput {
    fn get_player_input(&mut self, max: usize) -> usize {
        // Pop from queue if available, otherwise return 0 as default
        self.input_queue.pop_front().unwrap_or(0).min(max)
    }

    fn get_player_action(&mut self, _display_commands: &mut dyn FnMut()) -> PlayerAction {
        // Pop from queue if available, otherwise return Attack as default
        self.action_queue.pop_front().unwrap_or(PlayerAction::Attack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_input_new() {
        let input = WebInput::new();
        assert!(!input.has_pending_input());
        assert!(!input.has_pending_action());
    }

    #[test]
    fn test_push_and_get_input() {
        let mut input = WebInput::new();
        input.push_input(5);
        input.push_input(3);

        assert!(input.has_pending_input());
        assert_eq!(input.get_player_input(10), 5);
        assert_eq!(input.get_player_input(10), 3);
        assert!(!input.has_pending_input());
    }

    #[test]
    fn test_input_max_constraint() {
        let mut input = WebInput::new();
        input.push_input(15);

        // Should be clamped to max value
        assert_eq!(input.get_player_input(10), 10);
    }

    #[test]
    fn test_push_and_get_action() {
        let mut input = WebInput::new();
        input.push_action(PlayerAction::Spell);
        input.push_action(PlayerAction::Escape);

        assert!(input.has_pending_action());

        let mut dummy_display = || {};
        assert_eq!(input.get_player_action(&mut dummy_display), PlayerAction::Spell);
        assert_eq!(input.get_player_action(&mut dummy_display), PlayerAction::Escape);
        assert!(!input.has_pending_action());
    }

    #[test]
    fn test_default_values() {
        let mut input = WebInput::new();
        let mut dummy_display = || {};

        // Should return defaults when queue is empty
        assert_eq!(input.get_player_input(10), 0);
        assert_eq!(input.get_player_action(&mut dummy_display), PlayerAction::Attack);
    }

    #[test]
    fn test_clear() {
        let mut input = WebInput::new();
        input.push_input(5);
        input.push_action(PlayerAction::Item);

        assert!(input.has_pending_input());
        assert!(input.has_pending_action());

        input.clear();

        assert!(!input.has_pending_input());
        assert!(!input.has_pending_action());
    }
}
