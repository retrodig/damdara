# Damdara - Architecture Design

## System Overview

Damdara is a modular, trait-based Rust application that faithfully recreates Dragon Quest's password system and core gameplay mechanics. The architecture prioritizes **correctness**, **testability**, and **maintainability** over performance optimization.

### Design Principles

1. **Separation of Concerns**: Business logic isolated from I/O
2. **Explicit Dependencies**: No global state, all dependencies passed explicitly
3. **Fail-Fast**: Result types for error handling, no silent failures
4. **Data Immutability**: Constants stored as static immutable arrays
5. **Type Safety**: Strong typing prevents incorrect state transitions

## High-Level Architecture

### Layered Architecture

```
┌────────────────────────────────────────────────────────┐
│                    CLI Layer                           │
│  (main.rs, lib.rs::run_from_args)                     │
│  - Argument parsing (clap)                            │
│  - Mode dispatching                                   │
│  - Output formatting                                  │
└────────────────┬───────────────────────────────────────┘
                 │
┌────────────────▼───────────────────────────────────────┐
│                  Domain Layer                          │
│  (player.rs, monster.rs, battle.rs)                   │
│  - Game entities                                      │
│  - Business rules                                     │
│  - State management                                   │
└────────────────┬───────────────────────────────────────┘
                 │
┌────────────────▼───────────────────────────────────────┐
│                  Service Layer                         │
│  (save.rs, load.rs, growth_type.rs)                  │
│  - Password encoding/decoding                         │
│  - Stat calculations                                  │
│  - Damage formulas                                    │
└────────────────┬───────────────────────────────────────┘
                 │
┌────────────────▼───────────────────────────────────────┐
│               Infrastructure Layer                     │
│  (utility/*, traits/*, input/*, output/*)            │
│  - I/O abstraction (traits)                          │
│  - Utility functions                                  │
│  - Random number generation                           │
└────────────────┬───────────────────────────────────────┘
                 │
┌────────────────▼───────────────────────────────────────┐
│                  Data Layer                            │
│  (constants/*)                                        │
│  - Master data (monsters, items, stats)               │
│  - Text tables                                        │
│  - Configuration                                      │
└────────────────────────────────────────────────────────┘
```

### Module Dependency Graph

```
                    main.rs (binary)
                        ┃
                        ▼
                      lib.rs
                        ┃
        ┏━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━┓
        ▼               ▼               ▼
    player.rs      battle.rs       monster.rs
        ┃               ┃               ┃
        ┃       ┏━━━━━━━╋━━━━━━━┓       ┃
        ┃       ▼       ▼       ▼       ┃
        ┃   message.rs  ┃   traits/*    ┃
        ┃               ┃       ┃       ┃
        ┗━━━━━━━┳━━━━━━━┻━━━━━━━┻━━━━━━━┛
                ▼
        ┏━━━━━━━╋━━━━━━━┓
        ▼       ▼       ▼
    save.rs  load.rs  growth_type.rs
        ┃       ┃       ┃
        ┗━━━━━━━┻━━━━━━━┛
                ▼
        ┏━━━━━━━╋━━━━━━━┓
        ▼       ▼       ▼
   utility/*  input/*  output/*
        ┃       ┃       ┃
        ┗━━━━━━━┻━━━━━━━┛
                ▼
          constants/*
```

**Dependency Rules**:
- No circular dependencies
- Lower layers cannot import upper layers
- Cross-layer communication via interfaces (traits)

## Core Components

### 1. CLI Layer

#### Responsibility
- Parse command-line arguments
- Validate user input
- Dispatch to appropriate mode handler
- Format and display output

#### Key Files
- `src/main.rs`: Entry point, delegates to lib
- `src/lib.rs::run_from_args()`: Mode dispatcher
- `src/constants/config.rs`: CLI configuration

#### Data Flow
```
User Command
     │
     ▼
clap::Parser::parse()
     │
     ▼
Cli struct
     │
     ▼
Mode enum
     │
     ├─► Mode::Start → Display player stats
     ├─► Mode::Save → Generate password
     ├─► Mode::Load → Restore from password
     ├─► Mode::Display → Show master data
     └─► Mode::Battle → Run battle loop
     │
     ▼
Output to stdout
```

#### Error Handling
- Invalid arguments → clap displays help text and exits
- Mode execution errors → Result propagated to main, displayed in Japanese
- Graceful degradation: Invalid data shows default values

### 2. Domain Layer

#### 2.1 Player Entity

**Location**: `src/player.rs`

**Responsibilities**:
- Manage player state (HP, MP, exp, gold, equipment)
- Calculate derived stats (level, attack power, defense power)
- Handle stat modifications (damage, healing, level up)
- Serialize to/from password format

**Key Methods**:
```rust
impl Player {
    // Creation
    pub fn new(name: &str) -> Self
    pub fn new_with(args: PlayerArgs) -> Self
    pub fn from_password_string(s: &str) -> Result<Self, String>

    // Queries
    pub fn level(&self) -> u8
    pub fn attack_power(&self) -> u8
    pub fn defense_power(&self) -> u8

    // Commands
    pub fn adjust_hp(&mut self, amount: i16)
    pub fn consume_mp(&mut self, spell_info: &SpellInfo)
    pub fn use_herbs(&mut self)

    // Serialization
    pub fn to_password_string(&self) -> Result<String, String>
}
```

**Design Patterns**:
- **Builder Pattern**: `PlayerArgs` for flexible initialization
- **Factory Pattern**: Multiple constructors for different creation scenarios
- **Value Object**: Immutable computed properties (level, attack, defense)

#### 2.2 Monster Entity

**Location**: `src/monster.rs`

**Responsibilities**:
- Represent enemy monster instances
- Calculate damage to player
- Provide AI behavior data
- Manage monster HP and status

**Key Methods**:
```rust
impl Monster {
    // Creation
    pub fn new(index: usize) -> Self  // Random HP initialization

    // Queries
    pub fn has_support_magic(&self) -> bool
    pub fn has_attack_skill(&self) -> bool
    pub fn is_final_boss(&self) -> bool

    // Combat
    pub fn normal_damage(&self, player: &Player) -> u8
    pub fn adjust_hp(&mut self, amount: i16)
}
```

**Design Patterns**:
- **Prototype Pattern**: Clone master data for each instance
- **Strategy Pattern**: Different behaviors via `MonsterBehavior`

#### 2.3 Battle Orchestrator

**Location**: `src/battle.rs`

**Responsibilities**:
- Manage turn-based combat loop
- Execute player commands
- Execute enemy AI decisions
- Track status effects (sleep, seal)
- Display battle messages

**Architecture**:
```
Battle
  ├── player: Player
  ├── monster: Monster
  ├── player_state: BattleState
  ├── monster_state: BattleState
  ├── messages: BattleMessages<'a>
  └── input: &'a mut dyn PlayerInput
```

**State Machine**:
```
┌──────────┐
│  Start   │
└────┬─────┘
     │
     ▼
┌──────────────────┐
│  Player Turn     │◄─────┐
│  - Get input     │      │
│  - Execute       │      │
│  - Update state  │      │
└────┬─────────────┘      │
     │                    │
     ▼                    │
┌──────────────────┐      │
│ Check Continue?  │      │
└────┬─────────────┘      │
     │                    │
     ├─► Yes ─────────────┤
     │                    │
     ▼                    │
┌──────────────────┐      │
│  Monster Turn    │      │
│  - AI decision   │      │
│  - Execute       │      │
│  - Update state  │      │
└────┬─────────────┘      │
     │                    │
     ▼                    │
┌──────────────────┐      │
│ Check Continue?  │      │
└────┬─────────────┘      │
     │                    │
     ├─► Yes ─────────────┘
     │
     ▼ No
┌──────────┐
│   End    │
│  Result  │
└──────────┘
```

**Key Methods**:
```rust
impl Battle {
    pub fn start(&mut self)  // Main battle loop

    // Player actions
    fn player_action_attack(&mut self)
    fn player_action_spell(&mut self)
    fn player_action_item(&mut self)
    fn player_action_escape(&mut self)

    // Enemy actions
    fn decide_enemy_action(&self) -> EnemyAction
    fn handle_enemy_normal_attack(&mut self)
    fn handle_enemy_heal_spell(&mut self, ...)
    fn handle_enemy_attack_spell(&mut self, ...)
}
```

**Design Patterns**:
- **State Pattern**: BattleState for status effects
- **Command Pattern**: PlayerAction/EnemyAction enums
- **Strategy Pattern**: AI decision tree in `decide_enemy_action()`
- **Observer Pattern**: Message system accumulates events

### 3. Service Layer

#### 3.1 Password Encoding Service

**Location**: `src/save.rs`

**Responsibilities**:
- Convert SaveData → 20-character password
- Pack 120 bits of game state
- Calculate CRC-8 checksum
- Apply bit reordering and obfuscation

**Algorithm Pipeline**:
```
SaveData
    ↓
build_password_base()
    ↓ Pack into 15 bytes (120 bits)
[15 × 8-bit binary strings]
    ↓
calculate_crc_from_bits()
    ↓ CRC-8 checksum
Update byte 0
    ↓
build_password_bitstring()
    ↓ Concatenate
"120-bit string"
    ↓
reorder_password_bits()
    ↓ Block reversal + 6-bit splitting
[20 × 6-bit binary strings]
    ↓
apply_password_offsets()
    ↓ Cumulative addition mod 64
[20 × 6-bit indices]
    ↓
indices_to_password_kana()
    ↓ Character table lookup
"Password string"
```

**Error Handling**:
```rust
Result<String, String>
    ↑
    ├─ Unsupported character in name → Err("未対応の文字が含まれています: X")
    ├─ Invalid bit length → Err("ビット列は120bit必要です")
    └─ Character index out of range → Err("無効なインデックス: N")
```

#### 3.2 Password Decoding Service

**Location**: `src/load.rs`

**Responsibilities**:
- Convert 20-character password → SaveData
- Validate password format and checksum
- Reverse obfuscation and bit reordering
- Extract individual fields via bit masking

**Algorithm Pipeline**:
```
"Password string"
    ↓
decode_password_string()
    ↓ Character table lookup
[20 × 6-bit indices]
    ↓
undo_password_addition()
    ↓ Reverse cumulative addition
[20 × 6-bit values]
    ↓
reorder_blocks_back()
    ↓ Reverse block reordering
[15 × 8-bit bytes]
    ↓
Validate checksum
    ↓ Compare calculated vs stored
Match?
    ↓ Yes
parse_bitstring_to_save_data()
    ↓ Extract fields via bit masks
SaveData
```

**Error Handling**:
```rust
Result<SaveData, String>
    ↑
    ├─ Wrong length → Err("ふっかつのじゅもんは20文字である必要があります")
    ├─ Invalid character → Err("未対応の文字が含まれています: X")
    ├─ Checksum mismatch → Err("チェックサムが一致しません")
    └─ Invalid bit structure → Err("無効なビット構造")
```

#### 3.3 Growth Calculation Service

**Location**: `src/growth_type.rs`

**Responsibilities**:
- Calculate name-based stat modifiers
- Apply modifiers to base stats
- Generate adjusted stat tables

**Algorithm**:
```
Player Name (4 characters)
    ↓
name_normalize()
    ↓ Expand dakuten, pad to 4 chars
"Normalized name"
    ↓
calculate_growth_name_total()
    ↓ Sum (char_index % 16) for each char
Total value (0-15)
    ↓
calculate_abc()
    ↓ a = (total/4)%4, b = (total/2)%2, c = total%2
GrowthModifiers { a, b, c }
    ↓
get_adjusted_status_by_name_lv()
    ↓ Apply modifiers to base stats
Status (adjusted)
```

**Formula**:
```
HP_adjusted  = HP_base  + b
MP_adjusted  = MP_base  + c
STR_adjusted = STR_base + c
AGI_adjusted = AGI_base + b
```

### 4. Infrastructure Layer

#### 4.1 Trait Abstractions

**Design Goal**: Decouple I/O from business logic for testability

##### PlayerInput Trait
**Location**: `src/traits/player_input.rs`

```rust
pub trait PlayerInput {
    fn get_player_input(&mut self, max: usize) -> usize;
    fn get_player_action(&mut self, display_commands: &mut dyn FnMut()) -> PlayerAction;
}
```

**Implementations**:
- **CliInput**: Read from stdin, retry on invalid input
- **DummyInput** (tests): Return predetermined sequence

**Benefits**:
- Battle logic testable without human interaction
- Future support for network/AI opponents

##### MessageOutput Trait
**Location**: `src/traits/message_output.rs`

```rust
pub trait MessageOutput {
    fn output(&mut self, message: &str);
}
```

**Implementations**:
- **CliOutput**: Print to stdout
- **BufferOutput**: Collect in Vec for inspection
- **DummyOutput** (tests): No-op

**Benefits**:
- Capture battle messages in tests
- Future support for GUI/web output

#### 4.2 Utility Functions

**Organization**: Grouped by concern in `utility/*`

##### Binary Utilities (`utility/binary_utils.rs`)
- **Purpose**: Bit manipulation helpers
- **Key Functions**:
  - `combine_bits()`: Pack multiple values into single byte
  - `validate_6bit_array()`: Check 20×6bit structure
  - `validate_120bit()`: Check 15×8bit structure

##### String Utilities (`utility/string_utils.rs`)
- **Purpose**: Japanese text processing
- **Key Functions**:
  - `name_normalize()`: Standardize player names
  - `kana_index()`: Character → table index
  - `nth_char()`: UTF-8 safe character access

##### Random Utilities (`utility/random_utils.rs`)
- **Purpose**: RNG wrappers for game logic
- **Key Functions**:
  - `random_value()`: 0 to max inclusive
  - `generate_in_range()`: Min to max inclusive
  - `random_success_by_percent()`: Probability check
  - `check_escape_success()`: Escape formula

**Design Pattern**: **Facade Pattern** - Simplified interface over rand crate

##### Status Utilities (`utility/status_utils.rs`)
- **Purpose**: Stat table lookups
- **Key Functions**:
  - `get_level_by_exp()`: Binary search in stat table
  - `get_status_by_level()`: Array lookup
  - `resolve_experience()`: Handle level vs exp priority

##### Spell Utilities (`utility/spell_utils.rs`)
- **Purpose**: Spell effect calculations
- **Key Functions**:
  - `player_spell_effect()`: Damage/heal amount
  - `monster_action_effect()`: Enemy ability effects
  - `spells_learned_by_level()`: Filter spell list

##### Monster Utilities (`utility/monster_utils.rs`)
- **Purpose**: AI helper functions
- **Key Functions**:
  - `choose_action()`: Weighted random selection

### 5. Data Layer

#### 5.1 Constants Organization

**Design Goal**: Centralize game data as immutable constants

**Module Structure**:
```
constants/
├── mod.rs          # Re-exports
├── battle.rs       # Battle types (BattleState, PlayerAction, EnemyAction)
├── config.rs       # CLI config (Cli, Mode)
├── item_weapon.rs  # Equipment master data
├── monster.rs      # Monster master data
├── save_data.rs    # SaveData structure
├── spell.rs        # Spell definitions
├── status.rs       # Stat tables and flags
└── text.rs         # Character encoding tables
```

#### 5.2 Master Data Storage

**Storage Method**: Static arrays with `&'static` references

**Example**:
```rust
pub static MONSTER_MASTER: [MonsterStats; 40] = [
    MonsterStats {
        name: "スライム",
        hp: 3,
        attack: 5,
        defense: 2,
        exp: 1,
        gold: 2,
    },
    // ... 39 more entries
];
```

**Benefits**:
- **Zero Runtime Cost**: Data embedded in binary
- **Type Safety**: Compile-time validation
- **Immutability**: Cannot be accidentally modified
- **Fast Access**: Direct memory access, no parsing

**Tradeoffs**:
- Cannot modify data at runtime (by design)
- Binary size includes all data (~5 KB)

## Design Patterns

### 1. Builder Pattern

**Usage**: `PlayerArgs` for flexible player initialization

```rust
let player = Player::new_with(PlayerArgs {
    name: Some("だい".to_string()),
    exp: Some(5000),
    gold: Some(1000),
    weapon: Some(3),
    ..Default::default()  // Other fields use defaults
});
```

**Benefits**:
- Optional parameters without method overloading
- Clear initialization intent
- Partial configuration with defaults

### 2. Factory Pattern

**Usage**: Multiple constructors for `Player`

```rust
// From name only
Player::new("ゆうてい")

// From args
Player::new_with(args)

// From password
Player::from_password_string("ぢばげぞでぶいまももれぎざぞでぶいよごぜ")?

// From save data
Player::from_save_data(&save)
```

**Benefits**:
- Consistent creation interface
- Encapsulated initialization logic
- Named constructors for clarity

### 3. Strategy Pattern

**Usage**: Enemy AI behavior

```rust
pub struct MonsterBehavior {
    pub actions: Vec<MonsterAction>,  // Strategy pool
    pub resist: Resist,
    // ...
}

impl Monster {
    pub fn support_spells_actions(&self) -> Vec<MonsterAction> {
        self.behavior.actions.iter()
            .filter(|action| matches!(action, ActionType::Spell(healing/debuff)))
            .collect()
    }
}
```

**Benefits**:
- Different AI per monster
- Easy to add new behaviors
- Data-driven enemy design

### 4. State Pattern

**Usage**: Battle status effects

```rust
pub struct BattleState {
    pub sleep: bool,
    pub seal: bool,
    pub escaped: bool,
}

impl Battle {
    fn player_turn(&mut self) {
        if self.player_state.sleep {
            // Handle sleep state
            if random_success(33.33) {
                self.player_state.sleep = false;
            } else {
                return;  // Skip turn
            }
        }
        // Normal turn logic
    }
}
```

**Benefits**:
- Clear state transitions
- Encapsulated state behavior
- Easy to add new states

### 5. Command Pattern

**Usage**: Player/Enemy actions as enums

```rust
pub enum PlayerAction {
    Attack,
    Spell,
    Item,
    Escape,
}

impl Battle {
    fn commands(&mut self) {
        let action = self.input.get_player_action(...);
        match action {
            PlayerAction::Attack => self.player_action_attack(),
            PlayerAction::Spell => self.player_action_spell(),
            PlayerAction::Item => self.player_action_item(),
            PlayerAction::Escape => self.player_action_escape(),
        }
    }
}
```

**Benefits**:
- Decoupled action execution
- Easy to add new actions
- Testable action handlers

### 6. Observer Pattern (Simplified)

**Usage**: Battle message accumulation

```rust
pub struct BattleMessages<'a> {
    messages: Vec<String>,
    output: &'a mut dyn MessageOutput,
}

impl BattleMessages<'_> {
    pub fn add_monster_damage(&mut self, damage: u8) {
        self.messages.push(format!("{}ポイントのダメージをあたえた！", damage));
    }

    pub fn display(&mut self) {
        for msg in &self.messages {
            self.output.output(msg);
        }
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
```

**Benefits**:
- Batched message display
- Decoupled message generation from rendering
- Testable message content

### 7. Dependency Injection

**Usage**: Trait objects for I/O

```rust
pub struct Battle<'a> {
    input: &'a mut dyn PlayerInput,
    // ... (output via messages)
}

impl<'a> Battle<'a> {
    pub fn new(
        player: Player,
        monster: Monster,
        input: &'a mut dyn PlayerInput,
        output: &'a mut dyn MessageOutput,
    ) -> Self {
        // ...
    }
}
```

**Benefits**:
- Testable without real I/O
- Flexible implementations
- Clear dependencies

## Error Handling Strategy

### Error Types

**Approach**: `Result<T, String>` for domain errors

**Rationale**:
- User-facing errors need Japanese messages
- No need for complex error types (yet)
- `String` provides flexibility

**Example**:
```rust
pub fn to_password_string(&self) -> Result<String, String> {
    let bitstring = self.build_password_bitstring()?;  // Propagate error
    let reordered = reorder_password_bits(&bitstring)?;
    let kana_indices = apply_password_offsets(&reordered)?;
    indices_to_password_kana(&kana_indices)
}
```

### Error Propagation

**Pattern**: `?` operator for early return

**Example**:
```rust
let value = some_function()
    .map_err(|e| format!("Context: {}", e))?;  // Add context
```

### Error Recovery

**Strategy**: Retry or default values

**Example**:
```rust
// Retry on invalid input
loop {
    let input = read_line();
    match parse_input(input) {
        Ok(value) => return value,
        Err(e) => {
            println!("Invalid input: {}. Try again.", e);
            continue;
        }
    }
}
```

### Future: Custom Error Types

**Potential Structure**:
```rust
#[derive(Debug)]
pub enum DamdaraError {
    PasswordInvalid { reason: String },
    ChecksumMismatch { expected: u8, actual: u8 },
    CharacterNotFound { character: char },
    BattleActionInvalid { action: String },
}

impl std::fmt::Display for DamdaraError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PasswordInvalid { reason } => {
                write!(f, "パスワードが無効です: {}", reason)
            }
            // ...
        }
    }
}
```

## Testing Strategy

### Unit Tests

**Organization**: Inline `#[cfg(test)] mod tests` in each file

**Coverage**:
- Password encoding/decoding: 100% (critical path)
- Damage calculations: ~90%
- Stat calculations: ~90%
- Overall: ~70%

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_roundtrip() {
        let original = SaveData::new();
        let password = original.to_password_string().unwrap();
        let decoded = decode_from_password_string(&password).unwrap();
        assert_eq!(original.name, decoded.name);
        assert_eq!(original.experience, decoded.experience);
        // ... other fields
    }
}
```

### Integration Tests

**Approach**: Full battle simulation with mock I/O

**Example**:
```rust
#[test]
fn test_battle_player_wins() {
    let player = Player::new("ゆうてい");
    let monster = Monster::new(0);  // Slime
    let mut output = DummyOutput;
    let mut input = DummyInput::new(vec![
        PlayerAction::Attack,
        PlayerAction::Attack,
        // ... until monster defeated
    ]);
    let mut battle = Battle::new(player, monster, &mut input, &mut output);
    battle.start();
    assert!(battle.player.is_alive());
    assert!(!battle.monster.is_alive());
}
```

### Property-Based Testing (Future)

**Potential Use**: Password system validation

```rust
// Hypothetical with proptest
proptest! {
    #[test]
    fn test_password_always_20_chars(name in any_valid_name()) {
        let save = SaveData { name, ..default() };
        let password = save.to_password_string().unwrap();
        assert_eq!(password.chars().count(), 20);
    }
}
```

## Performance Considerations

### Hot Paths

**Identified**:
1. Damage calculation (called every turn)
2. Password encoding (one-time per save)
3. Password decoding (one-time per load)

**Optimization Strategy**:
- **Damage Calculation**: Integer-only arithmetic, inline small functions
- **Password**: Acceptable even at 50ms (infrequent operation)

### Memory Allocation

**Minimization**:
- Static master data (zero runtime cost)
- Small stack-allocated structs
- Vec reuse in battle messages (clear instead of recreate)

**Benchmark Target**:
- Battle turn: < 50ms (including I/O)
- Password ops: < 10ms (CPU only)

### Future Optimizations

**If needed**:
1. **Cache level lookups**: HashMap instead of linear search
2. **Preallocate vectors**: With `Vec::with_capacity()`
3. **Profile-guided optimization**: `cargo build --release` with PGO

## Extensibility

### Adding New Monsters

**Steps**:
1. Add entry to `MONSTER_MASTER` array
2. Add behavior to `MONSTER_BEHAVIORS` array
3. Update array size constants
4. No code changes required

**Example**:
```rust
MonsterStats {
    name: "ニューモンスター",
    hp: 50,
    attack: 30,
    defense: 20,
    exp: 100,
    gold: 50,
}
```

### Adding New Spells

**Steps**:
1. Add variant to `Spell` enum
2. Add entry to `SPELL_INFO_LIST`
3. Implement effect in `spell_utils.rs`
4. Add battle handler in `battle.rs`

**Example**:
```rust
// In spell.rs
pub enum Spell {
    // ... existing
    NewSpell,
}

// In spell_utils.rs
pub fn player_spell_effect(spell: Spell) -> u8 {
    match spell {
        Spell::NewSpell => generate_in_range(30, 40),
        // ...
    }
}
```

### Adding New Items

**Steps**:
1. Add entry to `ITEM_MASTER`
2. Implement use logic in `battle.rs::use_item()`
3. Update item ID validation

### Adding New Modes

**Steps**:
1. Add variant to `Mode` enum
2. Implement handler in `lib.rs::run_from_args()`
3. Update CLI help text

**Example**:
```rust
// In config.rs
pub enum Mode {
    // ... existing
    Multiplayer,
}

// In lib.rs
match mode {
    // ... existing
    Mode::Multiplayer => run_multiplayer_mode(&player),
}
```

## Security Considerations

### Input Validation

**Implemented**:
- Password length check (exactly 20 characters)
- Character whitelist (only hiragana in KANA_TABLE)
- Checksum validation
- Integer overflow protection (saturating ops)

### Memory Safety

**Guaranteed by Rust**:
- No buffer overflows
- No use-after-free
- No data races (single-threaded)

### Future Concerns

**Network Play** (if added):
- Authentication required
- Input sanitization
- Rate limiting

## Scalability

### Current Limitations

**By Design**:
- Single-threaded (sufficient for turn-based game)
- No persistent storage (password-only)
- Fixed master data (no dynamic content)

### Future Enhancements

**If needed**:
1. **Async I/O**: For network play
2. **Database**: For persistent worlds
3. **Parallelism**: For AI simulations

## Deployment Architecture

### Binary Distribution

```
damdara (executable)
  ↓
Statically linked
  ↓
~2 MB binary (stripped)
  ↓
No external dependencies
  ↓
Runs on bare OS (no runtime)
```

### Library Usage

```
Cargo.toml
  ↓
dependencies = { damdara = "0.8.4" }
  ↓
main.rs
  ↓
use damdara::{Player, Battle, Monster};
  ↓
Custom application logic
```

## Documentation Strategy

### Code Documentation

**Standard**: Rustdoc comments (`///`)

**Coverage**:
- All public APIs
- Complex algorithms (password encoding)
- Non-obvious design decisions

**Generation**: `cargo doc --open`

### Architecture Documentation

**This File**: High-level system design

**Companion Files**:
- `requirement.md`: What system does
- `tech-stack.md`: Technology choices
- `data-structure.md`: Data layout
- `directory-structure.md`: File organization
- `implementation-tasks.md`: Development roadmap

## Future Architecture Directions

### Potential Refactorings

1. **Type-State Pattern**: For battle phases
   ```rust
   struct Battle<State> { ... }
   impl Battle<Initialized> { ... }
   impl Battle<PlayerTurn> { ... }
   impl Battle<MonsterTurn> { ... }
   ```

2. **Entity-Component-System**: For complex interactions
   ```rust
   struct Entity { id: u32 }
   struct Health { current: u8, max: u8 }
   struct Attack { power: u8 }
   ```

3. **Actor Model**: For concurrent battles
   ```rust
   actor Battle {
       receive PlayerAction,
       receive MonsterAction,
       send BattleEvent,
   }
   ```

### Technology Migrations

**If requirements change**:
- **GraphQL API**: For web frontend
- **WASM**: For browser play
- **Bevy Engine**: For graphical version
- **Serde**: For JSON import/export

## Conclusion

Damdara's architecture prioritizes **correctness** and **maintainability** through:
- **Strong typing**: Prevents invalid states
- **Immutable data**: Eliminates entire classes of bugs
- **Explicit dependencies**: Clear module boundaries
- **Trait abstraction**: Testable without I/O
- **Layered design**: Separates concerns

The system is **extensible** (easy to add content) and **modular** (easy to modify components) while remaining **faithful** to the original Dragon Quest mechanics.
