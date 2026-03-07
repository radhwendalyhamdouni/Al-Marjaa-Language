#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# لغة المرجع - سكربت التثبيت الرئيسي
# Al-Marjaa Language - Main Installation Script
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# ═══════════════════════════════════════════════════════════════════════════════
# الألوان والثوابت
# ═══════════════════════════════════════════════════════════════════════════════

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ═══════════════════════════════════════════════════════════════════════════════
# دوال المساعدة
# ═══════════════════════════════════════════════════════════════════════════════

print_header() {
    echo ""
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}   $1${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
}

print_step() {
    echo -e "${BLUE}▸ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

check_command() {
    if command -v "$1" &> /dev/null; then
        return 0
    else
        return 1
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# فحص المتطلبات
# ═══════════════════════════════════════════════════════════════════════════════

check_requirements() {
    print_header "فحص المتطلبات"
    
    # Rust
    if check_command rustc; then
        RUST_VERSION=$(rustc --version)
        print_success "Rust: $RUST_VERSION"
    else
        print_error "Rust غير مثبت!"
        echo ""
        echo "قم بتثبيت Rust من: https://rustup.rs"
        echo "أو شغّل: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    # Cargo
    if check_command cargo; then
        CARGO_VERSION=$(cargo --version)
        print_success "Cargo: $CARGO_VERSION"
    else
        print_error "Cargo غير مثبت!"
        exit 1
    fi
    
    # Node.js (اختياري لـ VS Code)
    if check_command node; then
        NODE_VERSION=$(node --version)
        print_success "Node.js: $NODE_VERSION"
        HAS_NODE=true
    else
        print_warning "Node.js غير مثبت (مطلوب لإضافة VS Code)"
        HAS_NODE=false
    fi
    
    # VS Code (اختياري)
    if check_command code; then
        print_success "VS Code: مثبت"
        HAS_VSCODE=true
    else
        print_warning "VS Code غير مثبت (اختياري)"
        HAS_VSCODE=false
    fi
    
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
# بناء اللغة الأساسية
# ═══════════════════════════════════════════════════════════════════════════════

build_core() {
    print_header "بناء لغة المرجع"
    
    print_step "تنظيف البناء السابق..."
    cargo clean 2>/dev/null || true
    
    print_step "بناء المشروع (Release)..."
    cargo build --release
    
    print_success "تم بناء اللغة بنجاح!"
    echo ""
    echo -e "   ${GREEN}📁 الملف التنفيذي:${NC} target/release/almarjaa"
}

# ═══════════════════════════════════════════════════════════════════════════════
# بناء LSP Server
# ═══════════════════════════════════════════════════════════════════════════════

build_lsp() {
    print_header "بناء LSP Server"
    
    if [ ! -d "$SCRIPT_DIR/editors/lsp-server" ]; then
        print_error "مجلد LSP Server غير موجود!"
        return 1
    fi
    
    print_step "بناء LSP Server..."
    cd "$SCRIPT_DIR/editors/lsp-server"
    cargo build --release
    
    print_success "تم بناء LSP Server!"
    echo ""
    echo -e "   ${GREEN}📁 الملف التنفيذي:${NC} editors/lsp-server/target/release/almarjaa-lsp"
}

# ═══════════════════════════════════════════════════════════════════════════════
# بناء VS Code Extension
# ═══════════════════════════════════════════════════════════════════════════════

build_vscode() {
    print_header "بناء VS Code Extension"
    
    if [ "$HAS_NODE" = false ]; then
        print_error "Node.js غير مثبت! لا يمكن بناء الإضافة."
        return 1
    fi
    
    if [ ! -d "$SCRIPT_DIR/editors/vscode" ]; then
        print_error "مجلد VS Code Extension غير موجود!"
        return 1
    fi
    
    print_step "تثبيت التبعيات..."
    cd "$SCRIPT_DIR/editors/vscode"
    npm install
    
    print_step "بناء الإضافة..."
    npm run compile
    
    print_success "تم بناء VS Code Extension!"
}

# ═══════════════════════════════════════════════════════════════════════════════
# التثبيت
# ═══════════════════════════════════════════════════════════════════════════════

install_all() {
    print_header "تثبيت الملفات"
    
    # تثبيت اللغة
    print_step "تثبيت لغة المرجع..."
    sudo cp "$SCRIPT_DIR/target/release/almarjaa" /usr/local/bin/
    sudo chmod +x /usr/local/bin/almarjaa
    print_success "تم تثبيت almarjaa في /usr/local/bin/"
    
    # تثبيت LSP Server
    if [ -f "$SCRIPT_DIR/editors/lsp-server/target/release/almarjaa-lsp" ]; then
        print_step "تثبيت LSP Server..."
        sudo cp "$SCRIPT_DIR/editors/lsp-server/target/release/almarjaa-lsp" /usr/local/bin/
        sudo chmod +x /usr/local/bin/almarjaa-lsp
        print_success "تم تثبيت almarjaa-lsp في /usr/local/bin/"
    fi
    
    # تثبيت VS Code Extension
    if [ "$HAS_VSCODE" = true ] && [ -d "$SCRIPT_DIR/editors/vscode" ]; then
        print_step "تعبئة VS Code Extension..."
        cd "$SCRIPT_DIR/editors/vscode"
        npx vsce package 2>/dev/null || true
        
        if ls *.vsix 1> /dev/null 2>&1; then
            print_step "تثبيت VS Code Extension..."
            code --install-extension almarjaa-language-*.vsix
            print_success "تم تثبيت إضافة VS Code!"
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# التحقق من التثبيت
# ═══════════════════════════════════════════════════════════════════════════════

verify_installation() {
    print_header "التحقق من التثبيت"
    
    # التحقق من اللغة
    if check_command almarjaa; then
        print_success "almarjaa: مثبت"
    else
        print_error "almarjaa: غير مثبت"
    fi
    
    # التحقق من LSP
    if check_command almarjaa-lsp; then
        print_success "almarjaa-lsp: مثبت"
    else
        print_warning "almarjaa-lsp: غير مثبت"
    fi
    
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
# القائمة الرئيسية
# ═══════════════════════════════════════════════════════════════════════════════

show_menu() {
    echo ""
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}   ${BOLD}لغة المرجع - سكربت التثبيت${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "   ${GREEN}1.${NC} تثبيت كامل (اللغة + LSP + VS Code)"
    echo -e "   ${GREEN}2.${NC} تثبيت اللغة فقط"
    echo -e "   ${GREEN}3.${NC} تثبيت LSP Server فقط"
    echo -e "   ${GREEN}4.${NC} تثبيت VS Code Extension فقط"
    echo ""
    echo -e "   ${YELLOW}5.${NC} بناء بدون تثبيت"
    echo -e "   ${YELLOW}6.${NC} تشغيل الاختبارات"
    echo ""
    echo -e "   ${RED}7.${NC} إزالة التثبيت"
    echo ""
    echo -e "   ${BLUE}0.${NC} خروج"
    echo ""
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
# إزالة التثبيت
# ═══════════════════════════════════════════════════════════════════════════════

uninstall() {
    print_header "إزالة التثبيت"
    
    print_step "إزالة الملفات..."
    sudo rm -f /usr/local/bin/almarjaa 2>/dev/null || true
    sudo rm -f /usr/local/bin/almarjaa-lsp 2>/dev/null || true
    
    print_success "تمت إزالة التثبيت!"
}

# ═══════════════════════════════════════════════════════════════════════════════
# تشغيل الاختبارات
# ═══════════════════════════════════════════════════════════════════════════════

run_tests() {
    print_header "تشغيل الاختبارات"
    
    cd "$SCRIPT_DIR"
    cargo test --all-features
    
    print_success "اكتملت الاختبارات!"
}

# ═══════════════════════════════════════════════════════════════════════════════
# البرنامج الرئيسي
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    # إذا كان هناك معامل سطر أوامر
    if [ -n "$1" ]; then
        case "$1" in
            --all|-a)
                check_requirements
                build_core
                build_lsp
                build_vscode
                install_all
                verify_installation
                ;;
            --core|-c)
                check_requirements
                build_core
                install_all
                verify_installation
                ;;
            --lsp|-l)
                check_requirements
                build_lsp
                install_all
                verify_installation
                ;;
            --vscode|-v)
                check_requirements
                build_vscode
                install_all
                ;;
            --uninstall)
                uninstall
                ;;
            --help|-h)
                echo "الاستخدام: $0 [خيار]"
                echo ""
                echo "الخيارات:"
                echo "  --all, -a      تثبيت كامل"
                echo "  --core, -c     تثبيت اللغة فقط"
                echo "  --lsp, -l      تثبيت LSP Server فقط"
                echo "  --vscode, -v   تثبيت VS Code Extension فقط"
                echo "  --uninstall    إزالة التثبيت"
                echo "  --help, -h     عرض المساعدة"
                ;;
            *)
                print_error "خيار غير معروف: $1"
                echo "شغّل $0 --help للمساعدة"
                exit 1
                ;;
        esac
        exit 0
    fi
    
    # وضع تفاعلي
    check_requirements
    
    while true; do
        show_menu
        read -p "اختر خياراً: " choice
        
        case $choice in
            1)
                build_core
                build_lsp
                build_vscode
                install_all
                verify_installation
                ;;
            2)
                build_core
                install_all
                verify_installation
                ;;
            3)
                build_lsp
                install_all
                verify_installation
                ;;
            4)
                build_vscode
                install_all
                ;;
            5)
                build_core
                build_lsp
                build_vscode
                ;;
            6)
                run_tests
                ;;
            7)
                uninstall
                ;;
            0)
                echo ""
                echo -e "${GREEN}مع السلامة!${NC}"
                exit 0
                ;;
            *)
                print_error "خيار غير صحيح!"
                ;;
        esac
        
        echo ""
        read -p "اضغط Enter للمتابعة..."
    done
}

main "$@"
