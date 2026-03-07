#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# لغة المرجع - سكربت التثبيت الموحد v3.3.0
# الاستخدام: curl -fsSL https://install.almarjaa.io | bash
# ═══════════════════════════════════════════════════════════════════════════════

set -e

VERSION="3.3.0"
GITHUB_REPO="radhwendalyhamdouni/Al-Marjaa-Language"
BASE_URL="https://github.com/$GITHUB_REPO/releases/download/v$VERSION"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

print_banner() {
    echo -e "${CYAN}"
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║     🌙 لغة المرجع - Al-Marjaa Language v$VERSION                            ║"
    echo "║     لغة برمجة عربية متكاملة مع ذكاء اصطناعي                                ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

detect_system() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux*)     OS_TYPE="linux";;
        Darwin*)    OS_TYPE="macos";;
        *)          OS_TYPE="unknown";;
    esac
    
    case "$ARCH" in
        x86_64|amd64)   ARCH_TYPE="x86_64";;
        aarch64|arm64)  ARCH_TYPE="aarch64";;
        *)              ARCH_TYPE="unknown";;
    esac
    
    echo -e "${BLUE}🔍 النظام: $OS_TYPE | المعالج: $ARCH_TYPE${NC}"
}

install_binary() {
    INSTALL_DIR="$HOME/.almarjaa"
    mkdir -p "$INSTALL_DIR/bin"
    
    case "$OS_TYPE" in
        linux)  FILENAME="almarjaa-linux-${ARCH_TYPE}-unknown-linux-gnu.tar.gz" ;;
        macos)  FILENAME="almarjaa-macos-${ARCH_TYPE}-apple-darwin.tar.gz" ;;
        *)      echo -e "${RED}نظام غير مدعوم${NC}"; exit 1 ;;
    esac
    
    DOWNLOAD_URL="$BASE_URL/$FILENAME"
    
    echo -e "${BLUE}⬇️ تحميل من: $DOWNLOAD_URL${NC}"
    
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    curl -fSL "$DOWNLOAD_URL" -o "almarjaa.tar.gz" || wget "$DOWNLOAD_URL" -O "almarjaa.tar.gz"
    tar -xzf "almarjaa.tar.gz"
    mv almarjaa* "$INSTALL_DIR/bin/" 2>/dev/null || true
    
    rm -rf "$TEMP_DIR"
    
    # إضافة إلى PATH
    SHELL_RC="$HOME/.bashrc"
    [ -n "$(echo $SHELL | grep zsh)" ] && SHELL_RC="$HOME/.zshrc"
    
    if ! grep -q "almarjaa" "$SHELL_RC" 2>/dev/null; then
        echo "export PATH=\"\$PATH:$INSTALL_DIR/bin\"" >> "$SHELL_RC"
    fi
    
    export PATH="$PATH:$INSTALL_DIR/bin"
    
    echo -e "${GREEN}"
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║                      ✅ تم التثبيت بنجاح!                                   ║"
    echo "╠══════════════════════════════════════════════════════════════════════════════╣"
    echo "║  🚀 للتشغيل:                                                                 ║"
    echo "║     almarjaa                    # الوضع التفاعلي                           ║"
    echo "║     almarjaa script.mrj         # تشغيل ملف                                ║"
    echo "║  📖 التوثيق: https://docs.almarjaa.io                                       ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_banner
detect_system
install_binary
