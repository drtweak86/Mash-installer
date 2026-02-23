# First Run

## Launch the TUI

```bash
mash-setup
```

The cyberpunk cockpit appears: a 4-pane interface showing real-time CPU, RAM, network, and log stream.

## TUI Controls

| Key        | Action                                        |
|------------|-----------------------------------------------|
| `↑` / `↓` | Navigate menu items                           |
| `Space`    | Toggle module selection (multi-select menus)  |
| `Enter`    | Confirm selection / advance screen            |
| `Esc`      | Return to previous screen                     |
| `q`        | Abort and return to shell                     |

## Installation Flow

1. **Distribution detection** — MASH auto-detects your distro and arch
2. **Profile selection** — choose Minimal / Developer / Archive (or pass `--profile`)
3. **Software tier selection** — toggle individual feature groups
4. **Confirmation** — review your selections
5. **Installation** — phases run with real-time progress in the TUI
6. **Completion** — post-install notes with next steps

## Post-Install Steps

After installation completes:

```bash
# Verify everything is working
mash-setup doctor

# Log out and back in for Docker group membership
# (or: newgrp docker)

# Check your new shell
exec zsh
```

## Config Location

MASH creates a config file on first run:

```
~/.config/mash/mash.toml
```

View it with:

```bash
mash-setup config show
```

Reset to defaults:

```bash
mash-setup config init
```
