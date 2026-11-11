#!/bin/sh
# Instala o uaiType em ~/.local (padrão), ou em /usr/local com sudo.
# Uso: ./install.sh          → instala em ~/.local/bin + ~/.local/share
#      sudo ./install.sh   → instala em /usr/local (sistema todo)
#      PREFIX=/opt ./install.sh  → prefixo customizado

set -eu

HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

if [ "${PREFIX:-}" ]; then
  :
elif [ "$(id -u)" -eq 0 ]; then
  PREFIX=/usr/local
else
  PREFIX="$HOME/.local"
fi

echo "→ instalando uaiType em $PREFIX"

install -Dm755 "$HERE/bin/uaitype" "$PREFIX/bin/uaitype"

for s in 16 32 48 64 128 256 512; do
  install -Dm644 "$HERE/share/icons/hicolor/${s}x${s}/apps/io.github.luanxis.uaitype.png" \
    "$PREFIX/share/icons/hicolor/${s}x${s}/apps/io.github.luanxis.uaitype.png"
done

install -Dm644 "$HERE/share/icons/hicolor/scalable/apps/io.github.luanxis.uaitype.svg" \
  "$PREFIX/share/icons/hicolor/scalable/apps/io.github.luanxis.uaitype.svg"
install -Dm644 "$HERE/share/icons/hicolor/symbolic/apps/io.github.luanxis.uaitype-symbolic.svg" \
  "$PREFIX/share/icons/hicolor/symbolic/apps/io.github.luanxis.uaitype-symbolic.svg"
install -Dm644 "$HERE/share/applications/io.github.luanxis.uaitype.desktop" \
  "$PREFIX/share/applications/io.github.luanxis.uaitype.desktop"
install -Dm644 "$HERE/share/metainfo/io.github.luanxis.uaitype.appdata.xml" \
  "$PREFIX/share/metainfo/io.github.luanxis.uaitype.appdata.xml"

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$PREFIX/share/applications" || true
fi
if command -v gtk4-update-icon-cache >/dev/null 2>&1; then
  gtk4-update-icon-cache -q -t -f "$PREFIX/share/icons/hicolor" 2>/dev/null || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
  gtk-update-icon-cache -q -t -f "$PREFIX/share/icons/hicolor" || true
fi

echo "✓ uaiType instalado em $PREFIX/bin/uaitype"
echo "  (garanta que $PREFIX/bin está no seu PATH)"
