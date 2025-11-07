# Damdara - Technology Stack

## Language & Runtime

### Rust (Edition 2024)
**Version**: Latest stable
**Justification**:
- **Memory Safety**: Zero-cost abstractions without garbage collection overhead
- **Performance**: Comparable to C/C++ for calculation-heavy game logic
- **Type Safety**: Strong type system prevents common bugs in bit manipulation
- **Tooling**: Cargo provides excellent dependency management and testing
- **Cross-platform**: Single codebase compiles to multiple targets

**Key Language Features Used**:
- Pattern matching for state management
- Trait objects for dependency injection
- Zero-copy string slicing for password parsing
- Integer overflow protection (saturating/wrapping operations)
- Const generics for fixed-size arrays

## Dependencies

### Production Dependencies

#### clap v4.5
**Purpose**: Command-line argument parsing
**Features Used**: `derive` macro for declarative CLI definition
**Justification**:
- Industry-standard for Rust CLI applications
- Type-safe argument validation
- Automatic help generation
- Subcommand support for different modes

**Usage Example**:
```rust
#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long, default_value_t = String::from("ゆうてい"))]
    pub name: String,

    #[clap(short, long, default_value_t = 0)]
    pub exp: u16,
    // ...
}
```

#### rand v0.9.0
**Purpose**: Random number generation for battle calculations
**Justification**:
- Cryptographically secure RNG not required (game logic only)
- Portable across platforms
- Provides range-based random generation

**Usage**:
- Monster initial HP variation (75-100% of max)
- Damage randomization in combat
- Critical hit determination (1/32 chance)
- Escape success calculation
- Enemy action selection

**Key Functions**:
```rust
random_value(255)           // 0-255 random u8
generate_in_range(min, max) // Inclusive range
random_success_by_percent() // Probability check
```

## Development Dependencies

### Standard Rust Toolchain

#### cargo
**Purpose**: Build system, package manager, test runner
**Commands Used**:
- `cargo build`: Compile project
- `cargo test`: Run test suite
- `cargo run`: Execute binary
- `cargo doc`: Generate documentation
- `cargo install`: Install binary globally

#### rustfmt
**Purpose**: Code formatting
**Configuration**: Default style guide

#### clippy
**Purpose**: Linting and static analysis
**Configuration**: Strict warnings enabled

## Build System

### Cargo Configuration
**Location**: `Cargo.toml`

```toml
[package]
name = "damdara"
version = "0.8.4"
edition = "2024"

[lib]
path = "src/lib.rs"

[[bin]]
name = "damdara"
path = "src/main.rs"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
rand = "0.9.0"
```

**Key Configuration**:
- **Dual Output**: Both library (`lib.rs`) and binary (`main.rs`)
- **Edition 2024**: Latest language features
- **Minimal Dependencies**: Only 2 external crates

## Testing Framework

### Built-in Test System
**Framework**: `cargo test` with `#[test]` attributes

**Test Categories**:

#### Unit Tests
- Located in each module (`#[cfg(test)] mod tests`)
- Test individual functions in isolation
- Mock I/O via trait objects

**Example**:
```rust
#[test]
fn test_encode_name_to_bits_exact_value() {
    let save = SaveData::new();
    let result = save.encode_name_to_bits().unwrap();
    assert_eq!(result, 12109579);
}
```

#### Integration Tests
- Test multi-module workflows
- Battle system end-to-end tests
- Password roundtrip validation

**Coverage Targets**:
- Password encoding/decoding: 100%
- Damage calculations: >90%
- Overall project: >70%

### Test Utilities

#### DummyInput / DummyOutput
**Purpose**: Mock I/O for battle system testing
**Implementation**:
```rust
struct DummyOutput;
impl MessageOutput for DummyOutput {
    fn output(&mut self, _message: &str) {}
}

struct DummyInput {
    predefined_input: Vec<PlayerAction>,
    cursor: usize,
}
impl PlayerInput for DummyInput {
    fn get_player_action(&mut self, _: &mut dyn FnMut()) -> PlayerAction {
        // Return predetermined actions
    }
}
```

## Data Structures

### Bit Manipulation
**Approach**: Manual bit packing/unpacking using bitwise operations
**Rationale**:
- Original game uses exact bit layouts
- No bit-field support in stable Rust
- Explicit control over memory layout

**Techniques**:
```rust
// Packing
let byte = (weapon << 5) | (armor << 2) | shield;

// Unpacking
let weapon = (byte >> 5) & 0b111;
let armor = (byte >> 2) & 0b111;
let shield = byte & 0b11;
```

### String Handling
**Encoding**: UTF-8 for Japanese text
**Approach**:
- Character indexing via lookup tables
- Fixed 4-character name normalization
- Hiragana validation on input

## Architecture Patterns

### Trait-Based Abstraction

#### MessageOutput Trait
**Purpose**: Decouple output rendering from business logic
```rust
pub trait MessageOutput {
    fn output(&mut self, message: &str);
}
```

**Implementations**:
- `CliOutput`: Print to stdout
- `BufferOutput`: Collect in memory for testing

#### PlayerInput Trait
**Purpose**: Abstract user input for testability
```rust
pub trait PlayerInput {
    fn get_player_input(&mut self, max: usize) -> usize;
    fn get_player_action(&mut self, display_commands: &mut dyn FnMut()) -> PlayerAction;
}
```

**Implementations**:
- `CliInput`: Read from stdin
- `DummyInput`: Predetermined responses for tests

### Result-Based Error Handling
**Pattern**: Return `Result<T, String>` for fallible operations

**Rationale**:
- Explicit error propagation
- Error messages in Japanese for user-facing functions
- No unwrap() in production code paths

**Example**:
```rust
pub fn to_password_string(&self) -> Result<String, String> {
    let bitstring = self.build_password_bitstring()?;
    let reordered = reorder_password_bits(&bitstring)?;
    let kana_indices = apply_password_offsets(&reordered)?;
    indices_to_password_kana(&kana_indices)
}
```

## Memory Management

### Zero-Copy Optimizations
- Static string slices (`&'static str`) for master data
- Borrowed references in hot paths
- Small stack-allocated arrays (`[u8; 8]` for items)

### Allocation Strategy
- Player/Monster structs on stack (< 256 bytes)
- Battle messages accumulated in Vec, cleared between turns
- No heap allocation in damage calculation loops

## Distribution

### Installation Methods

#### Cargo
```bash
cargo install damdara
cargo add damdara  # As library dependency
```

#### Homebrew
```bash
brew tap webcyou-org/tap
brew install damdara
```

### Binary Distribution
- Compiled binaries for: macOS (ARM64/x86_64), Linux (x86_64), Windows (x86_64)
- Static linking for minimal dependencies
- Single executable (~2MB stripped)

## Documentation

### Inline Documentation
**Format**: Rustdoc comments (`///`)
**Generation**: `cargo doc --open`

### README
**Languages**: English (`README.md`), Japanese (`README_ja.md`)
**Sections**:
- Feature list with checkboxes
- Binary structure diagrams
- Usage examples with sample output

## Version Control

### Git Workflow
- **Main Branch**: Stable releases
- **Develop Branch**: Integration branch
- **Feature Branches**: Individual features
- **Release Process**: Develop → Release PR → Main

### Semantic Versioning
**Current**: v0.8.4
**Format**: MAJOR.MINOR.PATCH
- MAJOR: Breaking API changes
- MINOR: New features
- PATCH: Bug fixes

## Performance Considerations

### Optimization Level
**Release Profile**:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### Benchmarking
**Approach**: Manual timing in test suite
**Target Operations**:
- Password encoding: < 10ms
- Password decoding: < 10ms
- Battle turn: < 50ms
- 1000 damage calculations: < 1ms

## Platform Support

### Tier 1 (Tested)
- macOS 11+ (ARM64, x86_64)
- Linux (x86_64, glibc 2.31+)
- Windows 10+ (x86_64)

### Tier 2 (Expected to work)
- FreeBSD
- WSL2
- macOS 10.15 (Catalina)

### Not Supported
- WebAssembly (planned for future)
- 32-bit architectures
- Embedded platforms

## External Resources

### Documentation References
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Clap Documentation](https://docs.rs/clap/)

### Original Game Research
- [FC DQ1 Password Analysis (Qiita)](https://qiita.com/musemyuzu/items/eb08f7790df356434e0f)
- [Name Growth Rate Analysis](https://way78.com/dq1/fc/name.html)
- [DQ1 Master Data](https://gcgx.games/dq1/)

## Development Tools

### Recommended IDE Setup
- **VS Code** with rust-analyzer extension
- **IntelliJ IDEA** with Rust plugin
- **Vim/Neovim** with rust.vim + coc-rust-analyzer

### Debugging
- `RUST_BACKTRACE=1` for stack traces
- `println!` debugging (removed before commit)
- `dbg!` macro for temporary inspection

### Code Quality
- Pre-commit hooks: `cargo fmt --check`, `cargo clippy`
- CI/CD: GitHub Actions for automated testing
- Coverage: Manual review, no automated tool currently

## Future Technology Considerations

### Potential Additions
1. **Serde**: JSON serialization for save data export
2. **Criterion**: Proper benchmarking suite
3. **Proptest**: Property-based testing for password system
4. **WASM-bindgen**: Web browser support
5. **Bevy**: If graphical UI is added

### Architecture Evolution
- Plugin system for custom monsters/items
- Network multiplayer protocol
- Database backend for persistent worlds
