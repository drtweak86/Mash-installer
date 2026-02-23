# Developer Workflow

## Green Build Definition

A green build means all four checks pass:

```bash
cargo fmt --all -- --check       # formatting
cargo clippy --all-targets --all-features -- -D warnings   # lints
cargo test --all --all-features  # 114 tests
shellcheck install.sh            # shell script check
```

Run the full gate with one command:

```bash
cargo xtask release-check
```

## Daily Commands

```bash
# Format all code
cargo fmt --all

# Lint (warnings are errors)
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test --all --all-features

# Check doc links
cargo xtask check-docs
```

## Branch Strategy

- `main` — protected; requires CI pass; no direct push
- `work-shaft<N>` — feature branches, one per shaft (work unit)
- Merge via PR; squash or merge commit (no rebase squash of CI-verified commits)

## Forge Immutable Laws

All contributions must follow the 8 Laws:

| Law | Meaning |
|-----|---------|
| **ABB** | Always Be Backing up |
| **ABT** | Always Be Testing (green build required) |
| **ABD** | Always Be Documenting (maps + HISTORY every session) |
| **KCS** | Keep Commits Small |
| **KISS** | Keep It Simple Stupid |
| **F>F** | Function over Form |
| **SVR** | Semantic Versioning Rule (v-prefix tags, workspace-aligned) |
| **1.0** | v1.0.0 = public contract; backward compat is law |

## No Scope Creep

- No refactors outside the declared shaft objective
- No opportunistic cleanups during feature work
- No architectural changes without an explicit design phase

## Commit Message Style

```
type(scope): short description

Longer explanation if needed.
```

Common types: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `ci`

## PR Checklist

Before opening a PR:

1. `cargo xtask release-check` — all green
2. `git log --oneline` — commits are small and descriptive
3. `docs/forge-tavern/maps.md` — shaft phases checked off
4. PR description explains the "why" not just the "what"
