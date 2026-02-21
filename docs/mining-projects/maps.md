# ⚒️ The Miner's Active Maps: Current Shaft
> *“Focus the mind, strike the vein. One rune at a time.”* — Bard 🍺

## ✅ SHAFT I: The Sudo Plumbing <COMPLETED> 🛡️
**Result**: Sudo interaction fixed. Credentials now injected via `sudo -S`.

## ✅ SHAFT D: The Gate & Guardian <COMPLETED> 🚪
**Result**: `install.sh` now smart-detects arch and dependencies. `--arch` flag implemented.

## 📼 SHAFT F: The Black Box <ACTIVE> ⛏️
**Status**: Excavation Commencing.
**Objective**: Persistent logging to `~/mash-install.log` and a rigorous testing rig for ARM/x86 paths.

### 🛠️ Execution Plan (Shaft F)
1.  **Persistent Runes**: Update `logging.rs` to output to a file in the user's home directory.
2.  **Telemetry Audit**: Ensure all critical events are captured in the file log, even if hidden in the TUI.
3.  **Testing Rig**: Create new integration tests in `tests/` that mock the architecture and verify logic paths.
4.  **Script Validation**: Logic tests for `install.sh` to prevent regressions.

---
**Last Updated**: 2026-02-22  
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
