![Banner of the bard}(docs/assets/banner_final.png)
# MASH Installer
> Forged beneath the ruins of older systmes.

# 🌌 Mash-Installer: Cyber-Loom & Dwarven Forge Reborn
You step off the mag-rail and into the neon canyon of **Mash-Installer**, where Tolkien questcraft meets cyberpunk rain, George R.R. Martin scheming, and the Matrix’s terse code all pulse in sync. The CLI is a `ratatui` glyph-grid battle station, `phase_runner` is the dungeon master pulling the levers, and each log entry turns into an enchanted ledger. Every change is forged with the creed carved in this repository: **Always Be Backing up, Keep Commits Small, Always Be Testing**—and the lore keeps `/work` as the forge, leaving `main` for the drop-tested crown.

> **Neon Chronicle (Whimsical + Technical polish)**: This README thunders the saga, points to the ledger (`docs/improvement-plans.md`) as the current source of truth, and lays out why each upcoming act happens in this order. 🎤⚙️

## 🕹 Quick Invocation (Fast Path)
If you just want to trigger the ritual:
```bash
curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh | bash
```
Prefer to inspect the scroll before burning it:
```bash
curl -fsSL -o bootstrap.sh https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh
less bootstrap.sh
bash bootstrap.sh
```

## 🧱 Act Structure (Phases & Status)
| Act | Focus | Status |
| --- | --- | --- |
| Phase 1 – Deduplication | Shared helpers (PhaseContext, downloader, system utilities) were pulled apart, verbalized, and re-knit. | ✅ Complete (D-03 gate deferred until PhaseContext could shoulder it). |
| Phase 2 – Refactoring | Turbulent split of `lib.rs`, the `PhaseRunner` forge, context hardening, registry design, and CLI/TUI stabilization. | 🟡 Active (R-02 through R-10 in motion, with R-02/R-01/ R-03 already shaping the new surface). |
| Phase 3 – Pi 4B HDD | USB 3.0 tuning, HDD scheduler, Rust/Cargo home carving, smart I/O nodes. | ❄️ Pending Phase 2 API stability. |
| Phase 4 – Hardening | TLS shepherding, SMART guard, rollback rituals, lockfiles, and signal forensics. | 🛡️ Blocked until the new core settles. |

## ⚙️ Phase 2 Priority & Strategy (Proposed Order)
This ledger rearranges the remaining R-tasks to keep shared foundations stable, minimize rework, and ensure each consumer gets a clean API.
1. **D-03 – Dry-run gate (`PhaseContext::run_or_record`)** – Already merged into `PhaseRunner` so dry-run logic exists once. Gate the action, then build on it. 🛡️
2. **R-02 – Split `lib.rs`** – Module boundaries reduce cyclic noise; the rest of the team can evolve orchestrator, runner, and wiring without a monolithic tangle. 🧱
3. **R-01 – Harden `PhaseContext` helpers** – With modules split the shared context can absorb package, download, and service helpers without dragging the global namespace. 🔩
4. **R-03 – Structured `PhaseOutput`** – Before registries or reporting depend on the runner, we need a stable schema (`actions_taken`, `rollback_registered`, `errors`, `dry_run_state`). 🧾
5. **R-08 – Typed `PackageSpec`** – Registry and phases must know whether payloads are required, optional, or profile-gated before wiring them into installs. 📦
6. **R-05 – `ConfigService` error fidelity** – Blocks in configuration should bubble through the context with structured errors so downstream phases know when to bail. ⚠️
7. **R-04 – PhaseRegistry** – Once context, outputs, and package specs are stabilized, the registry can build honest metadata around features, profiles, and component dependencies. 🗂️
8. **R-07 – Pi detection helpers** – PlatformContext can now host clean hardware helpers after the registry can consume the metadata it produces. 🐧
9. **R-09 – Flatten `RunSummary` into `InstallationReport`** – With reporting pillars stable we collapse duplication, simplifying the CLI/TUI wiring and shared buffers. 📜
10. **R-06 – DriverTestHarness** – Tests can finally run against a stabilized `PhaseRunner`/`PhaseContext` with the new report surfaces in place. 🧪
11. **R-10 – CLI/TUI split** – Keep this for the end; the interface deserves to remix only once the data contracts are golden. 🎛️

Each order is intentional: shared core + context first, typing + output structures second, hardware/test layers afterward, and the UI refactor last.

## 📚 D-03: The Deferred Gate
### Why it was delayed
Phase 1 lacked a consolidated `PhaseContext`, and duplicate `if dry_run` checks had already been scattered. Early intervention risked inconsistent behavior across phases.
### Why it is now in place
`PhaseContext::run_or_record()` lives inside `PhaseRunner`. Every action now flows through a single gate instead of re-implementing dry-run checks, so logging, reporting, and rollbacks remain consistent. Documenting this ensures future travelers know the deferral was intentional and reversible.

## 🧰 Rules, Guidelines, and Tooling Rituals
- **Rules**: `Always Be Backing up`, `Keep Commits Small`, `Always Be Testing`. They’re etched in the README and in the forge:
  - `Always Be Backing up`: snapshot before refactor.
  - `Keep Commits Small`: every story is one commit, one strike of the hammer.
  - `Always Be Testing`: run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and suites before any merge.
- **Guidelines**: Only use SSH for GitHub actions, ask for clarification if unsure, think like a seasoned Rust engineer, work (and test) in `/work`, keep `/docs` updated.
- **Toolchain**: `cargo fmt`, `cargo clippy`, and `sccache` are our basic tooling. If `rustup` is missing, install it via `curl https://sh.rustup.rs -sSf | sh -s -- -y` and load `$HOME/.cargo/env` before running commands.
- **Testing logs**: Each run writes `.logs/test-<mode>-<timestamp>.log` linked from `docs/QA` (and soon the neon ledger). Keep tests green before pushing to `main`.

## 📜 Documentation Discipline
Treat `docs/improvement-plans.md` as the living config for Phase 2 sequencing. Every doc should note whether it received a Whimsical or Technical polish—this README carries one of each. Update `/docs` whenever plan or tooling shifts.

## 🔭 Next Steps (What Remains)
1. Continue Phase 2 in the order above (this README and `docs/improvement-plans.md` record the new priority). R-02 and R-01 are already shaping the split context/staging.
2. Keep wiring `installer-cli` and `installer-*` crates against the new shapes; they should compile once the registry/report shape stabilizes.
3. Re-run `cargo fmt`, `cargo clippy`, and `cargo test` from `/work` after any substantive change. Green builds are the only ones that leave for `main`.
4. After Phase 2 completes, move into Phase 3 (Pi 4B HDD) and Phase 4 (hardening) with the strengthened context.

If you still ask “what about deferred D-03?” the answer is in the dry-run ledger above and in `docs/improvement-plans.md`, because both now record the decision and the reasoning.
