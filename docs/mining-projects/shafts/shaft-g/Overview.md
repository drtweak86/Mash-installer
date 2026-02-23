# SHAFT-G OVERVIEW: MASH ECOSYSTEM INTEGRATION

**Objective**: Ensure seamless callability by the parent `MASH` project (https://github.com/user/drtweak86/MASH) while maintaining 100% independent standalone functionality for individual users.

## 📜 SCOPE
- Refactor `installer-cli/src/main.rs` for ecosystem-aware flags.
- Implement JSON-based status reporting and final results.
- Create a portable, single-binary build process for easy distribution.

## 🛠️ FILES TO BE TOUCHED
- `installer-cli/src/main.rs`: Add `--called-by-mash` and JSON report logic.
- `installer-core/src/status.rs`: Expand status reports to include ecosystem-relevant metadata.
- `xtask/src/main.rs`: Update the release process to prioritize portable binaries.

## 🏗️ METHODOLOGY
- **Handshake Protocol**: Implement a specific flag that `MASH` can use to signal its presence, unlocking headless mode and machine-readable output.
- **Graceful Degradation**: If `MASH` is not present, the installer defaults to its rich, interactive TUI (Ratatui) without any loss of functionality.
- **Environment Parity**: Whether called by `MASH` or a human, the underlying installation logic remains identical and uses the same `DistroDriver` system.

## ✅ DELIVERABLES
- A working `--called-by-mash` flag that outputs JSON status.
- A "Headless Summary" for the parent process.
- Verified standalone TUI operation (The "Independent Download" path).
