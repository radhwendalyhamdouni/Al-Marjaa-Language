// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات تصدير الهواتف المحمولة - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::mobile::{
    MobileExporter, MobileExportConfig, MobilePlatform, MobileFramework, export_mobile,
};
use std::path::PathBuf;

/// اختبار تحويل المنصة من العربية
#[test]
fn test_platform_from_arabic() {
    // أندرويد
    assert_eq!(
        MobilePlatform::from_arabic("أندرويد"),
        Some(MobilePlatform::Android)
    );
    assert_eq!(
        MobilePlatform::from_arabic("android"),
        Some(MobilePlatform::Android)
    );
    assert_eq!(
        MobilePlatform::from_arabic("اندرويد"),
        Some(MobilePlatform::Android)
    );

    // آيفون
    assert_eq!(
        MobilePlatform::from_arabic("آيفون"),
        Some(MobilePlatform::iOS)
    );
    assert_eq!(
        MobilePlatform::from_arabic("ios"),
        Some(MobilePlatform::iOS)
    );
    assert_eq!(
        MobilePlatform::from_arabic("ايفون"),
        Some(MobilePlatform::iOS)
    );

    // كلاهما
    assert_eq!(
        MobilePlatform::from_arabic("كلاهما"),
        Some(MobilePlatform::Both)
    );
    assert_eq!(
        MobilePlatform::from_arabic("both"),
        Some(MobilePlatform::Both)
    );
}

/// اختبار تحويل الإطار من العربية
#[test]
fn test_framework_from_arabic() {
    // Flutter
    assert_eq!(
        MobileFramework::from_arabic("فلاتر"),
        Some(MobileFramework::Flutter)
    );
    assert_eq!(
        MobileFramework::from_arabic("flutter"),
        Some(MobileFramework::Flutter)
    );

    // React Native
    assert_eq!(
        MobileFramework::from_arabic("ريأكت"),
        Some(MobileFramework::ReactNative)
    );
    assert_eq!(
        MobileFramework::from_arabic("react-native"),
        Some(MobileFramework::ReactNative)
    );

    // Native
    assert_eq!(
        MobileFramework::from_arabic("أصلي"),
        Some(MobileFramework::Native)
    );
    assert_eq!(
        MobileFramework::from_arabic("native"),
        Some(MobileFramework::Native)
    );

    // Capacitor
    assert_eq!(
        MobileFramework::from_arabic("كاباسيتور"),
        Some(MobileFramework::Capacitor)
    );
    assert_eq!(
        MobileFramework::from_arabic("capacitor"),
        Some(MobileFramework::Capacitor)
    );
}

/// اختبار الإعدادات الافتراضية
#[test]
fn test_default_config() {
    let config = MobileExportConfig::default();

    assert_eq!(config.project_name, "almarjaa_app");
    assert_eq!(config.platform, MobilePlatform::Android);
    assert_eq!(config.framework, MobileFramework::Flutter);
    assert_eq!(config.package_name, "com.almarjaa.app");
    assert_eq!(config.primary_color, "#667eea");
    assert!(config.rtl_support);
    assert!(!config.release_mode);
}

/// اختبار إنشاء المصدّر
#[test]
fn test_exporter_creation() {
    let config = MobileExportConfig {
        project_name: "test_app".to_string(),
        platform: MobilePlatform::Both,
        framework: MobileFramework::Flutter,
        ..Default::default()
    };

    let exporter = MobileExporter::new(config);
    // التحقق من أن المصدّر تم إنشاؤه بنجاح
    let _ = &exporter;
}

/// اختبار أسماء المنصات بالعربية
#[test]
fn test_platform_to_string_ar() {
    assert_eq!(MobilePlatform::Android.to_string_ar(), "أندرويد");
    assert_eq!(MobilePlatform::iOS.to_string_ar(), "آيفون");
    assert_eq!(MobilePlatform::Both.to_string_ar(), "كلاهما");
}

/// اختبار أسماء الأطر بالعربية
#[test]
fn test_framework_to_string_ar() {
    assert_eq!(MobileFramework::Flutter.to_string_ar(), "Flutter");
    assert_eq!(MobileFramework::ReactNative.to_string_ar(), "React Native");
    assert_eq!(MobileFramework::Native.to_string_ar(), "أصلي");
    assert_eq!(MobileFramework::Capacitor.to_string_ar(), "Capacitor");
}

/// اختبار التصدير السريع
#[test]
fn test_export_mobile_quick() {
    let source_code = r#"
        // برنامج اختبار
        اطبع("مرحباً بالعالم!")
    "#;

    let result = export_mobile(source_code, "test_app", "android", "flutter");

    assert!(result.success);
    assert!(result.output_path.is_some());
    assert_eq!(result.platform, MobilePlatform::Android);
    assert_eq!(result.framework, MobileFramework::Flutter);
}

/// اختبار التصدير إلى iOS
#[test]
fn test_export_ios() {
    let config = MobileExportConfig {
        project_name: "ios_test".to_string(),
        platform: MobilePlatform::iOS,
        framework: MobileFramework::Flutter,
        output_dir: PathBuf::from("build/test_mobile_ios"),
        ..Default::default()
    };

    let exporter = MobileExporter::new(config);
    let source = "اطبع('اختبار iOS')";

    let result = exporter.export(source);

    assert!(result.success);
    assert!(result.build_commands.iter().any(|c| c.contains("ios")));
}

/// اختبار التصدير إلى Android Native
#[test]
fn test_export_android_native() {
    let config = MobileExportConfig {
        project_name: "native_test".to_string(),
        platform: MobilePlatform::Android,
        framework: MobileFramework::Native,
        output_dir: PathBuf::from("build/test_mobile_native"),
        ..Default::default()
    };

    let exporter = MobileExporter::new(config);
    let source = "اطبع('اختبار Android Native')";

    let result = exporter.export(source);

    assert!(result.success);
    assert!(result.build_commands.iter().any(|c| c.contains("gradlew")));
}

/// اختبار التصدير إلى React Native
#[test]
fn test_export_react_native() {
    let config = MobileExportConfig {
        project_name: "rn_test".to_string(),
        platform: MobilePlatform::Android,
        framework: MobileFramework::ReactNative,
        output_dir: PathBuf::from("build/test_mobile_rn"),
        ..Default::default()
    };

    let exporter = MobileExporter::new(config);
    let source = "اطبع('اختبار React Native')";

    let result = exporter.export(source);

    assert!(result.success);
    assert!(result.build_commands.iter().any(|c| c.contains("npm")));
}

/// اختبار التصدير إلى Capacitor
#[test]
fn test_export_capacitor() {
    let config = MobileExportConfig {
        project_name: "cap_test".to_string(),
        platform: MobilePlatform::Both,
        framework: MobileFramework::Capacitor,
        output_dir: PathBuf::from("build/test_mobile_cap"),
        ..Default::default()
    };

    let exporter = MobileExporter::new(config);
    let source = "اطبع('اختبار Capacitor')";

    let result = exporter.export(source);

    assert!(result.success);
    assert!(result.build_commands.iter().any(|c| c.contains("cap")));
}

/// اختبار إعدادات RTL
#[test]
fn test_rtl_support() {
    let config = MobileExportConfig {
        rtl_support: true,
        ..Default::default()
    };

    assert!(config.rtl_support);
}

/// اختبار ألوان السمة
#[test]
fn test_custom_primary_color() {
    let config = MobileExportConfig {
        primary_color: "#FF5722".to_string(),
        ..Default::default()
    };

    assert_eq!(config.primary_color, "#FF5722");
}

/// اختبار اسم الحزمة
#[test]
fn test_package_name() {
    let config = MobileExportConfig {
        package_name: "com.example.myapp".to_string(),
        ..Default::default()
    };

    assert_eq!(config.package_name, "com.example.myapp");
}

/// اختبار وضع الإصدار
#[test]
fn test_release_mode() {
    let config = MobileExportConfig {
        release_mode: true,
        ..Default::default()
    };

    assert!(config.release_mode);
}
