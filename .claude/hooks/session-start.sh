#!/bin/bash
# SessionStart hook — garante que `cargo check`/`cargo test` em src-tauri/ e
# `npm run build`/`npm run dev` na raiz funcionem de cara em uma sessão nova.
# Idempotente: seguro de rodar de novo (apt/npm pulam o que já está instalado).
set -euo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

cd "${CLAUDE_PROJECT_DIR:-.}"

# Tauri (Linux) precisa destes headers de sistema para compilar — sem eles o
# `cargo check` falha em build scripts (gdk-sys, webkit2gtk-sys, etc.) com
# "Package 'gdk-3.0' not found" via pkg-config.
if ! pkg-config --exists webkit2gtk-4.1 2>/dev/null || ! pkg-config --exists gdk-3.0 2>/dev/null; then
  apt-get update -qq
  apt-get install -y -qq \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf
fi

if [ -f package.json ]; then
  npm install --no-audit --no-fund
fi
