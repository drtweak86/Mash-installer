# ⚒️ The Miner's Active Maps: Current Shaft
> *"The blade is hardened. The crown is on the table."* — Bard 🍺

---

## ✅ SHAFT J: The Overlord Protocols — ACTIVE
> *Branch*: `work-shaftj-overlord`
> *Risk*: MEDIUM (mitigated with phased approach)
> *Reward*: HIGH (long-term maintainability, performance, alignment)
> *Status*: ✅ PLANNING COMPLETE | 🔨 EXECUTION PENDING

### PHASE 1: OVERLORD CONFIG PROMOTION ✅ PLANNED
**Objective**: Promote BBC Acorn terminal configs from staging to production resources.

#### 1.1 — Update `resources/shell/kitty.conf`
- **File**: `resources/shell/kitty.conf`
- **Change**: Replace with `docs/incoming-files/kitty.txt`
- **Key**: `font_family JetBrainsMono Nerd Font`
- **Verification**: `grep "font_family" resources/shell/kitty.conf` → `JetBrainsMono Nerd Font`
- **Status**: ✅ PLANNED

#### 1.2 — Update `resources/shell/starship.toml`
- **File**: `resources/shell/starship.toml`
- **Change**: Replace with `docs/incoming-files/starship.toml.txt`
- **Key**: `[memory_usage]` module with 75% threshold
- **Verification**: `grep "memory_usage" resources/shell/starship.toml` → present
- **Status**: ✅ PLANNED

#### 1.3 — Update `resources/shell/eza_aliases.sh`
- **File**: `resources/shell/eza_aliases.sh`
- **Change**: Replace with `docs/incoming-files/eza-aliases.sh`
- **Key**: `alias goblin='cmatrix -a'` easter egg
- **Verification**: `grep "goblin" resources/shell/eza_aliases.sh` → present
- **Status**: ✅ PLANNED

### PHASE 2: ARCH DETECTION OPTIMIZATION ✅ PLANNED
**Objective**: Skip 15-second ArchDetected banner when exactly one driver matches.

#### 2.1 — Modify `run()` to Detect Single-Driver Match
- **File**: `installer-cli/src/tui/app.rs` (lines 938–940)
- **Change**: Replace 3-line block with 20-line single-match logic
- **Requires**: `installer_core::detect_platform()` re-exported in lib.rs
- **Verification**: Single-driver binary → no ArchDetected screen
- **Status**: ✅ PLANNED

#### 2.2 — Keep `handle_auto_arch()` Unchanged
- **File**: `installer-cli/src/tui/app.rs` (lines 274–278)
- **Change**: None (used in fallback case)
- **Status**: ✅ PLANNED

#### 2.3 — Keep `tick()` Unchanged
- **File**: `installer-cli/src/tui/app.rs`
- **Change**: None (timer only fires when `arch_timer` is `Some`)
- **Status**: ✅ PLANNED

### PHASE 3: NERD FONT UPGRADE ✅ PLANNED
**Objective**: Switch from Terminus to JetBrainsMono Nerd Font.

#### 3.1 — Change the Font Name Constant and Target File
- **File**: `installer-core/src/fonts.rs`
- **Changes**:
  - Rename: `install_terminess_nerd_font` → `install_jetbrains_nerd_font`
  - Add: `const NERD_FONT_VERSION: &str = "v3.3.0";`
  - Update: `target_font` → `JetBrainsMonoNerdFont-Regular.ttf`
  - Update: `font_name` → `JetBrainsMono.zip`
  - Update: URL format string to include version
- **Verification**: `ls ~/.local/share/fonts/ | grep JetBrains` → `.ttf` files present
- **Status**: ✅ PLANNED

#### 3.2 — Keep Terminus Base Packages
- **File**: `installer-core/src/fonts.rs`
- **Change**: None (system packages remain)
- **Status**: ✅ PLANNED

#### 3.3 — Keep File Filter for Zip Extraction
- **File**: `installer-core/src/fonts.rs`
- **Change**: None (`.ttf` filter is correct)
- **Status**: ✅ PLANNED

### PHASE 4: VERIFICATION & TESTING ⏳ PENDING
**Objective**: Ensure all changes work correctly.

#### 4.1 — Build Verification
- **Command**: `cargo build --workspace`
- **Status**: ⏳ PENDING

#### 4.2 — Test Verification
- **Command**: `cargo test --workspace`
- **Status**: ⏳ PENDING

#### 4.3 — Clippy Verification
- **Command**: `cargo clippy --all-targets -- -D warnings`
- **Status**: ⏳ PENDING

#### 4.4 — TUI Verification
- **Test**: Single-driver binary → no ArchDetected screen
- **Status**: ⏳ PENDING

#### 4.5 — Font Verification
- **Test**: `kitty +list-fonts | grep JetBrains` → correct font
- **Status**: ⏳ PENDING

### PHASE 5: FINAL COMMIT & PR ⏳ PENDING
**Objective**: Commit and merge the changes.

#### 5.1 — Commit Changes
- **Message**: `feat: overlord protocols — kitty/starship/eza configs + arch skip + jetbrains font`
- **Status**: ⏳ PENDING

#### 5.2 — Open PR
- **Branch**: `work-shaftj-overlord` → `main`
- **Status**: ⏳ PENDING

#### 5.3 — Wait for CI Green
- **Checks**: fmt, clippy, test, audit, build
- **Status**: ⏳ PENDING

#### 5.4 — Merge
- **Action**: Merge PR to main
- **Status**: ⏳ PENDING

---

## 🏗️ FILE TOUCH SUMMARY

| File | Section | Nature of Change |
|---|---|---|
| `resources/shell/kitty.conf` | §1.1 | Full replacement with BBC Acorn config |
| `resources/shell/starship.toml` | §1.2 | Full replacement with Goblin Starship config |
| `resources/shell/eza_aliases.sh` | §1.3 | Full replacement with Goblin mega aliases |
| `installer-cli/src/tui/app.rs` | §2.1 | Replace 3-line arch detection block with 20-line single-match logic |
| `installer-core/src/fonts.rs` | §3.1 | Add `NERD_FONT_VERSION` const; rename fn; change font name and target file |
| `installer-core/src/lib.rs` | §2.1 | Verify `detect_platform` is re-exported; add if missing |

---

## ⚠️ RISKS & MITIGATIONS

| Risk | Mitigation |
|---|---|
| `detect_platform()` fails early in `run()` before terminal is ready | Wrap in `.ok().and_then(...)` — on `Err`, fall through to `handle_auto_arch()` as before |
| JetBrainsMono.zip URL changes between Nerd Fonts releases | `NERD_FONT_VERSION` is a single constant to bump; document in CHANGELOG |
| `detect_platform()` call in `run()` adds startup latency | `detect_platform()` reads `/etc/os-release` and `/proc` — sub-millisecond; acceptable |
| Existing Terminus font users lose their font | Terminus base packages still installed via `ensure_packages`; only the Nerd Font variant changes |
| `include_str!` compile-time embed fails if resource file missing | Files are promoted in-place — no new paths introduced; existing build path unchanged |

---

## ⚙️ TEST CHECKLIST

- [ ] `cargo build --workspace` passes after resource file updates
- [ ] `cargo test --workspace` passes — especially `phase_runner` and `driver_harness` tests
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] TUI launched on single-driver binary: ArchDetected screen does NOT appear
- [ ] TUI launched on multi-driver binary: ArchDetected screen DOES appear, auto-advances at 15s or on Enter
- [ ] `install_phase` for fonts: `JetBrainsMonoNerdFont-Regular.ttf` present after dry-run log; URL is correct
- [ ] Kitty launched after install: `font_family` resolves to JetBrainsMono (check `kitty +list-fonts | grep JetBrains`)
- [ ] Starship prompt shows memory module when RAM > 75%
- [ ] `source ~/.eza_aliases` in zsh: `ls` invokes eza, `goblin` invokes cmatrix

---

**Status**: ✅ PLANNING COMPLETE | 🔨 EXECUTION PENDING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-02-24
**Next Review**: 2026-02-25 (Phase 1 kickoff)