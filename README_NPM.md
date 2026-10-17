# Damdara WASM

[![npm version](https://img.shields.io/npm/v/damdara.svg)](https://www.npmjs.com/package/damdara)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Dragon Quest core module compiled to WebAssembly. Experience the classic "Fukkatsu no Jumon" (Revival Password) system and turn-based battle mechanics in your browser or Node.js application.

🌐 **[Try Live Demo](https://retrodig.github.io/damdara/)**

## Features

- 🔐 **Password System** - Generate and decode 20-character hiragana passwords
- ⚔️ **Battle System** - Turn-based combat with monsters
- 📊 **Character Management** - Player stats, equipment, and progression
- 🎮 **Master Data** - Complete database of monsters, items, spells, and equipment
- 🦀 **Pure Rust** - Compiled to WebAssembly for performance
- 📦 **Zero Dependencies** - Standalone WASM module

## Installation

```bash
npm install damdara
```

or

```bash
yarn add damdara
```

## Usage

### Basic Example

```javascript
import init, { WasmGame } from 'damdara';

// Initialize the WASM module
await init();

// Create a new game instance
const game = new WasmGame();

// Create a player
const playerState = game.create_player("ゆうてい");
console.log(playerState);
// {
//   summary: { name: "ゆうてい", level: 1, hp: 15, mp: 0, ... },
//   strength_status: { strength: 4, agility: 4, ... },
//   items: []
// }
```

### Password System

```javascript
// Generate a password from current player state
const password = game.generate_password();
console.log(password); // "あいうえお..." (20 hiragana characters)

// Load player from password
const loadedState = game.load_from_password("あいうえおかきくけこ...");
console.log(loadedState);
```

### Battle System

```javascript
// Queue battle actions
game.queue_battle_action("attack");
game.queue_battle_action("attack");
game.queue_battle_action("spell");
game.queue_battle_input(1); // Spell selection

// Run battle against monster ID 0 (Slime)
const result = game.run_battle(0);
console.log(result);
// {
//   player_survived: true,
//   monster_defeated: true,
//   player_escaped: false,
//   monster_escaped: false,
//   messages: ["スライムがあらわれた！", ...],
//   final_player_state: { ... }
// }
```

### Master Data Access

```javascript
// Get all monsters
const monsters = game.get_monsters();
console.log(monsters);
// [
//   { id: 0, name: "スライム", hp: 3, attack: 5, defense: 3, ... },
//   { id: 1, name: "スライムベス", hp: 4, attack: 7, ... },
//   ...
// ]

// Note: since 0.10.0, monster ids follow the in-game enemy order
// (e.g. 7: メーダ, 11: がいこつ, 16: メタルスライム, 24: ゴーレム).
// The id is shared by get_monsters, run_battle, start_battle and get_enemy_sprite.

// Get all weapons
const weapons = game.get_weapons();

// Get all spells
const spells = game.get_spells();

// Get status progression table
const statusTable = game.get_status_table();
```

### Turn-based Battle

`run_battle` runs the whole battle at once. To run it one turn at a time, use `start_battle` and `battle_turn`.

```javascript
let result = game.start_battle(0); // スライム
console.log(result.messages); // ["スライムがあらわれた！", ...]

while (game.is_in_battle()) {
  // "attack" | "spell" | "item" | "escape"
  result = game.battle_turn("attack");
  // result = game.battle_turn("spell", 1); // first spell in the list
  console.log(result.turn, result.messages, result.monster_hp);
}

console.log(result.is_finished);      // true
console.log(result.monster_defeated); // true if the monster was defeated
```

`BattleTurnResult` contains `turn`, `is_finished`, `player_survived`, `monster_defeated`, `player_escaped`, `monster_escaped`, `monster_id`, `monster_name`, `monster_hp`, `player_sleep`, `player_seal`, `monster_sleep`, `monster_seal`, `messages` (this turn only) and `player_state`.

### Town / Dialogue / Map Data

```javascript
// Get all towns, castles and shrines
const towns = game.get_towns();
// [
//   { map_id: 3, name: "ドムドーラ", name_en: "Hauksness", kind: "town", world_entrance: [25, 89], npc_count: 0, dialogue_ids: [] },
//   { map_id: 4, name: "ラダトームのしろ", name_en: "Tantegel Castle", kind: "castle", ... },
//   ...
// ]

// Get all dialogues (variables such as {player} are kept as is)
const dialogues = game.get_dialogues();
// [{ id: 0, text: "{word}は　めをさました\n{wait}" }, ...]

// Get a dialogue with {player} replaced
const text = game.render_dialogue(4, "だい");
// "だいは　にじのしずくを\nてんに　かざした。\n▼"

// Get all map metadata (without tiles)
const maps = game.get_maps();
// [{ id: 1, name: "アレフガルド", name_en: "Alefgard", kind: "field", width: 120, height: 120, border_tile: 15, npc_count: 0 }, ...]

// Get one map by map id (1: Alefgard field) with tiles and NPCs
const map = game.get_map(5);
const tileId = map.tiles[y * map.width + x];

// Or get a tile id directly (out of range returns the border tile)
const tile = game.tile_at(1, 43, 43);

// Tile definitions and warps (stairs / entrances)
const tiles = game.get_tiles(); // [{ id: 0, name: "そうげん", name_en: "grass" }, ...]
const warps = game.get_warps(); // [{ from: [1, 2, 2], to: [9, 0, 14] }, ...]
```

Map names and tile names use commonly used names.

### Talk / Shop / Inn

Each NPC of `get_map` has a `talk_id` and what happens when spoken to.

```javascript
const map = game.get_map(8); // ラダトーム
for (const npc of map.npcs) {
  // talk_kind: "weapon_shop" | "item_shop" | "key_shop" | "fairy_water_shop" | "inn" | "message" | "question" | "event"
  console.log(npc.talk_kind, npc.items, npc.price, npc.role);
}

// Or get one talk by talk_id
const talk = game.get_talk(0);
// {
//   talk_id: 0, kind: "weapon_shop",
//   items: [{ kind: "weapon", index: 3, name: "どうのつるぎ", price: 180 }, ...],
//   price: null,            // key / fairy water / one night at the inn
//   dialogue_id: null,      // first dialogue of "message" / "question"
//   yes_dialogue_id: null, no_dialogue_id: null,
//   role: null, role_en: null, // role of an "event" (e.g. the king)
//   related_dialogue_ids: [38, 39, ...]
// }
```

`items[].index` is the index of `get_weapons` / `get_armors` / `get_shields` / `get_items`. Herbs (`kind: "herb"`) have no index.

### Event

`run_event` follows the conversation of an NPC with the current player, and returns the lines (with the player name etc. filled in) and the actions to perform. Actions are not applied to the player.

```javascript
game.create_player("ゆうてい");

// Progress not stored in the password (all optional)
const story = { carrying_princess: false, princess_delivered: false, left_throne_room: true, dragonlord_defeated: false };

// The Dragonlord (talk_id 112 on map 6). Answer the questions one by one
let result = game.run_event(112, 6, story, []);
// { lines: [{ dialogue_id: 199, text: "＊「よくきた　ゆうていよ。..." }, ...], actions: [], status: "waiting_answer" }

result = game.run_event(112, 6, story, [false]); // "いいえ"
// { lines: [... 4 lines], actions: [{ type: "battle", monster_id: 38 }], status: "finished" }

// The tree of steps (null for talks without an event)
const script = game.get_event_script(112);
// [{ type: "say", dialogue_id: 199 }, ..., { type: "ask", dialogue_id: 200, yes: [...], no: [...] }]
```

Step types: `say`, `ask`, `ask_until_yes` (asks again after "no"), `if` (`condition`, `then`, `otherwise`), `random` (`choices: [{ weight, steps }]`) and `do` (`action`).
Conditions and actions use `type` in snake_case, e.g. `{ type: "has_item", item: 7 }`, `{ type: "give_item", item: 14 }`.

### Monster Sprite

```javascript
// Metadata of all monster graphics (without pixels)
const sprites = game.get_enemy_sprites();
// [{ id: 0, name: "スライム", mirror: false, x: 128, y: 112, width: 20, height: 19, part_count: 7 }, ...]

// Draw a monster on a canvas (id is the same as the monster id)
const sprite = game.get_enemy_sprites()[0];
const rgba = game.get_enemy_sprite_rgba(sprite.id); // Uint8Array (width * height * 4)
const image = new ImageData(new Uint8ClampedArray(rgba), sprite.width, sprite.height);
canvas.getContext('2d').putImageData(image, 0, 0);

// Full data: color numbers (255 = transparent), palettes and 8x8 tile parts
const detail = game.get_enemy_sprite(0);
const nesPalette = game.get_nes_palette(); // color number -> [r, g, b]
```

The x / y values are the display position on the original game screen (256 x 240).

### Map / Character Sprite

Map tiles and characters are 16 x 16 pixels.

```javascript
// Draw a map area (in tiles) on a canvas. Out of range tiles are drawn with the border tile
const [x, y, w, h] = [0, 0, 20, 15];
const rgba = game.render_map_rgba(4, x, y, w, h); // 4: ラダトームのしろ
const image = new ImageData(new Uint8ClampedArray(rgba), w * 16, h * 16);
canvas.getContext('2d').putImageData(image, 0, 0);

// One map tile. The colors depend on the map (field / town / dungeon)
const tile = game.get_map_tile_rgba(3, 4); // tile id 3 on map 4

// Characters (2 walking frames each)
const characters = game.get_character_sprites();
// [{ id: 0, name: "まち の ひと（おとこ）", name_en: "townsman", frames: [{ parts: [...], pixels: [...] }, ...] }, ...]

// Draw the NPCs of a map
const map = game.get_map(4);
for (const npc of map.npcs) {
  const id = game.npc_character_id(npc.sprite, map.id, false);
  const frame = game.get_character_sprite_rgba(id, 0); // frame 0 or 1, transparent pixels have alpha 0
  // ... draw at (npc.x * 16, npc.y * 16)
}

// The hero (with weapon / shield / princess)
const heroId = game.hero_character_id(true, true, false);
```

### Battle Background

Battles on the field (map id 1) have a background (112 x 112). Other maps have no background and are filled with `get_battle_backdrop()`.

```javascript
// Background + monster, with the screen position (256 x 240)
const scene = game.get_battle_scene(0, 1); // スライム on the field
const image = new ImageData(new Uint8ClampedArray(scene.rgba), scene.width, scene.height);
canvas.getContext('2d').putImageData(image, scene.x, scene.y);

// Opening animation: the background appears block by block (16 x 16) from the center
const bg = game.get_battle_background(1); // null on maps without a background
for (let blocks = 0; blocks <= bg.reveal_order.length; blocks++) {
  const rgba = game.get_battle_background_rgba(1, blocks); // 112 * 112 * 4
  // ... draw at (bg.x, bg.y)
}
```

## API Reference

### `WasmGame`

Main game interface.

#### Constructor
- `new WasmGame()` - Create a new game instance

#### Player Management
- `create_player(name: string): PlayerState` - Create a new player
- `get_player_state(): PlayerState` - Get current player state
- `has_player(): boolean` - Check if player exists
- `generate_password(): string` - Generate 20-character password
- `load_from_password(password: string): PlayerState` - Load from password

#### Battle System
- `queue_battle_action(action: string): void` - Queue action ("attack", "spell", "item", "escape")
- `queue_battle_input(value: number): void` - Queue numeric input (menu selection)
- `run_battle(monster_id: number): BattleResult` - Execute battle
- `clear_battle_input(): void` - Clear action queue
- `start_battle(monster_id: number): BattleTurnResult` - Start a turn-based battle (turn 0: monster appears)
- `battle_turn(action: string, selection?: number): BattleTurnResult` - Run one turn. `selection` (1-based spell / item number) is required for `"spell"` and `"item"`
- `is_in_battle(): boolean` - Check if a turn-based battle is in progress
- `end_battle(): void` - Abort the turn-based battle in progress

#### Master Data
- `get_monsters(): MonsterData[]` - Get all monsters
- `get_weapons(): EquipmentData[]` - Get all weapons
- `get_armors(): EquipmentData[]` - Get all armor
- `get_shields(): EquipmentData[]` - Get all shields
- `get_items(): EquipmentData[]` - Get all items
- `get_spells(): SpellData[]` - Get all spells
- `get_status_table(): StatusData[]` - Get level progression data
- `get_towns(): TownData[]` - Get all towns / castles / shrines
- `get_dialogues(): DialogueData[]` - Get all dialogues (raw, with `{player}` etc.)
- `render_dialogue(id: number, player_name: string): string | undefined` - Get a dialogue with `{player}` replaced
- `get_maps(): MapSummaryData[]` - Get all map metadata (without tiles)
- `get_map(id: number): MapData` - Get one map with tiles (row-major, `y * width + x`) and NPCs (each NPC includes `talk_kind`, `items`, `price` and `role`)
- `tile_at(map_id: number, x: number, y: number): number | undefined` - Get tile id (out of range returns the border tile)
- `get_tiles(): TileData[]` - Get all tile definitions
- `get_warps(): WarpData[]` - Get all warps as `(map, x, y)` pairs
- `get_talk(talk_id: number): TalkData` - Get what an NPC does when spoken to (kind, shop items, price, dialogue, event role)
- `get_talks(): TalkData[]` - Get all talks
- `get_event_script(talk_id: number): StepData[] | null` - Get the event conversation as a tree of steps
- `run_event(talk_id: number, map_id: number, story?: StoryState, answers?: boolean[], seed?: number): EventResultData | null` - Follow the conversation with the current player and return `lines`, `actions` and `status` (`"finished"` or `"waiting_answer"`). `null` for shops and inns
- `get_enemy_sprites(): EnemySpriteSummaryData[]` - Get all monster graphic metadata (without pixels)
- `get_enemy_sprite(id: number): EnemySpriteData` - Get one monster graphic (color numbers, palettes, tile parts)
- `get_enemy_sprite_rgba(id: number): Uint8Array` - Get one monster graphic as RGBA for `ImageData`
- `get_nes_palette(): number[][]` - Get the color table (color number -> `[r, g, b]`)
- `get_map_tile_sprites(): MapTileSpriteData[]` - Get all map tile graphics (color numbers for field / town / dungeon)
- `get_map_tile_rgba(tile_id: number, map_id: number): Uint8Array` - Get one map tile as RGBA (16 x 16)
- `render_map_rgba(map_id: number, x: number, y: number, width: number, height: number): Uint8Array` - Render a map area (in tiles) as RGBA (`width * 16` x `height * 16`)
- `get_character_sprites(): CharacterSpriteData[]` - Get all character graphics (2 walking frames each)
- `get_character_sprite_rgba(id: number, frame: number): Uint8Array` - Get one character frame (0 or 1) as RGBA (16 x 16)
- `npc_character_id(sprite: number, map_id: number, after_ending: boolean): number` - Get the character id of an NPC (`npc.sprite`) on the map
- `hero_character_id(has_weapon: boolean, has_shield: boolean, with_princess: boolean): number` - Get the character id of the hero
- `get_battle_background(map_id: number): BattleBackgroundData | null` - Get the battle background (tiles, palettes, reveal order, color numbers). `null` if the map has none
- `get_battle_background_rgba(map_id: number, reveal_blocks?: number): Uint8Array` - Get the battle background as RGBA (112 x 112). `reveal_blocks` draws only that many blocks in reveal order
- `get_battle_backdrop(): number` - Get the color number for battles without a background
- `get_battle_scene(monster_id: number, map_id: number): BattleSceneData` - Get the background + monster as RGBA with `x`, `y`, `width`, `height`

#### Messages
- `get_messages(): string[]` - Get accumulated messages
- `clear_messages(): void` - Clear message buffer

## TypeScript Support

This package includes TypeScript definitions out of the box:

```typescript
import init, { WasmGame, PlayerState, BattleResult } from 'damdara';

await init();
const game: WasmGame = new WasmGame();
const state: PlayerState = game.create_player("ゆうてい");
```

## Browser Compatibility

- Chrome/Edge 90+
- Firefox 89+
- Safari 15+

Requires WebAssembly support.

## Node.js Usage

```javascript
import { readFile } from 'fs/promises';
import init, { WasmGame } from 'damdara';

// Load WASM file explicitly in Node.js
const wasmBuffer = await readFile('./node_modules/damdara/damdara_bg.wasm');
await init(wasmBuffer);

const game = new WasmGame();
// ... use normally
```

## Examples

See the [live demo](https://retrodig.github.io/damdara/) for a complete web application example.

Source code: [GitHub Repository](https://github.com/retrodig/damdara)

## License

MIT © Daisuke Takayama

## Links

- 📖 [Full Documentation](https://github.com/retrodig/damdara)
- 🌐 [Live Demo](https://retrodig.github.io/damdara/)
- 🐛 [Report Issues](https://github.com/retrodig/damdara/issues)
- 📦 [npm Package](https://www.npmjs.com/package/damdara)
- 📦 [crates.io](https://crates.io/crates/damdara)
