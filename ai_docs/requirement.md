# Damdara - Requirements Specification

## Overview
Damdara is a Rust-based recreation of the Dragon Quest (FC/NES version) core game mechanics, focusing on the "Fukkatsu no Jumon" (Revival Password) system and battle simulation.

## Project Objectives

### Primary Goals
1. **Accurate Recreation**: Faithfully reproduce the original Dragon Quest's password system and game mechanics
2. **Library & CLI Dual Support**: Provide both a library for integration and a CLI for direct usage
3. **Educational Resource**: Serve as a reference implementation for understanding retro game mechanics
4. **Cross-platform Compatibility**: Run on multiple platforms (macOS, Linux, Windows)

## Functional Requirements

### 1. Password System ("Fukkatsu no Jumon")

#### FR-1.1: Password Encoding
- **Description**: Convert game save data to a 20-character hiragana password
- **Input**: Player state (name, experience, gold, equipment, items, flags)
- **Output**: 20-character hiragana string
- **Constraints**:
  - Must use exactly 120 bits of data
  - Must include CRC-8 checksum for validation
  - Password must be reversible to original data

#### FR-1.2: Password Decoding
- **Description**: Restore game save data from a valid password
- **Input**: 20-character hiragana string
- **Output**: Complete player state
- **Constraints**:
  - Must validate checksum before decoding
  - Must handle invalid passwords gracefully
  - Must produce identical state to original encoding

#### FR-1.3: Data Encoding Process
1. Pack 120-bit data structure
2. Calculate CRC-8 checksum
3. Reorder bits in 5 blocks of 24 bits each
4. Apply cumulative addition offset
5. Map to hiragana character table

### 2. Player Management

#### FR-2.1: Character Creation
- **Description**: Create player character with name-based stat modifiers
- **Input**: Player name (up to 4 hiragana characters)
- **Output**: Initialized player with calculated stats
- **Rules**:
  - Name characters are converted to numerical values
  - Growth modifiers (a, b, c) are calculated from name sum
  - Initial HP/MP determined by growth type

#### FR-2.2: Experience & Leveling
- **Description**: Track experience points and calculate level
- **Input**: Experience points (0-65535)
- **Output**: Current level (1-30)
- **Rules**:
  - Level determined by experience thresholds
  - Stats increase per level based on growth type
  - HP/MP restored to maximum on level up

#### FR-2.3: Equipment Management
- **Description**: Manage weapons, armor, shields, and items
- **Constraints**:
  - Weapons: 8 types (3-bit index)
  - Armor: 8 types (3-bit index)
  - Shields: 4 types (2-bit index)
  - Items: 8 slots, 16 types (4-bit per slot)
  - Herbs: 0-6 count
  - Keys: 0-6 count

#### FR-2.4: Stat Calculation
- **Description**: Calculate derived stats from base stats and equipment
- **Formula**:
  - Attack Power = Strength + Weapon Attack + Ring Bonus
  - Defense Power = Agility/2 + Armor Defense + Shield Defense + Scale Bonus
- **Modifiers**:
  - Warrior Ring: +2 Attack
  - Dragon Scale: +2 Defense

### 3. Battle System

#### FR-3.1: Battle Initialization
- **Description**: Start battle encounter with enemy monster
- **Input**: Player state, Monster ID (0-39)
- **Output**: Battle instance with turn management
- **Initialization**:
  - Monster HP randomized (75-100% of max)
  - Battle state reset
  - Turn order determined by agility

#### FR-3.2: Player Actions
- **Available Commands**:
  1. **Attack**: Physical damage based on attack power vs enemy defense
  2. **Spell**: Cast learned spells (requires MP)
  3. **Item**: Use items from inventory
  4. **Escape**: Attempt to flee battle

#### FR-3.3: Damage Calculation
- **Player Attack**:
  - Base Damage = (Attack - Enemy Defense/2) / 4 + Random
  - Critical Hit: 1/32 chance, damage = Attack - (Attack/2 * Random)
  - Evasion: Monster-specific dodge rate

- **Enemy Attack**:
  - Base Damage = (Enemy Attack - Player Defense/2) / 4 + Random
  - Correction applies for low-stat scenarios

#### FR-3.4: Status Effects
- **Sleep**:
  - Affected unit skips turn
  - 33.33% chance to wake up each turn
  - Applied by Rarirho spell

- **Spell Seal**:
  - Prevents spell casting
  - 50% success rate for enemy Mahoton
  - Can be resisted with Roto Armor

#### FR-3.5: Enemy AI
- **Decision Priority**:
  1. Evaluate escape condition (player strength >> enemy attack)
  2. Check support magic needs (low HP → heal, status → debuff)
  3. Choose attack magic if available
  4. Default to physical attack

- **Action Weighting**: Based on probability table per monster

### 4. Display Modes

#### FR-4.1: Start Mode
- **Description**: Generate new character from name only
- **Output**: Display initial stats and equipment

#### FR-4.2: Save Mode
- **Description**: Generate password from current player state
- **Output**: 20-character hiragana password

#### FR-4.3: Load Mode
- **Description**: Restore player from password
- **Input**: Valid 20-character password
- **Output**: Player stats and inventory

#### FR-4.4: Display Mode
- **Description**: View master data lists
- **Supported Categories**:
  - Items
  - Weapons
  - Armor
  - Shields
  - Spells
  - Monsters
  - Status table

#### FR-4.5: Battle Mode
- **Description**: Interactive battle simulation
- **Input**: Player state, optional monster ID
- **Output**: Turn-based battle interface with command prompts

### 5. Growth System

#### FR-5.1: Name-based Growth Calculation
- **Algorithm**:
  ```
  1. Convert each hiragana character to index (0-63)
  2. Apply modulo 16 to each index
  3. Sum all four character values
  4. Calculate modifiers:
     - a = (sum / 4) % 4  (bonus modifier)
     - b = (sum / 2) % 2  (HP/AGI modifier)
     - c = sum % 2        (STR/MP modifier)
  ```

#### FR-5.2: Stat Growth Application
- **Per Level Increase**:
  - HP: Base + b modifier
  - MP: Base + c modifier
  - Strength: Base + c modifier
  - Agility: Base + b modifier

## Non-Functional Requirements

### NFR-1: Performance
- Password encoding/decoding must complete in < 10ms
- Battle turn processing must complete in < 50ms
- Memory footprint should remain under 50MB during operation

### NFR-2: Reliability
- Password system must have 100% accuracy with original algorithm
- All calculations must use integer arithmetic (no floating-point precision issues)
- Invalid inputs must be handled without panicking

### NFR-3: Maintainability
- Code must be modular with clear separation of concerns
- All public APIs must have documentation
- Test coverage should exceed 70%

### NFR-4: Usability
- CLI should provide clear error messages in Japanese
- Help text should explain all available options
- Default values should allow immediate execution without arguments

### NFR-5: Compatibility
- Support Rust edition 2024
- Minimum dependencies (currently: clap, rand)
- No platform-specific code in core logic

## Constraints

### Technical Constraints
1. **Bit-precise Operations**: Password system requires exact bit manipulation
2. **Integer-only Arithmetic**: All calculations must use integer math to match original
3. **Fixed Data Structures**: Array sizes must match original limits (8 items, 4-char names)

### Design Constraints
1. **Trait-based I/O**: Input/output must use traits for testability
2. **No Global State**: All state must be explicitly passed
3. **Immutable Master Data**: Game constants stored as static arrays

### Legal Constraints
1. **MIT License**: Code must remain open source
2. **Attribution**: Copyright notice must mention Square Enix for game content
3. **Educational Use**: Positioned as research/educational project

## Success Criteria

### Phase 1: Core System (Completed)
- ✅ Password encoding/decoding working
- ✅ Player stat calculation accurate
- ✅ Battle system functional
- ✅ CLI interface operational

### Phase 2: Feature Complete
- ✅ All 40 monsters implemented
- ✅ All items and equipment functional
- ✅ Status effects working correctly
- ⏸️ Field exploration (deferred)
- ⏸️ Town interactions (deferred)

### Phase 3: Polish
- Documentation complete
- Test coverage > 70%
- Benchmark suite established
- Installation via Homebrew supported

## Out of Scope

The following features from the original game are explicitly not included:
1. World map traversal
2. Town/dungeon exploration
3. NPC dialogue system
4. Shop purchasing mechanics
5. Story progression tracking
6. Save file persistence (only password-based)
7. Graphics/audio rendering

## Future Considerations

Potential extensions for future versions:
1. Web Assembly compilation for browser play
2. JSON export/import format
3. Battle replay system
4. AI opponent for multiplayer
5. Enhanced monster behavior customization
