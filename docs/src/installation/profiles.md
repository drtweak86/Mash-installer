# Profile Selection

MASH installs packages in three tiers. During the TUI, you choose a profile:

## Profiles

| Profile       | `--profile` flag | Description                                             |
|---------------|------------------|---------------------------------------------------------|
| **Minimal**   | `minimal`        | Core packages only — git, curl, essential tools         |
| **Developer** | `dev`            | Full forge suite — Rust, Docker, shell polish           |
| **Archive**   | `full`           | Complete system + wallpapers + Pi tuning + everything   |

## Choosing a Profile

**Minimal** is ideal for:
- Servers or containers where you only need the essentials
- Initial bootstraps where you plan to add more later

**Developer** (default for non-interactive mode) is ideal for:
- Daily development workstations
- Raspberry Pi 4B development rigs
- Includes: Rust toolchain, Docker, Zsh + Oh-My-Zsh, Starship, Kitty, eza

**Archive** is ideal for:
- Full workstation rebuild / disaster recovery
- Includes everything in Developer plus wallpapers, AI Spirits, optional Powerlevel10k

## Skipping the Menu

Pass `--profile` to bypass the interactive menu:

```bash
mash-setup --profile dev
mash-setup --profile minimal
mash-setup --profile full
```

## Software Tier Menu

Within the TUI, after selecting a profile you can individually toggle software tier groups:
- **Core Tools** — always installed with any profile
- **Shell Polish** — Zsh, Starship, Kitty, eza aliases
- **Developer Tools** — Rust, Docker, build tools
- **AI Spirits** — Claude Code, Gemini CLI, Mistral Vibe (npm-based)
- **Wallpapers** — curated retro/cyberpunk image library
