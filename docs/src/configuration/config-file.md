# Config File

MASH stores its configuration at:

```
~/.config/mash/mash.toml
```

## Creating the Config

Initialize with defaults:

```bash
mash-setup config init
```

This creates `~/.config/mash/mash.toml` with sensible defaults. It will not overwrite an
existing file.

## Viewing the Config

```bash
mash-setup config show
```

Prints the current configuration in TOML format.

## Config Sections

The config file controls logging, interaction behaviour, and feature flags.
MASH is designed to work well with defaults — you rarely need to edit it manually.

### Logging

```toml
[logging]
level = "info"   # trace | debug | info | warn | error
```

### Interaction

```toml
[interaction]
interactive = true   # set false for scripted runs
```

## Status Check

```bash
mash-setup status
```

Reports whether the config is `loaded`, `missing`, or `invalid` along with the path.

## Resetting

To reset to defaults, delete the config file and re-initialize:

```bash
rm ~/.config/mash/mash.toml
mash-setup config init
```
