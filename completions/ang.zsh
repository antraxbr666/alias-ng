# ============================================================================
# Alias Next Generation (ang) — Zsh widget integration
# ============================================================================
#
# Source this file in your .zshrc to enable the ang widget.
# Usage:
#   source /usr/local/share/ang/ang.zsh
#
# Then type 'ang' to launch the alias browser.
# ============================================================================

# Only load in zsh
[[ -n "$ZSH_VERSION" ]] || return 0

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
        # Insert the selected alias into the current line without executing
        LBUFFER="$selected"
        RBUFFER=""
    fi
}

# Register widget ------------------------------------------------------------
if [[ -o interactive ]]; then
    zle -N _ang_widget
fi
