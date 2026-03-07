// ═══════════════════════════════════════════════════════════════════════════════
// قوالب Flutter - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use crate::mobile::MobileExportConfig;

/// توليد pubspec.yaml
pub fn generate_pubspec(config: &MobileExportConfig) -> String {
    format!(
r#"name: {}
description: {}
publish_to: 'none'
version: {}

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  flutter:
    sdk: flutter
  flutter_localizations:
    sdk: flutter
  cupertino_icons: ^1.0.6
  provider: ^6.1.1
  shared_preferences: ^2.2.2
  http: ^1.1.0
  flutter_svg: ^2.0.9
  cached_network_image: ^3.3.0

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^3.0.0

flutter:
  uses-material-design: true
  
  assets:
    - assets/wasm/

  fonts:
    - family: Cairo
      fonts:
        - asset: fonts/Cairo-Regular.ttf
        - asset: fonts/Cairo-Bold.ttf
          weight: 700
"#,
        config.project_name,
        config.description,
        config.version
    )
}

/// توليد main.dart
pub fn generate_main(config: &MobileExportConfig) -> String {
    format!(
r#"import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'app_theme.dart';
import 'screens/home_screen.dart';
import 'i18n/strings.dart';

void main() {{
  runApp(const {}App());
}}

class {}App extends StatelessWidget {{
  const {}App({{super.key}});

  @override
  Widget build(BuildContext context) {{
    return MaterialApp(
      title: '{}',
      debugShowCheckedModeBanner: false,
      
      // دعم RTL للغة العربية
      textDirection: TextDirection.rtl,
      locale: const Locale('ar'),
      supportedLocales: const [
        Locale('ar'),
        Locale('en'),
      ],
      localizationsDelegates: const [
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      
      // الثيم
      theme: AppTheme.lightTheme,
      darkTheme: AppTheme.darkTheme,
      themeMode: ThemeMode.system,
      
      home: const HomeScreen(),
    );
  }}
}}
"#,
        config.project_name,
        config.project_name,
        config.project_name,
        config.project_name
    )
}

/// توليد app_theme.dart
pub fn generate_theme(config: &MobileExportConfig) -> String {
    let primary_color = &config.primary_color;
    format!(
r#"import 'package:flutter/material.dart';

class AppTheme {{
  // الألوان الرئيسية
  static const Color primaryColor = Color(0xFF{});
  static const Color secondaryColor = Color(0xFF764BA2);
  static const Color accentColor = Color(0xFFFF6B6B);
  
  // ألوان النص
  static const Color textPrimary = Color(0xFF1A1A2E);
  static const Color textSecondary = Color(0xFF666666);
  static const Color textLight = Color(0xFFFFFFFF);
  
  // ألوان الخلفية
  static const Color backgroundLight = Color(0xFFF8F9FA);
  static const Color backgroundDark = Color(0xFF1A1A2E);
  static const Color surfaceLight = Color(0xFFFFFFFF);
  static const Color surfaceDark = Color(0xFF252542);

  static ThemeData get lightTheme {{
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.light,
      primaryColor: primaryColor,
      scaffoldBackgroundColor: backgroundLight,
      
      colorScheme: ColorScheme.light(
        primary: primaryColor,
        secondary: secondaryColor,
        surface: surfaceLight,
        error: const Color(0xFFE74C3C),
      ),
      
      appBarTheme: const AppBarTheme(
        backgroundColor: primaryColor,
        foregroundColor: textLight,
        elevation: 0,
        centerTitle: true,
      ),
      
      cardTheme: CardTheme(
        color: surfaceLight,
        elevation: 4,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
        ),
      ),
      
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: primaryColor,
          foregroundColor: textLight,
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
        ),
      ),
      
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: backgroundLight,
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide.none,
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: primaryColor, width: 2),
        ),
      ),
      
      textTheme: const TextTheme(
        headlineLarge: TextStyle(
          fontSize: 28,
          fontWeight: FontWeight.bold,
          color: textPrimary,
        ),
        headlineMedium: TextStyle(
          fontSize: 24,
          fontWeight: FontWeight.bold,
          color: textPrimary,
        ),
        bodyLarge: TextStyle(
          fontSize: 16,
          color: textPrimary,
        ),
        bodyMedium: TextStyle(
          fontSize: 14,
          color: textSecondary,
        ),
      ),
    );
  }}

  static ThemeData get darkTheme {{
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.dark,
      primaryColor: primaryColor,
      scaffoldBackgroundColor: backgroundDark,
      
      colorScheme: ColorScheme.dark(
        primary: primaryColor,
        secondary: secondaryColor,
        surface: surfaceDark,
        error: const Color(0xFFE74C3C),
      ),
      
      appBarTheme: const AppBarTheme(
        backgroundColor: surfaceDark,
        foregroundColor: textLight,
        elevation: 0,
        centerTitle: true,
      ),
      
      cardTheme: CardTheme(
        color: surfaceDark,
        elevation: 4,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
        ),
      ),
      
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: primaryColor,
          foregroundColor: textLight,
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
        ),
      ),
    );
  }}
}}
"#,
        primary_color.replace("#", "")
    )
}

/// توليد home_screen.dart
pub fn generate_home_screen(config: &MobileExportConfig) -> String {
    format!(
r#"import 'package:flutter/material.dart';
import '../runtime/wasm_runtime.dart';
import '../i18n/strings.dart';

class HomeScreen extends StatefulWidget {{
  const HomeScreen({{super.key}});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}}

class _HomeScreenState extends State<HomeScreen> {{
  final WasmRuntime _runtime = WasmRuntime();
  String _output = '';
  bool _isLoading = false;

  @override
  void initState() {{
    super.initState();
    _initRuntime();
  }}

  Future<void> _initRuntime() async {{
    setState(() => _isLoading = true);
    await _runtime.initialize();
    setState(() => _isLoading = false);
  }}

  Future<void> _runCode() async {{
    setState(() => _isLoading = true);
    final result = await _runtime.execute();
    setState(() {{
      _output = result;
      _isLoading = false;
    }});
  }}

  @override
  Widget build(BuildContext context) {{
    return Scaffold(
      appBar: AppBar(
        title: Text(AppStrings.appTitle),
        actions: [
          IconButton(
            icon: const Icon(Icons.settings),
            onPressed: () {{
              // فتح الإعدادات
            }},
          ),
        ],
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : SingleChildScrollView(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  // بطاقة الترحيب
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(20),
                      child: Column(
                        children: [
                          Icon(
                            Icons.code,
                            size: 64,
                            color: Theme.of(context).primaryColor,
                          ),
                          const SizedBox(height: 16),
                          Text(
                            AppStrings.welcome,
                            style: Theme.of(context).textTheme.headlineMedium,
                            textAlign: TextAlign.center,
                          ),
                          const SizedBox(height: 8),
                          Text(
                            AppStrings.welcomeMessage,
                            style: Theme.of(context).textTheme.bodyMedium,
                            textAlign: TextAlign.center,
                          ),
                        ],
                      ),
                    ),
                  ),
                  
                  const SizedBox(height: 24),
                  
                  // زر التشغيل
                  ElevatedButton.icon(
                    onPressed: _runCode,
                    icon: const Icon(Icons.play_arrow),
                    label: Text(AppStrings.run),
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.symmetric(vertical: 16),
                    ),
                  ),
                  
                  const SizedBox(height: 24),
                  
                  // النتيجة
                  if (_output.isNotEmpty)
                    Card(
                      child: Padding(
                        padding: const EdgeInsets.all(16),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Row(
                              children: [
                                const Icon(Icons.output),
                                const SizedBox(width: 8),
                                Text(
                                  AppStrings.output,
                                  style: Theme.of(context).textTheme.titleMedium,
                                ),
                              ],
                            ),
                            const SizedBox(height: 12),
                            Container(
                              width: double.infinity,
                              padding: const EdgeInsets.all(12),
                              decoration: BoxDecoration(
                                color: Theme.of(context).colorScheme.surface,
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: SelectableText(
                                _output,
                                style: const TextStyle(
                                  fontFamily: 'monospace',
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                ],
              ),
            ),
    );
  }}
}}
"#,
        project_name = config.project_name
    )
}

/// توليد wasm_runtime.dart
pub fn generate_wasm_runtime(_config: &MobileExportConfig) -> String {
    String::from(
r#"import 'package:flutter/services.dart';

class WasmRuntime {
  bool _initialized = false;
  String _output = '';

  /// تهيئة Runtime
  Future<void> initialize() async {
    if (_initialized) return;
    
    try {
      // تحميل ملف WASM
      final wasmBytes = await rootBundle.load('assets/wasm/app.wasm');
      
      // في الإنتاج، سنستخدم wasm_run أو مشابه
      // للآن، نحاكي التشغيل
      _output = 'تم تحميل WASM بنجاح';
      _initialized = true;
    } catch (e) {
      _output = 'خطأ في تحميل WASM: $e';
    }
  }

  /// تنفيذ الكود
  Future<String> execute() async {
    if (!_initialized) {
      await initialize();
    }
    
    // محاكاة التنفيذ
    return _output;
  }

  /// تنفيذ دالة معينة
  Future<String> callFunction(String name, List<dynamic> args) async {
    return 'تنفيذ الدالة: $name مع المعاملات: $args';
  }

  /// الحصول على قيمة متغير
  Future<dynamic> getVariable(String name) async {
    return null;
  }

  /// تعيين قيمة متغير
  Future<void> setVariable(String name, dynamic value) async {
    // تعيين القيمة
  }
}
"""
    )
}

/// توليد strings.dart للترجمة
pub fn generate_strings(config: &MobileExportConfig) -> String {
    format!(
r#"class AppStrings {{
  // عام
  static const String appTitle = '{}';
  static const String welcome = 'مرحباً بك';
  static const String welcomeMessage = 'تطبيق مولد بلغة المرجع - لغة برمجة عربية متكاملة';
  
  // الأزرار
  static const String run = 'تشغيل';
  static const String stop = 'إيقاف';
  static const String clear = 'مسح';
  static const String save = 'حفظ';
  static const String cancel = 'إلغاء';
  
  // النتائج
  static const String output = 'النتيجة';
  static const String error = 'خطأ';
  static const String success = 'نجاح';
  
  // القوائم
  static const String settings = 'الإعدادات';
  static const String about = 'حول';
  static const String help = 'مساعدة';
  
  // الإعدادات
  static const String theme = 'السمة';
  static const String language = 'اللغة';
  static const String arabic = 'العربية';
  static const String english = 'English';
  static const String darkMode = 'الوضع الداكن';
  static const String lightMode = 'الوضع الفاتح';
}}
""",
        config.project_name
    )
}

/// توليد Android manifest
pub fn generate_android_manifest(config: &MobileExportConfig) -> String {
    format!(
r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <application
        android:label="{}"
        android:name="${{applicationName}}"
        android:icon="@mipmap/ic_launcher"
        android:supportsRtl="true">
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:launchMode="singleTop"
            android:theme="@style/LaunchTheme"
            android:configChanges="orientation|keyboardHidden|keyboard|screenSize|smallestScreenSize|locale|layoutDirection|fontScale|screenLayout|density|uiMode"
            android:hardwareAccelerated="true"
            android:windowSoftInputMode="adjustResize">
            <meta-data
                android:name="io.flutter.embedding.android.NormalTheme"
                android:resource="@style/NormalTheme" />
            <intent-filter>
                <action android:name="android.intent.action.MAIN"/>
                <category android:name="android.intent.category.LAUNCHER"/>
            </intent-filter>
        </activity>
        <meta-data
            android:name="flutterEmbedding"
            android:value="2" />
    </application>
</manifest>
"#,
        config.project_name
    )
}

/// توليد iOS Info.plist
pub fn generate_ios_plist(config: &MobileExportConfig) -> String {
    format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>ar</string>
    <key>CFBundleDisplayName</key>
    <string>{}</string>
    <key>CFBundleExecutable</key>
    <string>$(EXECUTABLE_NAME)</string>
    <key>CFBundleIdentifier</key>
    <string>{}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>{}</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
    <key>UILaunchStoryboardName</key>
    <string>LaunchScreen</string>
    <key>UIMainStoryboardFile</key>
    <string>Main</string>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UISupportedInterfaceOrientations~ipad</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationPortraitUpsideDown</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UIViewControllerBasedStatusBarAppearance</key>
    <false/>
</dict>
</plist>
"#,
        config.project_name,
        config.package_name,
        config.project_name,
        config.version
    )
}
