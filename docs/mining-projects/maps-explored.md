# Mining Projects – Maps Explored
> Historical ledger of completed shafts and sessions with technical diff analysis.

---

## SHAFT A <COMPLETED> ✅

### Summary
Strategic reconnaissance of the forge. Audited architecture, identified shell boundaries, and created the comprehensive strategic plan for retro integration.

### Technical Analysis
- **Architecture**: ~95% pure Rust core. 
- **Shell Boundaries**: Font installation, Docker setup, and Rust toolchain identified as hard shell-out points.
- **Improvements**: Abstracted filesystem operations via `SystemOps` trait.

### Files Touched
- `installer-core/src/system_ops.rs` (new)
- `installer-core/src/phase_context.rs` (updated)
- `installer-core/src/installation_report.rs` (updated)
- `installer-core/src/orchestrator.rs` (updated)
- `installer-core/src/lib.rs` (exports updated)

### Verification
- ✅ 82 tests passing.
- ✅ Clippy & Fmt clean.
- ✅ Cross-compilation verified for x86_64 and aarch64.

---

## SHAFT B <NOT_COMPLETED> ⏳

### Summary
Integration of the BBC/UNIX retro-futuristic theme (i3-gaps + Kitty) and wallpaper downloader into the TUI flow.

### Technical Analysis
- **Design**: Reordered the TUI flow: Detection → Profile → Options → Themes → Software → Install.
- **Aesthetics**: Preparation for i3-gaps and Kitty integration.

### Files Touched
- `installer-core/src/software_tiers.rs` (updated)
- `installer-cli/src/tui/menus.rs` (updated)
- `installer-cli/src/tui/app.rs` (updated)
- `docs/mining-projects/shaftb.md` (new)

### Status
- ⏳ Integration Pending...

---

## SHAFT C <NOT_COMPLETED> ⏳

### Summary
Aesthetic transformation from cyberpunk neon to BBC Micro/UNIX terminal (1984 meets 2026).

### Technical Analysis (Plan)
- **Palette Shift**: Replace neon cyans with Green/Amber phosphor (#00FF00, #FFBF00).
- **Layout Refactor**: Move from 4-pane layout to a single-pane scrolling UNIX terminal.
- **UI Interaction**: Transition from visual bars to command-line prompts (> _).

### Files to be Touched
- `installer-cli/src/tui/theme.rs` (palette refactor)
- `installer-cli/src/tui/render.rs` (layout refactor)
- `installer-cli/src/tui/menus.rs` (interaction refactor)
- `installer-core/src/fonts.rs` (typography enforcement)

### Status
- ⏳ Planned & Charted.

---
**Last Updated**: 2026-02-21
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
