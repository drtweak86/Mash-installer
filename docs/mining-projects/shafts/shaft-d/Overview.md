# SHAFT-D OVERVIEW: ARCHITECTURE FLEXIBILITY

**Objective**: Fix the architecture detection logic to provide users with manual overrides, skipping options, or the ability to cancel if detection fails or is incorrect.

## 📜 SCOPE
- Refactor `installer-core/src/platform.rs` and `wallpaper-downloader/src/config.rs`.
- Update the TUI/CLI flow in `installer-cli`.

## 🛠️ FILES TO BE TOUCHED
- `installer-core/src/platform.rs`: Add support for optional architecture overrides.
- `installer-cli/src/main.rs`: Implement the user prompt for architecture confirmation.
- `wallpaper-downloader/src/main.rs`: Ensure it respects the overridden architecture.

## 🏗️ METHODOLOGY
- **Incremental Refinement**: First, add the data models to support manual overrides, then implement the TUI prompts.
- **Fail-Safe Defaults**: If the user skips, the system will fall back to its best-guess detection.

## ✅ DELIVERABLES
- A working architecture prompt in the installer.
- Verified manual entry for `aarch64`, `x86_64`, etc.
- Clean exit if the user chooses to cancel.
