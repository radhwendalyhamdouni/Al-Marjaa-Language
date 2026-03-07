#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# لغة المرجع - سكربت التثبيت لنظام Linux (Debian/Ubuntu)
# Al-Marjaa Language - Linux Installation Script
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# ألوان للإخراج
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║         لغة المرجع - Al-Marjaa Language v3.0.0                              ║"
echo "║         سكربت التثبيت لنظام Linux                                          ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# التحقق من صلاحيات root
check_root() {
    if [ "$EUID" -ne 0 ]; then
        echo -e "${YELLOW}⚠️  يُفضل تشغيل هذا السكربت بصلاحيات sudo${NC}"
        echo -e "${YELLOW}   بعض الخطوات قد تتطلب كلمة المرور${NC}"
    fi
}

# تحديث النظام
update_system() {
    echo -e "${BLUE}[1/6] تحديث النظام...${NC}"
    sudo apt-get update -y
    sudo apt-get upgrade -y
    echo -e "${GREEN}✅ تم تحديث النظام${NC}"
}

# تثبيت المتطلبات الأساسية
install_dependencies() {
    echo -e "${BLUE}[2/6] تثبيت المتطلبات الأساسية...${NC}"

    sudo apt-get install -y \
        build-essential \
        curl \
        wget \
        git \
        pkg-config \
        libssl-dev \
        cmake

    echo -e "${GREEN}✅ تم تثبيت المتطلبات الأساسية${NC}"
}

# تثبيت Rust
install_rust() {
    echo -e "${BLUE}[3/6] تثبيت Rust...${NC}"

    if command -v rustc &> /dev/null; then
        echo -e "${GREEN}✅ Rust مثبت بالفعل: $(rustc --version)${NC}"
    else
        echo -e "${YELLOW}   جاري تثبيت Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

        # تحديث PATH
        source "$HOME/.cargo/env"

        echo -e "${GREEN}✅ تم تثبيت Rust: $(rustc --version)${NC}"
    fi
}

# استنساخ المشروع
clone_project() {
    echo -e "${BLUE}[4/6] استنساخ المشروع...${NC}"

    INSTALL_DIR="${HOME}/almarjaa"

    if [ -d "$INSTALL_DIR" ]; then
        echo -e "${YELLOW}   المجلد موجود بالفعل، جاري التحديث...${NC}"
        cd "$INSTALL_DIR"
        git pull
    else
        echo -e "${YELLOW}   جاري استنساخ المشروع...${NC}"
        git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git "$INSTALL_DIR"
    fi

    cd "$INSTALL_DIR"
    echo -e "${GREEN}✅ تم استنساخ المشروع في: $INSTALL_DIR${NC}"
}

# بناء المشروع
build_project() {
    echo -e "${BLUE}[5/6] بناء المشروع...${NC}"

    cd "$INSTALL_DIR"

    # تحميل المتطلبات
    echo -e "${YELLOW}   تحميل المكتبات...${NC}"
    cargo fetch

    # البناء
    echo -e "${YELLOW}   جاري البناء (قد يستغرق بضع دقائق)...${NC}"
    cargo build --release

    echo -e "${GREEN}✅ تم بناء المشروع بنجاح${NC}"
}

# إنشاء رابط رمزي
create_symlink() {
    echo -e "${BLUE}[6/6] إنشاء رابط رمزي...${NC}"

    BINARY_PATH="$INSTALL_DIR/target/release/almarjaa"
    SYMLINK_PATH="/usr/local/bin/almarjaa"

    if [ -f "$BINARY_PATH" ]; then
        sudo ln -sf "$BINARY_PATH" "$SYMLINK_PATH"
        echo -e "${GREEN}✅ تم إنشاء رابط: almarjaa${NC}"
    else
        echo -e "${RED}❌ لم يتم العثور على الملف التنفيذي${NC}"
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
    else
        echo -e "${YELLOW}⏭️  تم تخطي تحميل النموذج${NC}"
        echo -e "${YELLOW}   يمكنك تحميله لاحقاً من: $INSTALL_DIR/models/${NC}"
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

# تشغيل الاختبارات (اختياري)
run_tests() {
    echo ""
    echo -e "${YELLOW} هل تريد تشغيل الاختبارات؟ [y/N]: ${NC}"
    read -r response

    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo -e "${BLUE}   تشغيل الاختبارات...${NC}"
        cd "$INSTALL_DIR"
        cargo test
        echo -e "${GREEN}✅ تم تشغيل الاختبارات${NC}"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# التنفيذ الرئيسي
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    check_root
    update_system
    install_dependencies
    install_rust
    clone_project
    build_project
    create_symlink
    download_model
    run_tests
    show_success
}

main "$@"
