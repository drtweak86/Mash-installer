# TUI Interface

MASH ships with a full-screen cyberpunk Ratatui cockpit. It's the default interface — no flags needed.

```bash
mash-setup
```

## Layout

The TUI has 4 panes:

```
┌───────────────────────┬───────────────────────┐
│  BBS Message / Title  │  System Telemetry     │
│                       │  CPU ████░░░░ 42%      │
│                       │  RAM ███████░ 3.1 GiB  │
│                       │  Net ↑ 1.2 MB/s        │
├───────────────────────┴───────────────────────┤
│  Install Progress / Phase Output              │
│  [✓] Phase: zsh — complete                    │
│  [→] Phase: docker — running                  │
├───────────────────────────────────────────────┤
│  Status Bar / Controls                        │
└───────────────────────────────────────────────┘
```

## Controls

| Key        | Action                                        |
|------------|-----------------------------------------------|
| `↑` / `↓` | Navigate menu items                           |
| `Space`    | Toggle selection (multi-select menus)         |
| `Enter`    | Confirm / advance to next screen              |
| `Esc`      | Return to previous screen                     |
| `q`        | Quit and return to shell                      |

## BBS Messages

The top-left pane displays rotating messages from the Forge Tavern BBS — 68 messages covering
Rust idioms, dwarven wisdom, network sorcery, and package alchemy.

## Skipping the TUI

For SSH sessions or minimal terminals:

```bash
mash-setup --no-tui
```

Uses indicatif progress bars instead — all the same phases and features.

## Demo Mode

Preview the TUI without installing anything:

```bash
mash-setup --demo
```
