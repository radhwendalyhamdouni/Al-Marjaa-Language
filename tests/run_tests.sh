#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# سكربت تشغيل اختبارات لغة المرجع
# Al-Marjaa Language Test Runner
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# الألوان
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}   لغة المرجع - تشغيل الاختبارات${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# تحديد نوع الاختبار
TEST_TYPE="${1:-all}"

run_lexer_tests() {
    echo -e "${BLUE}▸ تشغيل اختبارات المحلل المعجمي...${NC}"
    cargo test lexer_comprehensive --all-features -- --nocapture
    echo -e "${GREEN}✓ اكتملت اختبارات المحلل المعجمي${NC}"
    echo ""
}

run_parser_tests() {
    echo -e "${BLUE}▸ تشغيل اختبارات المحلل النحوي...${NC}"
    cargo test parser --all-features -- --nocapture
    echo -e "${GREEN}✓ اكتملت اختبارات المحلل النحوي${NC}"
    echo ""
}

run_interpreter_tests() {
    echo -e "${BLUE}▸ تشغيل اختبارات المفسر...${NC}"
    cargo test interpreter_comprehensive --all-features -- --nocapture
    echo -e "${GREEN}✓ اكتملت اختبارات المفسر${NC}"
    echo ""
}

run_ai_tests() {
    echo -e "${BLUE}▸ تشغيل اختبارات تكامل AI...${NC}"
    cargo test ai_integration --all-features -- --nocapture
    echo -e "${GREEN}✓ اكتملت اختبارات تكامل AI${NC}"
    echo ""
}

run_lsp_tests() {
    echo -e "${BLUE}▸ تشغيل اختبارات LSP Server...${NC}"
    cargo test lsp_server --all-features -- --nocapture
    echo -e "${GREEN}✓ اكتملت اختبارات LSP Server${NC}"
    echo ""
}

run_integration_tests() {
    echo -e "${BLUE}▸ تشغيل اختبارات التكامل...${NC}"
    cargo test integration_tests --all-features -- --nocapture
    echo -e "${GREEN}✓ اكتملت اختبارات التكامل${NC}"
    echo ""
}

run_all_tests() {
    echo -e "${YELLOW}▸ تشغيل جميع الاختبارات...${NC}"
    echo ""
    
    run_lexer_tests
    run_parser_tests
    run_interpreter_tests
    run_ai_tests
    run_lsp_tests
    run_integration_tests
}

run_coverage() {
    echo -e "${BLUE}▸ تشغيل الاختبارات مع قياس التغطية...${NC}"
    
    if command -v cargo-tarpaulin &> /dev/null; then
        cargo tarpaulin --all-features --out Html --output-dir coverage/
        echo -e "${GREEN}✓ تم إنشاء تقرير التغطية: coverage/index.html${NC}"
    else
        echo -e "${YELLOW}⚠ cargo-tarpaulin غير مثبت${NC}"
        echo -e "  قم بتثبيته: cargo install cargo-tarpaulin"
    fi
    echo ""
}

run_benchmarks() {
    echo -e "${BLUE}▸ تشغيل اختبارات الأداء...${NC}"
    cargo test --all-features -- --nocapture performance
    echo -e "${GREEN}✓ اكتملت اختبارات الأداء${NC}"
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
# القائمة الرئيسية
# ═══════════════════════════════════════════════════════════════════════════════

case "$TEST_TYPE" in
    all)
        run_all_tests
        ;;
    lexer)
        run_lexer_tests
        ;;
    parser)
        run_parser_tests
        ;;
    interpreter)
        run_interpreter_tests
        ;;
    ai)
        run_ai_tests
        ;;
    lsp)
        run_lsp_tests
        ;;
    integration)
        run_integration_tests
        ;;
    coverage)
        run_coverage
        ;;
    bench|benchmark)
        run_benchmarks
        ;;
    help|--help|-h)
        echo "الاستخدام: $0 [خيار]"
        echo ""
        echo "الخيارات:"
        echo "  all          - تشغيل جميع الاختبارات (افتراضي)"
        echo "  lexer        - اختبارات المحلل المعجمي"
        echo "  parser       - اختبارات المحلل النحوي"
        echo "  interpreter  - اختبارات المفسر"
        echo "  ai           - اختبارات تكامل AI"
        echo "  lsp          - اختبارات LSP Server"
        echo "  integration  - اختبارات التكامل"
        echo "  coverage     - قياس التغطية"
        echo "  bench        - اختبارات الأداء"
        echo "  help         - عرض هذه المساعدة"
        ;;
    *)
        echo -e "${RED}خيار غير معروف: $TEST_TYPE${NC}"
        echo "شغّل '$0 help' للمساعدة"
        exit 1
        ;;
esac

echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}   ✓ اكتملت جميع الاختبارات بنجاح!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
