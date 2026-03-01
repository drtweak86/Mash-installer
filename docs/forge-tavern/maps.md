# ⚒️ The Miner's Active Maps: Current Shaft
> *"The blade is hardened. The crown is on the table."* — Bard 🍺

---

## 🏗️ SHAFT I: Software Catalog & Installation Flow Overhaul — ACTIVE
> *Branch*: `work-shaft-i-catalog` (to be created)
> *Risk*: MEDIUM (catalog restructuring, UI changes)
> *Reward*: HIGH (significantly improved user experience, logical organization)
> *Status*: 🔨 IMPLEMENTATION IN PROGRESS (Phase 1 Complete)

### OVERVIEW
Complete reorganization of MASH software catalog and installation flow to create a more logical, user-friendly, and efficient system with curated S-tier applications, multiple installation modes, and optimized dependency handling.

### PHASE 1: Software Catalog Curation ✅ COMPLETE
**Objective**: Create comprehensive software catalog with S-tier applications
- [x] Define Category Structure (10 main categories)
- [x] Curate S-Tier Applications (5 per category)
- [x] Include All Programming Languages
- [x] Create TOML Catalog Structure (`s-tier_catalog.toml`, `full_catalog.toml`, `programming_languages.toml`)
**Status**: ✅ COMPLETE 2026-03-01

### PHASE 2: Installation Modes
**Objective**: Implement Manual, Auto, and Bard's Recommendations modes
- [ ] Manual Mode: Individual program selection with search/filter
- [ ] Auto Mode: One-click category installation
- [ ] Bard's Recommendations: Opinionated S-tier selection
**Status**: ⏳ PENDING

### PHASE 3: Menu Restructuring
**Objective**: Organize software into logical categories
- [ ] Category/Subcategory Hierarchy implementation
- [ ] UI Implementation in `installer-cli`
**Status**: ⏳ PENDING

### PHASE 4: Installation Flow Optimization
**Objective**: Ensure proper installation order
- [ ] Dependency Resolution (ccache/sccache before heavy builds)
- [ ] Parallel Installation implementation
**Status**: ⏳ PENDING

### PHASE 5: UI Integration
**Objective**: Connect catalog to installer UI
- [ ] Software Selection Screens
- [ ] Visual Progress Tracking
**Status**: ⏳ PENDING

---

## 🏗️ SHAFT S: THE ALL-SEEING EYE (Auto-Detection & System Profiling) — PLANNING COMPLETE
> *Branch*: `work-shaft-s-profiler` (to be created)
> *Risk*: MEDIUM (complex system detection)
> *Reward*: HIGH (intelligent installer, context-aware decisions)
> *Status*: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING

### OVERVIEW
Implementation of a comprehensive auto-detection system that builds a `SystemProfile` of the machine's hardware, OS, and storage landscape.

### PHASE 1: System Profile Model
- **File**: `docs/mining-projects/shaft-s/EX_S01_System_Profile_Model.md`
- **Objective**: Define `SystemProfile` and sub-models with Serde support.
- **Status**: ⏳ PENDING

### PHASE 2: Hardware & OS Detection
- **File**: `docs/mining-projects/shaft-s/EX_S02_Hardware_OS_Detection.md`
- **Objective**: Scry CPU, RAM, Distro, and identify Raspberry Pi vs PC.
- **Status**: ⏳ PENDING

### PHASE 3: Storage & Filesystem Audit
- **File**: `docs/mining-projects/shaft-s/EX_S03_Storage_Analysis.md`
- **Objective**: Map block devices, partitions, and deep Btrfs runes.
- **Status**: ⏳ PENDING

### PHASE 4: TUI Summary & Persistence
- **File**: `docs/mining-projects/shaft-s/EX_S04_TUI_Display_Persistence.md`
- **Objective**: Visual summary in TUI and save to `system_profile.json`.
- **Status**: ⏳ PENDING

---

## 🏗️ SHAFT T: THE BARD'S WISDOM (Intelligent Advice Engine) — PLANNING COMPLETE
> *Branch*: `work-shaft-t-advice` (to be created)
> *Risk*: LOW (pure logic engine)
> *Reward*: MAXIMUM (user-friendly, expert-level system optimization)
> *Status*: ✅ PLANNING COMPLETE | ⏳ IMPLEMENTATION PENDING

### OVERVIEW
Implementation of an intelligent "Advice Engine" that translates the `SystemProfile` into actionable wisdom, warnings, and performance tuning for the smith.

### PHASE 1: Advice Engine Core
- **File**: `docs/mining-projects/shaft-t/EX_T01_Advice_Engine_Core.md`
- **Objective**: Define `AdviceEngine` and the `Rule` trait.
- **Status**: ⏳ PENDING

### PHASE 2: Hardware & Resource Wisdom
- **File**: `docs/mining-projects/shaft-s/EX_T02_Hardware_Wisdom.md`
- **Objective**: Rules for RAM, CPU, Platform (Pi/Laptop), and thermals.
- **Status**: ⏳ PENDING

### PHASE 3: Storage & Filesystem Wisdom
- **File**: `docs/mining-projects/shaft-t/EX_T03_Storage_Wisdom.md`
- **Objective**: Rules for Btrfs, SD Cards, and Workspace relocation.
- **Status**: ⏳ PENDING

### PHASE 4: Software Stability & Version Wisdom
- **File**: `docs/mining-projects/shaft-t/EX_T04_Software_Stability_Wisdom.md`
- **Objective**: Rules for ARM64 Node, Firmware hints, and Session stability.
- **Status**: ⏳ PENDING

---

**Next Review**: 2026-03-05 (Implementation kickoff)
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
**Last Updated**: 2026-03-01
