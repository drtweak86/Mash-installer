# Mash-Installer Improvement Plans
> **Neon Chronicle (Technical polish)**: this ledger keeps the beats precise so readers can see the prioritized order, the deferred D-03 decision, and why each refinement follows the last. ⚙️

## Phase Overview
| Phase | Focus | Status |
| --- | --- | --- |
| Phase 1 – Deduplication | Extract shared helpers, unify downloads, and remove duplicate system calls across the phases. | ✅ Completed (D-03 dry-run gate deferred until the new `PhaseContext` could document it). |
| Phase 2 – Refactoring | Split `lib.rs`, formalize `PhaseRunner`, harden `PhaseContext`, standardize reporting, theory-craft a registry, and make the core library-grade. | 🟡 In progress; the new priority queue is being followed, beginning with the dry-run gate and module split. |
| Phase 3 – Pi 4B HDD | Harden Pi 4B preflight, optimize USB 3.0 staging, and tune HDD/Rust homes for the blue ports. | ❄️ Pending Phase 2 stabilization. |
| Phase 4 – Hardening | Add parking locks, TLS wiggles, signal handling, rollback safety, and lock files. | 🛡️ Blocked on Phase 2 API stability. |

## Phase 2 Revised Priority (Decision + Order)
The Phase 2 queue now honors a single source of truth: the dry-run gate must exist before anything else touches context, modules, or reporting. Each entry below lists why it runs when it does.
1. **D-03 – Dry-run gating (`PhaseContext::run_or_record`)** — this gate is in place inside `PhaseRunner`. Recording the decision keeps the ledger honest and prevents duplicate `if dry_run` checks. 🛡️
2. **R-02 – Split `lib.rs`** — creating module boundaries reduces coupling and lets the rest of Phase 2 work within scoped crates (orchestrator, runner, options, sudo). 🧱
3. **R-01 – Harden `PhaseContext` helpers** — with the codebase split, the shared context can natively host downloader, package, and service helpers without dragging in the entire `lib.rs` monolith. 🔩
4. **R-03 – Structured `PhaseOutput`** — a clear metadata schema (`actions_taken`, `rollback_registered`, `errors`, `dry_run`) must exist before registries or reports rely on what the runner produces. 🧾
5. **R-08 – Typed `PackageSpec`** — packages signal whether they're required, optional, or profile-gated, so registry and phases can programmatically decide what to install. 📦
6. **R-05 – `ConfigService` error fidelity** — richer configuration errors travel through `PhaseContext`, keeping failure stories visible before registries or drivers run. ⚠️
7. **R-04 – PhaseRegistry** — with structured outputs, typed specs, and a hardened context, the registry can honor profiles, metadata, and feature gates without guessing. 🗂️
8. **R-07 – Pi detection helpers** — PlatformContext earns clean helpers after the registry can accept the metadata it produces. 🐧
9. **R-09 – Flatten `RunSummary` into `InstallationReport`** — once reporting pillars are solid, flattening removes duplication and simplifies CLI/TUI wiring. 📜
10. **R-06 – DriverTestHarness** — tests can safely exercise each distro driver once the core runner/context/report contract is stable. 🧪
11. **R-10 – CLI/TUI split** — defer until data contracts are stable and the driver harness has exercised the new surfaces. 🎛️

## D-03 Decision Record
- **Why it was deferred:** Phase 1 lacked the consolidated `PhaseContext`, so early gate insertion would have scattered `dry_run` checks across helpers.
- **Why it is now complete:** `PhaseContext::run_or_record()` lives in `PhaseRunner`, every new helper invokes it, and the dry-run visualizations go through one gate. Recording this decision in the improvement plan ensures future explorers know the gate went live once the context could sustain it.

## What Remains / Next Steps
1. Continue Phase 2 in the order above. R-02 and R-01 are already being staged so the new module/context surfaces settle first.
2. Wire `installer-cli`, `installer-*`, and the TUI to the new data shapes once `PhaseOutput`, `PackageSpec`, and the registry stabilizes.
3. Re-run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` from `/work` after each significant change. Only green runs leave for `main`.
4. Document every change: tag each doc with a Whimsical or Technical polish indicator, keep `/docs` up to date, and log build/test artifacts in `.logs`.
5. After Phase 2 finishes, move into Phase 3 (Pi 4B HDD tuning) and Phase 4 (hardening) with the refactored core.

> The ledger notes that Phase 2 still breathes fire. Keep the rustfmt/clippy flames alive and the `sccache` cache warm. 
