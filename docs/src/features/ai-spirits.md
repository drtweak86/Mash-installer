# AI Spirits

MASH can install AI coding assistants as part of the Archive (full) profile.

## Available Spirits

| Spirit        | Package                      | Description                              |
|---------------|------------------------------|------------------------------------------|
| Claude Code   | `@anthropic-ai/claude-code`  | Anthropic's CLI for Claude               |
| Gemini CLI    | `@google/gemini-cli`         | Google Gemini in your terminal           |
| Mistral Vibe  | `@mistral-ai/vibe`           | Mistral AI CLI assistant                 |

## Installation

Select **AI Spirits** in the software tier menu during the TUI, or use the Archive profile:

```bash
mash-setup --profile full
```

All spirits are installed via `npm` (Node.js is installed as a prerequisite).

## MCP Server Injection

When MASH detects a Claude Desktop, Zed, Cursor, or VS Code configuration, it automatically
injects a GitHub MCP server entry. This enables AI-assisted GitHub operations directly from
your editor.

Detected config locations:
- `~/.config/claude/claude_desktop_config.json`
- `~/.config/zed/settings.json`
- `~/.cursor/mcp.json`
- `~/.config/Code/User/settings.json`

## Requirements

- Node.js (installed automatically if missing)
- npm (installed alongside Node.js)
- Individual API keys for each AI service (not managed by MASH)

## Dry-Run

```bash
mash-setup --dry-run --profile full
```

Shows which AI Spirits would be installed without making any changes.
