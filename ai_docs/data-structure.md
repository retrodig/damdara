# Damdara - Data Structures

## Overview
This document details all major data structures in Damdara, including their memory layouts, relationships, and encoding schemes.

## Core Entity Structures

### Player

#### `Player` Struct
**Location**: `src/player.rs:18-34`
**Purpose**: Runtime player character state

```rust
pub struct Player {
    pub name: String,          // UTF-8 string, normalized to 4 hiragana
    pub hp: u8,                // Current HP (0-255)
    pub mp: u8,                // Current MP (0-255)
    pub exp: u16,              // Experience points (0-65535)
    pub gold: u16,             // Gold amount (0-65535)
    pub weapon: u8,            // Weapon ID (0-7, 3 bits)
    pub armor: u8,             // Armor ID (0-7, 3 bits)
    pub shield: u8,            // Shield ID (0-3, 2 bits)
    pub items: [u8; 8],        // Item IDs (0-15 each, 4 bits)
    pub herbs: u8,             // Herb count (0-6, 4 bits)
    pub keys: u8,              // Key count (0-6, 4 bits)
    pub is_curse_belt: bool,   // Runtime flag (not saved)
    pub is_curse_necklace: bool, // Runtime flag (not saved)
    pub flags: Flags,          // Status flags (5 bits total)
}
```

**Size**: ~64 bytes (dynamic String allocation)

**Initialization**:
- `Player::new(name: &str)`: Creates with default stats
- `Player::new_with(args: PlayerArgs)`: Custom initialization
- `Player::from_password_string(s: &str)`: Restore from password
- `Player::from_save_data(save: &SaveData)`: Restore from save structure

#### `PlayerArgs` Struct
**Location**: `src/player.rs:36-49`
**Purpose**: Builder pattern for optional Player initialization

```rust
pub struct PlayerArgs {
    pub name: Option<String>,
    pub level: Option<u8>,
    pub exp: Option<u16>,
    pub gold: Option<u16>,
    pub weapon: Option<u8>,
    pub armor: Option<u8>,
    pub shield: Option<u8>,
    pub items: Option<[u8; 8]>,
    pub herbs: Option<u8>,
    pub keys: Option<u8>,
    pub flags: Option<Flags>,
}
```

**Design Rationale**: Allows partial initialization with defaults for unspecified fields

#### `Flags` Struct
**Location**: `src/constants/status.rs`
**Purpose**: Boolean flags for quest progress and special items

```rust
#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub has_dragon_scale: bool,     // Equipped dragon scale (+2 defense)
    pub has_warrior_ring: bool,     // Equipped warrior ring (+2 attack)
    pub has_cursed_necklace: bool,  // Obtained cursed necklace
    pub defeated_dragon: bool,      // Defeated Dragon (boss)
    pub defeated_golem: bool,       // Defeated Golem (boss)
}
```

**Password Encoding**: Packed into 5 bits (1 bit per flag)

### Monster

#### `Monster` Struct
**Location**: `src/monster.rs:9-15`
**Purpose**: Runtime enemy monster instance

```rust
#[derive(Debug, Clone)]
pub struct Monster {
    pub id: u8,                    // Monster index (0-39)
    pub hp: u8,                    // Current HP (randomized on creation)
    pub stats: MonsterStats,       // Static stats from master data
    pub behavior: MonsterBehavior, // AI patterns from master data
}
```

**Size**: ~150 bytes (includes cloned master data)

**Initialization**:
- `Monster::new(index: usize)`: Creates with randomized HP (75-100% of max)

#### `MonsterStats` Struct
**Location**: `src/constants/monster.rs`
**Purpose**: Static monster statistics from master data

```rust
#[derive(Debug, Clone)]
pub struct MonsterStats {
    pub name: &'static str,   // Monster name (Japanese)
    pub hp: u8,               // Maximum HP
    pub mp: u8,               // Maximum MP (unused in most monsters)
    pub attack: u8,           // Attack power
    pub defense: u8,          // Defense power
    pub exp: u16,             // Experience reward
    pub gold: u8,             // Gold reward (max, actual is 75-100%)
}
```

**Storage**: Static array `MONSTER_MASTER[40]`

#### `MonsterBehavior` Struct
**Location**: `src/constants/monster.rs`
**Purpose**: AI behavior patterns and resistances

```rust
#[derive(Debug, Clone)]
pub struct MonsterBehavior {
    pub index: u8,                // Monster index (for special logic)
    pub evade_rate: u8,           // Evasion percentage (0-100)
    pub resist: Resist,           // Magic resistance rates
    pub actions: Vec<MonsterAction>, // Available actions with probabilities
}
```

#### `MonsterAction` Struct
**Location**: `src/constants/monster.rs`
**Purpose**: Individual AI action with weighted probability

```rust
#[derive(Debug, Clone)]
pub struct MonsterAction {
    pub action: ActionType,       // Spell or special ability
    pub probability: u8,          // Probability weight (summed for total)
}
```

#### `ActionType` Enum
**Location**: `src/constants/monster.rs`
**Purpose**: Type of monster action

```rust
#[derive(Debug, Clone)]
pub enum ActionType {
    Spell(Spell),                 // Cast a spell
    Special(&'static str),        // Use special ability (e.g., "ほのお(強)")
}
```

#### `Resist` Struct
**Location**: `src/constants/monster.rs`
**Purpose**: Magic resistance percentages

```rust
#[derive(Debug, Clone)]
pub struct Resist {
    pub gira: u8,      // Fire spell resistance (0-100%)
    pub rariho: u8,    // Sleep spell resistance (0-100%)
    pub mahoton: u8,   // Seal spell resistance (0-100%)
}
```

### Battle State

#### `Battle` Struct
**Location**: `src/battle.rs:19-26`
**Purpose**: Combat orchestration and state management

```rust
pub struct Battle<'a> {
    pub player: Player,                     // Player instance
    pub monster: Monster,                   // Enemy instance
    pub player_state: BattleState,          // Player status effects
    pub monster_state: BattleState,         // Monster status effects
    pub messages: BattleMessages<'a>,       // Message buffer
    pub input: &'a mut dyn PlayerInput,     // Input abstraction
}
```

**Lifetime**: `'a` ensures input/output references are valid

#### `BattleState` Struct
**Location**: `src/constants/battle.rs`
**Purpose**: Status effects and escape flags

```rust
#[derive(Debug, Default)]
pub struct BattleState {
    pub sleep: bool,     // Asleep (skip turn, 33.33% wake chance)
    pub seal: bool,      // Spell sealed (cannot cast magic)
    pub escaped: bool,   // Successfully fled battle
}
```

#### `BattleMessages` Struct
**Location**: `src/message.rs`
**Purpose**: Message queue with output abstraction

```rust
pub struct BattleMessages<'a> {
    player_name: String,            // Player name for messages
    monster_name: String,           // Monster name for messages
    messages: Vec<String>,          // Message buffer
    output: &'a mut dyn MessageOutput, // Output destination
}
```

**Design**: Accumulates messages between turns, then flushes to output

### Status & Stats

#### `Status` Struct
**Location**: `src/constants/status.rs`
**Purpose**: Base stats for a specific level

```rust
#[derive(Debug, Clone)]
pub struct Status {
    pub level: u8,          // Level (1-30)
    pub strength: u8,       // Base strength
    pub agility: u8,        // Base agility
    pub max_hp: u8,         // Maximum HP
    pub max_mp: u8,         // Maximum MP
    pub required_exp: u16,  // Experience needed for this level
}
```

**Storage**: Static array `STATUS_TABLE[30]` (index = level - 1)

**Modifier Application**:
```rust
impl Status {
    pub fn apply_abc_modifiers(&self, modifiers: &GrowthModifiers) -> Status {
        Status {
            level: self.level,
            strength: self.strength + modifiers.c as u8,
            agility: self.agility + modifiers.b as u8,
            max_hp: self.max_hp + modifiers.b as u8,
            max_mp: self.max_mp + modifiers.c as u8,
            required_exp: self.required_exp,
        }
    }
}
```

#### `GrowthModifiers` Struct
**Location**: `src/growth_type.rs:29-34`
**Purpose**: Name-based stat modifiers

```rust
#[derive(Debug)]
pub struct GrowthModifiers {
    pub a: u16,  // Bonus modifier (0-3, currently unused)
    pub b: u16,  // HP/Agility modifier (0-1)
    pub c: u16,  // Strength/MP modifier (0-1)
}
```

**Calculation**:
```rust
let total = calculate_growth_name_total(name); // 0-15
GrowthModifiers {
    a: (total / 4) % 4,  // 0-3
    b: (total / 2) % 2,  // 0-1
    c: total % 2,        // 0-1
}
```

### Equipment & Items

#### `Equipment` Struct
**Location**: `src/constants/item_weapon.rs`
**Purpose**: Common structure for items, weapons, armor, shields

```rust
#[derive(Debug, Clone)]
pub struct Equipment {
    pub name: &'static str,  // Japanese name
    pub price: u16,          // Purchase price
    pub sell: u16,           // Sell price (usually price / 2)
    pub attack: u8,          // Attack bonus (weapons only)
    pub defense: u8,         // Defense bonus (armor/shields/items)
}
```

**Storage**:
- `ITEM_MASTER[15]`: Items (torch, holy water, wings, scales, etc.)
- `WEAPON_MASTER[8]`: Weapons (unarmed, bamboo, club, copper, iron, steel, flame, Roto)
- `ARMOR_MASTER[8]`: Armor (cloth, leather, chain, iron, steel, magic, Roto)
- `SHIELD_MASTER[4]`: Shields (none, leather, iron, mirror)

#### `UnifiedItem` Struct
**Location**: `src/player.rs:76-82`
**Purpose**: Unified representation for inventory display

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnifiedItem {
    pub id: u8,              // Item ID
    pub name: &'static str,  // Item name
    pub count: u8,           // Quantity (for herbs/keys)
    pub kind: ItemKind,      // Item type
}
```

#### `ItemKind` Enum
**Location**: `src/player.rs:84-89`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemKind {
    Herb,       // Consumable healing item
    Key,        // Door key
    Equipment,  // Equippable/usable item
}
```

### Spells

#### `Spell` Enum
**Location**: `src/constants/spell.rs`
**Purpose**: All available spells

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spell {
    Hoimi,      // Heal (HP +10-17)
    Behoimi,    // Greater Heal (HP +85-100)
    Gira,       // Fire (damage 16-23)
    Begirama,   // Greater Fire (damage 58-65)
    Rarirho,    // Sleep (100% on success)
    Mahoton,    // Spell Seal (prevents magic)
    // ... other utility spells not used in battle
}
```

#### `SpellInfo` Struct
**Location**: `src/constants/spell.rs`
**Purpose**: Spell with metadata

```rust
#[derive(Debug, Clone)]
pub struct SpellInfo {
    pub spell: Spell,        // Spell enum
    pub mp_cost: u8,         // MP required
    pub learn_level: u8,     // Level when learned
}
```

**Storage**: Static array `SPELL_INFO_LIST[...]`

## Password System Structures

### SaveData

#### `SaveData` Struct
**Location**: `src/constants/save_data.rs:20-33`
**Purpose**: Intermediate representation for password encoding/decoding

```rust
pub struct SaveData {
    pub name: String,        // 4 hiragana characters (6 bits each)
    pub experience: u16,     // 16 bits (0-65535)
    pub gold: u16,           // 16 bits (0-65535)
    pub weapon: u8,          // 3 bits (0-7)
    pub armor: u8,           // 3 bits (0-7)
    pub shield: u8,          // 2 bits (0-3)
    pub items: [u8; 8],      // 4 bits each (0-15)
    pub herbs: u8,           // 4 bits (0-6)
    pub keys: u8,            // 4 bits (0-6)
    pub flags: Flags,        // 5 bits (5 booleans)
    pub pattern: u8,         // 3 bits (0-7, for bit distribution)
}
```

**Total Bits**: 120 (15 bytes)

### Bit Layout (Password Encoding)

#### 120-Bit Structure
**Visual**: See `assets/images/base_binary.png`

**Byte-by-byte Layout**:
```
Byte 0:  [Checksum: 8 bits]
Byte 1:  [Experience lower: 8 bits]
Byte 2:  [Pattern bit 2: 1][Cursed Necklace: 1][Name[2]: 6 bits]
Byte 3:  [Item[3]: 4 bits][Item[2]: 4 bits]
Byte 4:  [Gold lower: 8 bits]
Byte 5:  [Name[0]: 6 bits][Golem defeated: 1][Pattern bit 1: 1]
Byte 6:  [Item[7]: 4 bits][Item[6]: 4 bits]
Byte 7:  [Pattern bit 0: 1][Dragon defeated: 1][Name[3]: 6 bits]
Byte 8:  [Weapon: 3 bits][Armor: 3 bits][Shield: 2 bits]
Byte 9:  [Gold upper: 8 bits]
Byte 10: [Keys: 4 bits][Herbs: 4 bits]
Byte 11: [Item[5]: 4 bits][Item[4]: 4 bits]
Byte 12: [Experience upper: 8 bits]
Byte 13: [Dragon Scale: 1][Name[1]: 6 bits][Warrior Ring: 1]
Byte 14: [Item[1]: 4 bits][Item[0]: 4 bits]
```

**Design Rationale**:
- Checksum at byte 0 for early validation
- Name scattered across bytes for error distribution
- 16-bit values split into upper/lower bytes
- Flags integrated with name bits to save space

#### Reordered Structure (for Password Generation)
**Process**: `src/save.rs::reorder_password_bits()`

**Algorithm**:
1. Group 120 bits into 5 blocks of 24 bits
2. For each block:
   - Split into 3 bytes (A, B, C)
   - Reverse order: C, B, A
   - Split into 4 groups of 6 bits
   - Reverse groups: [s4, s3, s2, s1]
3. Result: 20 groups of 6 bits

**Example**:
```
Input:  [A7 A6 A5 A4 A3 A2 A1 A0][B7 B6 B5 B4 B3 B2 B1 B0][C7 C6 C5 C4 C3 C2 C1 C0]
        ↓ Reverse bytes
        [C7 C6 C5 C4 C3 C2 C1 C0][B7 B6 B5 B4 B3 B2 B1 B0][A7 A6 A5 A4 A3 A2 A1 A0]
        ↓ Split into 6-bit groups
        [C7..C2][C1..B4][B3..A6][A5..A0]
        ↓ Reverse groups
        [A5..A0][B3..A6][C1..B4][C7..C2]
```

**Visual**: See `assets/images/relocation.png`

#### Offset Application (Step 5)
**Process**: `src/save.rs::apply_password_offsets()`

**Algorithm**:
```rust
let mut result = Vec::new();
let mut previous = 0u8;

for bin in base {
    let mut value = parse_6bit(bin);
    value = (value + 4 + previous) % 64;  // Cumulative addition
    result.push(value);
    previous = value;
}
```

**Purpose**: Obfuscation and error propagation (any single-bit error affects all subsequent characters)

#### Character Mapping
**Table**: `src/constants/text.rs::PASSWORD_TABLE[64]`

```
Index 0-19: あいうえおかきくけこさしすせそたちつてと
Index 20-39: なにぬねのはひふへほまみむめもやゆよらり
Index 40-63: るれろわをんっゃゅょがぎぐげござじずぜぞ...
```

**Final Password**: 20 hiragana characters

### CRC-8 Checksum

#### Algorithm
**Location**: `src/save.rs:296-319`
**Polynomial**: 0x1021 (CRC-16-CCITT truncated to 8 bits)

```rust
pub fn calculate_crc_from_bits(bits: &[String]) -> Result<String, String> {
    let mut crc: u16 = 0;
    for i in 1..15 {  // Skip byte 0 (checksum itself)
        let mut octet = u8::from_str_radix(&bits[i], 2)?;
        for _ in 0..8 {
            let carry_bit = (((crc >> 8) as u8) ^ octet) & 0x80 != 0;
            crc = (crc << 1) & 0xffff;
            octet = (octet << 1) & 0xff;
            if carry_bit {
                crc ^= 0x1021;
            }
        }
    }
    Ok(format!("{:08b}", crc & 0xff))  // Lower 8 bits only
}
```

**Properties**:
- Single-bit error detection: 100%
- Two-bit error detection: High probability
- Burst error detection: Up to 8 bits

## Text Encoding

### Kana Table
**Location**: `src/constants/text.rs::KANA_TABLE[64]`
**Purpose**: Character-to-index mapping for name encoding

```
Index  Character
0-9:   ０１２３４５６７８９
10-25: あいうえおかきくけこさしすせそた
26-41: ちつてとなにぬねのはひふへほま
42-57: みむめもやゆよらりるれろわをん
58-62: っゃゅょ
63:    －
```

**Encoding**: 6 bits per character (0-63)

**Special Handling**:
- Dakuten (゛) and Handakuten (゜) decomposed from base character
- Invalid characters replaced with space (　)
- Names padded to exactly 4 characters

### Name Normalization
**Location**: `src/utility/string_utils.rs::name_normalize()`

**Algorithm**:
1. Expand dakuten: "ば" → "は゛"
2. Expand handakuten: "ぱ" → "は゜"
3. Filter to valid KANA_TABLE characters
4. Pad or truncate to 4 characters with space (　)

**Example**:
```
Input:  "だい"
Step 1: "た゛い"
Step 2: No handakuten
Step 3: All valid
Step 4: "た゛い　" (padded with space)
```

## Enums

### Mode Enum
**Location**: `src/constants/config.rs:77-85`

```rust
#[derive(Debug)]
pub enum Mode {
    Start,    // Create new player from name
    Save,     // Generate password
    Load,     // Restore from password
    Status,   // Display stat tables
    Display,  // Display master data
    Battle,   // Interactive battle
}
```

### PlayerAction Enum
**Location**: `src/constants/battle.rs`

```rust
#[derive(Debug, Clone)]
pub enum PlayerAction {
    Attack,   // Physical attack
    Spell,    // Cast magic
    Item,     // Use item
    Escape,   // Attempt to flee
}
```

### EnemyAction Enum
**Location**: `src/constants/battle.rs`

```rust
#[derive(Debug, Clone)]
pub enum EnemyAction {
    Attack,                    // Physical attack
    Special(MonsterAction),    // Spell or special ability
    Escape,                    // Flee battle
}
```

## Summary Structures (Display)

### PlayerSummary
**Location**: `src/constants/status.rs`
**Purpose**: User-facing player info

```rust
#[derive(Debug)]
pub struct PlayerSummary {
    pub name: String,
    pub level: u8,
    pub hp: u8,
    pub mp: u8,
    pub gold: u16,
    pub experience: u16,
}
```

**Output Format**:
```
PlayerSummary { name: "ゆうてい", level: 1, hp: 15, mp: 0, gold: 0, experience: 0 }
```

### StrengthStatus
**Location**: `src/constants/status.rs`
**Purpose**: Combat-ready stat summary

```rust
#[derive(Debug)]
pub struct StrengthStatus {
    pub level: u8,
    pub strength: u8,
    pub agility: u8,
    pub max_hp: u8,
    pub max_mp: u8,
    pub attack_power: u8,   // Calculated: strength + weapon + bonuses
    pub defense_power: u8,  // Calculated: agility/2 + armor + shield + bonuses
    pub weapon: String,
    pub armor: String,
    pub shield: String,
}
```

**Output Format**:
```
StrengthStatus { level: 1, strength: 4, agility: 3, max_hp: 15, max_mp: 0,
                 attack_power: 4, defense_power: 1, weapon: "なし",
                 armor: "なし", shield: "なし" }
```

## Memory Layout Considerations

### Stack vs Heap
**Stack-Allocated** (fast, fixed size):
- `Player` (excluding String): ~50 bytes
- `Monster`: ~150 bytes
- `Battle`: ~300 bytes
- `Status`: ~12 bytes

**Heap-Allocated** (dynamic size):
- `Player::name`: ~16-32 bytes (String)
- `BattleMessages::messages`: Variable (Vec)
- `MonsterBehavior::actions`: Variable (Vec)

### Optimization Strategies
1. **Static Master Data**: All `MASTER` arrays are `&'static [T]`
2. **Copy Semantics**: Small types (u8, u16) copied by value
3. **Clone on Demand**: MonsterStats/Behavior cloned only when creating instances
4. **Borrowed References**: Equipment lookups return `&Equipment`, not owned

### Alignment
**Rust Compiler Handles**:
- Natural alignment for primitives (u8: 1-byte, u16: 2-byte)
- Struct padding for alignment
- No manual packing required (except password bit manipulation)

## Data Flow

### Password Generation Flow
```
Player
  ↓
SaveData (constructor)
  ↓
build_password_base() → [15 × 8-bit strings]
  ↓
calculate_crc_from_bits() → Update byte 0
  ↓
build_password_bitstring() → "120-bit string"
  ↓
reorder_password_bits() → [20 × 6-bit strings]
  ↓
apply_password_offsets() → [20 × 6-bit indices]
  ↓
indices_to_password_kana() → "20-character password"
```

### Password Parsing Flow
```
"20-character password"
  ↓
decode_password_string() → [20 × 6-bit indices]
  ↓
undo_password_addition() → [20 × 6-bit values]
  ↓
reorder_blocks_back() → [15 × 8-bit bytes]
  ↓
Validate checksum
  ↓
parse_bitstring_to_save_data() → SaveData
  ↓
Player::from_save_data() → Player
```

### Battle Turn Flow
```
Battle::start()
  ↓
while is_battle_continue():
  ↓
  player_turn()
    ↓
    Display command menu
    ↓
    Get user input (via PlayerInput trait)
    ↓
    Execute action (attack/spell/item/escape)
    ↓
    Update player/monster HP
  ↓
  if monster alive:
    ↓
    monster_turn()
      ↓
      Check status effects (sleep, seal)
      ↓
      decide_enemy_action() (AI)
      ↓
      Execute action
      ↓
      Update player/monster HP
  ↓
  Display messages
  ↓
  Clear message buffer
↓
Display battle result
```

## Relationship Diagram

```
                    ┌─────────────┐
                    │   Player    │
                    ├─────────────┤
                    │ name        │
                    │ hp, mp      │
                    │ exp, gold   │
                    │ equipment   │
                    │ items       │
                    │ flags       │───┐
                    └──────┬──────┘   │
                           │          │
                     uses  │          │ contains
                           │          │
                    ┌──────▼──────┐   │
                    │   Status    │   │
                    ├─────────────┤   │
                    │ strength    │   │
                    │ agility     │   │      ┌───────────┐
                    │ max_hp      │   │      │   Flags   │
                    │ max_mp      │   └─────►├───────────┤
                    └─────────────┘          │ booleans  │
                                             └───────────┘
┌──────────────┐
│  SaveData    │
├──────────────┤
│ All fields   │◄────────────────┐
│ (120 bits)   │                 │
└──────┬───────┘                 │ converts
       │                         │
       │ encodes                 │
       ▼                  ┌──────┴──────┐
"Password String"         │   Player    │
       │                  └─────────────┘
       │ decodes
       ▼
┌──────────────┐
│  SaveData    │
└──────────────┘


                    ┌─────────────┐
                    │   Battle    │
                    ├─────────────┤
                    │ player      │───────┐
                    │ monster     │───┐   │
                    │ states      │   │   │
                    │ messages    │   │   │
                    │ input       │   │   │
                    └─────────────┘   │   │
                                      │   │
                            ┌─────────▼───▼──────┐
                            │     Monster        │
                            ├────────────────────┤
                            │ stats              │
                            │ behavior           │
                            └──────┬─────────────┘
                                   │
                         ┌─────────┴─────────┐
                         │                   │
                  ┌──────▼────────┐  ┌───────▼─────────┐
                  │ MonsterStats  │  │ MonsterBehavior │
                  ├───────────────┤  ├─────────────────┤
                  │ hp, attack... │  │ actions         │
                  └───────────────┘  │ resist          │
                                     └─────────────────┘
```

## Constants Storage

### Master Data Tables
**Type**: Static arrays (`&'static [T]`)
**Storage**: Binary `.rodata` section (read-only)

**Tables**:
- `MONSTER_MASTER: [MonsterStats; 40]` (~1.2 KB)
- `MONSTER_BEHAVIORS: [MonsterBehavior; 40]` (~2 KB)
- `STATUS_TABLE: [Status; 30]` (~360 bytes)
- `ITEM_MASTER: [Equipment; 15]` (~300 bytes)
- `WEAPON_MASTER: [Equipment; 8]` (~160 bytes)
- `ARMOR_MASTER: [Equipment; 8]` (~160 bytes)
- `SHIELD_MASTER: [Equipment; 4]` (~80 bytes)
- `KANA_TABLE: [char; 64]` (~256 bytes)
- `PASSWORD_TABLE: [char; 64]` (~256 bytes)

**Total Master Data**: ~5 KB (embedded in binary)

## Type Safety

### NewType Pattern
**Not currently used, but recommended for future**:
```rust
struct MonsterId(u8);
struct ItemId(u8);
struct LevelId(u8);
```

**Benefits**: Prevents mixing incompatible IDs

### Trait Bounds
**PlayerInput / MessageOutput**:
- Runtime polymorphism via trait objects
- `&mut dyn Trait` for flexible implementations
- Zero-cost abstraction (monomorphization at compile time)

## Future Extensions

### Potential Additions
1. **SerDe Support**: JSON import/export
   ```rust
   #[derive(Serialize, Deserialize)]
   pub struct Player { ... }
   ```

2. **Builder Pattern**: For complex initialization
   ```rust
   Player::builder()
       .name("ゆうてい")
       .level(30)
       .maximize_stats()
       .build()
   ```

3. **Type State Pattern**: For battle phases
   ```rust
   struct Battle<State> { ... }
   impl Battle<PlayerTurn> { ... }
   impl Battle<MonsterTurn> { ... }
   ```

4. **BitFlags Crate**: For cleaner flag handling
   ```rust
   bitflags! {
       struct Flags: u8 {
           const DRAGON_SCALE = 0b00001;
           const WARRIOR_RING = 0b00010;
           ...
       }
   }
   ```
