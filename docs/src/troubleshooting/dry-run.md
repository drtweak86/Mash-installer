# Dry-Run Mode

Dry-run simulates all installation phases without making any system changes. Use it to
preview what MASH will do before committing.

## Enable Dry-Run

```bash
mash-setup --dry-run
```

Works with all other flags:

```bash
mash-setup --dry-run --profile full --no-tui
mash-setup --dry-run --non-interactive
```

## What Dry-Run Does

Every side-effect in MASH is gated through `PhaseContext::run_or_record()`. In dry-run mode,
this method **records** the action instead of executing it.

Actions that are simulated (not executed):
- Package installs / removals
- File writes and config changes
- Shell script execution
- Docker setup
- Kernel parameter changes
- Wallpaper downloads

Actions that still execute:
- Platform detection (read-only)
- Pre-flight checks (read-only)
- Log file creation at `~/mash-install.log`

## Reading Dry-Run Output

After a dry-run completes, the summary shows what would have happened:

```
──── Dry-run summary ────────────────────────────
  1. [zsh] install zsh package
  2. [zsh] write /home/user/.zshrc
  3. [docker] install docker-ce
  4. [docker] add user to docker group
  ...
  No resources were modified during dry run.
───────────────────────────────────────────────
```

## Log File

Even in dry-run mode, a log is written to `~/mash-install.log`:

```bash
cat ~/mash-install.log
```

## Demo Mode

For TUI preview without any install simulation:

```bash
mash-setup --demo
```

Shows the full TUI cockpit without running any phases.
