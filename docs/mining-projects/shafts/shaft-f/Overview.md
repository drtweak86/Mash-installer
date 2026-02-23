# SHAFT-F OVERVIEW: DISTRO-CENTRIC INTELLIGENCE

**Objective**: Implement deep flavor/DE detection to provide targeted software recommendations.

## 📜 SCOPE
- Enhance `DistroDriver` to detect Desktop Environments (DEs).
- Update the software catalog with DE-aware "S-Tier" artifacts.
- Filter software recommendations in the TUI based on detected DE and Wayland status.

## 🛠️ FILES TO BE TOUCHED
- `installer-core/src/driver.rs`: Add DE detection logic.
- `installer-cli/src/software_catalog.rs`: Tag artifacts with DE-specific metadata.
- `installer-cli/src/software_tiers.rs`: Update the menu selection to filter based on detected DE.

## 🏗️ METHODOLOGY
- **Environment Probing**: Use `XDG_CURRENT_DESKTOP`, `DESKTOP_SESSION`, and installed binaries to detect the DE.
- **Dynamic Curation**: The software list for "KDE" should prioritize Qt-based tools, while "GNOME" should prioritize GTK-based tools.
- **Wayland-First**: If Wayland is detected, prioritize native Wayland tools (like `Foot` or `Sway`).

## ✅ DELIVERABLES
- A working DE detection phase.
- Software selection menus that dynamically adapt to the user's environment.
- A "Wayland-Optimized" flag in the final summary.
