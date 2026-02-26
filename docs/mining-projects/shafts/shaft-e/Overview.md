# SHAFT-E OVERVIEW: INSTALLER FLOW OPTIMIZATION

**Objective**: Streamline the installation sequence to reduce friction and improve user pacing.

## 📜 SCOPE
- Reorder the main menu flow in `installer-cli/src/main.rs`.
- Consolidate `sudo` and `doctor` checks into the absolute beginning of the process.

## 🛠️ FILES TO BE TOUCHED
- `installer-cli/src/main.rs`: Reorganize the `run_installer` function.
- `installer-core/src/doctor.rs`: Refactor to ensure it can be run as the first step of the install.
- `installer-core/src/sudo.rs`: Ensure sudo is requested and cached at the start.

## 🏗️ METHODOLOGY
- **Minimal Friction**: The goal is to collect all necessary inputs (passwords, preferences) at the start, then allow the install to proceed unattended.
- **Improved Feedback**: Add clearer "Step X/Y" indicators to each phase.

## ✅ DELIVERABLES
- A "Bard's Welcome" sequence that performs all pre-flight checks before asking for configuration.
- A smoother transition between menus.
- Clearer status indicators throughout the process.
