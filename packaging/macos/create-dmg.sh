#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# لغة المرجع - macOS DMG Builder
# ═══════════════════════════════════════════════════════════════════════════════

set -e

VERSION="3.3.0"
APP_NAME="Al-Marjaa"
DMG_NAME="almarjaa-${VERSION}-macos"

# ═══════════════════════════════════════════════════════════════════════════════
# إعداد المجلدات
# ═══════════════════════════════════════════════════════════════════════════════

echo "📦 إعداد مجلد DMG..."

DMG_DIR="dmg_temp"
rm -rf "$DMG_DIR"
mkdir -p "$DMG_DIR"

# ═══════════════════════════════════════════════════════════════════════════════
# إنشاء هيكل التطبيق
# ═══════════════════════════════════════════════════════════════════════════════

APP_DIR="$DMG_DIR/$APP_NAME.app"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"
mkdir -p "$APP_DIR/Contents/Frameworks"

# ═══════════════════════════════════════════════════════════════════════════════
# نسخ الملفات التنفيذية
# ═══════════════════════════════════════════════════════════════════════════════

echo "📋 نسخ الملفات..."

cp target/release/almarjaa "$APP_DIR/Contents/MacOS/"
cp target/release/almarjaa-lsp "$APP_DIR/Contents/MacOS/" 2>/dev/null || true

chmod +x "$APP_DIR/Contents/MacOS/"*

# ═══════════════════════════════════════════════════════════════════════════════
# إنشاء Info.plist
# ═══════════════════════════════════════════════════════════════════════════════

cat > "$APP_DIR/Contents/Info.plist" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>ar</string>
    <key>CFBundleExecutable</key>
    <string>almarjaa</string>
    <key>CFBundleIconFile</key>
    <string>almarjaa</string>
    <key>CFBundleIdentifier</key>
    <string>io.almarjaa.app</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Al-Marjaa</string>
    <key>CFBundleDisplayName</key>
    <string>المرجع</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>3.3.0</string>
    <key>CFBundleVersion</key>
    <string>3.3.0</string>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeExtensions</key>
            <array>
                <string>mrj</string>
            </array>
            <key>CFBundleTypeName</key>
            <string>Al-Marjaa Source File</string>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>LSHandlerRank</key>
            <string>Owner</string>
        </dict>
    </array>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>© 2026 رضوان دالي حمدوني - All Rights Reserved</string>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
    <key>NSAppleEventsUsageDescription</key>
    <string>Required for scripting integration</string>
    <key>NSNetworkVolumesUsageDescription</key>
    <string>Access to network volumes for project files</string>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
</dict>
</plist>
PLIST

# ═══════════════════════════════════════════════════════════════════════════════
# إضافة روابط إضافية
# ═══════════════════════════════════════════════════════════════════════════════

ln -sf /Applications "$DMG_DIR/Applications"

# ═══════════════════════════════════════════════════════════════════════════════
# إنشاء DMG
# ═══════════════════════════════════════════════════════════════════════════════

echo "💿 إنشاء DMG..."

hdiutil create -volname "$APP_NAME" \
    -srcfolder "$DMG_DIR" \
    -ov -format UDZO \
    "${DMG_NAME}.dmg"

# ═══════════════════════════════════════════════════════════════════════════════
# التوقيع (اختياري)
# ═══════════════════════════════════════════════════════════════════════════════

if [ -n "$APPLE_DEVELOPER_ID" ]; then
    echo "✍️ توقيع التطبيق..."
    
    # توقيع التطبيق
    codesign --deep --force --verify --verbose \
        --sign "$APPLE_DEVELOPER_ID" \
        --options runtime \
        --entitlements packaging/macos/entitlements.plist \
        "$APP_DIR"
    
    # توقيع الـ DMG
    codesign --force --sign "$APPLE_DEVELOPER_ID" "${DMG_NAME}.dmg"
    
    # Notarization
    if [ -n "$APPLE_APP_SPECIFIC_PASSWORD" ]; then
        echo "📤 Notarization..."
        xcrun notarytool submit "${DMG_NAME}.dmg" \
            --apple-id "$APPLE_ID" \
            --password "$APPLE_APP_SPECIFIC_PASSWORD" \
            --team-id "$APPLE_TEAM_ID" \
            --wait
        
        xcrun stapler staple "${DMG_NAME}.dmg"
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
# التنظيف
# ═══════════════════════════════════════════════════════════════════════════════

rm -rf "$DMG_DIR"

echo ""
echo "✅ تم إنشاء ${DMG_NAME}.dmg بنجاح!"
