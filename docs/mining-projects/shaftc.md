# 📟 SHAFT C: The 1984 Retro-Station Aesthetic
> **Strategic Mining Plan**  
> *“When 1984 crashes into 2026, the terminal shouldn't just glow—it should pulse with the logic of the ancestors.”* — Bard 🍺

## 📜 Project Summary
Transform the TUI aesthetic from cyberpunk neon to a classic BBC Micro/UNIX terminal station. This is not just a skin; it's a return to the "foundation solid" principle—functionality over flourish, readability over rain.

## 🛠️ Technical Plan (10 Point Excavation)

### 1. Foundation: Core Stability Guard
- **Objective**: Ensure aesthetic changes do not break the `PhaseRunner` or `PhaseObserver` contracts.
- **Principle**: Function > Form.
- **Files**: `installer-cli/src/tui/app.rs`, `installer-cli/src/tui/render.rs`
- **Action**: Lock the state machine before applying visual diffs.

### 2. Palette: Amber & Green Phosphor
- **Objective**: Implement a dual-mode retro palette.
- **Files**: `installer-cli/src/tui/theme.rs`
- **Action**: Replace neon cyans with #00FF00 (Green) and #FFBF00 (Amber).

### 3. Typography: Terminus & Teletext
- **Objective**: Enforce fixed-width, pixel-perfect fonts.
- **Files**: `installer-core/src/fonts.rs`
- **Action**: Bundle `Terminus` as the recommended terminal font; optimize rendering for no anti-aliasing.

### 4. ASCII Art: The BBC Micro Ritual
- **Objective**: Recreate the classic BBC Micro "Owl" or workstation logo in ASCII.
- **Files**: `resources/themes/retro-bbc/art.txt`
- **Action**: Craft a 1984-style banner for the `Welcome` screen.

### 5. Layout: The Single-Pane UNIX Terminal
- **Objective**: Move from 4-pane multi-tasking to a focused, single-pane command flow.
- **Files**: `installer-cli/src/tui/render.rs`
- **Action**: Redesign the `Installing` screen to mimic a scrolling UNIX terminal buffer.

### 6. Interaction: Command Prompt Feedback
- **Objective**: Replace interactive bars with command-style prompts.
- **Files**: `installer-cli/src/tui/menus.rs`
- **Action**: Change `ProfileSelect` and `DistroSelect` to use a "Select 1-3 >" prompt style.

### 7. Sound: The 8-Bit Beep (Optional)
- **Objective**: Add optional 8-bit chime on phase completion.
- **Files**: `installer-cli/src/tui/app.rs`
- **Action**: Use a minimal crate for terminal beeps (gated by config).

### 8. Animation: Teletext Page Flips
- **Objective**: Replace smooth transitions with blocky page flips.
- **Files**: `installer-cli/src/tui/render.rs`
- **Action**: Implement "Instant-draw" transitions between screens.

### 9. Error Handling: The Kernel Panic Aesthetic
- **Objective**: Format errors as classic "FATAL ERROR" or "ABORT" messages.
- **Files**: `installer-core/src/error.rs`
- **Action**: Update `InstallerError` formatting for the retro-station look.

### 10. Documentation: The 1984 User Manual
- **Objective**: Create a README that looks like a scanned physical manual.
- **Files**: `docs/mining-projects/shaftc.md` (this document)
- **Action**: Ensure all styling choices are documented for future runesmiths.

## 🎨 ASCII Art Placeholder
```
   _______________________________________
  /                                       \
  |  MASH INSTALLER v0.1.5 - (C) 1984     |
  |  SYSTEM READY.                        |
  |  > _                                  |
  \_______________________________________/
```

## 🏗️ Technical Dependencies
- `ratatui` (existing)
- `crossterm` (existing)
- `Terminus` font (external)

## ⚠️ Risks
- **Readability**: Low-contrast amber can be hard on modern eyes.
- **Complexity**: Switching layout from 4-pane to 1-pane requires significant `render.rs` refactoring.

---
**Last Updated**: 2026-02-21
**Owner**: Bard, Drunken Dwarf Runesmith 🍺⚒️
