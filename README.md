# Alias Next Generation (ang)

A modern, fast alias browser written in Rust. Browse, search, and copy your shell aliases with a beautiful TUI interface.

## Features

- **Native TUI** — No external dependencies like fzf, everything is built-in
- **Fuzzy search** — Real-time filtering as you type
- **Catppuccin Mocha theme** — Beautiful colors out of the box
- **Clipboard integration** — Selected alias is automatically copied
- **Fast** — Written in Rust, parses and displays instantly
- **Zero config** — Works with your existing alias files

## Installation

### From source

```bash
git clone https://github.com/antraxbr666/alias-ng.git
cd alias-ng
cargo build --release
# Binary will be at target/release/ang
sudo cp target/release/ang /usr/local/bin/
```

## Usage

```bash
ang              # Launch TUI browser
ang docker       # Filter by group name
ang --print      # Print all aliases as TSV
ang --help       # Show help
```

### TUI Controls

| Key | Action |
|-----|--------|
| `↑` / `↓` or `k` / `j` | Navigate |
| `/` or `s` | Enter search mode |
| `Enter` | Select and copy alias |
| `Esc` or `q` | Quit |
| `g` / `G` | Jump to top / bottom |

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ANG_ALIAS_FILES` | Colon-separated list of alias files | `~/.zsh/04-aliases.zsh` |

## Alias File Format

The parser expects standard zsh alias files with optional group headers:

```zsh
# Docker
alias dcud="docker compose up -d"  # Start containers in background
alias dcd="docker compose down"    # Stop containers

# Git
alias gencommit='git diff | sgpt "..."'  # Generate commit via AI
```

- **Group headers**: Lines starting with `#` (but not containing `:`)
- **Aliases**: Standard `alias name='command' # description` format
- **Reference lines**: Lines with `:` (like `# Sistema: ls, la`) are ignored

## License

MIT
