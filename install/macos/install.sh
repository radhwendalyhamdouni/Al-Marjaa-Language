#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# لغة المرجع - سكربت التثبيت لنظام macOS
# Al-Marjaa Language - macOS Installation Script
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# ألوان للإخراج
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}"
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║         لغة المرجع - Al-Marjaa Language v3.0.0                              ║"
echo "║         سكربت التثبيت لنظام macOS                                          ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

INSTALL_DIR="${HOME}/almarjaa"

# 1. تثبيت Homebrew (إذا لم يكن مثبتاً)
install_homebrew() {
    echo -e "${BLUE}[1/6] التحقق من Homebrew...${NC}"

    if command -v brew &> /dev/null; then
        echo -e "${GREEN}✅ Homebrew مثبت${NC}"
    else
        echo -e "${YELLOW}   جاري تثبيت Homebrew...${NC}"
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

        # إضافة Homebrew إلى PATH (Apple Silicon)
        if [[ $(uname -m) == 'arm64' ]]; then
            echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> "$HOME/.zprofile"
            eval "$(/opt/homebrew/bin/brew shellenv)"
        fi

        echo -e "${GREEN}✅ تم تثبيت Homebrew${NC}"
    fi
}

# 2. تثبيت المتطلبات
install_dependencies() {
    echo -e "${BLUE}[2/6] تثبيت المتطلبات...${NC}"

    brew install curl wget git cmake

    echo -e "${GREEN}✅ تم تثبيت المتطلبات${NC}"
}

# 3. تثبيت Rust
install_rust() {
    echo -e "${BLUE}[3/6] تثبيت Rust...${NC}"

    if command -v rustc &> /dev/null; then
        echo -e "${GREEN}✅ Rust مثبت: $(rustc --version)${NC}"
    else
        echo -e "${YELLOW}   جاري تثبيت Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

        source "$HOME/.cargo/env"

        echo -e "${GREEN}✅ تم تثبيت Rust: $(rustc --version)${NC}"
    fi
}

# 4. استنساخ المشروع
clone_project() {
    echo -e "${BLUE}[4/6] استنساخ المشروع...${NC}"

    if [ -d "$INSTALL_DIR" ]; then
        echo -e "${YELLOW}   المجلد موجود، جاري التحديث...${NC}"
        cd "$INSTALL_DIR"
        git pull
    else
        echo -e "${YELLOW}   جاري استنساخ المشروع...${NC}"
        git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git "$INSTALL_DIR"
    fi

    cd "$INSTALL_DIR"
    echo -e "${GREEN}✅ تم استنساخ المشروع في: $INSTALL_DIR${NC}"
}

# 5. بناء المشروع
build_project() {
    echo -e "${BLUE}[5/6] بناء المشروع...${NC}"

    cd "$INSTALL_DIR"

    # تحميل المكتبات
    echo -e "${YELLOW}   تحميل المكتبات...${NC}"
    cargo fetch

    # البناء
    echo -e "${YELLOW}   جاري البناء (قد يستغرق بضع دقائق)...${NC}"
    cargo build --release

    echo -e "${GREEN}✅ تم بناء المشروع${NC}"
}

# 6. إنشاء رابط رمزي
create_symlink() {
    echo -e "${BLUE}[6/6] إنشاء رابط رمزي...${NC}"

    BINARY_PATH="$INSTALL_DIR/target/release/almarjaa"
    SYMLINK_PATH="/usr/local/bin/almarjaa"

    if [ -f "$BINARY_PATH" ]; then
        sudo ln -sf "$BINARY_PATH" "$SYMLINK_PATH"
        echo -e "${GREEN}✅ تم إنشاء رابط: almarjaa${NC}"
    fi
}

# تحميل النموذج (اختياري)
download_model() {
    echo ""
    echo -e "${YELLOW} هل تريد تحميل نموذج AI؟ (اختياري) [y/N]: ${NC}"
    read -r response

    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo -e "${BLUE}   تحميل نموذج Qwen 2.5 (469 MB)...${NC}"

        mkdir -p "$INSTALL_DIR/models"

        wget -O "$INSTALL_DIR/models/qwen2.5-0.5b-instruct-q4_k_m.gguf" \
            'https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf'

        echo -e "${GREEN}✅ تم تحميل النموذج${NC}"
    fi
}

# عرض رسالة النجاح
show_success() {
    echo ""
    echo -e "${GREEN}"
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║                      ✅ تم التثبيت بنجاح!                                  ║"
    echo "╠══════════════════════════════════════════════════════════════════════════════╣"
    echo "║                                                                              ║"
    echo "║  📂 موقع التثبيت: ~/almarjaa                                               ║"
    echo "║                                                                              ║"
    echo "║  🚀 للتشغيل:                                                                 ║"
    echo "║     almarjaa                    # الوضع التفاعلي                           ║"
    echo "║     almarjaa script.mrj         # تشغيل ملف                                ║"
    echo "║     almarjaa --help             # المساعدة                                  ║"
    echo "║                                                                              ║"
    echo "║  📚 الأمثلة: ~/almarjaa/examples/                                           ║"
    echo "║  📖 التوثيق: ~/almarjaa/docs/                                               ║"
    echo "║                                                                              ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# التنفيذ الرئيسي
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    install_homebrew
    install_dependencies
    install_rust
    clone_project
    build_project
    create_symlink
    download_model
    show_success
}

main "$@"
