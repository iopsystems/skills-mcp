#!/bin/sh
# Install skills-mcp, the MCP server for embedded agent skills and workflows.
#
#   curl -fsSL https://raw.githubusercontent.com/iopsystems/skills-mcp/main/install.sh | sh
#
# The script downloads the prebuilt binary that matches your platform from the
# GitHub release, verifies its SHA-256 checksum, and installs it. When no
# prebuilt binary matches your platform it falls back to `cargo install`.
#
# Environment variables:
#   SKILLS_MCP_VERSION       release tag to install (default: latest)
#   SKILLS_MCP_INSTALL_DIR   install directory     (default: $HOME/.local/bin)
set -eu

REPO="iopsystems/skills-mcp"
BIN="skills-mcp"
VERSION="${SKILLS_MCP_VERSION:-latest}"
INSTALL_DIR="${SKILLS_MCP_INSTALL_DIR:-$HOME/.local/bin}"

info() { printf '%s\n' "skills-mcp: $*"; }
err() { printf '%s\n' "skills-mcp: error: $*" >&2; }
die() { err "$*"; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

print_next_steps() {
  cat <<EOF

Next: point your MCP client at the binary. For example:

  {
    "mcpServers": {
      "skills-mcp": {
        "command": "$(command -v "$BIN" 2>/dev/null || echo "${INSTALL_DIR}/${BIN}")"
      }
    }
  }

Then reconnect the client and confirm the skill_catalog tool is listed.
EOF
}

# --- Detect platform -> Rust target triple ----------------------------------
os="$(uname -s)"
arch="$(uname -m)"
case "$os" in
  Darwin) os_part="apple-darwin" ;;
  Linux) os_part="unknown-linux-gnu" ;;
  *) os_part="" ;;
esac
case "$arch" in
  arm64 | aarch64) arch_part="aarch64" ;;
  x86_64 | amd64) arch_part="x86_64" ;;
  *) arch_part="" ;;
esac

# --- Downloader -------------------------------------------------------------
download() { # download <url> <dest>; returns non-zero on any HTTP/network error
  if have curl; then
    curl -fsSL "$1" -o "$2"
  elif have wget; then
    wget -q "$1" -O "$2"
  else
    die "need curl or wget to download releases"
  fi
}

sha256_of() { # sha256_of <file> -> lowercase hex on stdout
  if have shasum; then
    shasum -a 256 "$1" | awk '{print $1}'
  elif have sha256sum; then
    sha256sum "$1" | awk '{print $1}'
  else
    die "need shasum or sha256sum to verify the download"
  fi
}

# --- cargo fallback ---------------------------------------------------------
install_from_source() {
  have cargo || die "no prebuilt binary for ${os}/${arch} and cargo is not installed;
  install a Rust toolchain from https://rustup.rs/ and re-run, or build from source."
  info "building from source with cargo (this compiles locally and may take a while)"
  if [ "$VERSION" = "latest" ]; then
    cargo install --git "https://github.com/${REPO}" --locked "$BIN"
  else
    cargo install --git "https://github.com/${REPO}" --tag "$VERSION" --locked "$BIN"
  fi
  info "installed $BIN via cargo into $(dirname "$(command -v "$BIN" 2>/dev/null || echo "$HOME/.cargo/bin/$BIN")")"
  print_next_steps
  exit 0
}

if [ -z "$os_part" ] || [ -z "$arch_part" ]; then
  info "no prebuilt binary is published for ${os}/${arch}"
  install_from_source
fi

target="${arch_part}-${os_part}"
asset="${BIN}-${target}.tar.gz"
if [ "$VERSION" = "latest" ]; then
  base_url="https://github.com/${REPO}/releases/latest/download"
else
  base_url="https://github.com/${REPO}/releases/download/${VERSION}"
fi

# --- Download + verify + install --------------------------------------------
tmp="$(mktemp -d "${TMPDIR:-/tmp}/skills-mcp-install.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT INT TERM

info "downloading ${asset} (${VERSION})"
if ! download "${base_url}/${asset}" "${tmp}/${asset}"; then
  info "prebuilt binary ${asset} is not available for this release"
  install_from_source
fi

if download "${base_url}/${asset}.sha256" "${tmp}/${asset}.sha256"; then
  expected="$(awk '{print $1}' "${tmp}/${asset}.sha256")"
  actual="$(sha256_of "${tmp}/${asset}")"
  [ "$expected" = "$actual" ] || die "checksum mismatch for ${asset}
  expected ${expected}
  actual   ${actual}"
  info "checksum verified"
else
  die "checksum file ${asset}.sha256 is missing; refusing to install unverified binary"
fi

tar -xzf "${tmp}/${asset}" -C "$tmp"
[ -f "${tmp}/${BIN}" ] || die "archive did not contain a ${BIN} binary"

mkdir -p "$INSTALL_DIR"
install -m 0755 "${tmp}/${BIN}" "${INSTALL_DIR}/${BIN}" 2>/dev/null \
  || { chmod 0755 "${tmp}/${BIN}" && cp "${tmp}/${BIN}" "${INSTALL_DIR}/${BIN}"; }
info "installed ${BIN} to ${INSTALL_DIR}/${BIN}"

case ":${PATH}:" in
  *":${INSTALL_DIR}:"*) ;;
  *) info "note: ${INSTALL_DIR} is not on your PATH; add it, e.g. export PATH=\"${INSTALL_DIR}:\$PATH\"" ;;
esac

print_next_steps
