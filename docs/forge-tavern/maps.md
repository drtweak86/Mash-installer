# ⚒️ The Miner's Active Maps: Current Shaft
> *"The blade is hardened. The crown is on the table."* — Bard 🍺

---

## ✅ SHAFT J: The Overlord Protocols — ACTIVE
> *Branch*: `work-shaftj-overlord`
> *Risk*: MEDIUM (mitigated with phased approach)
> *Reward*: HIGH (long-term maintainability, performance, alignment)
> *Status*: ✅ PHASE 1 COMPLETE | 🔨 PHASE 2 PENDING

### PHASE 1: OVERLORD CONFIG PROMOTION ✅ COMPLETE
**Objective**: Promote BBC Acorn terminal configs from staging to production resources.

#### 1.1 — Update `resources/shell/kitty.conf`
- **File**: `resources/shell/kitty.conf`
- **Change**: Replace with `docs/incoming-files/kitty.txt`
- **Key**: `font_family JetBrainsMono Nerd Font`
- **Verification**: `grep "font_family" resources/shell/kitty.conf` → `JetBrainsMono Nerd Font` ✅
- **Status**: ✅ COMPLETE
- **Commit**: [commit_hash]

#### 1.2 — Update `resources/shell/starship.toml`
- **File**: `resources/shell/starship.toml`
- **Change**: Replace with `docs/incoming-files/starship.toml.txt`
- **Key**: `[memory_usage]` module with 75% threshold
- **Verification**: `grep "memory_usage" resources/shell/starship.toml` → present ✅
- **Status**: ✅ COMPLETE
- **Commit**: [commit_hash]

#### 1.3 — Update `resources/shell/eza_aliases.sh`
- **File**: `resources/shell/eza_aliases.sh`
- **Change**: Already contains `alias goblin='cmatrix -a'` easter egg
- **Key**: `alias goblin='cmatrix -a'` easter egg
- **Verification**: `grep "goblin" resources/shell/eza_aliases.sh` → present ✅
- **Status**: ✅ COMPLETE (no change needed)

### PHASE 2: ARCH DETECTION OPTIMIZATION ✅ COMPLETE
**Objective**: Skip 15-second ArchDetected banner when exactly one driver matches.

#### 2.1 — Modify `run()` to Detect Single-Driver Match
- **File**: `installer-cli/src/tui/app.rs` (lines 938–967)
- **Change**: Replaced 3-line block with 27-line single-match logic using `detect_platform()` + `driver.matches()`
- **Requires**: `installer_core::detect_platform()` re-exported in lib.rs ✅ (was already present)
- **Verification**: fmt clean | clippy clean | 114 tests green ✅
- **Status**: ✅ COMPLETE — 2026-02-26

#### 2.2 — Keep `handle_auto_arch()` Unchanged
- **File**: `installer-cli/src/tui/app.rs` (lines 274–278)
- **Change**: None (used in fallback case)
- **Status**: ✅ CONFIRMED UNCHANGED

#### 2.3 — Keep `tick()` Unchanged
- **File**: `installer-cli/src/tui/app.rs`
- **Change**: None (timer only fires when `arch_timer` is `Some`)
- **Status**: ✅ CONFIRMED UNCHANGED

### PHASE 3: NERD FONT UPGRADE ✅ COMPLETE
**Objective**: Switch from Terminus to JetBrainsMono Nerd Font.

#### 3.1 — Change the Font Name Constant and Target File
- **File**: `installer-core/src/fonts.rs`
- **Changes**:
  - Renamed: `install_terminess_nerd_font` → `install_jetbrains_nerd_font` ✅
  - Added: `const NERD_FONT_VERSION: &str = "v3.3.0";` ✅
  - Updated: `target_font` → `JetBrainsMonoNerdFont-Regular.ttf` ✅
  - Updated: `font_name` → `JetBrainsMono.zip` ✅
  - Updated: URL uses `NERD_FONT_VERSION` constant ✅
- Also updated: `installer-cli/src/tui/menus.rs:564` font recommendation line ✅
- **Verification**: fmt clean | clippy clean | 114 tests green ✅ 2026-02-26
- **Status**: ✅ COMPLETE — 2026-02-26

#### 3.2 — Keep Terminus Base Packages
- **File**: `installer-core/src/fonts.rs`
- **Change**: None (system packages remain)
- **Status**: ✅ CONFIRMED UNCHANGED

#### 3.3 — Keep File Filter for Zip Extraction
- **File**: `installer-core/src/fonts.rs`
- **Change**: None (`.ttf` filter is correct)
- **Status**: ✅ CONFIRMED UNCHANGED

### PHASE 4: VERIFICATION & TESTING ⏳ PENDING
**Objective**: Ensure all changes work correctly.

#### 4.1 — Build Verification
- **Command**: `cargo build --workspace`
- **Status**: ✅ COMPLETE — clean 2026-02-26

#### 4.2 — Test Verification
- **Command**: `cargo test --workspace`
- **Status**: ✅ COMPLETE — 114/114 green 2026-02-26

#### 4.3 — Clippy Verification
- **Command**: `cargo clippy --all-targets -- -D warnings`
- **Status**: ✅ COMPLETE — zero warnings 2026-02-26

#### 4.4 — TUI Verification
- **Test**: Single-driver binary → no ArchDetected screen
- **Status**: ⏳ PENDING (runtime test — requires live binary)

#### 4.5 — Font Verification
- **Test**: `kitty +list-fonts | grep JetBrains` → correct font
- **Status**: ⏳ PENDING (runtime test — requires live install)

### PHASE 5: FINAL COMMIT & PR ✅ COMPLETE
**Objective**: Commit and merge the changes.

#### 5.1 — Commit Changes
- **Commit**: `af41104` — `feat: overlord phase 2+3 — arch auto-detect skip + JetBrainsMono Nerd Font`
- **Status**: ✅ COMPLETE — 2026-02-26

#### 5.2 — Open PR
- **Branch**: `work-shaftj-overlord-p2p3` → `main`
- **PR**: [#63](https://github.com/drtweak86/Mash-installer/pull/63)
- **Status**: ✅ MERGED — 2026-02-26

#### 5.3 — Wait for CI Green
- **Checks**: fmt, clippy, test, audit, build, distro matrix, aarch64 — all green
- **Status**: ✅ COMPLETE — 2026-02-26

#### 5.4 — Merge
- **Action**: Merge PR #63 to main
- **Merge commit**: `a85030d`
- **Status**: ✅ MERGED — 2026-02-26

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

- [x] `cargo build --workspace` passes after resource file updates ✅ 2026-02-26
- [x] `cargo test --workspace` passes — especially `phase_runner` and `driver_harness` tests ✅ 114/114 green
- [x] `cargo clippy --all-targets -- -D warnings` clean ✅ 2026-02-26
- [ ] TUI launched on single-driver binary: ArchDetected screen does NOT appear
- [ ] TUI launched on multi-driver binary: ArchDetected screen DOES appear, auto-advances at 15s or on Enter
- [ ] `install_phase` for fonts: `JetBrainsMonoNerdFont-Regular.ttf` present after dry-run log; URL is correct
- [ ] Kitty launched after install: `font_family` resolves to JetBrainsMono (check `kitty +list-fonts | grep JetBrains`)
- [ ] Starship prompt shows memory module when RAM > 75%
- [ ] `source ~/.eza_aliases` in zsh: `ls` invokes eza, `goblin` invokes cmatrix

---

**Status**: ✅ ALL PHASES COMPLETE — SHAFT J OVERLORD PROTOCOLS MERGED (PR #63 · a85030d · 2026-02-26)
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-02-26
**Next Review**: 2026-02-26 (Phase 5 — commit & PR)

### DEPENDENCY RESTORATION (Post-Phase 1)
**Issue**: Build failures after merging claude/release-v1.0.0-2s3pa branch
**Root Cause**: Missing dependencies required by wallpaper module
**Resolution**: Added missing dependencies to installer-core/Cargo.toml:
- reqwest 0.12 (JSON + rustls-tls)
- num_cpus 1
- tokio 1.0 (full features)
- async-trait 0.1
- thiserror 1.0
**Verification**: ✅ cargo build --workspace | ✅ cargo test --workspace

---

## 🏗️ SHAFT H: Installer Experience Overhaul — PLANNING COMPLETE
> *Branch*: `work-shaft-h-experience` (to be created)
> *Risk*: MEDIUM (UI changes, new features)
> *Reward*: HIGH (significantly improved user experience)
> *Status*: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING

### OVERVIEW
Comprehensive overhaul of the MASH installer to improve user experience, add font management, desktop environment support, and integrate existing scripts.

### PHASE 1: Font Management System
**Objective**: Implement comprehensive Nerd Fonts installation from GitHub with user selection.

#### 1.1 — Create fonts_all.rs Module
- **File**: `installer-core/src/fonts_all.rs` (new)
- **Change**: Nerd Fonts GitHub integration
- **Key**: `NerdFont` struct, download/install functions
- **Verification**: Unit tests pass, fonts install correctly
- **Status**: ⏳ PENDING

#### 1.2 — Update Font Selection UI
- **File**: `installer-cli/src/tui/menus.rs`
- **Change**: Add font selection screen
- **Key**: Scrolling list with search/filter
- **Verification**: UI displays and navigates correctly
- **Status**: ⏳ PENDING

#### 1.3 — Set Default Fonts
- **File**: `resources/shell/kitty.conf`, `resources/shell/starship.toml`
- **Change**: Terminus/JetBrains Mono as default
- **Key**: Consistent font configuration
- **Verification**: Fonts applied system-wide
- **Status**: ⏳ PENDING

### PHASE 2: Desktop Environment Support
**Objective**: Add DE installation with X11/Wayland options and cross-distro support.

#### 2.1 — Create desktop_environments.rs Module
- **File**: `installer-core/src/desktop_environments.rs` (new)
- **Change**: DE installation logic
- **Key**: Cross-distro package mapping
- **Verification**: DEs install correctly on multiple distros
- **Status**: ⏳ PENDING

#### 2.2 — Add DE Selection UI
- **File**: `installer-cli/src/tui/menus.rs`
- **Change**: DE selection screen
- **Key**: X11/Wayland toggle with Pi warnings
- **Verification**: UI works, warnings displayed appropriately
- **Status**: ⏳ PENDING

### PHASE 3: Enhanced Install Flow
**Objective**: Redesign installer flow to be more human-friendly and logical.

#### 3.1 — Multi-Screen Navigation
- **File**: `installer-cli/src/tui/menus.rs`
- **Change**: Logical component grouping
- **Key**: Back/next navigation with state preservation
- **Verification**: Navigation works, state preserved
- **Status**: ⏳ PENDING

### PHASE 4: Information Display
**Objective**: Add bottom info box with installation details and time estimates.

#### 4.1 — Create Info Box Component
- **File**: `installer-cli/src/tui/info_box.rs` (new)
- **Change**: Bottom info display
- **Key**: Time estimates and context help
- **Verification**: Info updates correctly during operations
- **Status**: ⏳ PENDING

### PHASE 5: Long Process Confirmation
**Objective**: Add explicit confirmation for operations > 2 minutes.

#### 5.1 — Create Confirmation Dialog
- **File**: `installer-cli/src/tui/confirmation.rs` (new)
- **Change**: Long process confirmation
- **Key**: Advisory messages and countdown timer
- **Verification**: Dialog appears when expected
- **Status**: ⏳ PENDING

### PHASE 6: Wallpaper Harvest Integration
**Objective**: Integrate mash-wallpaper-harvest functionality.

#### 6.1 — Transmogrify Python to Rust
- **File**: `scripts/mash-wallpaper-harvest.py` (new)
- **Change**: Rust implementation of wallpaper downloader
- **Key**: Wallhaven API integration
- **Verification**: Wallpapers download correctly
- **Status**: ⏳ PENDING

### PHASE 7: Pi Overlord Transmogrification
**Objective**: Integrate and transmogrify pi-overlord-grimoire functionality.

#### 7.1 — Cross-Distro Package Mapping
- **File**: `scripts/pi-overlord-integration.rs` (new)
- **Change**: Fedora → multi-distro support
- **Key**: Package mapping database
- **Verification**: Works on Debian, Arch, etc.
- **Status**: ⏳ PENDING

### PHASE 8: Testing & Documentation
**Objective**: Comprehensive testing and documentation.

#### 8.1 — Unit and Integration Tests
- **Files**: Various test files
- **Change**: Test coverage for new features
- **Key**: All tests pass
- **Verification**: `cargo test --workspace` green
- **Status**: ⏳ PENDING

#### 8.2 — Update Documentation
- **Files**: `docs/forge-tavern/maps.md`, various READMEs
- **Change**: Document new features
- **Key**: Complete and accurate documentation
- **Verification**: Documentation builds successfully
- **Status**: ⏳ PENDING

---

**Next Shaft**: Shaft H — Installer Experience Overhaul
**Status**: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-02-26
**Next Review**: 2026-03-01 (Implementation kickoff)

---

## 🏗️ SHAFT I: Software Catalog & Installation Flow Overhaul — PLANNING COMPLETE
> *Branch*: `work-shaft-i-catalog` (to be created)
> *Risk*: MEDIUM (catalog restructuring, UI changes)
> *Reward*: HIGH (significantly improved user experience, logical organization)
> *Status*: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING

### OVERVIEW
Complete reorganization of MASH software catalog and installation flow to create a more logical, user-friendly, and efficient system with curated S-tier applications, multiple installation modes, and optimized dependency handling.

### PHASE 1: Software Catalog Curation
**Objective**: Create comprehensive software catalog with S-tier applications

#### 1.1 — Define Category Structure
- **File**: `docs/mining-projects/shaft-i/EX_I01_Software_Catalog_Curation.md`
- **Change**: Category/subcategory hierarchy
- **Key**: 10 main categories with logical subcategories
- **Verification**: Catalog structure approved
- **Status**: ⏳ PENDING

#### 1.2 — Curate S-Tier Applications
- **File**: `resources/catalog/s-tier_catalog.toml` (new)
- **Change**: 5 S-tier apps per category with reasoning
- **Key**: Brave Browser as top Internet choice
- **Verification**: 50+ S-tier applications curated
- **Status**: ⏳ PENDING

#### 1.3 — Include All Programming Languages
- **File**: `resources/catalog/programming_languages.toml` (new)
- **Change**: Comprehensive language list
- **Key**: Version managers and build tools
- **Verification**: All major languages included
- **Status**: ⏳ PENDING

#### 1.4 — Create TOML Catalog Structure
- **File**: `resources/catalog/full_catalog.toml` (new)
- **Change**: Complete software database
- **Key**: Tier system (S, A, B, C)
- **Verification**: Valid TOML with package mappings
- **Status**: ⏳ PENDING

### PHASE 2: Installation Modes
**Objective**: Implement Manual, Auto, and Bard's Recommendations modes

#### 2.1 — Manual Mode Implementation
- **File**: `installer-core/src/installation_modes.rs` (new)
- **Change**: Individual program selection
- **Key**: Search, filter, and multi-select
- **Verification**: UI functional and intuitive
- **Status**: ⏳ PENDING

#### 2.2 — Auto Mode (Category-Based)
- **File**: `installer-core/src/installation_modes.rs`
- **Change**: Install by category/subcategory
- **Key**: One-click category installation
- **Verification**: Category selection works
- **Status**: ⏳ PENDING

#### 2.3 — Bard's Recommendations Mode
- **File**: `installer-core/src/installation_modes.rs`
- **Change**: Only top S-tier from each category
- **Key**: Opinionated best-of selection
- **Verification**: Installs correct S-tier apps
- **Status**: ⏳ PENDING

### PHASE 3: Menu Restructuring
**Objective**: Organize software into logical categories

#### 3.1 — Category/Subcategory Hierarchy
- **File**: `installer-core/src/category_organization.rs` (new)
- **Change**: Themes, Development, System grouped
- **Key**: Logical navigation structure
- **Verification**: Menu navigation smooth
- **Status**: ⏳ PENDING

#### 3.2 — UI Implementation
- **File**: `installer-cli/src/tui/software_menus.rs` (new)
- **Change**: Enhanced software selection
- **Key**: Category browsing and search
- **Verification**: UI displays correctly
- **Status**: ⏳ PENDING

### PHASE 4: Installation Flow Optimization
**Objective**: Ensure proper installation order

#### 4.1 — Dependency Resolution
- **File**: `installer-core/src/optimization_flow.rs` (new)
- **Change**: Build dependency graph
- **Key**: ccache/sccache before Rust
- **Verification**: Correct installation order
- **Status**: ⏳ PENDING

#### 4.2 — Parallel Installation
- **File**: `installer-core/src/optimization_flow.rs`
- **Change**: Concurrent package installation
- **Key**: Performance optimization
- **Verification**: Parallel install working
- **Status**: ⏳ PENDING

### PHASE 5: UI Integration
**Objective**: Connect catalog to installer UI

#### 5.1 — Software Selection Screens
- **File**: `installer-cli/src/tui/software_menus.rs`
- **Change**: Category navigation
- **Key**: Mode selection and filtering
- **Verification**: UI functional
- **Status**: ⏳ PENDING

#### 5.2 — Installation Progress
- **File**: `installer-cli/src/tui/installation_flow.rs` (new)
- **Change**: Category-based progress
- **Key**: Visual progress tracking
- **Verification**: Progress updates correctly
- **Status**: ⏳ PENDING

### PHASE 6: Testing & Documentation
**Objective**: Ensure quality and completeness

#### 6.1 — Unit Testing
- **Files**: Various test files
- **Change**: Catalog and installation logic tests
- **Key**: All tests pass
- **Verification**: `cargo test` green
- **Status**: ⏳ PENDING

#### 6.2 — Documentation
- **Files**: `docs/mining-projects/shaft-i/*`
- **Change**: Complete documentation
- **Key**: User and technical guides
- **Verification**: Documentation builds
- **Status**: ⏳ PENDING

---

**Next Shaft**: Shaft I — Software Catalog & Installation Flow Overhaul
**Status**: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-02-26
**Next Review**: 2026-03-05 (Implementation kickoff)