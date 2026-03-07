# ═══════════════════════════════════════════════════════════════════════════════
# Al-Marjaa Language - Makefile
# لغة المرجع - أول لغة برمجة عربية متكاملة مع الذكاء الاصطناعي
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: all build release dev clean test install docs lsp vscode help

# الهدف الافتراضي
all: build

# ═══════════════════════════════════════════════════════════════════════════════
# البناء والتطوير
# ═══════════════════════════════════════════════════════════════════════════════

## build: بناء المشروع (وضع التطوير)
build:
	@echo "🔧 بناء المشروع..."
	cargo build
	@echo "✓ تم البناء بنجاح"

## release: بناء المشروع (وضع الإنتاج)
release:
	@echo "🚀 بناء المشروع (Release)..."
	cargo build --release
	@echo "✓ تم البناء بنجاح"
	@echo "📁 الملف التنفيذي: target/release/almarjaa"

## dev: تشغيل وضع التطوير مع المراقبة
dev:
	@echo "🔧 تشغيل وضع التطوير..."
	cargo watch -x run

## run: تشغيل الملف التنفيذي
run:
	@echo "▶️ تشغيل المرجع..."
	cargo run

# ═══════════════════════════════════════════════════════════════════════════════
# الاختبارات
# ═══════════════════════════════════════════════════════════════════════════════

## test: تشغيل جميع الاختبارات
test:
	@echo "🧪 تشغيل الاختبارات..."
	cargo test --all-features
	@echo "✓ اكتملت الاختبارات"

## test-coverage: تشغيل الاختبارات مع التغطية
test-coverage:
	@echo "📊 تشغيل الاختبارات مع التغطية..."
	cargo tarpaulin --out Html --output-dir coverage/
	@echo "✓ تم إنشاء تقرير التغطية: coverage/index.html"

## bench: تشغيل اختبارات الأداء
bench:
	@echo "⚡ تشغيل اختبارات الأداء..."
	cargo bench

# ═══════════════════════════════════════════════════════════════════════════════
# التثبيت
# ═══════════════════════════════════════════════════════════════════════════════

## install: تثبيت المشروع كاملاً
install: install-core install-lsp
	@echo ""
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "   ✓ اكتمل التثبيت بنجاح!"
	@echo "═══════════════════════════════════════════════════════════════"

## install-core: تثبيت اللغة الأساسية فقط
install-core: release
	@echo "📦 تثبيت اللغة الأساسية..."
	sudo cp target/release/almarjaa /usr/local/bin/
	@echo "✓ تم تثبيت almarjaa في /usr/local/bin/"

## uninstall: إزالة التثبيت
uninstall:
	@echo "🗑️ إزالة التثبيت..."
	sudo rm -f /usr/local/bin/almarjaa
	sudo rm -f /usr/local/bin/almarjaa-lsp
	@echo "✓ تمت إزالة التثبيت"

# ═══════════════════════════════════════════════════════════════════════════════
# LSP Server
# ═══════════════════════════════════════════════════════════════════════════════

## lsp: بناء LSP Server
lsp:
	@echo "🔧 بناء LSP Server..."
	cd editors/lsp-server && cargo build --release
	@echo "✓ تم بناء LSP Server"

## install-lsp: تثبيت LSP Server
install-lsp: lsp
	@echo "📦 تثبيت LSP Server..."
	sudo cp editors/lsp-server/target/release/almarjaa-lsp /usr/local/bin/
	@echo "✓ تم تثبيت almarjaa-lsp في /usr/local/bin/"

## lsp-test: اختبار LSP Server
lsp-test:
	@echo "🧪 اختبار LSP Server..."
	cd editors/lsp-server && cargo test

# ═══════════════════════════════════════════════════════════════════════════════
# VS Code Extension
# ═══════════════════════════════════════════════════════════════════════════════

## vscode: بناء VS Code Extension
vscode:
	@echo "🔧 بناء VS Code Extension..."
	cd editors/vscode && npm install && npm run compile
	@echo "✓ تم بناء VS Code Extension"

## vscode-package: تعبئة VS Code Extension
vscode-package: vscode
	@echo "📦 تعبئة VS Code Extension..."
	cd editors/vscode && npx vsce package
	@echo "✓ تم إنشاء الحزمة"

## install-vscode: تثبيت VS Code Extension
install-vscode: vscode-package
	@echo "📦 تثبيت VS Code Extension..."
	cd editors/vscode && code --install-extension almarjaa-language-*.vsix
	@echo "✓ تم تثبيت الإضافة"

# ═══════════════════════════════════════════════════════════════════════════════
# التوثيق
# ═══════════════════════════════════════════════════════════════════════════════

## docs: إنشاء التوثيق
docs:
	@echo "📚 إنشاء التوثيق..."
	cargo doc --no-deps --open
	@echo "✓ تم إنشاء التوثيق"

## docs-all: إنشاء التوثيق الكامل
docs-all:
	@echo "📚 إنشاء التوثيق الكامل..."
	cargo doc --no-deps --document-private-items --open

# ═══════════════════════════════════════════════════════════════════════════════
# التنظيف
# ═══════════════════════════════════════════════════════════════════════════════

## clean: تنظيف ملفات البناء
clean:
	@echo "🧹 تنظيف ملفات البناء..."
	cargo clean
	rm -rf editors/vscode/node_modules
	rm -rf editors/vscode/out
	rm -rf coverage/
	@echo "✓ تم التنظيف"

## clean-all: تنظيف شامل
clean-all: clean
	@echo "🧹 تنظيف شامل..."
	rm -rf target/
	rm -rf editors/lsp-server/target/
	rm -rf editors/vscode/*.vsix
	@echo "✓ تم التنظيف الشامل"

# ═══════════════════════════════════════════════════════════════════════════════
# المساعدة
# ═══════════════════════════════════════════════════════════════════════════════

## help: عرض المساعدة
help:
	@echo ""
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "   لغة المرجع - Al-Marjaa Language"
	@echo "═══════════════════════════════════════════════════════════════"
	@echo ""
	@echo "📦 أوامر البناء:"
	@echo "   make build          - بناء المشروع (تطوير)"
	@echo "   make release        - بناء المشروع (إنتاج)"
	@echo "   make dev            - تشغيل مع المراقبة"
	@echo "   make run            - تشغيل البرنامج"
	@echo ""
	@echo "🧪 أوامر الاختبارات:"
	@echo "   make test           - تشغيل الاختبارات"
	@echo "   make test-coverage  - الاختبارات مع التغطية"
	@echo "   make bench          - اختبارات الأداء"
	@echo ""
	@echo "📦 أوامر التثبيت:"
	@echo "   make install        - تثبيت كامل"
	@echo "   make install-core   - تثبيت اللغة فقط"
	@echo "   make install-lsp    - تثبيت LSP Server"
	@echo "   make install-vscode - تثبيت إضافة VS Code"
	@echo "   make uninstall      - إزالة التثبيت"
	@echo ""
	@echo "🔧 أوامر المكونات:"
	@echo "   make lsp            - بناء LSP Server"
	@echo "   make vscode         - بناء VS Code Extension"
	@echo "   make vscode-package - تعبئة الإضافة"
	@echo ""
	@echo "📚 أوامر التوثيق:"
	@echo "   make docs           - إنشاء التوثيق"
	@echo "   make docs-all       - التوثيق الكامل"
	@echo ""
	@echo "🧹 أوامر التنظيف:"
	@echo "   make clean          - تنظيف ملفات البناء"
	@echo "   make clean-all      - تنظيف شامل"
	@echo ""
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "   GitHub: https://github.com/radhwendalyhamdouni/Al-Marjaa-Language"
	@echo "═══════════════════════════════════════════════════════════════"
	@echo ""
