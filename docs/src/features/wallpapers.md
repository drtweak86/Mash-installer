# Wallpaper Downloader

MASH downloads a curated library of retro and cyberpunk wallpapers from three sources.

## Sources

| Source    | Variable             | Auth required | Register at                             |
|-----------|----------------------|---------------|-----------------------------------------|
| Wallhaven | `MASH_WALLHAVEN_KEY` | Optional key  | https://wallhaven.cc/settings/account   |
| Pexels    | `MASH_PEXELS_KEY`    | Required key  | https://www.pexels.com/api/new/         |
| Pixabay   | `MASH_PIXABAY_KEY`   | Required key  | https://pixabay.com/api/docs/#api_key   |

Sources without keys are skipped. The installer succeeds regardless.

## Download Location

```
/usr/share/backgrounds/retro/
```

Up to 3 concurrent connections for efficient download.

## Retro BBC Category

The `wallpaper-downloader` crate also supports a curated BBC-style retro wallpaper set
(~6000 images) that does not require an API key:

```bash
wallpaper-downloader
```

This runs the standalone downloader binary (separate from the main installer).

## Configuration

Set API keys as environment variables:

```bash
export MASH_WALLHAVEN_KEY=your_key
export MASH_PEXELS_KEY=your_key
export MASH_PIXABAY_KEY=your_key
mash-setup
```

## Checking Key Status

```bash
mash-setup status
# or
mash-setup doctor
```

Both show PASS/WARN per provider. Doctor mode also displays setup URLs for missing keys.

## Wallpaper Phase Scope

The wallpaper phase runs as part of the **Archive** profile. In Developer or Minimal
profiles, it is skipped unless you explicitly toggle it on in the software tier menu.
