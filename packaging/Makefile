# ═══════════════════════════════════════════════════════════════════════════════
# لغة المرجع - Build Script
# بناء جميع حزم التثبيت الرسمية
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: all build binaries deb rpm msi dmg clean help

VERSION := 3.3.0
PROJECT_NAME := almarjaa
ARCH := $(shell uname -m)

# ═══════════════════════════════════════════════════════════════════════════════
# البناء الرئيسي
# ═══════════════════════════════════════════════════════════════════════════════

all: build

build:
	@echo "🔨 بناء $(PROJECT_NAME) v$(VERSION)..."
	cargo build --release
	@echo "✅ تم البناء بنجاح!"

# ═══════════════════════════════════════════════════════════════════════════════
# حزم Linux
# ═══════════════════════════════════════════════════════════════════════════════

deb: build
	@echo "📦 إنشاء حزمة DEB..."
	@mkdir -p packaging/debian/DEBIAN
	@mkdir -p packaging/debian/usr/bin
	@mkdir -p packaging/debian/usr/share/almarjaa/examples
	@cp target/release/almarjaa packaging/debian/usr/bin/
	@cp target/release/almarjaa-lsp packaging/debian/usr/bin/ 2>/dev/null || true
	@cp examples/*.mrj packaging/debian/usr/share/almarjaa/examples/ 2>/dev/null || true
	@cd packaging/debian && dpkg-deb --build . ../$(PROJECT_NAME)-$(VERSION)-$(ARCH).deb
	@echo "✅ تم إنشاء $(PROJECT_NAME)-$(VERSION)-$(ARCH).deb"

rpm: build
	@echo "📦 إنشاء حزمة RPM..."
	@mkdir -p ~/rpmbuild/{SOURCES,SPECS,RPMS,SRPMS,BUILD}
	@tar -czvf ~/rpmbuild/SOURCES/$(PROJECT_NAME)-$(VERSION).tar.gz \
		--exclude='.git' \
		--exclude='target' \
		--transform 's,^,$(PROJECT_NAME)-$(VERSION)/,' \
		.
	@cp packaging/rpm/$(PROJECT_NAME).spec ~/rpmbuild/SPECS/
	@rpmbuild -bb ~/rpmbuild/SPECS/$(PROJECT_NAME).spec
	@cp ~/rpmbuild/RPMS/$(ARCH)/$(PROJECT_NAME)-$(VERSION)-*.rpm .
	@echo "✅ تم إنشاء $(PROJECT_NAME)-$(VERSION)-$(ARCH).rpm"

# ═══════════════════════════════════════════════════════════════════════════════
# حزم Windows
# ═══════════════════════════════════════════════════════════════════════════════

msi: build
	@echo "📦 إنشاء حزمة MSI..."
	@mkdir -p packaging/windows/bin
	@cp target/release/$(PROJECT_NAME).exe packaging/windows/bin/
	@cp target/release/$(PROJECT_NAME)-lsp.exe packaging/windows/bin/ 2>/dev/null || true
	@cd packaging/windows && \
		candle $(PROJECT_NAME).wxs -o $(PROJECT_NAME).wixobj && \
		light $(PROJECT_NAME).wixobj -o ../$(PROJECT_NAME)-$(VERSION)-x64.msi
	@echo "✅ تم إنشاء $(PROJECT_NAME)-$(VERSION)-x64.msi"

# ═══════════════════════════════════════════════════════════════════════════════
# حزم macOS
# ═══════════════════════════════════════════════════════════════════════════════

dmg: build
	@echo "📦 إنشاء حزمة DMG..."
	@chmod +x packaging/macos/create-dmg.sh
	@./packaging/macos/create-dmg.sh
	@echo "✅ تم إنشاء $(PROJECT_NAME)-$(VERSION)-macos.dmg"

pkg: build
	@echo "📦 إنشاء حزمة PKG..."
	@mkdir -p packaging/macos/root/usr/local/bin
	@mkdir -p packaging/macos/root/usr/local/share/almarjaa/examples
	@cp target/release/$(PROJECT_NAME) packaging/macos/root/usr/local/bin/
	@cp target/release/$(PROJECT_NAME)-lsp packaging/macos/root/usr/local/bin/ 2>/dev/null || true
	@cp examples/*.mrj packaging/macos/root/usr/local/share/almarjaa/examples/ 2>/dev/null || true
	@pkgbuild --root packaging/macos/root \
		--scripts packaging/macos/scripts \
		--identifier io.almarjaa.pkg \
		--version $(VERSION) \
		--install-location / \
		$(PROJECT_NAME)-$(VERSION)-macos.pkg
	@echo "✅ تم إنشاء $(PROJECT_NAME)-$(VERSION)-macos.pkg"

# ═══════════════════════════════════════════════════════════════════════════════
# حزم ثنائية
# ═══════════════════════════════════════════════════════════════════════════════

binaries: build
	@echo "📦 إنشاء الحزم الثنائية..."
	@mkdir -p release
	
	# Linux x86_64
	@tar -czvf release/$(PROJECT_NAME)-$(VERSION)-linux-x86_64.tar.gz \
		-C target/release $(PROJECT_NAME) $(PROJECT_NAME)-lsp 2>/dev/null || true
	
	# macOS (Intel & ARM)
	@tar -czvf release/$(PROJECT_NAME)-$(VERSION)-macos-x86_64.tar.gz \
		-C target/release $(PROJECT_NAME) $(PROJECT_NAME)-lsp 2>/dev/null || true
	
	# Windows
	@zip -j release/$(PROJECT_NAME)-$(VERSION)-windows-x64.zip \
		target/release/$(PROJECT_NAME).exe target/release/$(PROJECT_NAME)-lsp.exe 2>/dev/null || true
	
	@echo "✅ تم إنشاء الحزم الثنائية في release/"

# ═══════════════════════════════════════════════════════════════════════════════
# بناء الكل
# ═══════════════════════════════════════════════════════════════════════════════

all-packages: build deb rpm dmg pkg binaries
	@echo ""
	@echo "╔══════════════════════════════════════════════════════════════════════════════╗"
	@echo "║                  ✅ تم إنشاء جميع الحزم بنجاح!                             ║"
	@echo "╠══════════════════════════════════════════════════════════════════════════════╣"
	@echo "║  📦 DEB:     $(PROJECT_NAME)-$(VERSION)-*.deb                              ║"
	@echo "║  📦 RPM:     $(PROJECT_NAME)-$(VERSION)-*.rpm                              ║"
	@echo "║  📦 DMG:     $(PROJECT_NAME)-$(VERSION)-macos.dmg                          ║"
	@echo "║  📦 PKG:     $(PROJECT_NAME)-$(VERSION)-macos.pkg                          ║"
	@echo "║  📦 Binaries: release/                                                    ║"
	@echo "╚══════════════════════════════════════════════════════════════════════════════╝"
	@echo ""

# ═══════════════════════════════════════════════════════════════════════════════
# التنظيف
# ═══════════════════════════════════════════════════════════════════════════════

clean:
	@echo "🧹 تنظيف..."
	cargo clean
	rm -rf release/
	rm -rf packaging/debian/usr/
	rm -rf packaging/macos/root/
	rm -f *.deb *.rpm *.msi *.dmg *.pkg
	@echo "✅ تم التنظيف!"

# ═══════════════════════════════════════════════════════════════════════════════
# المساعدة
# ═══════════════════════════════════════════════════════════════════════════════

help:
	@echo ""
	@echo "╔══════════════════════════════════════════════════════════════════════════════╗"
	@echo "║            لغة المرجع - نظام البناء v$(VERSION)                              ║"
	@echo "╠══════════════════════════════════════════════════════════════════════════════╣"
	@echo "║                                                                              ║"
	@echo "║  make build        - بناء المشروع                                           ║"
	@echo "║  make deb          - إنشاء حزمة DEB (Debian/Ubuntu)                        ║"
	@echo "║  make rpm          - إنشاء حزمة RPM (Fedora/RHEL)                          ║"
	@echo "║  make msi          - إنشاء حزمة MSI (Windows)                              ║"
	@echo "║  make dmg          - إنشاء حزمة DMG (macOS)                                ║"
	@echo "║  make pkg          - إنشاء حزمة PKG (macOS)                                ║"
	@echo "║  make binaries     - إنشاء الحزم الثنائية                                  ║"
	@echo "║  make all-packages - إنشاء جميع الحزم                                      ║"
	@echo "║  make clean        - تنظيف الملفات                                         ║"
	@echo "║                                                                              ║"
	@echo "╚══════════════════════════════════════════════════════════════════════════════╝"
	@echo ""
