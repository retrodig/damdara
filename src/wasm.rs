// WASM module for Damdara
// This module provides WebAssembly bindings for the core game logic

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use crate::player::Player;
use crate::output::buffer_output::BufferOutput;
use crate::input::web_input::WebInput;
use crate::constants::status::{PlayerSummary, StrengthStatus, STATUS_TABLE};
use crate::constants::battle::{BattleState, PlayerAction};
use crate::constants::monster::MONSTER_MASTER;
use crate::constants::item_weapon::{ITEM_MASTER, WEAPON_MASTER, ARMOR_MASTER, SHIELD_MASTER};
use crate::constants::spell::SPELL_INFO_LIST;
use crate::constants::town::TOWN_MASTER;
use crate::constants::dialogue::DIALOGUE_MASTER;
use crate::constants::map::{GameMap, MAP_MASTER, TILE_MASTER, WARP_MASTER, find_map};
use crate::utility::dialogue_utils::render_dialogue;
use crate::constants::battle_background::{BATTLE_BACKDROP, battle_background};
use crate::constants::enemy_sprite::{Layer, NES_PALETTE};
use crate::constants::event::{Action, Condition, Step, find_event_script};
use crate::event::{EventStatus, StoryState, run_event};
use crate::utility::random_utils::random_value;
use crate::constants::talk::{ShopEntry, TALK_MASTER, Talk, find_talk};
use crate::utility::talk_utils::{shop_item_index, shop_item_kind};
use crate::utility::battle_background_utils::{
    battle_background_pixels, battle_pixels_rgba, battle_scene,
};
use crate::constants::field_sprite::{
    CHARACTER_SPRITE_MASTER, MAP_TILE_SPRITE_MASTER, PaletteSet, hero_character_id,
    map_tile_pixels, npc_character_id,
};
use crate::utility::field_sprite_utils::{
    field_pixels_rgba, find_character_sprite, palette_set_name, render_map_rgba,
};
use crate::utility::enemy_sprite_utils::{
    EnemySpriteSummary, enemy_sprite_rgba, enemy_sprite_summaries, find_enemy_sprite, monster_name,
};
use crate::utility::map_utils::{MapSummary, tile_at};
use crate::battle::Battle;
use crate::monster::Monster;

/// Main WASM game interface
#[wasm_bindgen]
pub struct WasmGame {
    player: Option<Player>,
    output_buffer: BufferOutput,
    web_input: WebInput,
    battle_session: Option<BattleSession>,
}

/// Battle state kept between turns (used by start_battle / battle_turn)
struct BattleSession {
    monster: Monster,
    player_state: BattleState,
    monster_state: BattleState,
    turn: u32,
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

/// Turn-based battle result for JavaScript
#[derive(Serialize, Deserialize)]
pub struct BattleTurnResult {
    pub turn: u32,
    pub is_finished: bool,
    pub player_survived: bool,
    pub monster_defeated: bool,
    pub player_escaped: bool,
    pub monster_escaped: bool,
    pub monster_id: u8,
    pub monster_name: String,
    pub monster_hp: u8,
    pub player_sleep: bool,
    pub player_seal: bool,
    pub monster_sleep: bool,
    pub monster_seal: bool,
    pub messages: Vec<String>,
    pub player_state: PlayerState,
}

fn parse_player_action(action_str: &str) -> Result<PlayerAction, JsValue> {
    match action_str.to_lowercase().as_str() {
        "attack" => Ok(PlayerAction::Attack),
        "spell" => Ok(PlayerAction::Spell),
        "item" => Ok(PlayerAction::Item),
        "escape" => Ok(PlayerAction::Escape),
        _ => Err(JsValue::from_str(&format!("Invalid action: {}", action_str))),
    }
}

/// Enemy sprite metadata for JavaScript (without pixels)
#[derive(Serialize, Deserialize)]
pub struct EnemySpriteSummaryData {
    pub id: u8,
    pub name: String,
    pub mirror: bool,
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
    pub part_count: usize,
}

impl From<EnemySpriteSummary> for EnemySpriteSummaryData {
    fn from(summary: EnemySpriteSummary) -> Self {
        Self {
            id: summary.id,
            name: summary.name.to_string(),
            mirror: summary.mirror,
            x: summary.x,
            y: summary.y,
            width: summary.width,
            height: summary.height,
            part_count: summary.part_count,
        }
    }
}

/// Enemy sprite part (8x8 tile) for JavaScript
#[derive(Serialize, Deserialize)]
pub struct SpritePartData {
    pub layer: String,
    pub tile: u8,
    pub pattern: u16,
    pub x: u8,
    pub y: u8,
    pub palette: u8,
    pub flip_h: bool,
    pub flip_v: bool,
}

/// Enemy sprite data for JavaScript (with pixels)
#[derive(Serialize, Deserialize)]
pub struct EnemySpriteData {
    pub id: u8,
    pub name: String,
    pub mirror: bool,
    pub palette: Vec<Vec<u8>>,
    pub bg_palette: Option<Vec<Vec<u8>>>,
    pub parts: Vec<SpritePartData>,
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
    pub pixels: Vec<u8>,
}

/// Event step (tree) for JavaScript
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StepData {
    Say {
        dialogue_id: u16,
    },
    Ask {
        dialogue_id: u16,
        yes: Vec<StepData>,
        no: Vec<StepData>,
    },
    AskUntilYes {
        dialogue_id: u16,
        yes: Vec<StepData>,
        no: Vec<StepData>,
    },
    If {
        condition: ConditionData,
        then: Vec<StepData>,
        otherwise: Vec<StepData>,
    },
    Random {
        choices: Vec<RandomChoiceData>,
    },
    Do {
        action: ActionData,
    },
}

#[derive(Serialize, Deserialize)]
pub struct RandomChoiceData {
    pub weight: u8,
    pub steps: Vec<StepData>,
}

/// Event condition for JavaScript. `items` are ITEM_MASTER indexes, `weapon` is a WEAPON_MASTER index
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConditionData {
    PrincessRescued,
    CarryingPrincess,
    LeftThroneRoom,
    HasItem { item: u8 },
    HasAnyItem { items: Vec<u8> },
    HasAllItems { items: Vec<u8> },
    WeaponEquipped { weapon: u8 },
    RingEquipped,
    Cursed,
    InventoryFull,
    MaxLevel,
}

/// Event action for JavaScript. `monster_id` is a MONSTER_MASTER index
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionData {
    RemoveCurses,
    RestoreMp,
    Flash,
    TakeItem { item: u8 },
    GiveItem { item: u8 },
    PrincessTakesItem { items: Vec<u8> },
    NpcDisappears,
    LeaveMap,
    DeliverPrincess,
    TellPassword,
    ExpToNextLevel,
    ResetPlayer,
    GameOver,
    Battle { monster_id: u8 },
}

fn steps_data(steps: &[Step]) -> Vec<StepData> {
    steps.iter().map(StepData::from).collect()
}

impl From<&Step> for StepData {
    fn from(step: &Step) -> Self {
        match *step {
            Step::Say(dialogue_id) => StepData::Say { dialogue_id },
            Step::Ask {
                dialogue_id,
                yes,
                no,
            } => StepData::Ask {
                dialogue_id,
                yes: steps_data(yes),
                no: steps_data(no),
            },
            Step::AskUntilYes {
                dialogue_id,
                yes,
                no,
            } => StepData::AskUntilYes {
                dialogue_id,
                yes: steps_data(yes),
                no: steps_data(no),
            },
            Step::If {
                condition,
                then,
                otherwise,
            } => StepData::If {
                condition: condition.into(),
                then: steps_data(then),
                otherwise: steps_data(otherwise),
            },
            Step::Random(choices) => StepData::Random {
                choices: choices
                    .iter()
                    .map(|&(weight, steps)| RandomChoiceData {
                        weight,
                        steps: steps_data(steps),
                    })
                    .collect(),
            },
            Step::Do(action) => StepData::Do {
                action: action.into(),
            },
        }
    }
}

impl From<Condition> for ConditionData {
    fn from(condition: Condition) -> Self {
        match condition {
            Condition::PrincessRescued => ConditionData::PrincessRescued,
            Condition::CarryingPrincess => ConditionData::CarryingPrincess,
            Condition::LeftThroneRoom => ConditionData::LeftThroneRoom,
            Condition::HasItem(item) => ConditionData::HasItem { item },
            Condition::HasAnyItem(items) => ConditionData::HasAnyItem {
                items: items.to_vec(),
            },
            Condition::HasAllItems(items) => ConditionData::HasAllItems {
                items: items.to_vec(),
            },
            Condition::WeaponEquipped(weapon) => ConditionData::WeaponEquipped { weapon },
            Condition::RingEquipped => ConditionData::RingEquipped,
            Condition::Cursed => ConditionData::Cursed,
            Condition::InventoryFull => ConditionData::InventoryFull,
            Condition::MaxLevel => ConditionData::MaxLevel,
        }
    }
}

impl From<Action> for ActionData {
    fn from(action: Action) -> Self {
        match action {
            Action::RemoveCurses => ActionData::RemoveCurses,
            Action::RestoreMp => ActionData::RestoreMp,
            Action::Flash => ActionData::Flash,
            Action::TakeItem(item) => ActionData::TakeItem { item },
            Action::GiveItem(item) => ActionData::GiveItem { item },
            Action::PrincessTakesItem(items) => ActionData::PrincessTakesItem {
                items: items.to_vec(),
            },
            Action::NpcDisappears => ActionData::NpcDisappears,
            Action::LeaveMap => ActionData::LeaveMap,
            Action::DeliverPrincess => ActionData::DeliverPrincess,
            Action::TellPassword => ActionData::TellPassword,
            Action::ExpToNextLevel => ActionData::ExpToNextLevel,
            Action::ResetPlayer => ActionData::ResetPlayer,
            Action::GameOver => ActionData::GameOver,
            Action::Battle(monster_id) => ActionData::Battle { monster_id },
        }
    }
}

/// One line of an event for JavaScript
#[derive(Serialize, Deserialize)]
pub struct EventLineData {
    pub dialogue_id: u16,
    pub text: String,
}

/// Result of run_event for JavaScript
#[derive(Serialize, Deserialize)]
pub struct EventResultData {
    pub lines: Vec<EventLineData>,
    pub actions: Vec<ActionData>,
    /// "finished" | "waiting_answer"
    pub status: String,
}

/// Battle background for JavaScript
#[derive(Serialize, Deserialize)]
pub struct BattleBackgroundData {
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
    pub tiles: Vec<u8>,
    pub palette: Vec<Vec<u8>>,
    pub block_palettes: Vec<u8>,
    pub reveal_order: Vec<(u8, u8)>,
    pub pixels: Vec<u8>,
}

/// Battle scene (background + monster) for JavaScript
#[derive(Serialize, Deserialize)]
pub struct BattleSceneData {
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
    /// RGBA bytes (width * height * 4)
    pub rgba: Vec<u8>,
}

/// Map tile sprite (16x16) for JavaScript
#[derive(Serialize, Deserialize)]
pub struct MapTileSpriteData {
    pub id: u8,
    pub tiles: [u8; 4],
    pub palette: u8,
    /// Palette set name -> pixels ("field" | "town" | "dungeon")
    pub pixels: Vec<(String, Vec<u8>)>,
}

/// Character sprite part (8x8 tile) for JavaScript
#[derive(Serialize, Deserialize)]
pub struct CharacterPartData {
    pub tile: u8,
    pub x: u8,
    pub y: u8,
    pub palette: u8,
    pub flip_h: bool,
    pub flip_v: bool,
}

/// Character sprite frame (16x16) for JavaScript
#[derive(Serialize, Deserialize)]
pub struct CharacterFrameData {
    pub parts: Vec<CharacterPartData>,
    pub pixels: Vec<u8>,
}

/// Character sprite for JavaScript
#[derive(Serialize, Deserialize)]
pub struct CharacterSpriteData {
    pub id: u8,
    pub name: String,
    pub name_en: String,
    pub frames: Vec<CharacterFrameData>,
}

/// Town data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct TownData {
    pub map_id: u8,
    pub name: String,
    pub name_en: String,
    pub kind: String,
    pub world_entrance: Option<(u8, u8)>,
    pub npc_count: u8,
    pub dialogue_ids: Vec<u16>,
}

/// Dialogue data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct DialogueData {
    pub id: usize,
    pub text: String,
}

/// Tile data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct TileData {
    pub id: u8,
    pub name: String,
    pub name_en: String,
}

/// NPC data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct NpcData {
    pub x: u8,
    pub y: u8,
    pub sprite: u8,
    pub direction: u8,
    pub moving: bool,
    pub talk_id: u8,
    pub dialogue_id: Option<u16>,
    /// "weapon_shop" | "item_shop" | "key_shop" | "fairy_water_shop" | "inn" | "message" | "question" | "event"
    pub talk_kind: Option<String>,
    pub items: Vec<ShopItemData>,
    pub price: Option<u16>,
    pub role: Option<String>,
}

/// Shop item for JavaScript
#[derive(Serialize, Deserialize)]
pub struct ShopItemData {
    /// "weapon" | "armor" | "shield" | "item" | "herb"
    pub kind: String,
    /// Index of WEAPON_MASTER / ARMOR_MASTER / SHIELD_MASTER / ITEM_MASTER (none for herb)
    pub index: Option<u8>,
    pub name: String,
    pub price: u16,
}

impl From<&ShopEntry> for ShopItemData {
    fn from(entry: &ShopEntry) -> Self {
        Self {
            kind: shop_item_kind(entry.item).to_string(),
            index: shop_item_index(entry.item),
            name: entry.name.to_string(),
            price: entry.price,
        }
    }
}

/// Talk (what an NPC does when spoken to) for JavaScript
#[derive(Serialize, Deserialize)]
pub struct TalkData {
    pub talk_id: u8,
    pub kind: String,
    pub items: Vec<ShopItemData>,
    pub price: Option<u16>,
    pub dialogue_id: Option<u16>,
    pub yes_dialogue_id: Option<u16>,
    pub no_dialogue_id: Option<u16>,
    pub role: Option<String>,
    pub role_en: Option<String>,
    pub related_dialogue_ids: Vec<u16>,
}

impl From<&Talk> for TalkData {
    fn from(talk: &Talk) -> Self {
        Self {
            talk_id: talk.talk_id,
            kind: talk.kind.as_str().to_string(),
            items: talk.items.iter().map(ShopItemData::from).collect(),
            price: talk.price,
            dialogue_id: talk.dialogue_id,
            yes_dialogue_id: talk.yes_dialogue_id,
            no_dialogue_id: talk.no_dialogue_id,
            role: talk.role.map(str::to_string),
            role_en: talk.role_en.map(str::to_string),
            related_dialogue_ids: talk.related_dialogue_ids.to_vec(),
        }
    }
}

/// Map metadata for JavaScript (without tiles)
#[derive(Serialize, Deserialize)]
pub struct MapSummaryData {
    pub id: u8,
    pub name: String,
    pub name_en: String,
    pub kind: String,
    pub width: u8,
    pub height: u8,
    pub border_tile: u8,
    pub npc_count: usize,
}

/// Map data for JavaScript (with tiles)
#[derive(Serialize, Deserialize)]
pub struct MapData {
    pub id: u8,
    pub name: String,
    pub name_en: String,
    pub kind: String,
    pub width: u8,
    pub height: u8,
    pub border_tile: u8,
    pub tiles: Vec<u8>,
    pub npcs: Vec<NpcData>,
}

/// Warp data for JavaScript
#[derive(Serialize, Deserialize)]
pub struct WarpData {
    pub from: (u8, u8, u8),
    pub to: (u8, u8, u8),
}

impl From<&GameMap> for MapSummaryData {
    fn from(map: &GameMap) -> Self {
        let summary = MapSummary::from(map);
        Self {
            id: summary.id,
            name: summary.name.to_string(),
            name_en: summary.name_en.to_string(),
            kind: summary.kind.to_string(),
            width: summary.width,
            height: summary.height,
            border_tile: summary.border_tile,
            npc_count: summary.npc_count,
        }
    }
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
            battle_session: None,
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
        let action = parse_player_action(action_str)?;
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

    /// Start a turn-based battle
    /// Returns BattleTurnResult (turn 0: monster appears)
    /// Starting a new battle discards the battle in progress
    pub fn start_battle(&mut self, monster_id: usize) -> Result<JsValue, JsValue> {
        if self.player.is_none() {
            return Err(JsValue::from_str("プレイヤーが作成されていません"));
        }
        self.battle_session = Some(BattleSession {
            monster: Monster::new(monster_id),
            player_state: BattleState::default(),
            monster_state: BattleState::default(),
            turn: 0,
        });
        self.step_battle(None, None)
    }

    /// Run one turn (player -> monster) of the battle started by start_battle
    /// action: "attack" | "spell" | "item" | "escape"
    /// selection: spell / item number (1-based) when action is "spell" or "item"
    pub fn battle_turn(
        &mut self,
        action: &str,
        selection: Option<usize>,
    ) -> Result<JsValue, JsValue> {
        let action = parse_player_action(action)?;
        if matches!(action, PlayerAction::Spell | PlayerAction::Item)
            && selection.unwrap_or(0) == 0
        {
            return Err(JsValue::from_str("selection (1以上) を指定してください"));
        }
        if self.battle_session.is_none() {
            return Err(JsValue::from_str("戦闘中ではありません"));
        }
        self.step_battle(Some(action), selection)
    }

    /// Check if a turn-based battle is in progress
    pub fn is_in_battle(&self) -> bool {
        self.battle_session.is_some()
    }

    /// Abort the turn-based battle in progress (player state is kept)
    pub fn end_battle(&mut self) {
        self.battle_session = None;
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

    /// Get all town / castle / shrine data
    pub fn get_towns(&self) -> Result<JsValue, JsValue> {
        let towns: Vec<TownData> = TOWN_MASTER
            .iter()
            .map(|town| TownData {
                map_id: town.map_id,
                name: town.name.to_string(),
                name_en: town.name_en.to_string(),
                kind: town.kind.to_string(),
                world_entrance: town.world_entrance,
                npc_count: town.npc_count,
                dialogue_ids: town.dialogue_ids.to_vec(),
            })
            .collect();

        serde_wasm_bindgen::to_value(&towns)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all dialogue data (variables such as {player} are not replaced)
    pub fn get_dialogues(&self) -> Result<JsValue, JsValue> {
        let dialogues: Vec<DialogueData> = DIALOGUE_MASTER
            .iter()
            .enumerate()
            .map(|(id, text)| DialogueData {
                id,
                text: text.to_string(),
            })
            .collect();

        serde_wasm_bindgen::to_value(&dialogues)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get a dialogue with {player} replaced by the given name
    pub fn render_dialogue(&self, id: usize, player_name: &str) -> Option<String> {
        render_dialogue(id, player_name)
    }

    /// Get all map metadata (without tiles)
    pub fn get_maps(&self) -> Result<JsValue, JsValue> {
        let maps: Vec<MapSummaryData> = MAP_MASTER.iter().map(MapSummaryData::from).collect();

        serde_wasm_bindgen::to_value(&maps)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get a map by id (with tiles and NPCs)
    pub fn get_map(&self, id: u8) -> Result<JsValue, JsValue> {
        let map = find_map(id).ok_or_else(|| JsValue::from_str(&format!("Map not found: {}", id)))?;
        let data = MapData {
            id: map.id,
            name: map.name.to_string(),
            name_en: map.name_en.to_string(),
            kind: map.kind.to_string(),
            width: map.width,
            height: map.height,
            border_tile: map.border_tile,
            tiles: map.tiles.to_vec(),
            npcs: map
                .npcs
                .iter()
                .map(|npc| {
                    let talk = find_talk(npc.talk_id);
                    NpcData {
                        x: npc.x,
                        y: npc.y,
                        sprite: npc.sprite,
                        direction: npc.direction,
                        moving: npc.moving,
                        talk_id: npc.talk_id,
                        dialogue_id: npc.dialogue_id,
                        talk_kind: talk.map(|t| t.kind.as_str().to_string()),
                        items: talk
                            .map(|t| t.items.iter().map(ShopItemData::from).collect())
                            .unwrap_or_default(),
                        price: talk.and_then(|t| t.price),
                        role: talk.and_then(|t| t.role).map(str::to_string),
                    }
                })
                .collect(),
        };

        serde_wasm_bindgen::to_value(&data)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get tile id at (x, y). Out of range returns the map's border tile
    pub fn tile_at(&self, map_id: u8, x: i32, y: i32) -> Option<u8> {
        tile_at(map_id, x, y)
    }

    /// Get all tile definitions
    pub fn get_tiles(&self) -> Result<JsValue, JsValue> {
        let tiles: Vec<TileData> = TILE_MASTER
            .iter()
            .map(|tile| TileData {
                id: tile.id,
                name: tile.name.to_string(),
                name_en: tile.name_en.to_string(),
            })
            .collect();

        serde_wasm_bindgen::to_value(&tiles)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all enemy sprite metadata (without pixels)
    pub fn get_enemy_sprites(&self) -> Result<JsValue, JsValue> {
        let sprites: Vec<EnemySpriteSummaryData> = enemy_sprite_summaries()
            .into_iter()
            .map(EnemySpriteSummaryData::from)
            .collect();

        serde_wasm_bindgen::to_value(&sprites)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get an enemy sprite by monster id (pixels are color numbers, 255 is transparent)
    pub fn get_enemy_sprite(&self, id: u8) -> Result<JsValue, JsValue> {
        let sprite = find_enemy_sprite(id)
            .ok_or_else(|| JsValue::from_str(&format!("Sprite not found: {}", id)))?;
        let data = EnemySpriteData {
            id: sprite.id,
            name: monster_name(sprite.id).to_string(),
            mirror: sprite.mirror,
            palette: sprite.palette.iter().map(|p| p.to_vec()).collect(),
            bg_palette: sprite
                .bg_palette
                .map(|bg| bg.iter().map(|p| p.to_vec()).collect()),
            parts: sprite
                .parts
                .iter()
                .map(|part| SpritePartData {
                    layer: match part.layer {
                        Layer::Sprite => "sprite",
                        Layer::Bg => "bg",
                    }
                    .to_string(),
                    tile: part.tile,
                    pattern: part.pattern,
                    x: part.x,
                    y: part.y,
                    palette: part.palette,
                    flip_h: part.flip_h,
                    flip_v: part.flip_v,
                })
                .collect(),
            x: sprite.x,
            y: sprite.y,
            width: sprite.width,
            height: sprite.height,
            pixels: sprite.pixels.to_vec(),
        };

        serde_wasm_bindgen::to_value(&data)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get an enemy sprite as RGBA bytes (width * height * 4) for Canvas ImageData
    pub fn get_enemy_sprite_rgba(&self, id: u8) -> Result<Vec<u8>, JsValue> {
        find_enemy_sprite(id)
            .map(enemy_sprite_rgba)
            .ok_or_else(|| JsValue::from_str(&format!("Sprite not found: {}", id)))
    }

    /// Get the NES color table (color number -> [r, g, b])
    pub fn get_nes_palette(&self) -> Result<JsValue, JsValue> {
        let palette: Vec<[u8; 3]> = NES_PALETTE.iter().map(|&(r, g, b)| [r, g, b]).collect();

        serde_wasm_bindgen::to_value(&palette)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all map tile sprites (16x16, pixels for each palette set)
    pub fn get_map_tile_sprites(&self) -> Result<JsValue, JsValue> {
        let sets = [PaletteSet::Field, PaletteSet::Town, PaletteSet::Dungeon];
        let tiles: Vec<MapTileSpriteData> = MAP_TILE_SPRITE_MASTER
            .iter()
            .map(|tile| MapTileSpriteData {
                id: tile.id,
                tiles: tile.tiles,
                palette: tile.palette,
                pixels: sets
                    .iter()
                    .map(|&set| {
                        (
                            palette_set_name(set).to_string(),
                            tile.pixels[set.index()].to_vec(),
                        )
                    })
                    .collect(),
            })
            .collect();

        serde_wasm_bindgen::to_value(&tiles)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get a map tile as RGBA bytes (16 * 16 * 4). The colors depend on the map id
    pub fn get_map_tile_rgba(&self, tile_id: u8, map_id: u8) -> Result<Vec<u8>, JsValue> {
        map_tile_pixels(tile_id, map_id)
            .map(|pixels| field_pixels_rgba(pixels))
            .ok_or_else(|| JsValue::from_str(&format!("Tile sprite not found: {}", tile_id)))
    }

    /// Render a map area (in tiles) as RGBA bytes (width * 16 * height * 16 * 4).
    /// Out of range tiles are drawn with the border tile
    pub fn render_map_rgba(
        &self,
        map_id: u8,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, JsValue> {
        if width == 0 || height == 0 {
            return Err(JsValue::from_str("Invalid size"));
        }
        render_map_rgba(map_id, x, y, width, height)
            .ok_or_else(|| JsValue::from_str(&format!("Map not found: {}", map_id)))
    }

    /// Get all character sprites (16x16, 2 walking frames)
    pub fn get_character_sprites(&self) -> Result<JsValue, JsValue> {
        let sprites: Vec<CharacterSpriteData> = CHARACTER_SPRITE_MASTER
            .iter()
            .map(|sprite| CharacterSpriteData {
                id: sprite.id,
                name: sprite.name.to_string(),
                name_en: sprite.name_en.to_string(),
                frames: sprite
                    .frames
                    .iter()
                    .map(|frame| CharacterFrameData {
                        parts: frame
                            .parts
                            .iter()
                            .map(|part| CharacterPartData {
                                tile: part.tile,
                                x: part.x,
                                y: part.y,
                                palette: part.palette,
                                flip_h: part.flip_h,
                                flip_v: part.flip_v,
                            })
                            .collect(),
                        pixels: frame.pixels.to_vec(),
                    })
                    .collect(),
            })
            .collect();

        serde_wasm_bindgen::to_value(&sprites)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get a character frame (0 or 1) as RGBA bytes (16 * 16 * 4)
    pub fn get_character_sprite_rgba(&self, id: u8, frame: usize) -> Result<Vec<u8>, JsValue> {
        find_character_sprite(id)
            .and_then(|sprite| sprite.frames.get(frame))
            .map(|frame| field_pixels_rgba(frame.pixels))
            .ok_or_else(|| JsValue::from_str(&format!("Character not found: {} ({})", id, frame)))
    }

    /// Get the character sprite id of an NPC (`NpcData.sprite`) on the map
    pub fn npc_character_id(&self, sprite: u8, map_id: u8, after_ending: bool) -> u8 {
        npc_character_id(sprite, map_id, after_ending)
    }

    /// Get the character sprite id of the hero
    pub fn hero_character_id(&self, has_weapon: bool, has_shield: bool, with_princess: bool) -> u8 {
        hero_character_id(has_weapon, has_shield, with_princess)
    }

    /// Get the battle background of a map (field only). Returns null if the map has no background
    pub fn get_battle_background(&self, map_id: u8) -> Result<JsValue, JsValue> {
        let Some(bg) = battle_background(map_id) else {
            return Ok(JsValue::NULL);
        };
        let data = BattleBackgroundData {
            x: bg.x,
            y: bg.y,
            width: bg.width,
            height: bg.height,
            tiles: bg.tiles.to_vec(),
            palette: bg.palette.iter().map(|p| p.to_vec()).collect(),
            block_palettes: bg.block_palettes.to_vec(),
            reveal_order: bg.reveal_order.to_vec(),
            pixels: bg.pixels.to_vec(),
        };

        serde_wasm_bindgen::to_value(&data)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get the color number used to fill battles without a background
    pub fn get_battle_backdrop(&self) -> u8 {
        BATTLE_BACKDROP
    }

    /// Get the battle background as RGBA bytes (112 * 112 * 4). Maps without a background are filled with the backdrop.
    /// With `reveal_blocks`, only that many 16x16 blocks are drawn in reveal order (for the opening animation)
    pub fn get_battle_background_rgba(&self, map_id: u8, reveal_blocks: Option<usize>) -> Vec<u8> {
        battle_pixels_rgba(&battle_background_pixels(map_id, reveal_blocks))
    }

    /// Get a battle scene (background + monster) as RGBA with its screen position
    pub fn get_battle_scene(&self, monster_id: u8, map_id: u8) -> Result<JsValue, JsValue> {
        let scene = battle_scene(monster_id, map_id)
            .ok_or_else(|| JsValue::from_str(&format!("Monster not found: {}", monster_id)))?;
        let data = BattleSceneData {
            x: scene.x,
            y: scene.y,
            width: scene.width,
            height: scene.height,
            rgba: battle_pixels_rgba(&scene.pixels),
        };

        serde_wasm_bindgen::to_value(&data)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get what an NPC does when spoken to, by `NpcData.talk_id`
    /// (kind, shop items, price of a key / fairy water / one night at the inn, dialogue, event role)
    pub fn get_talk(&self, talk_id: u8) -> Result<JsValue, JsValue> {
        let talk = find_talk(talk_id)
            .ok_or_else(|| JsValue::from_str(&format!("Talk not found: {}", talk_id)))?;

        serde_wasm_bindgen::to_value(&TalkData::from(talk))
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all talks
    pub fn get_talks(&self) -> Result<JsValue, JsValue> {
        let talks: Vec<TalkData> = TALK_MASTER.iter().map(TalkData::from).collect();

        serde_wasm_bindgen::to_value(&talks)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get the event script (tree of steps) of a talk_id. Returns null if the talk has no script
    pub fn get_event_script(&self, talk_id: u8) -> Result<JsValue, JsValue> {
        let Some(script) = find_event_script(talk_id) else {
            return Ok(JsValue::NULL);
        };

        serde_wasm_bindgen::to_value(&steps_data(script.steps))
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Run what happens when the player talks to an NPC, and return the dialogue lines and actions.
    /// - `story`: `{ carrying_princess, princess_delivered, left_throne_room, dragonlord_defeated }` (all optional)
    /// - `answers`: yes / no answers in order (`[true, false]`). If they run out, `status` is `"waiting_answer"`
    /// - `seed`: random seed for random dialogues (random if omitted)
    ///
    /// Actions are not applied to the player. Returns null for shops and inns
    pub fn run_event(
        &self,
        talk_id: u8,
        map_id: u8,
        story: JsValue,
        answers: JsValue,
        seed: Option<u32>,
    ) -> Result<JsValue, JsValue> {
        let player = self
            .player
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No player created"))?;
        let story: StoryState = if story.is_undefined() || story.is_null() {
            StoryState::default()
        } else {
            serde_wasm_bindgen::from_value(story)
                .map_err(|e| JsValue::from_str(&format!("Invalid story: {}", e)))?
        };
        let answers: Vec<bool> = if answers.is_undefined() || answers.is_null() {
            Vec::new()
        } else {
            serde_wasm_bindgen::from_value(answers)
                .map_err(|e| JsValue::from_str(&format!("Invalid answers: {}", e)))?
        };
        let seed = seed.unwrap_or_else(|| {
            u32::from_le_bytes([
                random_value(255),
                random_value(255),
                random_value(255),
                random_value(255),
            ])
        });

        let Some(result) = run_event(talk_id, map_id, player, &story, &answers, seed) else {
            return Ok(JsValue::NULL);
        };
        let data = EventResultData {
            lines: result
                .lines
                .into_iter()
                .map(|line| EventLineData {
                    dialogue_id: line.dialogue_id,
                    text: line.text,
                })
                .collect(),
            actions: result.actions.into_iter().map(ActionData::from).collect(),
            status: match result.status {
                EventStatus::Finished => "finished",
                EventStatus::WaitingAnswer => "waiting_answer",
            }
            .to_string(),
        };

        serde_wasm_bindgen::to_value(&data)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all warp (stairs / entrance) data
    pub fn get_warps(&self) -> Result<JsValue, JsValue> {
        let warps: Vec<WarpData> = WARP_MASTER
            .iter()
            .map(|warp| WarpData {
                from: warp.from,
                to: warp.to,
            })
            .collect();

        serde_wasm_bindgen::to_value(&warps)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

impl WasmGame {
    /// Run the battle start (action: None) or one turn, and store the state back
    fn step_battle(
        &mut self,
        action: Option<PlayerAction>,
        selection: Option<usize>,
    ) -> Result<JsValue, JsValue> {
        let player = self
            .player
            .take()
            .ok_or_else(|| JsValue::from_str("プレイヤーが作成されていません"))?;
        let session = self
            .battle_session
            .take()
            .ok_or_else(|| JsValue::from_str("戦闘中ではありません"))?;

        self.output_buffer.buffer.clear();
        self.web_input.clear();
        if let Some(action) = &action {
            self.web_input.push_action(action.clone());
        }
        if let Some(value) = selection {
            self.web_input.push_input(value);
        }

        let mut battle = Battle::new(
            player,
            session.monster,
            &mut self.web_input,
            &mut self.output_buffer,
        );
        battle.player_state = session.player_state;
        battle.monster_state = session.monster_state;

        let turn = if action.is_some() {
            battle.run_turn();
            session.turn + 1
        } else {
            battle.begin();
            session.turn
        };

        let is_finished = !battle.is_battle_continue();
        if is_finished {
            battle.finish();
        }

        let Battle {
            player,
            monster,
            player_state,
            monster_state,
            ..
        } = battle;
        self.web_input.clear();

        let result = BattleTurnResult {
            turn,
            is_finished,
            player_survived: player.is_alive(),
            monster_defeated: !monster.is_alive(),
            player_escaped: player_state.escaped,
            monster_escaped: monster_state.escaped,
            monster_id: monster.id,
            monster_name: monster.name().to_string(),
            monster_hp: monster.hp,
            player_sleep: player_state.sleep,
            player_seal: player_state.seal,
            monster_sleep: monster_state.sleep,
            monster_seal: monster_state.seal,
            messages: self.output_buffer.buffer.clone(),
            player_state: PlayerState {
                summary: player.summary(),
                strength_status: player.strength_status(),
                items: player.item_list().iter().map(|s| s.to_string()).collect(),
            },
        };

        self.player = Some(player);
        if !is_finished {
            self.battle_session = Some(BattleSession {
                monster,
                player_state,
                monster_state,
                turn,
            });
        }

        serde_wasm_bindgen::to_value(&result)
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

    #[wasm_bindgen_test]
    fn test_get_towns_and_dialogues() {
        let game = WasmGame::new();
        assert!(game.get_towns().is_ok());
        assert!(game.get_dialogues().is_ok());
        assert_eq!(game.render_dialogue(4, "だい").unwrap(), render_dialogue(4, "だい").unwrap());
    }

    #[wasm_bindgen_test]
    fn test_get_map_data() {
        let game = WasmGame::new();
        assert!(game.get_maps().is_ok());
        assert!(game.get_map(1).is_ok());
        assert!(game.get_map(0).is_err());
        assert!(game.get_tiles().is_ok());
        assert!(game.get_warps().is_ok());
        assert_eq!(game.tile_at(1, -1, 0), Some(MAP_MASTER[0].border_tile));
    }

    #[wasm_bindgen_test]
    fn test_turn_based_battle() {
        let mut game = WasmGame::new();
        assert!(game.start_battle(0).is_err());

        game.create_player("だい").unwrap();
        assert!(game.battle_turn("attack", None).is_err());

        game.start_battle(0).unwrap();
        assert!(game.is_in_battle());
        assert!(game.battle_turn("spell", None).is_err());
        assert!(game.battle_turn("dance", None).is_err());

        let mut turns = 0;
        while game.is_in_battle() && turns < 200 {
            game.battle_turn("attack", None).unwrap();
            turns += 1;
        }
        assert!(!game.is_in_battle());
        assert!(game.has_player());
        assert!(game.battle_turn("attack", None).is_err());
    }

    #[wasm_bindgen_test]
    fn test_end_battle() {
        let mut game = WasmGame::new();
        game.create_player("だい").unwrap();
        game.start_battle(0).unwrap();
        game.end_battle();
        assert!(!game.is_in_battle());
        assert!(game.has_player());
    }

    #[wasm_bindgen_test]
    fn test_enemy_sprites() {
        let game = WasmGame::new();
        assert!(game.get_enemy_sprites().is_ok());
        assert!(game.get_enemy_sprite(0).is_ok());
        assert!(game.get_enemy_sprite(40).is_err());
        assert!(game.get_nes_palette().is_ok());

        let sprite = find_enemy_sprite(0).unwrap();
        let rgba = game.get_enemy_sprite_rgba(0).unwrap();
        assert_eq!(rgba.len(), sprite.width as usize * sprite.height as usize * 4);
    }

    #[wasm_bindgen_test]
    fn test_event() {
        let mut game = WasmGame::new();
        assert!(game.get_event_script(112).is_ok());
        assert!(game.get_event_script(0).unwrap().is_null());
        assert!(game.run_event(112, 6, JsValue::NULL, JsValue::NULL, Some(1)).is_err());

        game.create_player("ゆうてい").unwrap();
        let answers = serde_wasm_bindgen::to_value(&vec![false]).unwrap();
        let result = game
            .run_event(112, 6, JsValue::UNDEFINED, answers, Some(1))
            .unwrap();
        let result: EventResultData = serde_wasm_bindgen::from_value(result).unwrap();
        assert_eq!(result.status, "finished");
        assert_eq!(
            result.lines.iter().map(|l| l.dialogue_id).collect::<Vec<_>>(),
            vec![199, 164, 200, 201]
        );

        let story = serde_wasm_bindgen::to_value(&StoryState {
            left_throne_room: true,
            ..Default::default()
        })
        .unwrap();
        assert!(game.run_event(110, 5, story, JsValue::UNDEFINED, None).is_ok());
        assert!(game.run_event(0, 7, JsValue::UNDEFINED, JsValue::UNDEFINED, None).unwrap().is_null());
    }

    #[wasm_bindgen_test]
    fn test_talk() {
        let game = WasmGame::new();
        assert!(game.get_talks().is_ok());
        assert!(game.get_talk(0).is_ok());
        assert!(game.get_talk(254).is_ok());
        assert!(game.get_talk(200).is_err());

        let data = TalkData::from(find_talk(0).unwrap());
        assert_eq!(data.kind, "weapon_shop");
        assert!(!data.items.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_battle_background() {
        let game = WasmGame::new();
        assert!(game.get_battle_background(1).is_ok());
        assert!(game.get_battle_background(15).unwrap().is_null());
        assert_eq!(game.get_battle_background_rgba(1, None).len(), 112 * 112 * 4);
        assert_eq!(game.get_battle_background_rgba(15, Some(3)).len(), 112 * 112 * 4);
        assert!(game.get_battle_scene(0, 1).is_ok());
        assert!(game.get_battle_scene(40, 1).is_err());
    }

    #[wasm_bindgen_test]
    fn test_field_sprites() {
        let game = WasmGame::new();
        assert!(game.get_map_tile_sprites().is_ok());
        assert_eq!(game.get_map_tile_rgba(0, 1).unwrap().len(), 16 * 16 * 4);
        assert!(game.get_map_tile_rgba(24, 1).is_err());
        assert_eq!(game.render_map_rgba(1, 0, 0, 2, 3).unwrap().len(), 2 * 3 * 16 * 16 * 4);
        assert!(game.render_map_rgba(0, 0, 0, 1, 1).is_err());

        assert!(game.get_character_sprites().is_ok());
        assert_eq!(game.get_character_sprite_rgba(8, 1).unwrap().len(), 16 * 16 * 4);
        assert!(game.get_character_sprite_rgba(8, 2).is_err());
        assert!(game.get_character_sprite_rgba(16, 0).is_err());
        assert_eq!(game.hero_character_id(false, false, false), 8);
    }
}
