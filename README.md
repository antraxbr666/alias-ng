# ⚡ Alias Next Generation (ang)

> A modern alias browser for zsh powered by fzf.

Browse, search, and select shell aliases interactively. Aliases are automatically organized by groups with descriptions extracted from inline comments.

---

## ✨ Features

- 🔍 Fuzzy search through all shell aliases
- 📂 Automatic grouping by category (Docker, Git, Sistema, etc.)
- 🎯 Filter by group: `ang docker`
- 👁️ Preview pane with alias details
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
ang sistema      # Browse system aliases only
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

The plugin parses standard zsh alias files. It detects:

1. **Group headers** — comment-only lines that set the current group:
   ```zsh
   # Docker
   alias dcud="docker compose up -d"  # Sobe containers em background
   ```

2. **Descriptions** — inline comments after the alias definition:
   ```zsh
   alias ls='eza --color'  # Lista arquivos com cores
   ```

### Parsing Rules

- ✅ Comment-only lines (`# Group Name`) set the active group
- ✅ Separator lines (`# ===`, `# ---`) are ignored
- ✅ Reference summaries with colons (`# Sistema: ls, la`) are ignored
- ✅ Descriptions are extracted from `# comment` at the end of alias lines
- ✅ Single and double quoted values are supported

### Display Format

Aliases are displayed in fzf with four columns:

```
GROUP    │ ALIAS      │ COMMAND                          │ DESCRIPTION
─────────┼────────────┼──────────────────────────────────┼─────────────
docker   │ dcud       │ docker compose up -d             │ Sobe containers
docker   │ dclf       │ docker compose logs -f           │ Acompanha logs
git      │ gencommit  │ git diff | sgpt "Generate..."    │ Gera commit
sistema  │ la         │ eza --git --icons -lgha          │ Lista detalhada
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
