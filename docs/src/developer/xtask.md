# xtask Commands

All project tooling lives in the `xtask/` crate. Invoke with `cargo xtask <subcommand>`.

## Available Subcommands

| Subcommand       | Description                                               |
|------------------|-----------------------------------------------------------|
| `check-docs`     | Check for broken markdown links in `docs/`                |
| `bump`           | Bump version: `cargo xtask bump <patch\|minor\|major>`    |
| `release-check`  | Pre-release gate: fmt + clippy + tests + doc links        |
| `hygiene`        | Move scratch docs older than 7 days to `docs/legacy/`     |
| `branch-prune`   | Delete local branches older than 7 days                   |
| `test-infra`     | Run tests (maelstrom mode or fallback to cargo)           |
| `test-theme`     | Verify theme resource files and module structure           |

## check-docs

```bash
cargo xtask check-docs
```

Scans all `.md` files in `docs/` for links and verifies they resolve. Exits non-zero if
any broken links are found. Used as part of `release-check`.

## bump

```bash
cargo xtask bump patch     # 1.0.0 → 1.0.1
cargo xtask bump minor     # 1.0.0 → 1.1.0
cargo xtask bump major     # 1.0.0 → 2.0.0
```

Updates version in all workspace `Cargo.toml` files in sync. Prefer `cargo release` for
full release workflow (see [Release Process](./release.md)).

## release-check

```bash
cargo xtask release-check
```

Runs the complete pre-release gate:
1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all --all-features`
4. `cargo xtask check-docs`

Run before every PR and before tagging a release.

## hygiene

```bash
cargo xtask hygiene
```

Moves files in `docs/scratch/` older than 7 days to `docs/legacy/`. Run automatically by
cron at 03:00 daily via `~/.local/bin/mash-doc-hygiene`.

## branch-prune

```bash
cargo xtask branch-prune
```

Deletes local git branches that have no commits newer than 7 days and are fully merged.
Run automatically by cron at 02:00 on Sundays via `~/.local/bin/mash-branch-prune`.

## Cron Integration

| Binary                 | Schedule          | Command                   |
|------------------------|-------------------|---------------------------|
| `mash-doc-hygiene`     | Daily 03:00       | `cargo xtask hygiene`     |
| `mash-branch-prune`    | Sunday 02:00      | `cargo xtask branch-prune`|
