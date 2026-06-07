<h1 align="center">⚡ Alias Next Generation — ang</h1>

<p align="center">
  <img src="https://img.shields.io/github/v/tag/antraxbr666/alias-ng?label=version&color=blue&sort=semver" alt="Version">
  <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
</p>

<p align="center">A modern alias browser written in Rust 🦀</p>

Browse, search, and select shell aliases interactively with a beautiful TUI. Selected alias is copied to clipboard.

---

## ✨ Features

- 🔍 Search through all shell aliases
- 📂 Automatic grouping by category (Docker, Git, System, etc.)
- 🎯 Filter by group: `ang docker`
- 📋 Copies selected alias to clipboard
- 🎨 Catppuccin Mocha theme
- ⚡ Auto-detects all zsh aliases (plugins, frameworks, custom files)
- 🔄 Runtime collection via `alias -L` with metadata enrichment

---

## 📦 Installation

### From source

```bash
git clone https://github.com/antraxbr666/alias-ng.git
cd alias-ng
cargo build --release
sudo cp target/release/ang /usr/local/bin/
```

### Clipboard dependencies

ang uses native clipboard tools to copy aliases. Install the one for your display server:

| Display Server | Detection               | Package                               | Install Command                                 |
| -------------- | ----------------------- | ------------------------------------- | ----------------------------------------------- |
| **Wayland**      | `$WAYLAND_DISPLAY` set  | `wl-clipboard` (provides `wl-copy`)       | `sudo pacman -S wl-clipboard` (Arch)            |
| **X11**          | fallback (no Wayland)   | `xclip` or `xsel`                       | `sudo pacman -S xclip` (Arch)                   |

ang checks for these dependencies on startup and shows an error if none are found.

---

## 🚀 Usage

### Interactive TUI

```bash
ang              # Browse all aliases (auto-detected)
ang docker       # Filter Docker aliases only
```

After selecting an alias, it is copied to clipboard and a notification is displayed.

### Non-interactive

```bash
ang --print              # Print all aliases as TSV
ang --print docker       # Print Docker aliases as TSV
ang --version            # Show version
ang --help               # Show help
```

### ⌨️ TUI Keybindings

| Key      | Action                  |
| -------- | ----------------------- |
| `↑` / `↓` | Navigate                |
| `k` / `j` | Navigate (vim-style)    |
| `/` / `s` | Enter search mode       |
| `Enter`  | Select alias            |
| `Esc` / `q` | Cancel                  |
| `g` / `G` | Jump top / bottom       |

---

## ⚙️ How It Works

ang automatically discovers and collects all your zsh aliases:

1. **Runtime Collection**: Executes `zsh -fc 'alias -L'` to get all loaded aliases (plugins, frameworks, custom)
2. **File Discovery**: Scans `~/.zsh/*.zsh` for metadata (groups and descriptions)
3. **Metadata Enrichment**: Merges groups and descriptions from static files into runtime aliases
4. **Clipboard**: Copies selected alias to clipboard with notification

No configuration needed — ang finds everything automatically.

### `--file` Flag (Legacy Mode)

If you want to parse a specific file only:

```bash
ang --file ~/.zsh/04-aliases.zsh
```

---

## 🔧 Alias File Format

The parser expects standard zsh alias files:

```zsh
# Docker
alias dcud="docker compose up -d"  # Start containers in background
alias dcd="docker compose down"    # Stop containers

# Git
alias gencommit='git diff | sgpt "..."'  # Generate commit via AI
```

### Group Headers

Groups are defined by comment-only lines (lines that contain only a `#` followed by the group name). These lines tell ang that all aliases below belong to this group, until the next group header is found.

```zsh
# Docker
alias dcud="docker compose up -d"  # Start containers in background
alias dclf="docker compose logs -f"  # Follow logs
```

### Alias Definitions

Each alias follows the standard zsh syntax with an optional inline comment for the description:

```zsh
alias name='command'  # Description
```

- **Single-quoted values:** `alias ls='eza --color'  # List files`
- **Double-quoted values:** `alias dcud="docker compose up -d"  # Start containers`
- **Unquoted values:** `alias c=clear  # Clear terminal`

### Description

The description is extracted from the inline comment (everything after `#` at the end of the line). If no comment is provided, the description column will be empty.

### Reference Section (Optional)

You can add a reference summary at the top of your alias file for quick overview. Lines with colons (`:`) are automatically ignored by the parser:

```zsh
######################################################################
# ALIASES REFERENCE
# ─────────────────────────────────────────────────────────────────────
# Sistema:     ls, la, l, cat, c, clean, vim
# Docker:      dcud, dclf, dcd, dcr
######################################################################

# Sistema
alias ls='eza --color'  # List files with colors
...
```

### Full Example

```zsh
######################################################################
# ALIASES REFERENCE
# ─────────────────────────────────────────────────────────────────────
# Sistema:     ls, la, l, cat, c, clean
# Git:         gencommit
# Docker:      dcud, dclf, dcd, dcr
######################################################################

# Sistema
alias ls='eza --color'                    # List files with colors
alias la='eza --git --icons -lgha'        # Detailed list + git status
alias c='clear'                           # Clear terminal

# Git
alias gencommit='git diff | sgpt "..."'   # Generate commit via AI

# Docker
alias dcud="docker compose up -d"         # Start containers in background
alias dclf="docker compose logs -f"       # Follow logs
alias dcd="docker compose down"           # Stop containers
alias dcr="docker compose restart"        # Restart containers
```

### Parsing Rules

| Rule                         | Behavior                        |
| ---------------------------- | ------------------------------- |
| `# Group Name`               | Sets the active group           |
| `# ===` / `# ---`            | Ignored (separator lines)       |
| `# Key: value`               | Ignored (reference summaries)   |
| `alias name='cmd'  # Desc`   | Extracts alias + description    |
| `alias name='cmd'`           | Extracts alias (no description) |

---

## 📋 Requirements

- 🐚 zsh (required for runtime alias collection)
- 🔧 Rust toolchain (to build from source)
- 📋 Clipboard tool: `wl-clipboard` (Wayland) or `xclip`/`xsel` (X11)

---

## 📄 License

MIT

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/antraxbr666">antrax</a>
</p>
