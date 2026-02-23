# ⚒️ MASH MASTER MINING ROADMAP

*A Strategic Blueprint for the Next Evolution of the Forge*

Following the curation of the Software Grimoire and the establishment of the Bard's recommendations, this roadmap outlines the technical path for the next set of **Mining Projects**. All excavation must adhere to **MINING_GOVERNANCE**.

---

## 🏗️ SHAFT-A: SOFTWARE CATALOG EVOLUTION
**Objective**: Integrate the full Software Grimoire into the installer's logic.

### **EX_A01: Tier System Overhaul**
- **Action**: Expand the `Tier` enum in `installer-core` and `installer-cli` to support the full range: `S, A, B, C, D, F`.
- **Action**: Update the `SoftwareOption` struct to include these new tiers.
- **Action**: Refactor `software_catalog.rs` to match the 5-entry-per-category structure defined in the Software Grimoire.

### **EX_A02: "Bard Recommends" (The Works / Install All)**
- **Action**: Create a new installation profile/preset: `Bard Recommends`.
- **Action**: This profile will automatically select **EVERYTHING** from the Dev Environment and all **S-Tier** artifacts from every category.
- **Action**: Add a shortcut in the main menu: `[B] Bard Recommends (The Works / Install All)`.
- **Action**: Include the complete theme and wallpaper pack in this bundle by default.

### **EX_A03: Pi 4B Auto-Detection & Tuning**
- **Action**: Ensure that "Bard Recommends" detects a Raspberry Pi 4B during the install.
- **Action**: Automatically apply Pi-specific GPU tuning for Kitty.
- **Action**: Enable `pi4b_hdd` module if an external drive is detected, ensuring swap lives off the SD card.
- **Action**: Add a "Pi First-Class Citizen" banner to the TUI if a Pi is detected.

### **EX_A04: Distro-Specific Package Mapping**
- **Action**: Audit all S-tier and A-tier artifacts from the Grimoire for package name differences across Debian, Arch, and Fedora.
- **Action**: Update `installer-debian`, `installer-arch`, and `installer-fedora` drivers with these mappings.
- **Action**: Ensure "Bard Recommends" uses these drivers to install the correct package names on each system.

---

## 🎨 SHAFT-B: THEME & DESKTOP ENVIRONMENT EXPANSION
**Objective**: Make the visual experience more flexible and universal.

### **EX_B01: Expansive DE Selection**
- **Action**: Add a new step in the installer for "Desktop Environment / Window Manager selection".
- **Action**: Support options for: `i3-gaps (default)`, `Sway (Wayland)`, `KDE Plasma`, `GNOME`, and `Skip (Keep Current)`.
- **Action**: Implement a "Skip Theme" option for users who only want the software without the visual changes.

### **EX_B02: Universal i3 and Kitty Themes**
- **Action**: Refactor the theme installation logic to be "DE-agnostic" where possible.
- **Action**: Ensure the `kitty.conf` and `i3/config` (or `sway/config`) are applied even if a full DE like KDE or GNOME is present.
- **Action**: For KDE/GNOME, provide a "MASH Integration" script that sets Kitty as the default terminal and applies matching color schemes to the DE's native terminal/window borders.

---

## 🌌 SHAFT-C: WAYLAND OPTIMIZATION & FIXES
**Objective**: Ensure a smooth experience on modern display protocols.

### **EX_C01: Automated Wayland Detection**
- **Action**: Add logic to detect if the user is running a Wayland session or has selected a Wayland-native DE (like Sway).
- **Action**: Apply `MOZ_ENABLE_WAYLAND=1` and other environment variables for tool compatibility (Firefox, LibreWolf, etc.).

### **EX_C02: Optimization Patches**
- **Action**: For Wayland DEs, automatically swap `i3` for `Sway` in the theme scripts.
- **Action**: Apply XWayland scaling fixes for high-DPI displays.
- **Action**: Ensure terminal transparency and blur effects are correctly configured for Wayland compositors (Hyprland, Sway).

---

## 🏗️ SHAFT-D: ARCHITECTURE FLEXIBILITY (DOWNLOADER FIX)
**Objective**: Refactor architecture detection to allow manual override, skip, or cancellation.

### **EX_D01: Interactive Architecture Probe**
- **Action**: Add a "Manual Override" hook to the platform detection logic.
- **Action**: Provide a TUI prompt: `[C] Cancel, [M] Manual Entry, [S] Skip (Default Detect)`.

---

## 🔄 SHAFT-E: INSTALLER FLOW OPTIMIZATION
**Objective**: Streamline the installation sequence to reduce friction and improve user pacing.

### **EX_E01: The "Bard's Welcome" Refactor**
- **Action**: Move `sudo` and `doctor` checks to the absolute beginning to prevent mid-install interruptions.
- **Action**: Reorder menus: `Platform Detect -> Driver -> Modules -> Software Tiers -> Summary`.

---

## 🧠 SHAFT-F: DISTRO-CENTRIC INTELLIGENCE
**Objective**: Implement deep flavor/DE detection to provide targeted software recommendations.

### **EX_F01: Desktop Environment (DE) Probe**
- **Action**: Detect active environment (KDE, GNOME, XFCE) via environment variables and installed binaries.
- **Action**: Filter "S-Tier" recommendations based on DE (e.g., recommend `Gwenview` for KDE and `Eye of GNOME` for GNOME).

---

## 🔗 SHAFT-G: MASH ECOSYSTEM INTEGRATION
**Objective**: Ensure seamless callability by the parent `MASH` project while maintaining independent standalone functionality.

### **EX_G01: The "Called-by-MASH" Handshake**
- **Action**: Add a `--called-by-mash` flag to `installer-cli`.
- **Action**: When active, this flag suppresses certain "welcome" banners and switches logging to a format easily parsed by the parent `MASH` process (e.g., JSON status updates).
- **Action**: Implement a "Headless Summary" that provides a final machine-readable report of the installation results.

### **EX_G02: Environment-Driven Configuration**
- **Action**: Allow `MASH` to pass the entire software selection and profile via environment variables or a temporary JSON file.
- **Action**: Ensure this mechanism respects the "Independence" rule—the installer remains 100% functional via TUI when run without these triggers.

### **EX_G03: Universal Binary & Update Protocol**
- **Action**: Refactor the release process to produce a single, portable binary for each architecture (x86_64, aarch64).
- **Action**: Implement a version-check command (`mash-setup version --check`) that `MASH` can use to determine if the installer needs updating.

---

## 📜 EXCAVATION PROTOCOL

1.  **Pre-Dig**: Consult `maps-explored.md` and `maps.md` for historical context and active conflicts.
2.  **Active Dig**: For each Excavation Task (`EX_...`), update `maps.md` to reflect the active directory and shaft status.
3.  **Refinement**: Perform atomic commits (KCS) and update code comments (ABD).
4.  **Extraction**: Complete the task and update `maps-explored.md` with the newly mined artifacts.
5.  **Chronicle**: Record the completion of the shaft in `HISTORY.md`.

---

*“The forge never cools. We refine, we improve, we evolve.”*
*— Bard*
