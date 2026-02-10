## Block 4 Findings (WO-016 to WO-019)

### WO-019 (Commit `9d9eb3f`): Formalize Release Process

**Summary:** This commit introduces `docs/release-process.md` and `scripts/check-docs.py` to formalize the release process and aid in documentation checks.

**Findings:** None.

### WO-018 (Commit `8af00ea`): Formalize Testing Infrastructure

**Summary:** This commit introduces `docs/testing-infrastructure.md` and `scripts/test-infrastructure.sh` to formalize the testing infrastructure and orchestrate different test suites.

**Findings:** None.

### WO-017 (Commit `9dd1cf0`): Expand Dry-Run Fidelity

**Summary:** This commit lays the groundwork for enhanced dry-run mode by introducing `DryRunLog` (implicitly) and `record_dry_run` in `PhaseContext`, and demonstrating its use in `docker.rs`.

**Findings:**
1.  **Resolved:** **Missing Tests.** The commit `43e274b` (`test: cover dry-run logging`) adds a unit test for `DryRunLog` itself and integration tests in `installer-core/src/docker.rs` to verify that phases correctly record dry-run actions.

### WO-016 (Commit `9441219`): Add Structured Logging Framework

**Summary:** This commit introduces and integrates a structured logging framework using `tracing-subscriber` and allows configuration via `MashConfig`.

**Findings:**
1.  **Missing Tests:** No dedicated unit or integration tests were added to verify the logging framework's configuration (e.g., if different `LogFormat`s or levels produce the expected output) or its output for various log messages.