# Environment Variables

MASH reads the following environment variables at startup.

## Wallpaper API Keys

| Variable             | Provider  | Required |
|----------------------|-----------|----------|
| `MASH_WALLHAVEN_KEY` | Wallhaven | Optional |
| `MASH_PEXELS_KEY`    | Pexels    | Optional |
| `MASH_PIXABAY_KEY`   | Pixabay   | Optional |

Without these keys, MASH skips the corresponding wallpaper source. The installer still
completes successfully — wallpapers are optional.

Set them before running:

```bash
export MASH_WALLHAVEN_KEY=your_key_here
export MASH_PEXELS_KEY=your_key_here
export MASH_PIXABAY_KEY=your_key_here
mash-setup
```

For persistent configuration, add to your shell profile (`~/.bashrc`, `~/.zshrc`):

```bash
export MASH_WALLHAVEN_KEY="your_key_here"
```

## Checking Key Status

```bash
mash-setup status
```

Reports PASS/WARN for each wallpaper provider key without making network calls.

```bash
mash-setup doctor
```

Includes key status plus setup URLs for registration.

## Logging

| Variable    | Effect                          |
|-------------|----------------------------------|
| `RUST_LOG`  | Override tracing log level (e.g., `RUST_LOG=debug mash-setup`) |

## Architecture Override

Pass `--arch` flag (not an env var) to force a specific target architecture:

```bash
mash-setup --arch aarch64
```
