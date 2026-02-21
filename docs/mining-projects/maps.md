# ⚒️ The Miner's Active Maps: Current Shaft
> *“Focus the mind, strike the vein. One rune at a time.”* — Bard 🍺

## ✅ SHAFT I: The Sudo Plumbing <COMPLETED> 🛡️
**Result**: Sudo interaction fixed. Credentials now injected via `sudo -S` from TUI prompts.

## 🚪 SHAFT D: The Gate & Guardian <ACTIVE> ⛏️
**Status**: Excavation Commencing.
**Objective**: Upgrade `install.sh` with architecture detection and dependency checks. Pass results to `installer-cli`.

### 🛠️ Execution Plan (Shaft D)
1.  **Script Logic**: Refactor `install.sh` to use `uname -m` for detection.
2.  **The Guardian**: Implement dependency checks (`curl`, `tar`, etc.) in the shell script.
3.  **TUI Handoff**: Update `installer-cli` to accept `--arch` and skip the manual selection screen.
4.  **Verification**: Test detection on local hardware.

---
**Last Updated**: 2026-02-22  
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
