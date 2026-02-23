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

The `wallpaper-downloader` standalone binary downloads a curated BBC-style retro wallpaper
set (~6000 images) across 8 categories (retro, games, anime, dc, marvel, judge_dredd,
star_wars, cyberpunk):

```bash
wallpaper-downloader                      # download all categories
wallpaper-downloader --category retro     # single category
wallpaper-downloader --limit 500          # cap at 500 images
```

This binary is separate from the main installer. It reads `MASH_WALLHAVEN_KEY` for its
Wallhaven API key — the same env var used by `mash-setup`. Output goes to
`~/Pictures/RetroWallpapers/` (user-local, no sudo required).

It includes SHA256 deduplication — re-running it skips already-downloaded images.

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
