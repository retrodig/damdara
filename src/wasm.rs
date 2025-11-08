// WASM module for Damdara
// This module provides WebAssembly bindings for the core game logic

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use crate::player::Player;
use crate::output::buffer_output::BufferOutput;
use crate::input::web_input::WebInput;
use crate::constants::status::{PlayerSummary, StrengthStatus, STATUS_TABLE};
use crate::constants::battle::PlayerAction;
use crate::constants::monster::MONSTER_MASTER;
use crate::constants::item_weapon::{ITEM_MASTER, WEAPON_MASTER, ARMOR_MASTER, SHIELD_MASTER};
use crate::constants::spell::SPELL_INFO_LIST;
use crate::battle::Battle;
use crate::monster::Monster;

/// Main WASM game interface
#[wasm_bindgen]
pub struct WasmGame {
    player: Option<Player>,
    output_buffer: BufferOutput,
    web_input: WebInput,
}

/// Player state for JavaScript
#[derive(Serialize, Deserialize)]
pub struct PlayerState {
    pub summary: PlayerSummary,
    pub strength_status: StrengthStatus,
    pub items: Vec<String>,
}

/// Battle result for JavaScript
#[derive(Serialize, Deserialize)]
pub struct BattleResult {
    pub player_survived: bool,
    pub monster_defeated: bool,
    pub player_escaped: bool,
    pub monster_escaped: bool,
    pub messages: Vec<String>,
    pub final_player_state: PlayerState,
}

/// Monster data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct MonsterData {
    pub id: usize,
    pub name: String,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub exp: u8,
    pub gold: u8,
}

/// Equipment data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct EquipmentData {
    pub id: usize,
    pub name: String,
    pub price: u16,
    pub sell: u16,
    pub attack: u8,
    pub defense: u8,
}

/// Spell data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct SpellData {
    pub id: usize,
    pub name: String,
    pub learn_level: u8,
    pub mp_cost: u8,
    pub description: String,
}

/// Status data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct StatusData {
    pub level: u8,
    pub strength: u8,
    pub agility: u8,
    pub max_hp: u8,
    pub max_mp: u8,
    pub required_exp: u16,
    pub spell: Option<String>,
}

#[wasm_bindgen]
impl WasmGame {
    /// Create a new WasmGame instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set panic hook for better error messages in the browser console
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        Self {
            player: None,
            output_buffer: BufferOutput::new(),
            web_input: WebInput::new(),
        }
    }

    /// Create a new player with the given name
    /// Returns PlayerState as JsValue
    pub fn create_player(&mut self, name: &str) -> Result<JsValue, JsValue> {
        let player = Player::new(name);

        let state = PlayerState {
            summary: player.summary(),
            strength_status: player.strength_status(),
            items: player.item_list().iter().map(|s| s.to_string()).collect(),
        };

        self.player = Some(player);

        serde_wasm_bindgen::to_value(&state)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Generate a password from the current player state
    pub fn generate_password(&self) -> Result<String, JsValue> {
        match &self.player {
            Some(player) => {
                player.to_password_string()
                    .map_err(|e| JsValue::from_str(&e))
            }
            None => Err(JsValue::from_str("プレイヤーが作成されていません"))
        }
    }

    /// Load player from a password string
    pub fn load_from_password(&mut self, password: &str) -> Result<JsValue, JsValue> {
        let player = Player::from_password_string(password)
            .map_err(|e| JsValue::from_str(&e))?;

        let state = PlayerState {
            summary: player.summary(),
            strength_status: player.strength_status(),
            items: player.item_list().iter().map(|s| s.to_string()).collect(),
        };

        self.player = Some(player);

        serde_wasm_bindgen::to_value(&state)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get current player state
    pub fn get_player_state(&self) -> Result<JsValue, JsValue> {
        match &self.player {
            Some(player) => {
                let state = PlayerState {
                    summary: player.summary(),
                    strength_status: player.strength_status(),
                    items: player.item_list().iter().map(|s| s.to_string()).collect(),
                };

                serde_wasm_bindgen::to_value(&state)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            }
            None => Err(JsValue::from_str("プレイヤーが作成されていません"))
        }
    }

    /// Check if player exists
    pub fn has_player(&self) -> bool {
        self.player.is_some()
    }

    /// Get accumulated messages
    pub fn get_messages(&self) -> Vec<String> {
        self.output_buffer.buffer.clone()
    }

    /// Clear message buffer
    pub fn clear_messages(&mut self) {
        self.output_buffer.buffer.clear();
    }

    /// Queue a battle action (Attack, Spell, Item, Escape)
    /// Actions should be queued before running battle
    pub fn queue_battle_action(&mut self, action_str: &str) -> Result<(), JsValue> {
        let action = match action_str.to_lowercase().as_str() {
            "attack" => PlayerAction::Attack,
            "spell" => PlayerAction::Spell,
            "item" => PlayerAction::Item,
            "escape" => PlayerAction::Escape,
            _ => return Err(JsValue::from_str(&format!("Invalid action: {}", action_str))),
        };
        self.web_input.push_action(action);
        Ok(())
    }

    /// Queue a numeric input (for menu selections like spell/item choice)
    pub fn queue_battle_input(&mut self, value: usize) {
        self.web_input.push_input(value);
    }

    /// Run a complete battle with pre-queued actions
    /// Actions must be queued beforehand using queue_battle_action and queue_battle_input
    /// Returns BattleResult as JsValue
    pub fn run_battle(&mut self, monster_id: usize) -> Result<JsValue, JsValue> {
        let player = self.player.take()
            .ok_or_else(|| JsValue::from_str("プレイヤーが作成されていません"))?;

        let monster = Monster::new(monster_id);

        // Clear previous messages
        self.output_buffer.buffer.clear();

        // Create and run battle
        let mut battle = Battle::new(
            player,
            monster,
            &mut self.web_input,
            &mut self.output_buffer,
        );

        battle.start();

        // Extract results
        let player_survived = battle.player.is_alive();
        let monster_defeated = !battle.monster.is_alive();
        let player_escaped = battle.player_state.escaped;
        let monster_escaped = battle.monster_state.escaped;

        // Extract player back
        let final_player = battle.player;

        let player_state = PlayerState {
            summary: final_player.summary(),
            strength_status: final_player.strength_status(),
            items: final_player.item_list().iter().map(|s| s.to_string()).collect(),
        };

        // Store player back
        self.player = Some(final_player);

        let result = BattleResult {
            player_survived,
            monster_defeated,
            player_escaped,
            monster_escaped,
            messages: self.output_buffer.buffer.clone(),
            final_player_state: player_state,
        };

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Clear battle input queue
    pub fn clear_battle_input(&mut self) {
        self.web_input.clear();
    }

    /// Get all monster data
    pub fn get_monsters(&self) -> Result<JsValue, JsValue> {
        let monsters: Vec<MonsterData> = MONSTER_MASTER
            .iter()
            .enumerate()
            .map(|(id, stats)| MonsterData {
                id,
                name: stats.name.to_string(),
                hp: stats.hp,
                attack: stats.attack,
                defense: stats.defense,
                exp: stats.exp,
                gold: stats.gold,
            })
            .collect();

        serde_wasm_bindgen::to_value(&monsters)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all weapon data
    pub fn get_weapons(&self) -> Result<JsValue, JsValue> {
        let weapons: Vec<EquipmentData> = WEAPON_MASTER
            .iter()
            .enumerate()
            .map(|(id, eq)| EquipmentData {
                id,
                name: eq.name.to_string(),
                price: eq.price,
                sell: eq.sell,
                attack: eq.attack,
                defense: eq.defense,
            })
            .collect();

        serde_wasm_bindgen::to_value(&weapons)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all armor data
    pub fn get_armors(&self) -> Result<JsValue, JsValue> {
        let armors: Vec<EquipmentData> = ARMOR_MASTER
            .iter()
            .enumerate()
            .map(|(id, eq)| EquipmentData {
                id,
                name: eq.name.to_string(),
                price: eq.price,
                sell: eq.sell,
                attack: eq.attack,
                defense: eq.defense,
            })
            .collect();

        serde_wasm_bindgen::to_value(&armors)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all shield data
    pub fn get_shields(&self) -> Result<JsValue, JsValue> {
        let shields: Vec<EquipmentData> = SHIELD_MASTER
            .iter()
            .enumerate()
            .map(|(id, eq)| EquipmentData {
                id,
                name: eq.name.to_string(),
                price: eq.price,
                sell: eq.sell,
                attack: eq.attack,
                defense: eq.defense,
            })
            .collect();

        serde_wasm_bindgen::to_value(&shields)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all item data (tools, keys, etc.)
    pub fn get_items(&self) -> Result<JsValue, JsValue> {
        let items: Vec<EquipmentData> = ITEM_MASTER
            .iter()
            .enumerate()
            .map(|(id, eq)| EquipmentData {
                id,
                name: eq.name.to_string(),
                price: eq.price,
                sell: eq.sell,
                attack: eq.attack,
                defense: eq.defense,
            })
            .collect();

        serde_wasm_bindgen::to_value(&items)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all spell data
    pub fn get_spells(&self) -> Result<JsValue, JsValue> {
        let spells: Vec<SpellData> = SPELL_INFO_LIST
            .iter()
            .enumerate()
            .map(|(id, spell_info)| SpellData {
                id,
                name: spell_info.spell.as_str().to_string(),
                learn_level: spell_info.learn_level,
                mp_cost: spell_info.mp_cost,
                description: spell_info.description.to_string(),
            })
            .collect();

        serde_wasm_bindgen::to_value(&spells)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all status progression data (level 1-30)
    pub fn get_status_table(&self) -> Result<JsValue, JsValue> {
        let statuses: Vec<StatusData> = STATUS_TABLE
            .iter()
            .map(|status| StatusData {
                level: status.level,
                strength: status.strength,
                agility: status.agility,
                max_hp: status.max_hp,
                max_mp: status.max_mp,
                required_exp: status.required_exp,
                spell: status.spell.map(|s| s.to_string()),
            })
            .collect();

        serde_wasm_bindgen::to_value(&statuses)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

// Default implementation
impl Default for WasmGame {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_create_player() {
        let mut game = WasmGame::new();
        let result = game.create_player("ゆうてい");
        assert!(result.is_ok());
        assert!(game.has_player());
    }

    #[wasm_bindgen_test]
    fn test_password_roundtrip() {
        let mut game = WasmGame::new();
        game.create_player("だい").unwrap();

        let password = game.generate_password().unwrap();
        assert_eq!(password.chars().count(), 20);

        let mut game2 = WasmGame::new();
        let result = game2.load_from_password(&password);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_no_player_error() {
        let game = WasmGame::new();
        let result = game.generate_password();
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_battle_integration() {
        let mut game = WasmGame::new();
        game.create_player("ゆうてい").unwrap();

        // Queue battle actions (simple attack-only strategy)
        for _ in 0..10 {
            game.queue_battle_action("attack").unwrap();
        }

        // Run battle against slime (monster_id 0)
        let result = game.run_battle(0);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_battle_queue_actions() {
        let mut game = WasmGame::new();
        game.create_player("ゆうてい").unwrap();

        // Test queueing different action types
        assert!(game.queue_battle_action("attack").is_ok());
        assert!(game.queue_battle_action("spell").is_ok());
        assert!(game.queue_battle_action("item").is_ok());
        assert!(game.queue_battle_action("escape").is_ok());

        // Test invalid action
        assert!(game.queue_battle_action("invalid").is_err());

        // Clear queue
        game.clear_battle_input();
    }

    #[wasm_bindgen_test]
    fn test_battle_without_player() {
        let mut game = WasmGame::new();
        // No player created

        game.queue_battle_action("attack").unwrap();
        let result = game.run_battle(0);
        assert!(result.is_err());
    }
}
