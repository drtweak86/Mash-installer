# Shell Polish

MASH installs and configures a complete shell environment.

## Components

| Component    | What MASH Does                                           |
|--------------|----------------------------------------------------------|
| Zsh          | Installs Zsh, sets as default shell                      |
| Oh-My-Zsh    | Installs the framework + MASH-curated plugin set         |
| Starship     | Installs + writes `~/.config/starship.toml` (retro theme)|
| Kitty        | Installs + writes `~/.config/kitty/kitty.conf`           |
| eza aliases  | Installs eza + writes `~/.eza_aliases` (sourced from Zsh)|
| Powerlevel10k| Optional — pass `--enable-p10k` (skipped by default)    |

## eza Aliases

Modern `ls` replacements are sourced from `~/.eza_aliases`:

```bash
alias ls='eza --icons'
alias ll='eza -la --icons --git'
alias lt='eza --tree --icons --level=2'
alias la='eza -a --icons'
```

These are automatically sourced from `~/.zshrc` after install.

## Starship Prompt

MASH installs a retro-themed Starship config with:
- Git branch and status glyphs
- Rust version indicator (when in a Rust project)
- Error exit code display
- Compact prompt designed for small Pi terminal windows

## Kitty Terminal

The forge-tuned Kitty config includes:
- Custom font and font size optimised for development
- Retro color scheme matching the MASH theme

## Enabling Powerlevel10k

```bash
mash-setup --enable-p10k
```

Installs Powerlevel10k as the Zsh theme. Note: requires a Nerd Font installed in your terminal.

## Post-Install

After the shell phase, start the new shell:

```bash
exec zsh
```

Or log out and back in.
