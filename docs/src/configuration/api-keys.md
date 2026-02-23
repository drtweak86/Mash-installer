# Wallpaper API Keys

MASH can download wallpapers from three sources. API keys are **optional** — without them,
MASH simply skips those sources.

## Providers

| Provider  | Variable             | Register at                             |
|-----------|----------------------|-----------------------------------------|
| Wallhaven | `MASH_WALLHAVEN_KEY` | https://wallhaven.cc/settings/account   |
| Pexels    | `MASH_PEXELS_KEY`    | https://www.pexels.com/api/new/         |
| Pixabay   | `MASH_PIXABAY_KEY`   | https://pixabay.com/api/docs/#api_key   |

## Getting Keys

### Wallhaven
1. Create an account at wallhaven.cc
2. Go to **Settings → Account**
3. Copy your API key

### Pexels
1. Create a Pexels account
2. Go to **Image & Video API → Your API Key**
3. Copy the key (free tier is sufficient)

### Pixabay
1. Create a Pixabay account
2. Go to **API Docs**
3. Your key appears after login

## Setting Keys

```bash
# Temporary (current session only)
export MASH_WALLHAVEN_KEY=your_key
export MASH_PEXELS_KEY=your_key
export MASH_PIXABAY_KEY=your_key

# Permanent (add to ~/.zshrc or ~/.bashrc)
echo 'export MASH_WALLHAVEN_KEY=your_key' >> ~/.zshrc
```

## Verifying Keys

```bash
mash-setup status
```

Output:
```
── Wallpaper API keys ──
  Wallhaven    PASS
  Pexels       WARN (not set)
  Pixabay      WARN (not set)
```

```bash
mash-setup doctor
```

Shows PASS/WARN with setup URLs for any missing keys.

## Without Keys

MASH works fully without API keys. Wallpaper sources that require a key are skipped;
the `retro-bbc` category downloads BBC-style images without authentication.
