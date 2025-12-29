# CLAUDE.md - AI Assistant Guide for Rust Learning Repository

## Repository Overview

**Purpose**: This is a Rust learning repository for exploring and practicing Rust programming concepts.

**Current State**: Early-stage learning repository with minimal structure.

**Repository Name**: rust (Myrustlearning)

**Primary Language**: Rust

---

## Repository Structure

### Current Structure
```
/home/user/rust/
├── README.md           # Repository description
└── CLAUDE.md          # This file - AI assistant guide
```

### Expected Future Structure
As this repository grows, expect the following typical Rust project structure:

```
/home/user/rust/
├── Cargo.toml         # Package manifest and dependencies
├── Cargo.lock         # Dependency lock file (don't edit manually)
├── README.md          # Project documentation
├── CLAUDE.md          # AI assistant guide
├── src/               # Source code directory
│   ├── main.rs        # Binary entry point
│   ├── lib.rs         # Library root (if applicable)
│   └── modules/       # Additional modules
├── tests/             # Integration tests
├── benches/           # Benchmarks
├── examples/          # Example code
└── target/            # Build artifacts (ignored by git)
```

---

## Development Workflows

### Setting Up a New Rust Project

#### Binary Project (Executable)
```bash
cargo new project_name
cd project_name
```

#### Library Project
```bash
cargo new --lib project_name
cd project_name
```

### Common Cargo Commands

| Command | Description |
|---------|-------------|
| `cargo build` | Compile the project |
| `cargo build --release` | Compile with optimizations |
| `cargo run` | Build and run the binary |
| `cargo test` | Run all tests |
| `cargo check` | Check code without building |
| `cargo fmt` | Format code with rustfmt |
| `cargo clippy` | Run linter for best practices |
| `cargo doc --open` | Generate and open documentation |
| `cargo clean` | Remove build artifacts |
| `cargo update` | Update dependencies |

### Development Cycle

1. **Write Code**: Edit `.rs` files in `src/`
2. **Check Syntax**: `cargo check` (fast feedback)
3. **Format Code**: `cargo fmt` (before committing)
4. **Lint Code**: `cargo clippy` (catch common mistakes)
5. **Run Tests**: `cargo test`
6. **Build**: `cargo build` or `cargo run`
7. **Commit**: Use clear commit messages

---

## Rust Code Conventions

### File Organization

- **`src/main.rs`**: Entry point for binary applications
- **`src/lib.rs`**: Entry point for library crates
- **Module files**: Use `mod.rs` or file-based modules
- **Tests**: Can be inline (`#[cfg(test)]`) or in `tests/` directory

### Naming Conventions

- **Files/Modules**: `snake_case.rs`
- **Functions**: `snake_case`
- **Variables**: `snake_case`
- **Types/Structs/Enums**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Traits**: `PascalCase` (often adjectives like `Readable`)

### Code Style

- **Indentation**: 4 spaces (enforced by `rustfmt`)
- **Line Length**: 100 characters (configurable in `rustfmt.toml`)
- **Imports**: Group by `std`, external crates, local modules
- **Comments**: Use `//` for line comments, `///` for doc comments

Example:
```rust
/// Calculates the sum of two numbers.
///
/// # Arguments
///
/// * `a` - First number
/// * `b` - Second number
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Best Practices

1. **Ownership**: Understand borrowing, ownership, and lifetimes
2. **Error Handling**: Use `Result<T, E>` and `Option<T>` instead of panicking
3. **Immutability**: Prefer `let` over `let mut` when possible
4. **Type Inference**: Let the compiler infer types when obvious
5. **Pattern Matching**: Use `match` for exhaustive handling
6. **Iterators**: Prefer iterator methods over loops when appropriate
7. **Avoid `unwrap()`**: Use proper error handling in production code

---

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/integration_test.rs
use my_crate::add;

#[test]
fn integration_test_add() {
    assert_eq!(add(5, 5), 10);
}
```

### Running Tests

```bash
cargo test                    # Run all tests
cargo test test_name         # Run specific test
cargo test --lib             # Run library tests only
cargo test --test test_file  # Run specific integration test
cargo test -- --nocapture    # Show println! output
```

---

## Dependencies Management

### Adding Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

Or use cargo:
```bash
cargo add serde
cargo add tokio --features full
```

### Common Useful Crates

- **serde**: Serialization/deserialization
- **tokio**: Async runtime
- **clap**: CLI argument parsing
- **reqwest**: HTTP client
- **anyhow**: Error handling
- **thiserror**: Custom error types
- **log/env_logger**: Logging
- **chrono**: Date/time handling

---

## Build and Release

### Debug Build
```bash
cargo build  # Output in target/debug/
```

### Release Build
```bash
cargo build --release  # Output in target/release/ (optimized)
```

### Running Binaries
```bash
cargo run               # Debug mode
cargo run --release     # Release mode
cargo run -- arg1 arg2  # With arguments
```

---

## Git Workflow

### Branch Naming

- Feature branches: `claude/claude-md-mjqtr6ss0reo46um-iP85U` (current)
- Following the pattern: `claude/description-sessionid`

### Commit Guidelines

- Use clear, descriptive commit messages
- Start with a verb: "Add", "Fix", "Update", "Refactor"
- Keep commits focused on single changes

### Files to Ignore

Standard `.gitignore` for Rust:
```
/target/
**/*.rs.bk
Cargo.lock  # Ignore for libraries, keep for binaries
.DS_Store
```

---

## AI Assistant Guidelines

### When Adding New Features

1. **Read existing code first** using the Read tool
2. **Check for similar patterns** in the codebase
3. **Follow existing conventions** for naming and structure
4. **Add tests** for new functionality
5. **Run `cargo fmt` and `cargo clippy`** before committing
6. **Ensure code compiles** with `cargo check`

### When Fixing Bugs

1. **Reproduce the issue** if possible
2. **Write a failing test** that demonstrates the bug
3. **Fix the code** to make the test pass
4. **Verify** with `cargo test`
5. **Check for similar issues** elsewhere in the codebase

### When Refactoring

1. **Ensure all tests pass** before starting
2. **Make small, incremental changes**
3. **Run tests after each change**
4. **Preserve existing behavior** unless explicitly asked to change it
5. **Update documentation** if interfaces change

### Code Quality Checklist

Before committing, ensure:

- [ ] Code compiles: `cargo check`
- [ ] Tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Documentation is updated
- [ ] Commit message is clear

---

## Common Rust Patterns

### Error Handling

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

### Option Handling

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

// Usage
match divide(10, 2) {
    Some(result) => println!("Result: {}", result),
    None => println!("Cannot divide by zero"),
}
```

### Iterators

```rust
let numbers = vec![1, 2, 3, 4, 5];
let sum: i32 = numbers.iter().sum();
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

---

## Performance Considerations

### When to Optimize

1. **Profile first**: Use `cargo flamegraph` or similar tools
2. **Optimize hot paths**: Focus on code that runs frequently
3. **Benchmark**: Use `cargo bench` to measure improvements
4. **Don't prematurely optimize**: Write clear code first

### Common Optimizations

- Use `&str` instead of `String` when you don't need ownership
- Use `Vec::with_capacity()` when size is known
- Avoid cloning when borrowing is sufficient
- Use iterators instead of collecting intermediate vectors
- Consider using `Box`, `Rc`, or `Arc` for large data structures

---

## Learning Resources

### Official Documentation
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Standard Library: https://doc.rust-lang.org/std/

### Best Practices
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- Rust Design Patterns: https://rust-unofficial.github.io/patterns/

### Tools
- Rustfmt: Code formatter
- Clippy: Linter for catching common mistakes
- Rust Analyzer: IDE language server

---

## Current Repository Status

**Last Updated**: 2025-12-29

**Git Branch**: `claude/claude-md-mjqtr6ss0reo46um-iP85U`

**Project Stage**: Initialization - No Rust code yet

**Next Steps**:
1. Initialize a Cargo project with `cargo new` or `cargo init`
2. Set up basic project structure
3. Add first Rust programs/exercises
4. Configure `.gitignore` for Rust
5. Add dependencies as needed

---

## Notes for AI Assistants

- This is a **learning repository**, so prioritize clarity and educational value
- Include **explanatory comments** in code
- Suggest **idiomatic Rust** patterns
- Point out **common pitfalls** and how to avoid them
- Encourage **exploration** of different Rust features
- When suggesting code, explain **why** it works that way
- Consider adding **exercises** or **challenges** for practice

---

*This document will be updated as the repository evolves and new patterns emerge.*
