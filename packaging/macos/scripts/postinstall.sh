#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# لغة المرجع - Post-install Script
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# إنشاء روابط
ln -sf /usr/local/share/almarjaa/bin/almarjaa /usr/local/bin/almarjaa
ln -sf /usr/local/share/almarjaa/bin/almarjaa-lsp /usr/local/bin/almarjaa-lsp 2>/dev/null || true

# تحديث PATH
if [ -f "$HOME/.zshrc" ]; then
    echo 'export PATH="$PATH:/usr/local/bin"' >> "$HOME/.zshrc"
fi

if [ -f "$HOME/.bashrc" ]; then
    echo 'export PATH="$PATH:/usr/local/bin"' >> "$HOME/.bashrc"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# رسالة النجاح
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║                      ✅ تم تثبيت لغة المرجع بنجاح!                          ║"
echo "╠══════════════════════════════════════════════════════════════════════════════╣"
echo "║                                                                              ║"
echo "║  🚀 للتشغيل:                                                                 ║"
echo "║     almarjaa                    # الوضع التفاعلي                           ║"
echo "║     almarjaa script.mrj         # تشغيل ملف                                ║"
echo "║     almarjaa --help             # المساعدة                                  ║"
echo "║                                                                              ║"
echo "║  📚 الأمثلة: /usr/local/share/almarjaa/examples/                            ║"
echo "║  📖 التوثيق: https://docs.almarjaa.io                                       ║"
echo "║                                                                              ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo ""
