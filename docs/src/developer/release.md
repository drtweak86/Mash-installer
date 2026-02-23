# Release Process

MASH uses [`cargo-release`](https://github.com/crate-ci/cargo-release) for version bumping,
tagging, and triggering the release pipeline.

## Prerequisites

```bash
cargo install cargo-release
```

## Workflow

### 1. Verify the gate is green

```bash
cargo xtask release-check
```

All four checks must pass: fmt, clippy, tests, doc links.

### 2. Dry-run (no commits or tags)

```bash
cargo release patch        # for a patch bump (1.0.0 → 1.0.1)
cargo release minor        # for a minor bump (1.0.0 → 1.1.0)
cargo release major        # for a major bump (1.0.0 → 2.0.0)
```

Review what will happen — version bump in all Cargo.toml files, MANUAL.md update, commit
message preview.

### 3. Execute the release

```bash
cargo release patch --execute
```

`cargo release` will:
1. Run `cargo xtask release-check` (pre-release hook)
2. Bump all workspace crate versions in sync
3. Update version strings in `docs/MANUAL.md`
4. Commit: `chore: bump version to X.Y.Z`
5. Create git tag: `vX.Y.Z`
6. Push commit + tag to `origin/main`

### 4. CI pipeline fires automatically

Pushing a `v*` tag triggers `.github/workflows/release.yml`:
- Builds `mash-setup` for `x86_64` and `aarch64`
- Packages `.deb` and `.rpm`
- Creates a GitHub Release with all artifacts + SHA256 checksums

## Semantic Versioning

MASH follows strict [semver](https://semver.org/):

| Change type | Version bump |
|-------------|-------------|
| Bug fix, dependency update | `patch` |
| New feature, new subcommand | `minor` |
| Breaking change to CLI or config | `major` |

Post-v1.0.0, backward compatibility is law (Forge Law #8: 1.0 Threshold).

## Manual Tag (emergency)

If `cargo release` is unavailable:

```bash
# Bump versions manually in all Cargo.toml files
cargo xtask bump patch

# Commit and tag
git add -p
git commit -m "chore: bump version to X.Y.Z"
git tag vX.Y.Z
git push origin main --tags
```

## Checking the Published Release

```bash
gh release view --repo drtweak86/Mash-installer
```
