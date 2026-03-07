# تصدير الهواتف المحمولة - لغة المرجع

## نظرة عامة

يدعم لغة المرجع تصدير التطبيقات إلى منصات الهواتف المحمولة (Android و iOS) باستخدام عدة أطر عمل:

| الإطار | المنصات | الصعوبة | الأداء | موصى به |
|--------|---------|---------|--------|---------|
| Flutter | Android, iOS | سهل | ممتاز | ✅ نعم |
| React Native | Android, iOS | متوسط | جيد | ✅ نعم |
| Native | Android أو iOS | متقدم | ممتاز | للمحترفين |
| Capacitor | Android, iOS, Web | سهل | جيد | للمشاريع الويب |

## التثبيت

### Flutter
```bash
# تثبيت Flutter
# من https://flutter.dev

# التحقق من التثبيت
flutter doctor
```

### React Native
```bash
# تثبيت Node.js
# من https://nodejs.org

# إنشاء مشروع جديد
npx react-native init MyApp
```

### Capacitor
```bash
# تثبيت Node.js ثم
npm install @capacitor/cli @capacitor/core
```

## الاستخدام

### سطر الأوامر

```bash
# تصدير إلى Android باستخدام Flutter
almarjaa mobile export app.mrj -p android -f flutter -n my_app

# تصدير إلى iOS باستخدام React Native
almarjaa mobile export app.mrj -p ios -f react-native -n my_app

# تصدير إلى كلا المنصتين
almarjaa mobile export app.mrj -p both -f flutter -n my_app

# عرض الخيارات المتاحة
almarjaa mobile list
```

### المنصات المدعومة

| الاسم العربي | الاسم الإنجليزي | الوصف |
|--------------|-----------------|-------|
| أندرويد | android | نظام Android |
| آيفون | ios | نظام iOS |
| كلاهما | both | المنصتان معاً |

### الأطر المدعومة

| الاسم العربي | الاسم الإنجليزي | الوصف |
|--------------|-----------------|-------|
| فلاتر | flutter | إطار Google |
| ريأكت | react-native | إطار Meta |
| أصلي | native | كود Kotlin/Swift |
| كاباسيتور | capacitor | إطار Ionic |

## بنية المشروع المُصدّر

### Flutter
```
my_app/
├── lib/
│   ├── main.dart           # نقطة الدخول
│   ├── app_theme.dart      # الثيم
│   ├── screens/
│   │   └── home_screen.dart
│   ├── runtime/
│   │   └── wasm_runtime.dart
│   └── i18n/
│       └── strings.dart
├── assets/
│   └── wasm/
│       └── app.wasm
├── android/
├── ios/
└── pubspec.yaml
```

### React Native
```
my_app/
├── App.tsx                 # نقطة الدخول
├── src/
│   ├── screens/
│   │   └── HomeScreen.tsx
│   ├── runtime/
│   │   └── wasmRuntime.ts
│   └── i18n/
│       └── rtl.ts
├── assets/
│   └── wasm/
│       └── app.wasm
├── android/
├── ios/
└── package.json
```

### Android Native
```
android/
├── app/
│   ├── src/main/
│   │   ├── java/com/almarjaa/app/
│   │   │   ├── MainActivity.kt
│   │   │   └── WasmRuntime.kt
│   │   ├── res/
│   │   │   ├── layout/
│   │   │   ├── values/
│   │   │   └── values-ar/
│   │   └── assets/wasm/
│   └── build.gradle
├── settings.gradle
└── build.gradle
```

### iOS Native
```
ios/
├── AlmarjaaApp/
│   ├── AppDelegate.swift
│   ├── ViewController.swift
│   ├── WasmRuntime.swift
│   ├── Info.plist
│   └── Resources/
│       ├── Base.lproj/
│       ├── ar.lproj/
│       └── app.wasm
└── AlmarjaaApp.xcodeproj
```

## الـ WASM Runtime

### كيف يعمل

1. يتم تحويل كود المرجع إلى WebAssembly
2. يتم تحميل WASM في التطبيق المحمول
3. يتم تنفيذ الكود عبر JavaScript bridge أو native WASM runtime

### الأداء

| المنصة | طريقة التنفيذ | الأداء النسبي |
|--------|---------------|---------------|
| Flutter | wasm_run | 95% |
| React Native | react-native-wasm | 90% |
| Android Native | wasmer | 98% |
| iOS Native | wasmer | 98% |

## دعم RTL

جميع القوالب تدعم RTL تلقائياً:

- Flutter: `textDirection: TextDirection.rtl`
- React Native: `I18nManager.forceRTL(true)`
- Android: `android:supportsRtl="true"`
- iOS: `semanticContentAttribute = .forceRightToLeft`

## أوامر البناء

### Flutter
```bash
cd my_app
flutter pub get
flutter build apk --release      # Android
flutter build ios --release      # iOS
```

### React Native
```bash
cd my_app
npm install
npx react-native run-android     # Android
cd ios && pod install && cd ..
npx react-native run-ios         # iOS
```

### Android Native
```bash
cd android
./gradlew assembleDebug
./gradlew assembleRelease
```

### iOS Native
```bash
cd ios
xcodebuild -scheme AlmarjaaApp -sdk iphoneos
```

## التخصيص

### تغيير اللون الرئيسي

```bash
# في ملف التكوين
primary_color: "#FF5722"
```

### تغيير اسم الحزمة

```bash
# في ملف التكوين
package_name: "com.mycompany.myapp"
```

### تفعيل وضع الإصدار

```bash
# في ملف التكوين
release_mode: true
signing: true
keystore_path: "./my-release-key.jks"
```

## أمثلة

### مثال بسيط

```almarjaa
// app.mrj
اطبع("مرحباً بالعالم!")

// تصدير
// almarjaa mobile export app.mrj -p android -f flutter
```

### مثال مع واجهة

```almarjaa
// calculator.mrj
أنشئ نافذة بعنوان "حاسبة"
أضف حقل تلميح "أدخل الرقم الأول"
أضف حقل تلميح "أدخل الرقم الثاني"
أضف زر مكتوب عليه "اجمع" باللون أخضر
أضف تسمية نصها "النتيجة:"

// تصدير
// almarjaa mobile export calculator.mrj -p both -f flutter -n calculator_app
```

## استكشاف الأخطاء

### Flutter: "Flutter not found"
```bash
# تأكد من تثبيت Flutter
export PATH="$PATH:/path/to/flutter/bin"
flutter doctor
```

### React Native: "Node not found"
```bash
# تأكد من تثبيت Node.js
node --version
npm --version
```

### iOS: "Building iOS requires macOS"
iOS builds can only be done on macOS. Use a cloud service like:
- Codemagic
- Bitrise
- GitHub Actions (macOS runner)

## المساهمة

للمساهمة في تطوير تصدير الهواتف:

1. Fork المشروع
2. أنشئ فرعاً جديداً
3. أضف تحسيناتك
4. أرسل Pull Request

---

© 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
جميع الحقوق محفوظة | All Rights Reserved
