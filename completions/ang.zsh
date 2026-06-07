# ============================================================================
# Alias Next Generation (ang) — Zsh widget integration
# ============================================================================
#
# Source this file in your .zshrc to enable the ang widget.
# Usage:
#   source /usr/local/share/ang/ang.zsh
#   # Or wherever you installed the completion file
#
# Then press Ctrl+A (default) to launch the alias browser.
# ============================================================================

# Only load in zsh
[[ -n "$ZSH_VERSION" ]] || return 0

# Configuration --------------------------------------------------------------
ANG_KEYBIND="${ANG_KEYBIND:-^a}"

# Widget function ------------------------------------------------------------
_ang_widget() {
    # Create a temporary file for ang to write the selected alias
    local tmpfile
    tmpfile=$(mktemp /tmp/ang-widget.XXXXXX)

    # Run ang, writing selected alias to the temp file
    ang --output-file "$tmpfile" 2>/dev/null

    # Read the selected alias from the temp file
    local selected
    selected=$(cat "$tmpfile" 2>/dev/null)
    rm -f "$tmpfile"

    if [[ -n "$selected" ]]; then
        # Replace the entire current line with the selected alias
        LBUFFER="$selected"
        RBUFFER=""
        zle accept-line
    fi
}

# Register widget ------------------------------------------------------------
if [[ -o interactive ]]; then
    zle -N _ang_widget
    [[ -n "$ANG_KEYBIND" ]] && bindkey "$ANG_KEYBIND" _ang_widget
fi
