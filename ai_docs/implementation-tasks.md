# Damdara - Implementation Tasks & Roadmap

## Project Status Overview

**Current Version**: v0.8.4
**Status**: Core Features Complete, Polish Phase
**Latest Branch**: `main` (stable), `develop` (integration)

## Completed Features ✅

### Phase 1: Core Password System (v0.1.0 - v0.3.0)
- ✅ Name normalization with dakuten/handakuten expansion
- ✅ 120-bit data structure packing
- ✅ CRC-8 checksum calculation
- ✅ Bit reordering (5 blocks × 24 bits → 20 groups × 6 bits)
- ✅ Cumulative addition offset
- ✅ Hiragana character mapping (PASSWORD_TABLE)
- ✅ Password encoding (SaveData → 20-char string)
- ✅ Password decoding (20-char string → SaveData)
- ✅ Roundtrip validation tests

**Key Commits**:
- `e6287ef`: Merge release PR
- `5788b29`: Merge develop branch
- `9836cfc`: feat: v0.8.4

### Phase 2: Player Management (v0.4.0 - v0.5.0)
- ✅ Player struct with all game state
- ✅ Name-based growth calculation (a/b/c modifiers)
- ✅ Level calculation from experience
- ✅ Stat adjustment by name and level
- ✅ Equipment system (weapon, armor, shield)
- ✅ Inventory system (8 items + herbs + keys)
- ✅ Flags for quest progress (dragon scale, warrior ring, bosses)
- ✅ Attack/defense power calculation
- ✅ HP/MP adjustment with bounds checking
- ✅ Spell learning by level
- ✅ Item usage (herbs, keys, special items)

**Key Commits**:
- `aace83b`: fix: create DummyInput for test
- `a601200`: refactor: move input traits

### Phase 3: Battle System (v0.6.0 - v0.7.0)
- ✅ Turn-based battle loop
- ✅ Player actions (Attack, Spell, Item, Escape)
- ✅ Enemy AI decision tree
- ✅ Status effects (Sleep, Seal)
- ✅ Damage calculation (normal, critical, spell, fire)
- ✅ Evasion system
- ✅ Escape mechanics
- ✅ Message system with Japanese text
- ✅ Trait-based I/O abstraction (PlayerInput, MessageOutput)
- ✅ Monster behavior patterns (40 monsters)
- ✅ Spell effects (Hoimi, Behoimi, Gira, Begirama, Rarirho, Mahoton)
- ✅ Special abilities (Fire breath weak/strong)
- ✅ Battle state management

### Phase 4: CLI & Modes (v0.8.0 - v0.8.4)
- ✅ CLI argument parsing (clap)
- ✅ Start mode (create player from name)
- ✅ Save mode (generate password)
- ✅ Load mode (restore from password)
- ✅ Display mode (view master data)
- ✅ Battle mode (interactive combat)
- ✅ Status option (--option max for maxed stats)
- ✅ View filters (--view for specific data/monsters)

### Phase 5: Testing & Documentation (Current)
- ✅ Unit tests for password system (100% coverage)
- ✅ Unit tests for player/monster/battle (~80% coverage)
- ✅ Integration tests for battle flow
- ✅ README.md (English) with usage examples
- ✅ README_ja.md (Japanese)
- ✅ Inline rustdoc comments
- ✅ Asset images (bit structure diagrams)
- ✅ Homebrew distribution setup
- ✅ Cargo installation support

## In-Progress Tasks 🚧

### Documentation (v0.8.5)
- 🚧 AI-generated architecture documentation (this file!)
- ⏸ Code coverage report (manual review)
- ⏸ Performance benchmarks

**Priority**: Medium
**Estimated Effort**: 2-3 days
**Blockers**: None

**Subtasks**:
- [x] Create requirement.md
- [x] Create tech-stack.md
- [x] Create directory-structure.md
- [x] Create data-structure.md
- [x] Create architecture.md
- [x] Create implementation-tasks.md
- [ ] Review and update existing rustdoc comments
- [ ] Add examples to public API documentation
- [ ] Generate and publish rustdoc to GitHub Pages

### Testing Improvements (v0.8.6)
- ⏸ Add property-based tests for password system (proptest)
- ⏸ Increase battle system test coverage to 90%
- ⏸ Add fuzzing tests for password decoding
- ⏸ Benchmark suite with criterion

**Priority**: Low
**Estimated Effort**: 5-7 days
**Blockers**: None (nice-to-have)

**Subtasks**:
- [ ] Add proptest dependency
- [ ] Write generators for valid SaveData
- [ ] Test password roundtrip with random data
- [ ] Add edge case tests (max values, min values, all flags)
- [ ] Write fuzzer for decode_password_string()
- [ ] Set up criterion benchmarks
- [ ] Establish baseline performance metrics

## Backlog 📋

### Deferred Features (Original Scope)
These features were mentioned in the original DQ1 but are explicitly **not** in current scope:

#### World Exploration
**Status**: ⏸ Deferred indefinitely
**Reason**: Focus on core password/battle system

**Would Require**:
- World map data structure (grid/graph)
- Tile definitions (grass, water, mountains, towns)
- Movement mechanics
- Collision detection
- Encounter rate system
- Town/dungeon data

**Estimated Effort**: 3-4 weeks

#### Town/NPC System
**Status**: ⏸ Deferred indefinitely
**Reason**: Out of scope for CLI-focused tool

**Would Require**:
- Town layouts
- NPC dialogue trees
- Shop system (buy/sell)
- Inn (healing)
- Quest tracking

**Estimated Effort**: 2-3 weeks

#### Persistent Save System
**Status**: ⏸ Deferred (password-only by design)
**Reason**: Password system is the core feature

**Would Require**:
- File I/O
- Serialization (JSON/binary)
- Multiple save slots
- Auto-save

**Estimated Effort**: 1 week

### Enhancement Ideas

#### High Priority
**Target Version**: v0.9.0

1. **JSON Import/Export**
   - **Description**: Alternative serialization format for save data
   - **Use Case**: Easier editing, sharing, analysis
   - **Dependencies**: Add `serde`, `serde_json`
   - **API**:
     ```rust
     pub fn to_json(&self) -> Result<String, Error>
     pub fn from_json(json: &str) -> Result<SaveData, Error>
     ```
   - **Estimated Effort**: 2 days
   - **Tests Required**: Roundtrip validation, schema validation

2. **Enhanced Error Messages**
   - **Description**: Replace `String` errors with typed enum
   - **Benefits**: Better error handling, internationalization-ready
   - **API**:
     ```rust
     pub enum DamdaraError {
         PasswordInvalid { reason: String },
         ChecksumMismatch { expected: u8, actual: u8 },
         // ...
     }
     ```
   - **Estimated Effort**: 3 days
   - **Refactoring**: Update all `Result<T, String>` → `Result<T, DamdaraError>`

3. **Configuration File**
   - **Description**: Support `.damdararc` for default options
   - **Format**: TOML
   - **Location**: `~/.config/damdara/config.toml`
   - **Example**:
     ```toml
     [default]
     name = "ゆうてい"
     mode = "start"

     [display]
     format = "json"
     ```
   - **Estimated Effort**: 2 days

#### Medium Priority
**Target Version**: v0.10.0

4. **Battle Replay System**
   - **Description**: Record and replay battles
   - **Use Cases**: Debugging, sharing, analysis
   - **Data Structure**:
     ```rust
     pub struct BattleReplay {
         initial_state: (Player, Monster),
         actions: Vec<(Turn, Action)>,
     }
     ```
   - **Estimated Effort**: 5 days

5. **AI Opponent Mode**
   - **Description**: Player vs Player with AI controlling second player
   - **AI Strategy**: Simple heuristics (use herbs at low HP, etc.)
   - **Estimated Effort**: 7 days

6. **Custom Monster Editor**
   - **Description**: CLI tool to create custom monster data
   - **Output**: JSON file with monster stats/behavior
   - **Usage**: `damdara --edit-monster slime.json`
   - **Estimated Effort**: 5 days

7. **Stat Calculator Tool**
   - **Description**: Calculate optimal name for desired stats
   - **Algorithm**: Brute-force all 64^4 combinations
   - **Output**: Ranked list of names by criteria
   - **Usage**: `damdara --calc-name --target hp=200,mp=180`
   - **Estimated Effort**: 3 days

#### Low Priority (Nice-to-Have)
**Target Version**: v1.0.0+

8. **Web Assembly Support**
   - **Description**: Compile to WASM for browser play
   - **Dependencies**: `wasm-bindgen`, `web-sys`
   - **Challenges**: Replace stdin/stdout with web I/O
   - **Estimated Effort**: 2 weeks

9. **Graphical UI (Bevy)**
   - **Description**: Replace CLI with game-like interface
   - **Features**: Pixel art graphics, animations, sound
   - **Estimated Effort**: 8-12 weeks
   - **Status**: Research phase

10. **Multiplayer (Network)**
    - **Description**: Online battles
    - **Protocol**: WebSocket or custom TCP
    - **Security**: Authentication, input validation
    - **Estimated Effort**: 4-6 weeks

11. **Localization (i18n)**
    - **Description**: Support multiple languages
    - **Target Languages**: English, Japanese (done), Chinese, Korean
    - **Dependencies**: `fluent`, `unic-langid`
    - **Estimated Effort**: 3 weeks

12. **Plugin System**
    - **Description**: Load custom monsters/items from external files
    - **Format**: Dynamic library (`.so`, `.dylib`, `.dll`)
    - **API**:
      ```rust
      pub trait MonsterPlugin {
          fn name(&self) -> &str;
          fn stats(&self) -> MonsterStats;
          fn behavior(&self) -> MonsterBehavior;
      }
      ```
    - **Estimated Effort**: 2 weeks

## Bug Fixes & Improvements

### Known Issues

#### Priority 1 (Critical)
**None currently**

#### Priority 2 (High)
**None currently**

#### Priority 3 (Medium)

1. **Issue**: Cursor belt and curse necklace flags not saved to password
   - **Location**: `src/player.rs:418-426`
   - **Impact**: Runtime-only flags lost on save
   - **Fix**: Add to SaveData structure (requires password format change)
   - **Status**: Won't fix (breaking change, low priority)

2. **Issue**: Pattern field in SaveData not utilized
   - **Location**: `src/constants/save_data.rs:32`
   - **Impact**: 3 bits wasted
   - **Fix**: Use for future features or remove
   - **Status**: Keeping for forward compatibility

3. **Issue**: raw_bits.rs appears unused
   - **Location**: `src/raw_bits.rs`
   - **Impact**: Dead code in codebase
   - **Fix**: Remove or document purpose
   - **Status**: Needs investigation

#### Priority 4 (Low)

4. **Issue**: No validation for item ID ranges in CLI
   - **Location**: `src/constants/config.rs`
   - **Impact**: Can specify invalid item IDs, silently clamped
   - **Fix**: Add validation in `Cli::to_player_args()`
   - **Estimated Effort**: 1 hour

5. **Issue**: Magic number '255' used directly
   - **Locations**: Multiple files
   - **Impact**: Harder to understand intent
   - **Fix**: Use `BIT_8_MAX` constant consistently
   - **Estimated Effort**: 2 hours

6. **Issue**: Some test functions commented out
   - **Location**: `src/battle.rs:652-667`, `src/monster.rs:233-244`
   - **Impact**: Reduced test coverage
   - **Fix**: Uncomment or remove
   - **Status**: Need clarification on purpose

### Performance Improvements

**Current Performance** (approximate):
- Password encoding: ~5ms
- Password decoding: ~3ms
- Battle turn: ~20ms (with I/O)

**Optimization Opportunities**:

1. **Use HashMap for level lookups**
   - **Current**: Linear search in `get_level_by_exp()`
   - **Proposed**: Precomputed HashMap
   - **Benefit**: O(n) → O(1)
   - **Impact**: Minimal (only called on player creation)
   - **Priority**: Very low

2. **Preallocate message vectors**
   - **Current**: `Vec::new()` each turn
   - **Proposed**: `Vec::with_capacity(10)`
   - **Benefit**: Fewer allocations
   - **Impact**: ~5% faster turn processing
   - **Priority**: Low

3. **Cache name growth calculations**
   - **Current**: Recalculate on every stat query
   - **Proposed**: Store in Player struct
   - **Benefit**: Eliminates redundant calculation
   - **Impact**: Negligible (calculation is fast)
   - **Priority**: Very low

## Development Workflow

### Adding a New Feature

**Standard Process**:

1. **Create feature branch**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/new-feature-name
   ```

2. **Implement feature**
   - Write failing test first (TDD)
   - Implement minimum code to pass test
   - Refactor for clarity
   - Add documentation

3. **Run checks**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   cargo build --release
   ```

4. **Create PR to develop**
   ```bash
   git push origin feature/new-feature-name
   # Open PR on GitHub: feature/new-feature-name → develop
   ```

5. **Merge to main (after testing)**
   ```bash
   git checkout main
   git merge develop
   git tag v0.x.x
   git push origin main --tags
   ```

### Release Process

**Versioning**: Semantic Versioning (MAJOR.MINOR.PATCH)

1. **Update version in Cargo.toml**
   ```toml
   [package]
   version = "0.9.0"
   ```

2. **Update CHANGELOG.md** (create if needed)
   ```markdown
   ## [0.9.0] - 2025-XX-XX
   ### Added
   - JSON import/export
   ### Fixed
   - Item validation
   ```

3. **Create release branch**
   ```bash
   git checkout -b release/v0.9.0 develop
   ```

4. **Final testing**
   ```bash
   cargo test --release
   ./target/release/damdara --version
   ```

5. **Merge to main and tag**
   ```bash
   git checkout main
   git merge release/v0.9.0
   git tag -a v0.9.0 -m "Release v0.9.0"
   git push origin main --tags
   ```

6. **Publish to crates.io**
   ```bash
   cargo publish
   ```

7. **Update Homebrew tap**
   - Update formula in `homebrew-tap` repo
   - Update version and SHA256

### Code Review Checklist

**Before submitting PR**:
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] New code has tests (>70% coverage)
- [ ] New public APIs have rustdoc comments
- [ ] No breaking changes (or marked with BREAKING CHANGE)
- [ ] Error messages in Japanese
- [ ] Example usage added to README (if applicable)

**Reviewer checklist**:
- [ ] Code follows existing patterns
- [ ] Tests cover edge cases
- [ ] Error handling is appropriate
- [ ] No panics in production code
- [ ] Documentation is clear
- [ ] Performance impact is acceptable

## Contribution Guidelines

### How to Contribute

**Welcome contributions**:
- Bug fixes
- Documentation improvements
- Test coverage
- Performance optimizations
- New monster/item data

**Discouraged** (discuss first):
- Major architecture changes
- Breaking API changes
- New dependencies
- Scope expansion (world exploration, etc.)

### Reporting Bugs

**Template**:
```markdown
**Environment**:
- OS: macOS 14.0
- Rust version: 1.75.0
- Damdara version: 0.8.4

**Description**:
Brief description of the issue

**Steps to Reproduce**:
1. Run `damdara -n だい -m save`
2. Observe output

**Expected**:
Should output valid password

**Actual**:
Outputs error message

**Additional Context**:
Any other relevant information
```

### Suggesting Features

**Template**:
```markdown
**Feature Name**: JSON Import/Export

**Problem**:
Current password system is hard to edit manually

**Proposed Solution**:
Add JSON serialization for SaveData

**Alternatives Considered**:
- Binary format (less human-readable)
- TOML format (harder to parse)

**Additional Context**:
Example JSON structure: {...}
```

## Long-Term Vision

### Version 1.0.0 Goals

**Target Date**: 2026 Q2

**Requirements for v1.0.0**:
- [ ] All Phase 1-5 features complete
- [ ] Test coverage >80%
- [ ] Documentation complete (rustdoc, guides, examples)
- [ ] Performance benchmarks established
- [ ] JSON import/export
- [ ] Typed error system
- [ ] Stable public API
- [ ] Production-ready (no known bugs)
- [ ] Cross-platform binaries (macOS, Linux, Windows)
- [ ] Homebrew formula stable

### Post-1.0 Roadmap

**Version 2.0.0**: Graphical UI
- Bevy-based game engine
- Pixel art graphics
- Battle animations
- Sound effects
- Touch/gamepad support

**Version 3.0.0**: Online Features
- Multiplayer battles
- Leaderboards
- Replay sharing
- Tournament mode

## Notes for AI Assistants

### When Adding Features

**Always**:
1. Write tests first
2. Update documentation
3. Follow existing patterns
4. Use `Result<T, String>` for errors (with Japanese messages)
5. Add inline comments for complex logic
6. Update this roadmap file

**Never**:
- Break existing tests
- Add dependencies without justification
- Use unsafe code (except with clear safety comments)
- Panic in production code
- Change password format (breaking change)

### Common Development Commands

```bash
# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests for specific file
cargo test --test integration_test

# Check without building
cargo check

# Build and run
cargo run -- -n だい -m save

# Profile build time
cargo build --release --timings

# Generate docs
cargo doc --open --no-deps

# Clean build artifacts
cargo clean
```

### Useful Git Commands

```bash
# View commit history
git log --oneline --graph --all

# Show changes in a commit
git show <commit-hash>

# Revert a commit
git revert <commit-hash>

# Cherry-pick a commit
git cherry-pick <commit-hash>

# Compare branches
git diff main..develop

# Find when a bug was introduced
git bisect start
```

## Conclusion

Damdara is feature-complete for its core mission: faithfully recreating the Dragon Quest password system and battle mechanics. The current focus is on **polish** (documentation, testing) and **stability** (bug fixes, performance).

Future development will prioritize:
1. **Usability**: JSON export, better error messages
2. **Extensibility**: Plugin system, custom content
3. **Accessibility**: Web version, graphical UI
4. **Community**: Multiplayer, sharing features

All enhancements will maintain the core principle: **accuracy over convenience** for the password system.
