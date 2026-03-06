# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Flow is a CLI tool for developer context preservation. It captures what you're working on before interruptions and enables instant resumption. It's git-aware, fast (< 50ms), offline-first, and uses simple JSON storage.

## Development Commands

### Build and Install
```bash
cargo build              # Debug build
cargo install --path .   # Install locally
cargo release            # Release build (if using cargo-release)
```

### Testing
```bash
cargo test               # Run all tests
cargo test -- <test_name>  # Run specific test
cargo test -- --nocapture  # Show print output in tests
```

### Linting and Formatting
```bash
cargo fmt                # Format code
cargo fmt --check        # Check formatting without modifying
cargo clippy             # Run linter
cargo clippy --all-targets --all-features -- -D warnings  # Strict mode (used in pre-commit)
```

### Pre-commit Hooks
This project uses prek for fast, built-in Rust hooks:
- Conventional commits validation (title ≤72 chars)
- `cargo fmt --check` and `cargo clippy` on pre-commit
- `cargo test` runs before commit
- Custom commit message check via `scripts/check-commit-msg.sh`

Install hooks: `prek install` (if prek is available)

## Architecture

### Module Structure
```
src/
├── main.rs          # Entry point, parses CLI and dispatches to commands
├── lib.rs           # Library exports
├── cli.rs           # Clap-derived CLI structure (Commands enum)
├── context.rs       # Context struct with note, timestamp, git info
├── storage.rs       # Persistence layer (JSON files in ~/.flow)
├── git.rs           # Git detection using git2 library
└── commands/
    ├── mod.rs       # Re-exports command runners
    ├── note.rs      # Save current context
    ├── status.rs    # Display current context
    ├── resume.rs    # Show resume guidance
    ├── history.rs   # View past contexts (v0.2.0)
    └── done.rs      # Archive to history, clear current
```

### Key Data Flow
1. `main.rs` parses CLI via Clap, matches Commands enum
2. Each command in `commands/` uses `Storage::default_location()` to get ~/.flow
3. `context::Context` holds note + optional git repo/branch
4. `git::detect_git_info()` uses git2 to find repo and branch
5. `storage::Storage` reads/writes `context.json` and `history.json`

### Storage Location
- Base directory: `~/.flow/`
- `context.json` - Current active context
- `history.json` - Array of HistoryEntry (completed contexts with duration)

### Adding New Commands
1. Add variant to `Commands` enum in `src/cli.rs`
2. Create file in `src/commands/` with `pub fn run(...) -> Result<()>`
3. Export in `src/commands/mod.rs`
4. Add match arm in `src/main.rs`

## Conventional Commits

Commit messages must follow: `type(scope): description`
- Types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert
- Title max 72 characters
- Validated by `scripts/check-commit-msg.sh` and commitizen hook
