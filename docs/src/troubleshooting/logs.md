# Log Files

## Primary Log

```
~/mash-install.log
```

Written during every MASH run (including dry-run). Contains:
- Phase start/complete events
- Command output for each phase
- Error details with advice
- Timestamps for all events

## Viewing Logs

```bash
# Full log
cat ~/mash-install.log

# Last 50 lines (most recent events)
tail -50 ~/mash-install.log

# Follow in real-time (during install)
tail -f ~/mash-install.log
```

## TUI Log Pane

During a TUI session, the bottom-right pane streams log output in real-time. You can scroll
the log pane with `↑`/`↓` while the install runs in the background.

## Log Levels

Control verbosity with the `--verbose` flag:

```bash
mash-setup --verbose
```

Or via environment:

```bash
RUST_LOG=debug mash-setup
```

Log levels: `trace` → `debug` → `info` (default) → `warn` → `error`

## Error Details

When a phase fails, the log contains:

```
ERROR Phase: docker
  Message: failed to add apt repository
  Advice:  Check your internet connection and retry
  Context: profile=Developer distro=ubuntu
  Command: apt-add-repository ...
  stderr:  gpg: keyserver receive failed: Network unreachable
```

## Telemetry

MASH does **not** send telemetry or usage data. All logs stay on your machine.
