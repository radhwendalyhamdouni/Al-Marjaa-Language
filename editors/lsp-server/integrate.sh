#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# سكربت دمج LSP Server مع مشروع المرجع الأصلي
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# الألوان
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}   دمج LSP Server مع مشروع المرجع${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# التحقق من المسارات
if [ -z "$1" ]; then
    echo -e "${YELLOW}الاستخدام: $0 <مسار_مشروع_المرجع>${NC}"
    echo ""
    echo "مثال:"
    echo "  $0 /path/to/Al-Marjaa-Language"
    exit 1
fi

PROJECT_DIR="$1"
LSP_DIR="$(cd "$(dirname "$0")" && pwd)"

# التحقق من وجود المشروع
if [ ! -d "$PROJECT_DIR" ]; then
    echo -e "${RED}✗ المسار غير موجود: $PROJECT_DIR${NC}"
    exit 1
fi

if [ ! -f "$PROJECT_DIR/Cargo.toml" ]; then
    echo -e "${RED}✗ لم يتم العثور على Cargo.toml في: $PROJECT_DIR${NC}"
    exit 1
fi

echo -e "${BLUE}→ مشروع المرجع:${NC} $PROJECT_DIR"
echo -e "${BLUE}→ LSP Server:${NC} $LSP_DIR"
echo ""

# 1. نسخ LSP Server إلى مجلد editors
echo -e "${YELLOW}▸ إنشاء مجلد editors...${NC}"
mkdir -p "$PROJECT_DIR/editors"

echo -e "${YELLOW}▸ نسخ LSP Server...${NC}"
cp -r "$LSP_DIR" "$PROJECT_DIR/editors/lsp-server"

# 2. تحديث Cargo.toml
echo -e "${YELLOW}▸ تحديث Cargo.toml...${NC}"
cat > "$PROJECT_DIR/editors/lsp-server/Cargo.toml" << 'CARGO_EOF'
[package]
name = "almarjaa-lsp"
version = "3.0.0"
edition = "2021"
authors = ["رضوان دالي حمدوني"]
description = "LSP Server للغة المرجع - متكامل مع المحلل الأصلي"

[[bin]]
name = "almarjaa-lsp"
path = "src/main.rs"

[dependencies]
# LSP الأساسي
lsp-types = "0.95"
crossbeam-channel = "0.5"

# JSON-RPC
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# معالجة النصوص
unicode-segmentation = "1.11"

# التزامن
rayon = "1.10"

# الربط مع المشروع الأصلي
almarjaa = { path = "../.." }

[features]
default = []
CARGO_EOF

# 3. نسخ VS Code Extension
echo -e "${YELLOW}▸ نسخ VS Code Extension...${NC}"
if [ -d "$LSP_DIR/../almarjaa-vscode" ]; then
    cp -r "$LSP_DIR/../almarjaa-vscode" "$PROJECT_DIR/editors/vscode"
    echo -e "${GREEN}✓ تم نسخ VS Code Extension${NC}"
fi

# 4. بناء LSP Server
echo ""
echo -e "${YELLOW}▸ بناء LSP Server...${NC}"
cd "$PROJECT_DIR/editors/lsp-server"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ تم البناء بنجاح${NC}"
else
    echo -e "${RED}✗ فشل البناء${NC}"
    exit 1
fi

# 5. نسخ الملف التنفيذي
echo -e "${YELLOW}▸ نسخ الملف التنفيذي...${NC}"
mkdir -p "$PROJECT_DIR/bin"
cp "$PROJECT_DIR/editors/lsp-server/target/release/almarjaa-lsp" "$PROJECT_DIR/bin/"
chmod +x "$PROJECT_DIR/bin/almarjaa-lsp"

# 6. إنشاء ملف التثبيت
echo -e "${YELLOW}▸ إنشاء سكربت التثبيت...${NC}"
cat > "$PROJECT_DIR/install-lsp.sh" << 'INSTALL_EOF'
#!/bin/bash
# تثبيت LSP Server للغة المرجع

set -e

echo "═══════════════════════════════════════════════════════════════"
echo "   تثبيت Al-Marjaa LSP Server"
echo "═══════════════════════════════════════════════════════════════"

# نسخ الملف التنفيذي
sudo cp bin/almarjaa-lsp /usr/local/bin/
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
INSTALL_EOF
chmod +x "$PROJECT_DIR/install-lsp.sh"

# 7. تحديث Makefile
if [ -f "$PROJECT_DIR/Makefile" ]; then
    echo -e "${YELLOW}▸ إضافة أهداف LSP إلى Makefile...${NC}"
    
    if ! grep -q "lsp-build" "$PROJECT_DIR/Makefile"; then
        cat >> "$PROJECT_DIR/Makefile" << 'MAKEFILE_EOF'

# ═══════════════════════════════════════════════════════════════════════════════
# LSP Server
# ═══════════════════════════════════════════════════════════════════════════════

lsp-build:
	@echo "بناء LSP Server..."
	cd editors/lsp-server && cargo build --release

lsp-install: lsp-build
	@echo "تثبيت LSP Server..."
	sudo cp editors/lsp-server/target/release/almarjaa-lsp /usr/local/bin/

lsp-test:
	@echo "اختبار LSP Server..."
	cd editors/lsp-server && cargo test

vscode-build:
	@echo "بناء VS Code Extension..."
	cd editors/vscode && npm install && npm run compile

vscode-package: vscode-build
	@echo "تعبئة VS Code Extension..."
	cd editors/vscode && npx vsce package

vscode-install: vscode-package
	@echo "تثبيت VS Code Extension..."
	cd editors/vscode && code --install-extension almarjaa-language-*.vsix

.PHONY: lsp-build lsp-install lsp-test vscode-build vscode-package vscode-install
MAKEFILE_EOF
        echo -e "${GREEN}✓ تم تحديث Makefile${NC}"
    fi
fi

# 8. إضافة التبعيات المطلوبة
echo ""
echo -e "${YELLOW}▸ إضافة التبعيات إلى Cargo.toml الرئيسي...${NC}"
cd "$PROJECT_DIR"

# إضافة lsp-types إذا لم تكن موجودة
if ! grep -q "lsp-types" Cargo.toml; then
    # إضافة التبعية في قسم [dependencies]
    sed -i '/\[dependencies\]/a lsp-types = "0.95"' Cargo.toml
    echo -e "${GREEN}✓ تمت إضافة lsp-types${NC}"
fi

# إضافة crossbeam-channel إذا لم تكن موجودة
if ! grep -q "crossbeam-channel" Cargo.toml; then
    sed -i '/\[dependencies\]/a crossbeam-channel = "0.5"' Cargo.toml
    echo -e "${GREEN}✓ تمت إضافة crossbeam-channel${NC}"
fi

# النتيجة النهائية
echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}   ✓ اكتمل الدمج بنجاح!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}الملفات المنشأة:${NC}"
echo -e "  • ${GREEN}editors/lsp-server/${NC}  - LSP Server"
echo -e "  • ${GREEN}editors/vscode/${NC}      - VS Code Extension"
echo -e "  • ${GREEN}bin/almarjaa-lsp${NC}     - الملف التنفيذي"
echo -e "  • ${GREEN}install-lsp.sh${NC}       - سكربت التثبيت"
echo ""
echo -e "${BLUE}الخطوات التالية:${NC}"
echo ""
echo -e "  ${YELLOW}1.${NC} تثبيت LSP Server:"
echo -e "     cd $PROJECT_DIR && ./install-lsp.sh"
echo ""
echo -e "  ${YELLOW}2.${NC} بناء VS Code Extension:"
echo -e "     make vscode-package"
echo ""
echo -e "  ${YELLOW}3.${NC} تثبيت VS Code Extension:"
echo -e "     make vscode-install"
echo ""
echo -e "  ${YELLOW}4.${NC} أو يدوياً:"
echo -e "     cd editors/vscode && npm install && npm run compile"
echo -e "     npx vsce package"
echo -e "     code --install-extension almarjaa-language-*.vsix"
echo ""
