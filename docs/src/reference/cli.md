# CLI Flags & Subcommands

## Synopsis

```
mash-setup [FLAGS] [SUBCOMMAND]
```

## Global Flags

| Flag                   | Description                                            |
|------------------------|--------------------------------------------------------|
| `--dry-run`            | Simulate all phases — no packages installed or files written |
| `--no-tui`             | Use stdio progress bars instead of the Ratatui TUI     |
| `--non-interactive`    | Fully scripted — no prompts, use defaults              |
| `--profile <LEVEL>`    | Profile: `minimal` \| `dev` \| `full` (skips menu)    |
| `--arch <ARCH>`        | Force architecture: `x86_64` \| `aarch64`             |
| `--staging-dir <PATH>` | Override default staging directory                     |
| `--continue-on-error`  | Continue installation if a phase fails                 |
| `--verbose`, `-v`      | Enable debug-level logging                             |
| `--demo`               | Preview TUI without running any phases                 |
| `--enable-p10k`        | Install Powerlevel10k (disabled by default)            |
| `--version`            | Print version and exit                                 |
| `--help`               | Print help and exit                                    |

## Subcommands

### `mash-setup status [--format <FORMAT>]`

Instant system overview — no network calls.

| Flag              | Values          | Default   |
|-------------------|-----------------|-----------|
| `--format`        | `pretty` / `json` | `pretty`  |

### `mash-setup doctor [--format <FORMAT>]`

Full diagnostic including connectivity checks (~15s).

| Flag              | Values          | Default   |
|-------------------|-----------------|-----------|
| `--format`        | `pretty` / `json` | `pretty`  |

### `mash-setup catalog [--json]`

Browse the software catalog.

| Flag     | Description              |
|----------|--------------------------|
| `--json` | Output in JSON format    |

### `mash-setup config <ACTION>`

Manage the MASH config file.

| Action | Description                                  |
|--------|----------------------------------------------|
| `init` | Create default config at `~/.config/mash/mash.toml` |
| `show` | Print current configuration                  |

## Examples

```bash
# Interactive TUI (default)
mash-setup

# Headless developer install
mash-setup --no-tui --profile dev

# Non-interactive full install (CI/automation)
mash-setup --non-interactive --profile full

# Dry-run preview
mash-setup --dry-run --profile full

# Quick health check
mash-setup status

# Full diagnostic
mash-setup doctor

# JSON diagnostic for scripts
mash-setup doctor --format json | jq '.preflight'

# View software catalog
mash-setup catalog
```
