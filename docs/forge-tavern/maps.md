# ⚒️ The Miner's Active Maps: Current Shaft
> *"The blade is hardened. The crown is on the table."* — Bard 🍺

---

## ✅ SHAFT J: WALLPAPER DOWNLOADER RUST CONVERSION — COMPLETE

All previous shafts (A through J) are complete. See `maps-explored.md` for full history.

---

## ✅ SHAFT K: FORGE HARDENING — COMPLETE
> *Branch*: `work-shaftj-phase1`
> *PR*: [#35](https://github.com/drtweak86/Mash-installer/pull/35) — awaiting CI green + merge

### PHASE 1: BASELINE COMMIT ✅ COMPLETE — Checkpoint α
- [x] K1.1 Run build trinity: fmt clean | clippy clean | 107 tests passing
- [x] K1.2 Staged and committed 49 files (commit f89d203)
- [x] K1.3 PR #35 opened on `work-shaftj-phase1` → CI → merge

### PHASE 2: LEGACY ARTIFACT PURGE ✅ COMPLETE — Checkpoint β
- [x] K2.1 Deleted `resources/themes/retro-bbc/wallpaper_downloader_final.py`
- [x] K2.2 Deleted `docs/incoming-files/wallpaper_downloader_final.py`
- [x] K2.3 Deleted `docs/incoming-files/eza-aliases.sh` (staging duplicate)
- [x] K2.4 `resources/shell/eza_aliases.sh` confirmed kept as resource data file

### PHASE 3: THIN SHIM ELIMINATION ✅ COMPLETE — Checkpoint γ
- [x] K3.1 Deleted `installer-core/src/registry.rs` (was 1-line re-export)
- [x] K3.2 Updated `lib.rs`: `pub use phase_registry::PhaseRegistry`
- [x] K3.3 Deleted `installer-core/src/runner.rs` (was 4-line re-export)
- [x] K3.4 Updated `lib.rs`: `pub use phase_runner::{Phase, PhaseRunner, ...}`
- [x] K3.5 Build check: green

### PHASE 4: CRATE CONSOLIDATION ✅ COMPLETE — Checkpoint δ
- [x] K4.1.d Bumped wallpaper-downloader version to 1.0.0 (workspace aligned)
  - NOTE: Full thin-CLI fold deferred post-v1.0.0 (PhaseContext coupling architectural decision)
- [x] K4.2 Deleted `.github/workflows/rust.yml` (subsumed by ci.yml)

### PHASE 5: DEPENDENCY HYGIENE ✅ COMPLETE — Checkpoint ε
- [x] K5.1 Aligned `indicatif`: core 0.17 → 0.18 (matches installer-cli)
- [x] K5.2 Removed dead `which = "4"` from installer-cli
- [x] K5.3 `once_cell` → `std::sync::OnceLock` — COMPLETE (sudo_password.rs; dep removed)
- [x] K5.4 Toolchain — CONFIRMED current (1.93.1 IS stable tip)
- [x] K5.5 Build: fmt clean | clippy clean | build green

### PHASE 6: SHAFT K COMMIT ✅ COMPLETE — Checkpoint ζ
- [x] K6.1 Commits: f89d203 | 3a7b7e0 | e4430b2 | dfbfe16 | d73bec3
- [x] K6.2 PR #35 opened `work-shaftj-phase1` → main

**Risk**: LOW | **Reward**: HIGH

---

## ✅ SHAFT L: FINAL RELEASE CLEARANCE — COMPLETE (pending PR merge + tag)
> *PR*: #35 (includes all Shaft L commits)
> *Next action*: CI green → merge → `git tag v1.0.0 && git push --tags`

**Objective**: Code quality pass + UX improvements + docs + release gate → v1.0.0 tag.

### PHASE 1: CODE QUALITY ✅ COMPLETE
- [x] L1.1 verify.rs: changed `#[allow(dead_code)] mod` → `pub mod` (exposed as public API)
- [x] L1.2 ai_agents.rs: confirmed wired in phase_registry.rs:129; added module doc comment
- [x] L1.3 once_cell → OnceLock — DONE in K5.3
- [x] L1.4 software_tiers boundary: doc comments added to both core (data+install) and cli (UI)

### PHASE 2: UX IMPROVEMENTS ✅ COMPLETE
- [x] L2.1 WallpaperConfig::with_env_keys() — reads MASH_WALLHAVEN_KEY, MASH_PEXELS_KEY, MASH_PIXABAY_KEY
- [x] L2.2 Doctor mode: "Wallpaper API keys" section — PASS/WARN per key + setup URLs
- [x] L2.3 include_str!() eza_aliases.sh — CONFIRMED already done in zsh.rs:23

### PHASE 3: DOCUMENTATION ✅ COMPLETE
- [x] L3.1 HISTORY.md: bardic entry for Shaft J (wallpaper Rust conversion)
- [x] L3.2 HISTORY.md: bardic entry for Shaft K (forge hardening, cron, laws)
- [x] L3.3 MANUAL.md: full refresh (TUI, wallpapers, Pi tuning, doctor, API keys, AI Spirits)
- [x] L3.4 check_docs.rs: 4 broken links fixed in QA_SUMMARY.md — passes clean

### PHASE 4: FINAL RELEASE GATE ✅ COMPLETE (local)
- [x] L4.1 release_checklist.rs: fmt clean | clippy clean | 110 tests passing | docs clean
- [x] L4.2 cargo audit: via CI (not installed locally on Pi)
- [x] L4.3 shellcheck install.sh: clean (zero warnings)
- [x] L4.4 Version consistency: all 6 crates bumped to 1.0.0
- [x] L4.5 Merge to main — PR #35 merged 2026-02-23T01:29:16Z
- [x] L4.7 `git tag v1.0.0 && git push --tags` — DONE 2026-02-23
- [x] L4.8 GitHub Actions release pipeline fires → .deb, .rpm published ✅ COMPLETE (2026-02-23T02:03:04Z)

**Risk**: LOW | **Reward**: MAXIMUM (v1.0.0 shipped)

---

## ✅ SESSION 2026-02-23 — THRESHOLD CROSSING — COMPLETE

**Commits this session** (branch `work-shaftj-phase1`, merged to main):
- `3abf4bd` fix: aarch64 cross-compile — switch to rustls-tls, harden doc CI
- `11c875f` feat: CLI — wire doctor/config subcommands + bard easter egg
- `4c66539` docs: hygiene — move release scratch docs; add mdBook sources
- `ff09418` chore: remove orphaned root src/main.rs
- `4fadf1f` merge: sync origin/main → resolve MANUAL.md conflict
- `549a494` fix: docs build — remove [output.linkcheck] from book.toml

**CI**: ALL 10 checks GREEN
**PR #35**: MERGED → main at `d9ded60` on 2026-02-23T01:29:16Z
**Tag**: `v1.0.0` at `98737cb` — release FULLY PUBLISHED 2026-02-23T02:03:04Z

---

## ✅ THE THRESHOLD IS CROSSED — v1.0.0 SHIPPED

```
PR #35 CI ──► MERGE ──► git tag v1.0.0 ──► git push --tags ──► RELEASE PIPELINE ✅
```

---

---

## ✅ SHAFT M: TOOLING MODERNIZATION — COMPLETE
> *Risk*: LOW | *Reward*: HIGH (developer QoL)
> *Branch*: `work-shaftm`

**Objective**: Consolidate scripts/*.rs into a proper `xtask` crate, replace auto_bump.rs with cargo-release, configure Dependabot.

### PHASE 1: xtask Scaffold ✅ COMPLETE
- [x] M1.1 Create `xtask/` crate in workspace root
- [x] M1.2 Add `xtask` to `[workspace].members` in root Cargo.toml
- [x] M1.3 Port each script as a subcommand: `cargo xtask check-docs | bump | hygiene | release-check | test-infra | test-theme | branch-prune`
  - `release-check` calls `check_docs::check()` directly — no code duplication
- [x] M1.4 Update cron binaries (`~/.local/bin/`) to shell scripts calling `cargo xtask`
- [x] M1.5 Delete `scripts/` directory (7 .rs files consolidated; git tracked as renames)

### PHASE 2: cargo-release Integration ✅ COMPLETE
- [x] M2.1 Add `release.toml` config at workspace root
- [x] M2.2 Configure: `pre-release-hook = ["cargo","xtask","release-check"]`, tag-name `v{{version}}`, shared-version workspace mode, MANUAL.md replacements
- [x] M2.3 `auto_bump.rs` → `cargo xtask bump` (ported to xtask in M1, complementary to cargo-release)
- [x] M2.4 Developer Workflow section added to MANUAL.md (xtask table + cargo release workflow)

### PHASE 3: Dependabot ✅ COMPLETE
- [x] M3.1 Created `.github/dependabot.yml`
- [x] M3.2 Cargo ecosystem: weekly Monday, `deps` label, patch updates grouped
- [x] M3.3 GitHub Actions ecosystem: monthly

**Risk**: LOW | **Reward**: HIGH

---

## ⏳ SHAFT N: REQWEST 0.12 UPGRADE — PLANNED
> *Risk*: HIGH | *Reward*: MEDIUM (keeps HTTP client current)
> *Branch*: `work-shaftn` (to be created)
> *Note*: Run in isolation — no other changes in this shaft.

**Objective**: Upgrade reqwest 0.11.27 → 0.12.x in installer-core and wallpaper-downloader.

**Scope** (4 files with reqwest usage):
- `installer-core/src/wallpaper/api/wallhaven.rs`
- `installer-core/src/wallpaper/api/pexels.rs`
- `installer-core/src/wallpaper/api/pixabay.rs`
- `wallpaper-downloader/src/api.rs`

**Known API surface** (all must be verified against 0.12 migration guide):
- `reqwest::Client`, `Client::builder()`, `.timeout()`, `.get()`, `.query()`, `.header()`, `.send().await`, `.json().await`, `.status()`, `.is_success()`

### PHASE 1: Audit & Plan
- [ ] N1.1 Read reqwest 0.12 migration guide — document all breaking changes
- [ ] N1.2 Map each 0.11 API call in all 4 files to its 0.12 equivalent
- [ ] N1.3 Identify any test mocking changes needed (mockito, etc.)

### PHASE 2: installer-core Migration
- [ ] N2.1 Update `installer-core/Cargo.toml`: reqwest `"0.11"` → `"0.12"` (keep `rustls-tls`)
- [ ] N2.2 Fix `wallhaven.rs` API calls
- [ ] N2.3 Fix `pexels.rs` API calls
- [ ] N2.4 Fix `pixabay.rs` API calls
- [ ] N2.5 Build + test: `cargo test -p installer-core`

### PHASE 3: wallpaper-downloader Migration
- [ ] N3.1 Update `wallpaper-downloader/Cargo.toml`: same version bump
- [ ] N3.2 Fix `api.rs` API calls
- [ ] N3.3 Build + test: `cargo test -p wallpaper-downloader`

### PHASE 4: Verification
- [ ] N4.1 Full workspace build: `cargo build --workspace`
- [ ] N4.2 Full test suite: `cargo test --workspace`
- [ ] N4.3 Clippy clean
- [ ] N4.4 aarch64 cross-compile check

**Risk**: HIGH | **Reward**: MEDIUM

---

## ⏳ SHAFT O: UX & FEATURE EXPANSION — PLANNED
> *Risk*: LOW–MEDIUM | *Reward*: HIGH (user-visible)
> *Branch*: `work-shafto` (to be created)

**Objective**: BBS message bank expansion + mash-setup status subcommand + multi-distro CI matrix.

### PHASE 1: BBS Message Bank Expansion (45 → 65+)
- [ ] O1.1 Review existing 45 messages in `installer-cli/src/tui/bbs.rs` for tone/themes
- [ ] O1.2 Draft 20+ new messages (categories: Forge Lore, Rust idioms, dwarven wisdom, network sorcery, package alchemy)
- [ ] O1.3 Add messages — ensure no duplicates, even distribution across themes
- [ ] O1.4 Update count in MEMORY.md

### PHASE 2: mash-setup status Subcommand
- [ ] O2.1 Design `StatusReport` data model in installer-core
  - Installation state (not started / in progress / complete)
  - Detected distro + architecture
  - Config validity (loaded / missing / invalid)
  - Wallpaper API key presence (per-key PASS/WARN)
  - Doctor summary (pass/warn/fail counts)
- [ ] O2.2 Implement `installer_core::status()` function
- [ ] O2.3 Wire `mash-setup status [--format pretty|json]` in installer-cli
- [ ] O2.4 Write tests for status output (stdout capture pattern from doctor)
- [ ] O2.5 Update MANUAL.md with status subcommand docs

### PHASE 3: Multi-Distro Parallel CI Matrix
- [ ] O3.1 Add `distro-test` job to `ci.yml` using container matrix (ubuntu, fedora, archlinux images)
- [ ] O3.2 Test: binary runs `mash-setup --version` in each container
- [ ] O3.3 Test: `mash-setup doctor` exits 0 in dry-run mode in each container
- [ ] O3.4 Verify matrix doesn't add excessive CI time (parallelize)

**Risk**: LOW–MEDIUM | **Reward**: HIGH

---

## ⏳ SHAFT P: DOCUMENTATION — PLANNED
> *Risk*: LOW | *Reward*: MEDIUM (improves discoverability)
> *Branch*: `work-shaftp` (to be created)

**Objective**: Expand mdBook from a 20-line stub to a complete user manual; deploy to GitHub Pages.

**Current state**: `docs/src/` has `SUMMARY.md` (1 entry) + `manual.md` (20 lines). Book builds but is nearly empty.

### PHASE 1: Structure & Scaffold
- [ ] P1.1 Design chapter tree: Installation → Configuration → Features → Troubleshooting → Reference
- [ ] P1.2 Write full `SUMMARY.md` with nested chapter structure
- [ ] P1.3 Create stub `.md` files for each chapter

### PHASE 2: Content — Installation & Configuration
- [ ] P2.1 Installation chapter: prerequisites, one-liner, profile selection, first run
- [ ] P2.2 Configuration chapter: `mash-config`, profiles (minimal/desktop/full), env vars, API keys

### PHASE 3: Content — Features & Troubleshooting
- [ ] P3.1 Features chapter: TUI navigation, doctor mode, wallpapers, AI Spirits, eza aliases
- [ ] P3.2 Troubleshooting chapter: common errors, dry-run mode, log files, Pi SD card sync
- [ ] P3.3 Reference chapter: all CLI flags, subcommands (catalog, doctor, config, status)

### PHASE 4: CI & Deployment
- [ ] P4.1 Pin compatible mdBook + mdbook-linkcheck versions (resolve 0.7.7 API mismatch)
- [ ] P4.2 Add GitHub Pages deploy workflow (`gh-pages` branch)
- [ ] P4.3 Update `docs/book.toml` with correct [output.linkcheck] once versions aligned
- [ ] P4.4 Add badge to README

**Risk**: LOW | **Reward**: MEDIUM

---

## ⏳ SHAFT Q: WALLPAPER CONSOLIDATION — DESIGN PHASE FIRST
> *Risk*: MEDIUM | *Reward*: MEDIUM (architecture cleanliness)
> *Note*: Phase 1 is design-only. No code until design is approved.

**Objective**: Fold `wallpaper-downloader/` into a thin CLI that delegates to `installer-core/src/wallpaper/`. Currently two independent implementations.

**Current separation**:
- `wallpaper-downloader/` — standalone async binary (retro BBC categories: ~6000 images)
- `installer-core/src/wallpaper/` — phase-integrated API abstraction (Wallhaven/Pexels/Pixabay)
- They do NOT share code; both are kept in the workspace

**The question**: should they share code, or stay separate with a common CLI entry point?

### PHASE 1: Architecture Design (NO CODE)
- [ ] Q1.1 Audit PhaseContext coupling in installer-core/wallpaper — what prevents reuse?
- [ ] Q1.2 Define target architecture: shared `wallpaper-core` sub-crate, or adapter pattern?
- [ ] Q1.3 Determine if wallpaper-downloader's retro categories belong in installer-core or stay separate
- [ ] Q1.4 Write design decision to `docs/scratch/shaft-q-design.md`
- [ ] Q1.5 Get design approved before any implementation

### PHASE 2: Implementation (post-design approval)
- [ ] Q2.1 Extract shared types/traits to agreed boundary
- [ ] Q2.2 Refactor wallpaper-downloader to thin CLI
- [ ] Q2.3 Update tests
- [ ] Q2.4 Verify both entry points still work

**Risk**: MEDIUM | **Reward**: MEDIUM

---

## RECOMMENDED ORDER

```
M (Tooling) → O (UX) → P (Docs) → Q (Design) → N (reqwest 0.12)
```

Rationale:
- M first — better tools make all subsequent work easier
- O next — user-visible wins + low risk
- P after O — docs reflect the features from O
- Q design — architecture decision with no blast radius
- N last — HIGH RISK isolated upgrade, do it when forge is calm

---

**Last Updated**: 2026-02-23
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
