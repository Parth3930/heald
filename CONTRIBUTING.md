# Contributing to Heald

Thank you for your interest in contributing! This document explains the process.

## Before You Start

- **Bug fixes and docs:** No issue needed, just open a PR.
- **New features or significant changes:** Open an issue first so we can discuss scope and design before you write code.

## Development Setup

**Requirements:** Rust stable (1.70+)

```bash
git clone https://github.com/Parth3930/heald
cd heald
cargo build
cargo test
```

## Making Changes

1. Fork the repo and create a branch from `main`:
   ```bash
   git checkout -b feat/your-feature
   # or
   git checkout -b fix/your-bugfix
   ```

2. Make your changes. Keep commits focused — one logical change per commit.

3. Run tests:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

4. Open a pull request against `main` with:
   - A clear title (e.g. `feat: add heald forget command`)
   - A description of *what* changed and *why*
   - Any relevant issue numbers (`Fixes #42`)

## Commit Style

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add budget pruning to heald context
fix: prevent panic on missing frontmatter type
docs: update per-agent integration guide
chore: bump serde to 1.0.200
```

## Code Style

- `cargo fmt` before every commit
- `cargo clippy` must pass with no warnings
- No `unwrap()` in production paths — use `?` or explicit error handling
- No silent fallbacks that hide bugs — fail loudly with a clear message

## What Belongs in Heald

Heald is deliberately scoped to: **rules/skills sync** and **budget-aware memory**. It should stay a single binary with no runtime dependencies.

Proposals that fall outside this scope (dashboards, ML ranking, cloud sync, web UI) will be declined — not because they're bad ideas, but because staying small is a feature.

## Areas That Need Help

- Codex native format compiler (`src/cmd/sync.rs`)
- File locking for concurrent safety (`fs2` crate)
- `heald forget --title "..."` command
- Shell completions (bash, zsh, fish, PowerShell)
- Windows path normalization edge cases

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).

