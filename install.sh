#!/bin/bash
set -e

REPO="antraxbr666/alias-ng"
BINARY="ang"
INSTALL_DIR="/usr/local/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    echo -e "${GREEN}[ang]${NC} $1" >&2
}

warn() {
    echo -e "${YELLOW}[ang]${NC} $1" >&2
}

error() {
    echo -e "${RED}[ang]${NC} $1" >&2
    exit 1
}

# Detect architecture
detect_arch() {
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64|amd64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "aarch64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            ;;
    esac
}

# Detect OS
detect_os() {
    local os
    os=$(uname -s)
    case "$os" in
        Linux)
            echo "linux"
            ;;
        *)
            error "Unsupported OS: $os (only Linux is supported)"
            ;;
esac
}

# Get latest release version
get_latest_version() {
    local version
    version=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"v([^"]+)".*/\1/')
    if [ -z "$version" ]; then
        error "Failed to get latest version"
    fi
    echo "$version"
}

# Download binary
download_binary() {
    local arch=$1
    local version=$2
    local url="https://github.com/$REPO/releases/download/v${version}/ang-${arch}-linux"
    local tmp_file="/tmp/ang-${arch}-linux"

    info "Downloading ang v${version} for ${arch}..."
    curl -sL "$url" -o "$tmp_file"

    if [ ! -f "$tmp_file" ]; then
        error "Download failed"
    fi

    chmod +x "$tmp_file"
    echo "$tmp_file"
}

# Install binary
install_binary() {
    local tmp_file=$1

    if [ -w "$INSTALL_DIR" ]; then
        mv "$tmp_file" "$INSTALL_DIR/$BINARY"
    else
        warn "Need sudo to install to $INSTALL_DIR"
        sudo mv "$tmp_file" "$INSTALL_DIR/$BINARY"
    fi
}

# Verify installation
verify_installation() {
    if command -v "$BINARY" &> /dev/null; then
        local installed_version
        installed_version=$("$BINARY" --version 2>/dev/null | awk '{print $2}')
        info "ang ${installed_version} installed successfully!"
    else
        warn "ang installed but not in PATH. Add $INSTALL_DIR to your PATH."
    fi
}

# Main
main() {
    echo "" >&2
    echo "  Alias Next Generation (ang) Installer" >&2
    echo "  ──────────────────────────────────────" >&2
    echo "" >&2

    local arch os version tmp_file

    arch=$(detect_arch)
    os=$(detect_os)
    version=$(get_latest_version)

    info "Detected: ${os}-${arch}"

    tmp_file=$(download_binary "$arch" "$version")
    install_binary "$tmp_file"
    verify_installation

    echo "" >&2
    info "Run 'ang' to get started!"
    echo "" >&2
}

main
