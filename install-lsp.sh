#!/bin/bash
# تثبيت LSP Server للغة المرجع

set -e

echo "═══════════════════════════════════════════════════════════════"
echo "   تثبيت Al-Marjaa LSP Server"
echo "═══════════════════════════════════════════════════════════════"

# بناء LSP Server
echo "▸ بناء LSP Server..."
cd editors/lsp-server && cargo build --release

# نسخ الملف التنفيذي
echo "▸ نسخ الملف التنفيذي..."
sudo cp target/release/almarjaa-lsp /usr/local/bin/
echo "✓ تم تثبيت almarjaa-lsp في /usr/local/bin/"

# التثبيت لـ VS Code
if command -v code &> /dev/null; then
    echo ""
    echo "▸ لتثبيت إضافة VS Code:"
    echo "  cd editors/vscode && npm install && npm run compile"
    echo "  npx vsce package"
    echo "  code --install-extension almarjaa-language-*.vsix"
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "✓ اكتمل التثبيت!"
echo "═══════════════════════════════════════════════════════════════"
