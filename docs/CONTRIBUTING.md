# Contributing to Flow

First off, thank you for considering contributing to Flow! It's people like you that make Flow such a great tool.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pre-commit Setup](#pre-commit-setup)
- [Code Style](#code-style)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Project Structure](#project-structure)

## Getting Started

### Prerequisites

You'll need the following installed:

- **Rust** (latest stable): Install via [rustup](https://rustup.rs/)
- **prek** (fast pre-commit hooks for Rust):
  ```bash
  cargo install prek
  ```
- **Git**: For version control

### Clone and Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/onoht/flow.git
   cd flow
   ```

2. Install development dependencies:
   ```bash
   cargo install prek
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Install the CLI locally (optional):
   ```bash
   cargo install --path .
   ```

5. Set up pre-commit hooks (recommended):
   ```bash
   prek install
   ```

That's it! You're ready to start contributing.

## Development Workflow

### Branch Naming

Use descriptive branch names following this pattern:
- `feat/feature-name` - New features
- `fix/bug-description` - Bug fixes
- `docs/what-changed` - Documentation changes
- `refactor/what-refactored` - Code refactoring
- `test/what-tested` - Test improvements

Examples:
- `feat/project-specific-contexts`
- `fix/git-branch-detection`
- `docs/contributing-guide`

### Making Changes

1. Create a new branch from `master`:
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. Make your changes following the [code style](#code-style) guidelines

3. Test your changes:
   ```bash
   cargo test
   ```

4. Commit your changes following [commit guidelines](#commit-guidelines):
   ```bash
   git add .
   git commit -m "feat(scope): description"
   ```

5. Push to your fork:
   ```bash
   git push origin feat/your-feature-name
   ```

### Testing

Before submitting, ensure all tests pass:

```bash
# Run all tests
cargo test

# Run tests with output (useful for debugging)
cargo test -- --nocapture

# Run specific test
cargo test -- test_name
```

## Pre-commit Setup

Flow uses **prek** for fast, zero-setup Rust hooks. The hooks run automatically when you commit.

### Installing Hooks

If you haven't already:
```bash
prek install
```

### What the Hooks Check

1. **Code Quality**:
   - `cargo fmt --check` - Ensures consistent formatting
   - `cargo clippy` - Catches common Rust mistakes
   - `cargo test` - All tests must pass

2. **Commit Message**:
   - Follows [conventional commits](#commit-guidelines) format
   - Title ≤ 72 characters

3. **File Checks**:
   - No trailing whitespace
   - Files end with newline
   - No large files (> 500KB)
   - No private keys
   - No merge conflict markers

### Running Hooks Manually

If you want to run checks without committing:
```bash
# Run all pre-commit hooks
prek run --all

# Run specific hook
prek run fmt
prek run clippy
prek run test
```

### Skipping Hooks (Not Recommended)

If you absolutely need to skip hooks:
```bash
git commit --no-verify -m "message"
```

## Code Style

### Rust Formatting

We use `rustfmt` for consistent code style:

```bash
# Format code
cargo fmt

# Check formatting without modifying
cargo fmt --check
```

**Never commit with formatting issues.** The pre-commit hooks will catch this.

### Clippy Lints

We use Clippy with strict warnings:

```bash
# Run linter
cargo clippy

# Strict mode (used in pre-commit)
cargo clippy --all-targets --all-features -- -D warnings
```

All warnings must be addressed before committing.

### Conventions

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable and function names
- Keep functions focused and concise
- Add doc comments for public APIs
- Prefer idiomatic Rust patterns over writing "Rust like $LANGUAGE"

## Commit Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Format

```
type(scope): description

[optional body]

[optional footer]
```

### Types

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style changes (formatting, etc.)
- `refactor` - Code refactoring
- `perf` - Performance improvements
- `test` - Adding or updating tests
- `build` - Build system changes
- `ci` - CI/CD changes
- `chore` - Other changes (dependencies, etc.)
- `revert` - Revert a previous commit

### Rules

- **Title max 72 characters** (enforced by hook)
- Use lowercase for type and scope
- Use imperative mood ("add" not "added" or "adds")
- Don't end with a period
- Skip scope if not needed

### Examples

```
feat: add context history command

fix: correct git branch detection in subdirectories

docs: update contributing guide

test: add tests for storage layer

refactor(cli): simplify command parsing

perf(context): optimize JSON serialization
```

### What Happens If You Don't Follow This

The pre-commit hook will reject your commit:
```
❌ Commit title must follow conventional commits format:
   type(scope): description
   Types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert
```

## Pull Request Process

### Creating a PR

1. Ensure your branch is up to date:
   ```bash
   git checkout master
   git pull upstream master
   git checkout feat/your-feature
   git rebase master
   ```

2. Push your changes and create a PR on GitHub

3. Fill out the PR template with:
   - Description of changes
   - Related issues
   - Testing performed
   - Screenshots (if applicable)

### What to Expect

- All checks must pass (CI, pre-commit hooks)
- Code review from maintainers
- Feedback may be requested - respond promptly
- Keep the PR focused on a single change

### After Merge

- Update your local master
- Delete your feature branch
- Celebrate! 🎉

## Project Structure

```
flow/
├── src/
│   ├── main.rs          # Entry point, CLI parsing
│   ├── lib.rs           # Library exports
│   ├── cli.rs           # Command-line interface (Clap)
│   ├── context.rs       # Context struct and logic
│   ├── storage.rs       # JSON persistence layer
│   ├── git.rs           # Git detection (git2)
│   └── commands/
│       ├── mod.rs       # Command exports
│       ├── note.rs      # Save context
│       ├── status.rs    # Display context
│       ├── resume.rs    # Show resume guidance
│       ├── history.rs   # View past contexts
│       └── done.rs      # Archive and clear
├── scripts/
│   └── check-commit-msg.sh  # Commit validation
├── Cargo.toml           # Dependencies and metadata
├── .pre-commit-config.yaml  # Hook configuration
└── CLAUDE.md            # Contributor guidance (for AI)

Storage:
└── ~/.flow/
    ├── context.json     # Current active context
    └── history.json     # Completed contexts
```

### Key Concepts

- **Commands**: Each CLI command lives in `src/commands/*.rs`
- **Context**: Core data structure (note + optional git info)
- **Storage**: Simple JSON files in `~/.flow`
- **Git**: Optional enhancement (repo/branch detection)

## Questions?

Feel free to:
- Open an issue for bugs or feature requests
- Start a discussion for questions
- Check existing issues for similar topics

Happy contributing! 🚀
