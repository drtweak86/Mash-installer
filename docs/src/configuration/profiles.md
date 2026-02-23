# Installation Profiles

MASH's three profiles control which package groups are installed.

## Minimal

Installs only the essential tools required for basic system operation:

- `curl`, `git`, `tar`, `wget`
- Core system utilities

Use for headless servers or minimal containers.

```bash
mash-setup --profile minimal
```

## Developer (Default)

The full forge suite for a productive development workstation:

- Everything in Minimal
- **Rust toolchain** (rustup, cargo, clippy, rustfmt)
- **Docker** + docker-compose
- **Zsh** + Oh-My-Zsh + Starship prompt
- **Kitty** terminal (forge-tuned config)
- **eza** with aliases (`~/.eza_aliases`)
- Common build tools (make, gcc/clang, cmake)
- **Raspberry Pi 4B tuning** (auto-detected)

```bash
mash-setup --profile dev
```

## Archive (Full)

Complete system rebuild — everything in Developer plus:

- **Wallpaper library** (Wallhaven, Pexels, Pixabay — API keys optional)
- **AI Spirits** (Claude Code, Gemini CLI, Mistral Vibe)
- **Powerlevel10k** (if `--enable-p10k` passed)
- Retro BBC-style wallpaper category downloads

```bash
mash-setup --profile full
```

## Comparing Profiles

| Feature                | Minimal | Developer | Archive |
|------------------------|---------|-----------|---------|
| Core tools             | ✅      | ✅        | ✅      |
| Rust toolchain         | ❌      | ✅        | ✅      |
| Docker                 | ❌      | ✅        | ✅      |
| Shell polish           | ❌      | ✅        | ✅      |
| Pi 4B tuning           | ❌      | ✅ (auto) | ✅ (auto)|
| Wallpapers             | ❌      | ❌        | ✅      |
| AI Spirits             | ❌      | ❌        | ✅      |
