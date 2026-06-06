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
ANG_VERSION="0.2.0"

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

    # Build formatted display
    for entry in "${entries[@]}"; do
        [[ -z "$entry" ]] && continue
        local group name value desc
        IFS=$'\t' read -r group name value desc <<< "$entry"

        if [[ -n "$filter" ]]; then
            [[ "${group:l}" != *"${filter:l}"* ]] && continue
        fi

        fzf_input+="$(printf '%-10s │ %-12s │ %-25.25s │ %s' "$group" "$name" "$value" "$desc")"$'\n'
    done

    if [[ -z "$fzf_input" ]]; then
        echo "ang: No aliases found for group '$filter'."
        return 1
    fi

    # Launch fzf (override FZF_DEFAULT_OPTS to disable preview pane)
    local selected
    selected=$(echo -e "$fzf_input" | FZF_DEFAULT_OPTS="" fzf \
        --height="$ANG_FZF_HEIGHT" \
        --layout="$ANG_FZF_LAYOUT" \
        --border \
        --header="$(printf '%-10s │ %-12s │ %-25.25s │ %s' 'GROUP' 'ALIAS' 'COMMAND' 'DESCRIPTION')" \
        --prompt="ang> ")

    # Insert selected alias into command line
    if [[ -n "$selected" ]]; then
        local alias_name
        alias_name=$(echo "$selected" | awk -F'│' '{gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2}')
        LBUFFER+="$alias_name"
        zle redisplay 2>/dev/null
    fi
}
