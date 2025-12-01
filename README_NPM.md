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

// Get all weapons
const weapons = game.get_weapons();

// Get all spells
const spells = game.get_spells();

// Get status progression table
const statusTable = game.get_status_table();
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

#### Master Data
- `get_monsters(): MonsterData[]` - Get all monsters
- `get_weapons(): EquipmentData[]` - Get all weapons
- `get_armors(): EquipmentData[]` - Get all armor
- `get_shields(): EquipmentData[]` - Get all shields
- `get_items(): EquipmentData[]` - Get all items
- `get_spells(): SpellData[]` - Get all spells
- `get_status_table(): StatusData[]` - Get level progression data

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
