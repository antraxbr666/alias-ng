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
ANG_VERSION="0.1.3"

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
#
# Parses a zsh source file and extracts alias definitions with their
# groups and descriptions.
#
# Output format (tab-separated):
#   group\talias_name\tcommand\tdescription
#
# Group detection:
#   - Comment-only lines set the current group
#   - Separator lines (===, ---, ***) are ignored
#   - Lines with colons (reference summaries) are ignored
#
# Description extraction:
#   - Inline comments after the alias definition
#   - Format: alias name='command'  # Description
#
_ang_parse() {
    local file="$1"
    local group="ungrouped"

    while IFS= read -r line; do
        # Skip empty lines
        [[ -z "${line// /}" ]] && continue

        # Group header detection
        # Matches comment-only lines that aren't separators or reference summaries
        if [[ "$line" =~ '^\s*#\s*([^#].*?)\s*$' ]]; then
            local candidate="${match[1]}"
            # Skip: separator lines (===, ---, ***), lines with colons, very short
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

            # Parse quoted value and optional inline description
            # Case 1: 'value' # description
            if [[ "$raw_value" =~ "^'([^']*)'\s+#\s*(.+)$" ]]; then
                value="${match[1]}" ; desc="${match[2]}"
            # Case 2: 'value' (no description)
            elif [[ "$raw_value" =~ "^'([^']*)'$" ]]; then
                value="${match[1]}"
            # Case 3: "value" # description
            elif [[ "$raw_value" =~ '^"([^"]*)"\s+#\s*(.+)$' ]]; then
                value="${match[1]}" ; desc="${match[2]}"
            # Case 4: "value" (no description)
            elif [[ "$raw_value" =~ '^"([^"]*)"$' ]]; then
                value="${match[1]}"
            # Case 5: unquoted value # description
            elif [[ "$raw_value" =~ '^(.+?)\s+#\s*(.+)$' ]]; then
                value="${match[1]}" ; desc="${match[2]}"
            # Case 6: unquoted value (no description)
            else
                value="$raw_value"
            fi

            # Trim whitespace
            name="${name## }" ; name="${name%% }"
            desc="${desc## }" ; desc="${desc%% }"

            printf '%s\t%s\t%s\t%s\n' "$group" "$name" "$value" "$desc"
        fi
    done < "$file"
}

# ============================================================================
# Preview: Format alias details for fzf preview pane
# ============================================================================
_ang_create_preview() {
    local preview_file=$(mktemp /tmp/ang-preview.XXXXXX)
    cat > "$preview_file" << 'PREVIEW_SCRIPT'
#!/bin/sh
# Parse the selected line and display formatted alias details
echo "$1" | awk -F '│' '{
    for (i = 1; i <= NF; i++) gsub(/^[ \t]+|[ \t]+$/, "", $i)
    if ($1 != "") printf "  \033[1;36mGroup:\033[0m\n    %s\n\n", $1
    if ($2 != "") printf "  \033[1;33mAlias:\033[0m\n    %s\n\n", $2
    if ($3 != "") printf "  \033[1;32mCommand:\033[0m\n    %s\n\n", $3
    if ($4 != "") printf "  \033[1;35mDescription:\033[0m\n    %s\n", $4
}'
PREVIEW_SCRIPT
    chmod +x "$preview_file"
    echo "$preview_file"
}

# ============================================================================
# Main function: Browse aliases with fzf
# ============================================================================
ang() {
    # Version flag
    if [[ "$1" == "--version" || "$1" == "-v" ]]; then
        echo "ang $ANG_VERSION"
        return 0
    fi

    # Check dependencies
    if (( ! $+commands[fzf] )); then
        echo "ang: fzf is required but not installed."
        return 1
    fi

    local filter="${1:-}"
    local entries=()

    # Parse all configured alias files
    for file in ${(s.:.)ANG_ALIAS_FILES}; do
        [[ -f "$file" ]] && entries+=("${(@f)$(_ang_parse "$file")}")
    done

    if [[ ${#entries} -eq 0 ]]; then
        echo "ang: No aliases found in: $ANG_ALIAS_FILES"
        return 1
    fi

    # Build formatted display for fzf
    local fzf_input=""
    for entry in "${entries[@]}"; do
        [[ -z "$entry" ]] && continue
        local group name value desc
        IFS=$'\t' read -r group name value desc <<< "$entry"

        # Apply group filter if argument provided
        if [[ -n "$filter" ]]; then
            [[ "${group:l}" != *"${filter:l}"* ]] && continue
        fi

        fzf_input+="$(printf '%-10s │ %-12s │ %-35s │ %s' "$group" "$name" "$value" "$desc")"$'\n'
    done

    if [[ -z "$fzf_input" ]]; then
        echo "ang: No aliases found for group '$filter'."
        return 1
    fi

    # Create preview script
    local preview_file
    preview_file=$(_ang_create_preview)

    # Launch fzf with preview
    local selected
    selected=$(echo -e "$fzf_input" | fzf \
        --height="$ANG_FZF_HEIGHT" \
        --layout="$ANG_FZF_LAYOUT" \
        --border \
        --header="$(printf '%-10s │ %-12s │ %-35s │ %s' 'GROUP' 'ALIAS' 'COMMAND' 'DESCRIPTION')" \
        --prompt="ang> " \
        --preview="$preview_file {}" \
        --preview-window="right:40%:wrap")

    # Cleanup preview script
    command rm -f "$preview_file"

    # Insert selected alias into command line
    if [[ -n "$selected" ]]; then
        local alias_name
        alias_name=$(echo "$selected" | awk -F'│' '{gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2}')
        LBUFFER+="$alias_name"
        zle redisplay 2>/dev/null
    fi
}
