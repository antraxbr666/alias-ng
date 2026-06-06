# ============================================================================
# Alias Next Generation (ang) - A modern alias browser for zsh
# ============================================================================
#
# A lightweight plugin that lists shell aliases with fzf, organized by groups.
# Aliases are parsed from source files, grouped by comment headers, and
# descriptions are extracted from inline comments.
#
# Usage:
#   ang              # Browse all aliases
#   ang <group>      # Filter by group (e.g., ang docker)
#
# Requirements: zsh, fzf
# ============================================================================

# Only load in zsh
[[ -n "$ZSH_VERSION" ]] || return 0

# ============================================================================
# Version
# ============================================================================
ANG_VERSION="0.3.3"

# ============================================================================
# Configuration
# ============================================================================

# Colon-separated list of alias files to scan
ANG_ALIAS_FILES="${ANG_ALIAS_FILES:-$HOME/.zsh/04-aliases.zsh}"

# fzf window height
ANG_FZF_HEIGHT="${ANG_FZF_HEIGHT:-80%}"

# fzf layout direction (reverse = top-down)
ANG_FZF_LAYOUT="${ANG_FZF_LAYOUT:-reverse}"

# ============================================================================
# Parser: Extract aliases from source files
# ============================================================================
_ang_parse() {
    local file="$1"
    local group="ungrouped"

    while IFS= read -r line; do
        [[ -z "${line// /}" ]] && continue

        # Group header detection
        if [[ "$line" =~ '^\s*#\s*([^#].*?)\s*$' ]]; then
            local candidate="${match[1]}"
            if [[ ! "$candidate" =~ '^[=\-\*\s]+$' ]] && \
               [[ ! "$candidate" =~ ':' ]] && \
               (( ${#candidate} > 2 )); then
                group="$candidate"
            fi
            continue
        fi

        # Alias definition extraction
        if [[ "$line" =~ '^\s*alias\s+([^=]+)=(.*)' ]]; then
            local name="${match[1]}"
            local raw_value="${match[2]}"
            local value=""
            local desc=""

            if [[ "$raw_value" =~ "^'([^']*)'\s+#\s*(.+)$" ]]; then
                value="${match[1]}" ; desc="${match[2]}"
            elif [[ "$raw_value" =~ "^'([^']*)'$" ]]; then
                value="${match[1]}"
            elif [[ "$raw_value" =~ '^"([^"]*)"\s+#\s*(.+)$' ]]; then
                value="${match[1]}" ; desc="${match[2]}"
            elif [[ "$raw_value" =~ '^"([^"]*)"$' ]]; then
                value="${match[1]}"
            elif [[ "$raw_value" =~ '^(.+?)\s+#\s*(.+)$' ]]; then
                value="${match[1]}" ; desc="${match[2]}"
            else
                value="$raw_value"
            fi

            name="${name## }" ; name="${name%% }"
            desc="${desc## }" ; desc="${desc%% }"

            printf '%s\t%s\t%s\t%s\n' "$group" "$name" "$value" "$desc"
        fi
    done < "$file"
}

# ============================================================================
# Main function
# ============================================================================
ang() {
    if [[ "$1" == "--version" || "$1" == "-v" ]]; then
        echo "ang $ANG_VERSION"
        return 0
    fi

    if (( ! $+commands[fzf] )); then
        echo "ang: fzf is required but not installed."
        return 1
    fi

    local filter="${1:-}"
    local entries=()
    local fzf_input=""

    # Parse all configured alias files
    for file in ${(s.:.)ANG_ALIAS_FILES}; do
        [[ -f "$file" ]] && entries+=("${(@f)$(_ang_parse "$file")}")
    done

    if [[ ${#entries} -eq 0 ]]; then
        echo "ang: No aliases found in: $ANG_ALIAS_FILES"
        return 1
    fi

    # Catppuccin Mocha colors
    local c_mauve="\033[38;2;203;166;247m"
    local c_blue="\033[38;2;137;180;250m"
    local c_green="\033[38;2;166;227;161m"
    local c_subtext="\033[38;2;186;194;222m"
    local c_overlay="\033[38;2;108;112;134m"
    local c_text="\033[38;2;205;214;244m"
    local c_yellow="\033[38;2;249;226;175m"
    local c_red="\033[38;2;243;139;168m"
    local c_peach="\033[38;2;250;179;135m"
    local c_teal="\033[38;2;148;226;213m"
    local c_pink="\033[38;2;245;194;231m"
    local c_sky="\033[38;2;137;220;235m"
    local reset="\033[0m"

    # Build formatted display with Catppuccin colors
    for entry in "${entries[@]}"; do
        [[ -z "$entry" ]] && continue
        local group name value desc
        IFS=$'\t' read -r group name value desc <<< "$entry"

        if [[ -n "$filter" ]]; then
            [[ "${group:l}" != *"${filter:l}"* ]] && continue
        fi

        fzf_input+="$(printf "${c_mauve}%-10s${reset} ${c_overlay}│${reset} ${c_blue}%-12s${reset} ${c_overlay}│${reset} ${c_green}%-25.25s${reset} ${c_overlay}│${reset} ${c_yellow}%s${reset}" "$group" "$name" "$value" "$desc")"$'\n'
    done

    if [[ -z "$fzf_input" ]]; then
        echo "ang: No aliases found for group '$filter'."
        return 1
    fi

    # Launch fzf with Catppuccin Mocha theme (--no-preview disables preview from FZF_DEFAULT_OPTS)
    local selected
    selected=$(echo -e "$fzf_input" | fzf \
        --ansi \
        --no-preview \
        --height="$ANG_FZF_HEIGHT" \
        --layout="$ANG_FZF_LAYOUT" \
        --border \
        --border-label=" ang $ANG_VERSION " \
        --border-label-pos=3 \
        --color="bg:#1e1e2e,fg:#cdd6f4,hl:#f38ba8,hl+:#f5c2e7" \
        --color="info:#cba6f7,marker:#a6e3a1,pointer:#89b4fa,prompt:#cba6f7" \
        --color="border:#6c7086,label:#cba6f7,query:#cdd6f4" \
        --header="$(printf '\033[38;2;203;166;247m%-10s\033[0m \033[38;2;108;112;134m│\033[0m \033[38;2;137;180;250m%-12s\033[0m \033[38;2;108;112;134m│\033[0m \033[38;2;166;227;161m%-25.25s\033[0m \033[38;2;108;112;134m│\033[0m \033[38;2;249;226;175m%s\033[0m' 'GROUP' 'ALIAS' 'COMMAND' 'DESCRIPTION')" \
        --prompt="ang> " \
        --pointer="▶" \
        --marker="✓")

    # Insert selected alias into command line
    if [[ -n "$selected" ]]; then
        local alias_name
        alias_name=$(echo "$selected" | awk -F'│' '{gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2}')
        LBUFFER+="$alias_name"
        zle redisplay 2>/dev/null
    fi
}
