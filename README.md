<h1 align="center">⚡ Alias Next Generation - ang</h1>

<p align="center">
  <img src="https://img.shields.io/github/v/tag/antraxbr666/alias-ng?label=version&color=blue" alt="Version">
  <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
</p>

<p align="center">A modern alias browser for zsh powered by fzf.</p>

Browse, search, and select shell aliases interactively. Aliases are automatically organized by groups with descriptions extracted from inline comments.

---

## ✨ Features

- 🔍 Fuzzy search through all shell aliases
- 📂 Automatic grouping by category (Docker, Git, System, etc.)
- 🎯 Filter by group: `ang docker`
- ⌨️ Inserts selected alias directly into the command line
- 🔌 Zero configuration — works with your existing alias files

---

## 📦 Installation

### zinit

```zsh
zinit light antraxbr666/alias-ng
```

### Oh-My-Zsh

```zsh
git clone https://github.com/antraxbr666/alias-ng.git ~/.oh-my-zsh/custom/plugins/alias-ng
```

Add `alias-ng` to the `plugins` array in `.zshrc`:

```zsh
plugins=(... alias-ng)
```

### Manual

```zsh
source /path/to/alias-ng/ang.plugin.zsh
```

---

## 🚀 Usage

```zsh
ang              # Browse all aliases
ang docker       # Browse Docker aliases only
ang git          # Browse Git aliases only
ang system       # Browse system aliases only
ang --version    # Show version
```

### ⌨️ fzf Keybindings

| Key      | Action                  |
| -------- | ----------------------- |
| `↑` / `↓` | Navigate                |
| `Enter`  | Select and insert alias |
| `Esc`    | Cancel                  |
| Type     | Fuzzy filter            |

---

## ⚙️ Configuration

### `ANG_ALIAS_FILES`

Colon-separated list of files to scan for aliases.

```zsh
ANG_ALIAS_FILES="$HOME/.zsh/04-aliases.zsh:$HOME/.zsh/05-custom.zsh"
```

**Default:** `$HOME/.zsh/04-aliases.zsh`

### `ANG_FZF_HEIGHT`

Height of the fzf window.

```zsh
ANG_FZF_HEIGHT="50%"
```

**Default:** `80%`

### `ANG_FZF_LAYOUT`

Layout direction for fzf.

```zsh
ANG_FZF_LAYOUT="reverse"
```

**Default:** `reverse`

---

## 🔧 How It Works

### Alias File Format

The plugin parses standard zsh alias files and expects the following structure:

#### Group Headers

Groups are defined by comment-only lines (lines that contain only a `#` followed by the group name). These lines tell the plugin that all aliases below belong to this group, until the next group header is found.

```zsh
# Docker
alias dcud="docker compose up -d"  # Start containers in background
alias dclf="docker compose logs -f"  # Follow logs
```

#### Alias Definitions

Each alias follows the standard zsh syntax with an optional inline comment for the description:

```zsh
alias name='command'  # Description
```

- **Single-quoted values:** `alias ls='eza --color'  # List files`
- **Double-quoted values:** `alias dcud="docker compose up -d"  # Start containers`
- **Unquoted values:** `alias c=clear  # Clear terminal`

#### Description

The description is extracted from the inline comment (everything after `#` at the end of the line). If no comment is provided, the description column will be empty.

#### Reference Section (Optional)

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

#### Full Example

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

### Display Format

Aliases are displayed in fzf with three columns:

```
GROUP    │ ALIAS      │ COMMAND                          │ DESCRIPTION
─────────┼────────────┼──────────────────────────────────┼─────────────
docker   │ dcud       │ docker compose up -d             │ Start containers
docker   │ dclf       │ docker compose logs -f           │ Follow logs
git      │ gencommit  │ git diff | sgpt "Generate..."    │ Generate commit
system   │ la         │ eza --git --icons -lgha          │ Detailed list
```

---

## 📋 Requirements

- 🐚 zsh
- 🔍 fzf

---

## 📄 License

MIT

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/antraxbr666">antrax</a>
</p>
