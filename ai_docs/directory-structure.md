# Damdara - Directory Structure

## Project Root

```
damdara/
├── Cargo.toml              # Package manifest and dependencies
├── Cargo.lock              # Dependency lock file (generated)
├── LICENSE                 # MIT license
├── README.md               # English documentation
├── README_ja.md            # Japanese documentation
├── ai_docs/                # AI-generated documentation
│   ├── architecture.md
│   ├── data-structure.md
│   ├── directory-structure.md
│   ├── implementation-tasks.md
│   ├── requirement.md
│   └── tech-stack.md
├── assets/                 # Static assets
│   └── images/             # Documentation images
│       ├── base_binary.png
│       ├── fukkatsu_no_jumon.png
│       ├── main_logo_cmp.png
│       ├── relocation.png
│       ├── strongest_parameters.png
│       └── strongest_parameters_2.png
├── src/                    # Source code
│   ├── main.rs             # Binary entry point
│   ├── lib.rs              # Library entry point
│   ├── battle.rs           # Battle system logic
│   ├── growth_type.rs      # Name-based stat growth
│   ├── load.rs             # Password decoding
│   ├── message.rs          # Battle message management
│   ├── monster.rs          # Monster entity
│   ├── player.rs           # Player entity
│   ├── raw_bits.rs         # Low-level bit operations
│   ├── save.rs             # Password encoding
│   ├── constants/          # Game master data
│   │   ├── mod.rs
│   │   ├── battle.rs
│   │   ├── config.rs
│   │   ├── item_weapon.rs
│   │   ├── monster.rs
│   │   ├── save_data.rs
│   │   ├── spell.rs
│   │   ├── status.rs
│   │   └── text.rs
│   ├── input/              # Input abstraction
│   │   ├── mod.rs
│   │   └── cli_input.rs
│   ├── output/             # Output abstraction
│   │   ├── mod.rs
│   │   ├── buffer_output.rs
│   │   └── cli_output.rs
│   ├── traits/             # Trait definitions
│   │   ├── mod.rs
│   │   ├── message_output.rs
│   │   └── player_input.rs
│   └── utility/            # Helper functions
│       ├── mod.rs
│       ├── binary_utils.rs
│       ├── monster_utils.rs
│       ├── random_utils.rs
│       ├── spell_utils.rs
│       ├── status_utils.rs
│       └── string_utils.rs
├── target/                 # Build artifacts (gitignored)
└── .git/                   # Git repository data
```

## Source Directory Structure (`src/`)

### Entry Points

#### `main.rs` (Binary Entry)
**Lines of Code**: ~8
**Purpose**: CLI application launcher
**Dependencies**:
- Uses `clap::Parser` to parse command-line arguments
- Delegates to `lib.rs::run_from_args()`

**Key Function**:
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    damdara::run_from_args(args)
}
```

#### `lib.rs` (Library Entry)
**Lines of Code**: ~115
**Purpose**: Public library API and mode dispatcher
**Exports**:
- All major modules (player, monster, battle, etc.)
- `run_from_args()` function for CLI integration
- `Cli` struct for configuration

**Responsibilities**:
- Parse CLI arguments into internal structures
- Route to appropriate mode handler (Start/Save/Load/Display/Battle)
- Coordinate between modules
- Handle printing and formatting

**Mode Dispatch Logic**:
```rust
match mode {
    Mode::Start => { /* Display player stats */ }
    Mode::Save => { /* Generate password */ }
    Mode::Load => { /* Restore from password */ }
    Mode::Display => { /* Show master data */ }
    Mode::Battle => { /* Run battle loop */ }
}
```

### Core Game Logic

#### `player.rs` (Player Entity)
**Lines of Code**: ~527
**Purpose**: Player character state and behavior
**Key Structures**:
- `Player`: Main player struct (name, HP, MP, exp, gold, equipment)
- `PlayerArgs`: Builder pattern for initialization
- `UnifiedItem`: Inventory item representation
- `ItemKind`: Enum for item types (Herb, Key, Equipment)

**Key Methods**:
- `new(name: &str) -> Self`: Create with default stats
- `new_with(args: PlayerArgs) -> Self`: Create with custom stats
- `from_password_string(s: &str) -> Result<Self, String>`: Restore from password
- `to_password_string(&self) -> Result<String, String>`: Generate password
- `level(&self) -> u8`: Calculate current level from experience
- `attack_power(&self) -> u8`: Calculate total attack
- `defense_power(&self) -> u8`: Calculate total defense
- `adjust_hp(&mut self, amount: i16)`: Modify HP with bounds checking
- `maximize(&mut self)`: Set to max stats (for testing)

**Dependencies**:
- `constants::item_weapon` for equipment data
- `constants::status` for stat tables
- `growth_type` for name-based modifiers
- `utility::status_utils` for level calculations

#### `monster.rs` (Monster Entity)
**Lines of Code**: ~246
**Purpose**: Enemy monster state and AI behavior
**Key Structures**:
- `Monster`: Monster instance (id, HP, stats, behavior)
- `MonsterStats`: Static stats from master data
- `MonsterBehavior`: AI patterns and resistances

**Key Methods**:
- `new(index: usize) -> Self`: Create monster with randomized HP
- `normal_damage(&self, player: &Player) -> u8`: Calculate damage to player
- `has_support_magic(&self) -> bool`: Check for healing/debuff spells
- `attack_spells_actions(&self) -> Vec<MonsterAction>`: Get offensive abilities
- `is_final_boss(&self) -> bool`: Special boss logic

**AI Decision Making**:
- Evaluated in `battle.rs::decide_enemy_action()`
- Priority: Escape → Support Magic → Attack Magic → Normal Attack

#### `battle.rs` (Battle System)
**Lines of Code**: ~667
**Purpose**: Turn-based combat orchestration
**Key Structures**:
- `Battle`: Combat state manager
- `BattleState`: Status flags (sleep, seal, escaped)
- `PlayerAction`: Enum for player commands
- `EnemyAction`: Enum for monster actions

**Lifecycle**:
1. `new()`: Initialize battle with player and monster
2. `start()`: Main battle loop
3. `player_turn()`: Handle player input and action
4. `monster_turn()`: Execute AI decision
5. Loop until victory, defeat, or escape

**Key Methods**:
- `is_battle_continue(&self) -> bool`: Check if battle should continue
- `player_action_attack(&mut self)`: Execute player attack
- `player_action_spell(&mut self)`: Cast spell (with menu)
- `player_action_item(&mut self)`: Use item (with menu)
- `player_action_escape(&mut self)`: Attempt to flee
- `decide_enemy_action(&self) -> EnemyAction`: AI decision tree
- `handle_enemy_normal_attack(&mut self)`: Monster physical attack
- `handle_enemy_heal_spell(&mut self, ...)`: Monster healing

**Dependencies**:
- `traits::{PlayerInput, MessageOutput}` for I/O abstraction
- `message::BattleMessages` for text management
- `utility::random_utils` for RNG
- `utility::spell_utils` for spell effects

#### `message.rs` (Battle Messages)
**Lines of Code**: ~300+ (estimated)
**Purpose**: Battle text generation and display
**Key Structure**:
- `BattleMessages`: Message queue with output abstraction

**Responsibilities**:
- Format Japanese battle text
- Buffer messages between turns
- Abstract output destination (CLI vs buffer for tests)

**Example Messages**:
- "スライムがあらわれた！" (Slime appeared!)
- "ゆうていのこうげき！" (Yutei's attack!)
- "13ポイントのダメージをあたえた！" (Dealt 13 damage!)

### Password System

#### `save.rs` (Password Encoding)
**Lines of Code**: ~581
**Purpose**: Convert game state → 20-character password
**Algorithm Steps**:
1. Pack data into 120-bit structure
2. Calculate CRC-8 checksum
3. Reorder bits in blocks (reverse 24-bit chunks into 6-bit groups)
4. Apply cumulative addition offset
5. Map to hiragana character table

**Key Functions**:
- `SaveData::to_password_string(&self) -> Result<String, String>`: Main entry
- `encode_name_to_bits(&self) -> Result<u32, String>`: Name → 24-bit value
- `build_password_base(&self) -> Result<Vec<String>, String>`: Pack 15 bytes
- `calculate_crc_from_bits(bits: &[String]) -> Result<String, String>`: CRC-8
- `reorder_password_bits(bitstring: &str) -> Result<Vec<String>, String>`: Block reorder
- `apply_password_offsets(base: &[String]) -> Result<Vec<u8>, String>`: Cumulative add
- `indices_to_password_kana(indices: &[u8]) -> Result<String, String>`: Map to kana

**Bit Layout** (see `constants/save_data.rs` for full mapping):
```
bits[0]  = Checksum
bits[1]  = Experience (lower 8 bits)
bits[2]  = Pattern bit 2 + Cursed Necklace + Name[2] (6 bits)
bits[3]  = Item[3] (4 bits) + Item[2] (4 bits)
...
bits[14] = Item[1] (4 bits) + Item[0] (4 bits)
```

#### `load.rs` (Password Decoding)
**Lines of Code**: ~297
**Purpose**: Convert 20-character password → game state
**Algorithm Steps** (reverse of save.rs):
1. Validate 20 hiragana characters
2. Map characters to 6-bit indices
3. Undo cumulative addition offset
4. Reorder blocks back to original structure
5. Extract fields via bit masking
6. Validate checksum

**Key Functions**:
- `decode_from_password_string(password: &str) -> Result<SaveData, String>`: Main entry
- `decode_password_string(s: &str) -> Result<Vec<u8>, String>`: Kana → indices
- `undo_password_addition(values: &[u8]) -> Result<Vec<u8>, String>`: Reverse offset
- `reorder_blocks_back(bits: &[u8]) -> Result<Vec<u8>, String>`: Reverse reorder
- `parse_bitstring_to_save_data(bits: &[u8]) -> Result<SaveData, String>`: Extract fields

**Error Handling**:
- Invalid characters → descriptive error in Japanese
- Incorrect length → length error
- Checksum mismatch → validation error

#### `growth_type.rs` (Name-based Growth)
**Lines of Code**: ~121
**Purpose**: Calculate stat modifiers from player name
**Key Algorithm**:
```rust
1. Normalize name to 4 hiragana characters (pad with spaces)
2. Convert each character to index (0-63 from kana table)
3. Apply modulo 16 to each index
4. Sum: total = (char[0] % 16) + (char[1] % 16) + (char[2] % 16) + (char[3] % 16)
5. Calculate modifiers:
   - a = (total / 4) % 4  (bonus modifier, 0-3)
   - b = (total / 2) % 2  (HP/Agility modifier, 0-1)
   - c = total % 2        (Strength/MP modifier, 0-1)
```

**Key Functions**:
- `calculate_growth_name_total(name: &str) -> u16`: Name → total value
- `calculate_abc(total: u16) -> GrowthModifiers`: Total → (a, b, c)
- `get_adjusted_status_by_name_lv(name: &str, lv: u8) -> Status`: Apply modifiers

**Impact on Stats**:
- HP growth: base + b
- MP growth: base + c
- Strength growth: base + c
- Agility growth: base + b

### Constants & Master Data

#### `constants/mod.rs`
**Purpose**: Re-export all constant modules
**Modules**:
- `battle`: Battle-related enums and types
- `config`: CLI configuration and modes
- `item_weapon`: Equipment master data
- `monster`: Monster master data
- `save_data`: Password data structures
- `spell`: Spell definitions
- `status`: Stat tables and flags
- `text`: Text tables (kana, password chars)

#### `constants/config.rs`
**Lines of Code**: ~112
**Purpose**: CLI argument parsing and mode definitions
**Key Structures**:
- `Cli`: Clap-derived command-line interface
- `Mode`: Enum for operation modes (Start, Save, Load, Display, Battle)

**CLI Parameters**:
- `-n, --name`: Player name (default: "ゆうてい")
- `-e, --exp`: Experience points (0-65535)
- `-g, --gold`: Gold amount (0-65535)
- `-w, --weapon`: Weapon ID (0-7)
- `-a, --armor`: Armor ID (0-7)
- `-s, --shield`: Shield ID (0-3)
- `-i, --item`: Item IDs (comma-separated)
- `-y, --herbs`: Herb count (0-6)
- `-k, --keys`: Key count (0-6)
- `--flags`: Status flags (5-digit binary string)
- `-p, --password`: Password for load mode
- `-m, --mode`: Operation mode
- `--view`: Display filter (for Display/Battle modes)
- `-o, --option`: Special options (e.g., "max" for maxed stats)

#### `constants/save_data.rs`
**Lines of Code**: ~90
**Purpose**: Define save data structure for password system
**Key Structure**:
```rust
pub struct SaveData {
    pub name: String,        // 6bit × 4 characters
    pub experience: u16,     // 16 bits (0-65535)
    pub gold: u16,          // 16 bits (0-65535)
    pub weapon: u8,         // 3 bits (0-7)
    pub armor: u8,          // 3 bits (0-7)
    pub shield: u8,         // 2 bits (0-3)
    pub items: [u8; 8],     // 4 bits each (0-15)
    pub herbs: u8,          // 4 bits (0-6)
    pub keys: u8,           // 4 bits (0-6)
    pub flags: Flags,       // 5 bits
    pub pattern: u8,        // 3 bits (0-7)
}
// Total: 120 bits
```

**Bit Packing Documentation**: Inline comments explain exact bit positions

#### `constants/status.rs`
**Purpose**: Player stat tables and level progression
**Key Structures**:
- `Status`: Base stats per level (strength, agility, HP, MP, required exp)
- `Flags`: Boolean flags for equipment and bosses
- `PlayerSummary`: Display-friendly player info
- `StrengthStatus`: Combat-ready stat summary

**Data**:
- `STATUS_TABLE`: Array of 30 levels with base stats
- `DEFAULT_STATUS`: Level 1 fallback

**Example**:
```rust
Status {
    level: 1,
    strength: 4,
    agility: 4,
    max_hp: 15,
    max_mp: 0,
    required_exp: 0,
}
```

#### `constants/monster.rs`
**Purpose**: Monster master data (40 monsters)
**Key Structures**:
- `MonsterStats`: Name, HP, MP, attack, defense, exp, gold
- `MonsterBehavior`: AI patterns, evasion, resistance, actions
- `MonsterAction`: Spell or special ability with probability
- `ActionType`: Enum (Spell or Special)
- `Resist`: Spell resistance percentages

**Data Arrays**:
- `MONSTER_MASTER`: 40 monster stat entries
- `MONSTER_BEHAVIORS`: 40 AI behavior entries

**Example**:
```rust
MonsterStats {
    name: "スライム",
    hp: 3,
    mp: 0,
    attack: 5,
    defense: 2,
    exp: 1,
    gold: 2,
}
```

#### `constants/item_weapon.rs`
**Purpose**: Equipment and item master data
**Key Structure**:
- `Equipment`: Name, price, sell price, attack, defense

**Data Arrays**:
- `ITEM_MASTER`: 15 items (torch, holy water, wings, etc.)
- `WEAPON_MASTER`: 8 weapons (unarmed to Roto sword)
- `ARMOR_MASTER`: 8 armor types (cloth to Roto armor)
- `SHIELD_MASTER`: 4 shields (none, leather, iron, mirror)

**Example**:
```rust
Equipment {
    name: "どうのつるぎ",
    price: 180,
    sell: 90,
    attack: 10,
    defense: 0,
}
```

#### `constants/spell.rs`
**Purpose**: Spell definitions and MP costs
**Key Structures**:
- `Spell`: Enum of all spells
- `SpellInfo`: Spell + MP cost + required level

**Data**:
- `SPELL_INFO_LIST`: Array of learnable spells by level

**Spells**:
- Hoimi (HP recovery)
- Gira (fire damage)
- Rarirho (sleep)
- Behoimi (greater heal)
- Begirama (greater fire)
- Mahoton (spell seal)

#### `constants/text.rs`
**Purpose**: Character encoding tables
**Data**:
- `KANA_TABLE`: 64-character hiragana table (6-bit encoding)
- `PASSWORD_TABLE`: 64-character password table (different order)
- `DEFAULT_NAME`: "ゆうてい" (default player name)

**Mapping Functions**:
- `build_kana_map() -> HashMap<char, u8>`: Character → index
- `build_password_map() -> HashMap<char, u8>`: Password char → index

#### `constants/battle.rs`
**Purpose**: Battle-related type definitions
**Key Types**:
- `BattleState`: Battle status flags (sleep, seal, escaped)
- `PlayerAction`: Enum (Attack, Spell, Item, Escape)
- `EnemyAction`: Enum (Attack, Special, Escape)

### Traits & Abstractions

#### `traits/mod.rs`
**Purpose**: Re-export trait definitions

#### `traits/player_input.rs`
**Lines of Code**: ~10
**Purpose**: Abstract user input for testability
**Trait Definition**:
```rust
pub trait PlayerInput {
    fn get_player_input(&mut self, max: usize) -> usize;
    fn get_player_action(&mut self, display_commands: &mut dyn FnMut()) -> PlayerAction;
}
```

**Implementations**:
- `input::cli_input::CliInput`: Read from stdin with validation
- Test mocks: `DummyInput` with predetermined responses

#### `traits/message_output.rs`
**Lines of Code**: ~5
**Purpose**: Abstract text output for testability
**Trait Definition**:
```rust
pub trait MessageOutput {
    fn output(&mut self, message: &str);
}
```

**Implementations**:
- `output::cli_output::CliOutput`: Print to stdout
- `output::buffer_output::BufferOutput`: Collect in Vec<String>
- Test mocks: `DummyOutput` (no-op)

### Input/Output Implementations

#### `input/cli_input.rs`
**Purpose**: Read user input from stdin
**Key Methods**:
- `get_player_input(&mut self, max: usize) -> usize`: Read number 0-max
- `get_player_action(&mut self, ...) -> PlayerAction`: Read command 1-4

**Validation**:
- Retry on invalid input
- Clear screen on error
- Display help text

#### `output/cli_output.rs`
**Purpose**: Print messages to stdout
**Implementation**: Calls `println!`

#### `output/buffer_output.rs`
**Purpose**: Collect messages in memory for testing
**Implementation**: Stores messages in `Vec<String>`

### Utility Functions

#### `utility/mod.rs`
**Purpose**: Re-export utility modules

#### `utility/binary_utils.rs`
**Purpose**: Bit manipulation helpers
**Key Functions**:
- `combine_bits(parts: &[(u8, u8)]) -> Result<u8, String>`: Pack multiple values
- `validate_6bit_array(bits: &[u8]) -> Result<(), String>`: Check 20×6bit
- `validate_120bit(bits: &[u8]) -> Result<(), String>`: Check 15×8bit

#### `utility/string_utils.rs`
**Purpose**: Japanese text processing
**Key Functions**:
- `name_normalize(name: &str) -> String`: Pad to 4 chars, expand dakuten
- `kana_index(c: char) -> Result<u8, String>`: Character → table index
- `nth_char(s: &str, n: usize) -> Result<char, String>`: UTF-8 safe indexing

#### `utility/status_utils.rs`
**Purpose**: Level and stat calculations
**Key Functions**:
- `get_level_by_exp(exp: u16) -> u8`: Experience → level
- `get_status_by_level(lv: u8) -> Option<Status>`: Level → base stats
- `resolve_experience(level: u8, exp: Option<u16>) -> u16`: Handle level/exp priority

#### `utility/random_utils.rs`
**Purpose**: Random number generation wrappers
**Key Functions**:
- `random_value(max: u8) -> u8`: 0 to max inclusive
- `generate_in_range(min: u8, max: u8) -> u8`: Inclusive range
- `random_success_by_percent(percent: f64) -> bool`: Probability check
- `check_escape_success(agility: u16, defense: u16, rand_max: u8) -> bool`: Escape formula

#### `utility/spell_utils.rs`
**Purpose**: Spell effect calculations
**Key Functions**:
- `player_spell_effect(spell: Spell) -> u8`: Damage/heal amount for player spells
- `monster_action_effect(action: &ActionType) -> u8`: Effect for monster abilities
- `spells_learned_by_level(level: u8) -> Vec<&'static SpellInfo>`: Available spells

#### `utility/monster_utils.rs`
**Purpose**: Monster AI helpers
**Key Functions**:
- `choose_action(candidates: &[MonsterAction]) -> Option<MonsterAction>`: Weighted random selection

### Raw Bits (Optional)

#### `raw_bits.rs`
**Status**: Appears to be unused/deprecated
**Purpose**: Low-level bit manipulation (superseded by binary_utils)

## Build Artifacts (`target/`)

**Contents** (gitignored):
- `debug/`: Development builds with debug symbols
- `release/`: Optimized production builds
- `doc/`: Generated rustdoc HTML
- `.rustc_info.json`: Compiler cache
- `CACHEDIR.TAG`: Build cache marker

**Key Binaries**:
- `target/release/damdara`: Optimized executable
- `target/debug/damdara`: Debug executable

## Assets (`assets/`)

**Purpose**: Documentation images for README
**Files**:
- `main_logo_cmp.png`: Project logo
- `base_binary.png`: 120-bit structure diagram
- `relocation.png`: Bit reordering visualization
- `fukkatsu_no_jumon.png`: Password screen mockup
- `strongest_parameters.png`: Max stats screenshot
- `strongest_parameters_2.png`: Inventory screenshot

## Documentation (`ai_docs/`)

**Purpose**: AI-generated architecture documentation
**Files**:
- `requirement.md`: Functional and non-functional requirements
- `tech-stack.md`: Technology choices and justifications
- `directory-structure.md`: This file
- `data-structure.md`: Data layout and relationships
- `architecture.md`: System design and patterns
- `implementation-tasks.md`: Development roadmap

## Version Control

**Git Structure**:
- `.git/`: Repository metadata
- `.gitignore`: Excludes `target/`, `Cargo.lock` (for libraries), etc.

**Key Branches**:
- `main`: Stable release branch
- `develop`: Integration branch
- Feature branches: `feature/*`

## Module Dependency Graph

```
main.rs
  └── lib.rs
      ├── player.rs
      │   ├── constants/save_data.rs
      │   ├── constants/status.rs
      │   ├── constants/item_weapon.rs
      │   ├── growth_type.rs
      │   ├── load.rs
      │   └── save.rs
      ├── monster.rs
      │   └── constants/monster.rs
      ├── battle.rs
      │   ├── player.rs
      │   ├── monster.rs
      │   ├── message.rs
      │   ├── traits/player_input.rs
      │   ├── traits/message_output.rs
      │   └── utility/random_utils.rs
      ├── save.rs
      │   ├── constants/save_data.rs
      │   ├── constants/text.rs
      │   └── utility/binary_utils.rs
      ├── load.rs
      │   ├── constants/save_data.rs
      │   └── constants/text.rs
      ├── growth_type.rs
      │   ├── constants/status.rs
      │   └── constants/text.rs
      ├── input/cli_input.rs
      ├── output/cli_output.rs
      └── constants/* (all)
```

## Code Statistics

**Approximate Lines of Code** (excluding comments/blanks):
- Total: ~5,000 LOC
- Business Logic: ~3,000 LOC
- Constants/Data: ~1,500 LOC
- Tests: ~500 LOC

**File Count**:
- Rust source files: 35
- Test modules: Embedded in each file
- Documentation: 8 markdown files

**Largest Modules**:
1. `battle.rs`: ~667 LOC
2. `save.rs`: ~581 LOC
3. `player.rs`: ~527 LOC
4. `load.rs`: ~297 LOC
5. `monster.rs`: ~246 LOC

## Navigation Tips

### Finding Functionality
- **Password generation**: `save.rs::SaveData::to_password_string()`
- **Password parsing**: `load.rs::decode_from_password_string()`
- **Battle loop**: `battle.rs::Battle::start()`
- **Damage calculation**: `player.rs::normal_damage()`, `monster.rs::normal_damage()`
- **Name growth**: `growth_type.rs::calculate_abc()`
- **Master data**: `constants/*` modules

### Testing Specific Features
- **Password roundtrip**: `save.rs::tests::test_to_password_string_is_20_chars()`
- **Battle mechanics**: `battle.rs::tests::test_decide_enemy_action_for_all_monsters()`
- **Growth calculation**: `growth_type.rs::tests::test_normal_name()`
- **Stat calculation**: `player.rs::tests::test_player_status_parameter()`

### Entry Points for Different Modes
- **CLI**: `main.rs` → `lib.rs::run_from_args()`
- **Library**: Import `damdara::*` and call functions directly
- **Testing**: `cargo test` runs all `#[test]` functions
- **Documentation**: `cargo doc --open` generates browsable docs
