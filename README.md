# MASH Installer
> **"Beneath the neon rain, where cyber-looms hum and dwarven forges glow, the MASH Installer awaits your command."**

![Banner of the bard](docs/assets/banner_final.png)

## 🌌 The Legend of MASH Installer

You stand at the crossroads of **MASH Installer**, where the sagas of George R.R. Martin collide with the lush detail of Tolkien, tempered by the cyberpunk grit of Blade Runner and the Matrix's digital rhythm. This is no mere installer—it is a **cyber-loom**, a **dwarven forge**, and a **terminal spellbook** all in one.

### The Prophecy
The CLI is your **ratatui glyph-grid battle station**, the `phase_runner` is your **dungeon master**, and every log entry becomes an **enchanted ledger**. The ledger—`docs/improvement-plans.md`—is the **One Ring of Truth** for the phases and their ordering. The creed carved into this repo still echoes:

> **"Always Be Backing up, Keep Commits Small, Always Be Testing, Always Be Documenting."**

We build and test in the **/work/Mash-installer** forge, leaving `main` for the **drop-tested crowns** of battle.

## 📖 The Tome of Invocation

### Quick Path (For the Hasty Adventurer)
```bash
curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh | bash
```

### The Wise Path (For Those Who Read the Scrolls First)
```bash
curl -fsSL -o bootstrap.sh https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh
less bootstrap.sh
bash bootstrap.sh
```

**Every invocation chants the sacred rituals:**
- `cargo fmt` (the rune alignment)
- `cargo clippy --all-targets --all-features -- -D warnings` (the spell check)
- `cargo test` (the trial by combat)

**Before anything touches `main`, the forge must be green.**

## 🏰 The Acts of the Saga

| Act | Focus | Status |
| --- | --- | --- |
| **Phase 1 – Deduplication** | Helpers untangled, downloads unified, duplicate system calls cleared. D-03 deferred until `PhaseContext` was ready. | ✅ **Complete** |
| **Phase 2 – Refactoring** | `lib.rs` split, `PhaseRunner` forged, `PhaseContext` hardened, registry drafted, data contracts shaped, CLI/TUI teased apart. | ✅ **Complete (R-02 through R-10 settled)** |
| **Phase 3 – Pi 4B HDD** | Preflight, USB 3.0, and HDD tuning for the blue ports. | ❄️ **Paused until Phase 2 stabilizes** |
| **Phase 4 – Hardening** | TLS shepherding, rollback rituals, lockfiles, and signal forensics. | 🛡️ **Blocked on Phase 2 API stability** |

## 🔮 The Phase Constellation

The **10-point plan** from the ancient scrolls (`docs/QA/PlanA.md`) now rides the rails behind a single gate: **`PhaseContext::run_or_record()`** (D-03). The saga continues with this explainable order:

1. **D-03 – Dry-run gate**: Phase actions now pass through `PhaseRunner`'s single portal; no duplicate `if dry_run`. This is recorded in the ledger so the deferral is transparent. 🛡️
2. **Refactor `InstallContext`**: Break the god object into focused contexts (`PlatformContext`, `UserOptionsContext`, `PhaseContext` slices) so phases only receive what they need. Tight coupling drops, testability rises. 🔧
3. **Formal `Phase` trait**: Define `name()`, `description()`, `execute()`, `should_run()`, and let each phase implement it. The trait rests on the slimmed contexts and lets the runner reason about metadata. 🧭
4. **Phase runner redesign**: Build `PhaseRunner` to iterate trait objects, capture structured reporting, and enforce the dry-run gate. It also becomes the public API that the CLI and TUI can consume. 🧱
5. **Centralize configuration**: `ConfigService` validates, surfaces defaults, and feeds every context slice, keeping config errors consistent. 🗂️
6. **Abstract system helpers**: Commands, downloads, file ops, and services move into shared helpers that master logging and dry-run behavior. 🛠️
7. **Structured error handling**: `ConfigService` and every phase wrap outcomes in rich error enums so regulators can triage without rerunning the phase stack. ⚠️
8. **Externalize strings**: UI text leaves the source and lands in config (TOML/JSON) so phases stay agnostic and localization becomes possible. 🗣️
9. **CLI/TUI decoupling**: The CLI consumes events and reports instead of printing directly; the core returns metadata, letting the interface stay declarative. 🎛️
10. **Library API cleanup**: `installer-core` returns structured `InstallationReport` data; the UI layers play conductor with those rich results. 📜
11. **Driver test harness**: Once the data contracts stabilize, fire up the harness to exercise each distro driver against the new surfaces and catch regressions early. 🧪

## ⚔️ The Rules of the Forge

### The Sacred Oaths
1. **ABB - Always Be Backing up** – Snapshot the world before major refactors.
2. **KCS - Keep Commits Small** – Each logical change deserves a single hammer strike.
3. **ABT - Always Be Testing** – Run the rituals: `cargo fmt`, `cargo clippy`, `cargo test`.
4. **ABD - Always Be Documenting** – Every change, decision, and finding must be recorded in the appropriate doc.

### The Toolbelt
- `rustfmt` – Aligns the runes
- `clippy` – Reveals hidden cracks
- `sccache` – Keeps the forge warm
- `cargo test` – Proves the blade holds

### The Workflow
1. Build and test in the **`work`** branch
2. Only merge to **`main`** when every log tells a green story
3. The ledger (`docs/improvement-plans.md`) is the single source of truth

## 📚 The Libraries of Lore

### Active Scrolls
- `docs/mining-projects/maps.md` – The active expedition map
- `docs/mining-projects/maps-explored.md` – Archive of completed work
- `docs/mining-projects/shafta.md` – Strategic reconnaissance reports
- `docs/HISTORY.md` – The drunken dwarf bard's chronicles

### Legacy Scrolls (Archived)
- `docs/legacy/` – Old tomes, preserved but no longer active

## 🔮 The Road Ahead

### Next Quests
1. **Driver Test Harness** – Test the walls before mining deeper
2. **Phase 3: Pi 4B HDD Tuning** – Optimize the primary hardware
3. **Phase 4: Hardening** – Seal the forge against the neon rain
4. **System Packaging** – Let the system's courier deliver the blade
5. **TUI Rendering** – Make the forge glow with Ratatui

## 🎤 The Bard's Final Words

> "The code is the blade, the tests are the forge, and the documentation is the map. Without all three, the journey is doomed."

-- **The Drunk Dwarf Bard** 🎤🪙🧱

**The forge is locked. The first blade is stamped (v0.1.0). The gates are secure. The scaffolding is retired. The documentation is complete. The mine awaits your next descent.** 🗺️⛏️🔥