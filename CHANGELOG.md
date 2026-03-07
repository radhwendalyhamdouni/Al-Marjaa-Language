# سجل التغييرات | Changelog

جميع التغييرات المهمة في هذا المشروع موثقة في هذا الملف.

All notable changes to this project are documented in this file.

---

## [3.3.0] - 2026-02

### 🆕 ميزات جديدة | New Features

#### LSP Server المتكامل والمتقدم | Complete Advanced LSP Server
- ✅ **LSP Server 3.3.0** (`editors/lsp-server/`)
  - خادم LSP كامل متكامل مع المحلل الأصلي
  - Semantic Tokens للتلوين الدلالي المتقدم
  - Code Actions للإصلاح السريع وإعادة البناء
  - Code Lens للعمليات التفاعلية السريعة
  - Inlay Hints لتلميحات الأنواع المضمنة
  - Call Hierarchy لتتبع الاستدعاءات
  - Rename الذكي لإعادة التسمية
  - Formatting التلقائي للتنسيق
  - Folding Ranges لطي الكود
  - Signature Help لمساعدة التواقيع
  - دعم محسن للغة العربية (RTL)
  - Type Inference لاستنتاج الأنواع
  - Workspace Symbols للبحث في المشروع
  - Linked Editing للتحرير المرتبط
  - ذاكرة مؤقتة ذكية (LRU Cache)
  - إحصائيات الأداء المدمجة

- ✅ **القدرات المدعومة**
  - `initialize` + `initialized` مع capability negotiation
  - `textDocument/didOpen` + `didChange` + `didClose`
  - `textDocument/publishDiagnostics` incremental
  - `textDocument/hover` + `definition` + `references`
  - `textDocument/completion` + `completionItem/resolve`
  - `textDocument/signatureHelp`
  - `textDocument/codeAction` + `codeAction/resolve`
  - `textDocument/codeLens` + `codeLens/resolve`
  - `textDocument/semanticTokens/full` + `range`
  - `textDocument/inlayHint` + `inlayHint/resolve`
  - `textDocument/foldingRange`
  - `textDocument/formatting` + `rangeFormatting`
  - `textDocument/prepareRename` + `rename`
  - `textDocument/documentSymbol` + `workspace/symbol`
  - `textDocument/prepareCallHierarchy`
  - `callHierarchy/incomingCalls` + `outgoingCalls`
  - `textDocument/linkedEditingRange`
  - `workspace/executeCommand`
  - `workspace/didChangeConfiguration`

- ✅ **الأوامر المخصصة**
  - `almarjaa.runFile` - تشغيل الملف
  - `almarjaa.format` - تنسيق الكود
  - `almarjaa.fixAll` - إصلاح كل المشاكل
  - `almarjaa.organizeImports` - تنظيم الاستيرادات
  - `almarjaa.extractFunction` - استخراج كدالة
  - `almarjaa.extractVariable` - استخراج كمتغير
  - `almarjaa.generateTests` - توليد اختبارات
  - `almarjaa.explainCode` - شرح الكود
  - `almarjaa.optimizeCode` - تحسين الكود
  - `almarjaa.convertToTypeScript` - تحويل لـ TypeScript
  - `almarjaa.convertToPython` - تحويل لـ Python

#### نظام الحزم الكامل | Complete Package Manager

- ✅ **نظام Lockfile المتقدم** (`src/package_manager/lockfile.rs`)
  - ملف قفل ذكي مع تجزئة المحتوى
  - ترتيب طوبولوجي للتبعيات
  - التحقق من التكامل
  - مقارنة ملفات القفل

- ✅ **التوزيع الثنائي** (`src/package_manager/binary.rs`)
  - دعم Linux (x86_64, aarch64)
  - دعم macOS (x86_64, aarch64)
  - دعم Windows (x86_64)
  - تحميل وتثبيت تلقائي
  - التحقق من التحديثات

- ✅ **توقيع الحزم** (`src/package_manager/signing.rs`)
  - Ed25519 و RSA للتوقيع
  - سجل المفاتيح الموثوقة
  - التحقق من صحة التوقيع
  - إدارة المفاتيح

- ✅ **نظام Workspace** (`src/package_manager/workspace.rs`)
  - دعم المشاريع المتعددة (Monorepo)
  - تبعيات مشتركة
  - بناء متوازي
  - فرز طوبولوجي

- ✅ **كلمات مفتاحية عربية جديدة**
  - `أنشئ_مشروع`, `أضف`, `أزل`, `ثبّت`
  - `ابحث`, `معلومات`, `قائمة`, `حدّث`
  - `انشر`, `نظّف`, `تحقق`
  - `حمّل`, `ابنِ`, `وقّع`

#### المكتبة القياسية الشاملة | Complete Standard Library

- ✅ **وحدة stdlib جديدة** (`src/stdlib/`)
  - `mod.rs`: الوحدة الرئيسية
  - `http/`: عميل وخادم HTTP المتقدم
  - `database/`: دعم قواعد البيانات المتعددة
  - `regex/`: التعابير النمطية المتقدمة
  - `crypto/`: التشفير والأمان الشامل

#### 1. وحدة HTTP المتقدمة
- ✅ **HTTP Client**
  - دعم GET, POST, PUT, DELETE, PATCH
  - إعدادات قابلة للتخصيص (timeout, retries, proxy)
  - دعم JSON و multipart/form-data
  - Connection pooling
  - Rate limiting
  
- ✅ **HTTP Server**
  - خادم HTTP كامل مع مسارات ديناميكية
  - برمجيات وسيطة (Middleware)
  - دعم WebSocket
  - Cookies & Sessions
  - CORS support

- ✅ **WebSocket**
  - اتصال ثنائي الاتجاه
  - بث الرسائل (Broadcast)
  - إدارة الاتصالات

- ✅ **كلمات مفتاحية عربية جديدة**
  - `احضر` (GET), `ارسل` (POST), `ضع` (PUT), `احذف` (DELETE)
  - `عميل_جديد`, `خادم_جديد`, `استجابة`

#### 2. وحدة Database الشاملة
- ✅ **قواعد البيانات المدعومة**
  - MySQL/MariaDB
  - PostgreSQL
  - SQLite (مضمن)
  - MongoDB (NoSQL)

- ✅ **Query Builder**
  - بناء استعلامات بطريقة السلسلة
  - استعلامات آمنة (Prepared Statements)
  - معاملات (Transactions)
  - Connection Pooling

- ✅ **كلمات مفتاحية عربية جديدة**
  - `اتصال_sqlite`, `اتصال_mysql`, `اتصال_postgres`, `اتصال_mongodb`
  - `استعلام`, `اختر`, `ادخل`, `عدل`, `احذف`

#### 3. وحدة Regex المتقدمة
- ✅ **التعابير النمطية**
  - Pattern matching كامل
  - Capture groups (مجموعات الالتقاط)
  - Named groups (مجموعات مسماة)
  - دعم Unicode العربي
  - استبدال وتقسيم النصوص

- ✅ **أنماط جاهزة**
  - التحقق من البريد الإلكتروني
  - التحقق من رقم الهاتف
  - التحقق من URL
  - التحقق من التاريخ والوقت
  - استخراج النصوص العربية

- ✅ **كلمات مفتاحية عربية جديدة**
  - `نمط`, `يطابق`, `ابحث`, `استبدل`, `قسم`

#### 4. وحدة Crypto الشاملة
- ✅ **دوال الهاش**
  - SHA-256, SHA-384, SHA-512
  - MD5, SHA-1 (للاستخدام غير الأمني)
  - HMAC-SHA256, HMAC-SHA512

- ✅ **التشفير المتماثل**
  - AES-128-GCM, AES-256-GCM
  - AES-128-CBC, AES-256-CBC
  - ChaCha20-Poly1305

- ✅ **التشفير غير المتماثل**
  - RSA-2048, RSA-4096
  - Ed25519 (توقيع رقمي)
  - توليد أزواج المفاتيح

- ✅ **تشفير كلمات المرور**
  - bcrypt
  - Argon2
  - PBKDF2
  - Scrypt
  - فحص قوة كلمة المرور

- ✅ **JWT Tokens**
  - إنشاء JWT
  - التحقق من JWT
  - claims مخصصة

- ✅ **توليد عشوائي آمن**
  - بايتات عشوائية
  - كلمات مرور عشوائية
  - UUID v4
  - رموز OTP

- ✅ **كلمات مفتاحية عربية جديدة**
  - `هاش`, `شا256`, `شفر`, `فك_التشفير`
  - `زوج_مفاتيح`, `وقع`, `تحقق`
  - `شفر_كلمة_المرور`, `تحقق_كلمة_المرور`
  - `أنشئ_jwt`, `تحقق_jwt`

### 📚 التوثيق | Documentation
- ✅ دليل المكتبة القياسية (`docs/STDLIB.md`)
- ✅ أمثلة شاملة لكل وحدة
- ✅ تحديث README.md
- ✅ تحديث Cargo.toml بالتبعيات الجديدة

### 🔧 التحسينات | Improvements
- تحديث الإصدار إلى 3.3.0
- إضافة 100+ كلمة مفتاحية عربية جديدة
- إضافة تبعيات: md5, sha1, sha2, hmac, hex
- تحسين هيكلية المشروع

---

## [3.2.0] - 2026-01-20

### 🆕 ميزات جديدة | New Features

#### نظام واجهات المستخدم المتكامل | Complete UI System

- ✅ **وحدة UI جديدة** (`src/ui/`)
  - `mod.rs`: الوحدة الرئيسية ومحرك UIEngine
  - `types.rs`: أنواع البيانات الأساسية (UIColor, UIFont, UIMargin, UIPadding, UISize, UIPosition, UIRect, UIPoint, UITransform)
  - `layout.rs`: نظام التخطيط التلقائي (Row, Column, Grid, FlexContainer)
  - `binding.rs`: ربط البيانات التلقائي (Observable, Computed, Watcher, DataContext)
  - `components.rs`: المكونات القابلة لإعادة الاستخدام (Button, TextField, Card, Table, etc.)
  - `responsive.rs`: التصميم المتجاوب (Breakpoints, MediaQuery, ResponsiveValue)
  - `themes.rs`: نظام الثيمات (Theme, ThemeColors, ThemeTypography, ThemeManager)
  - `charts.rs`: الرسوم البيانية (LineChart, BarChart, PieChart, AreaChart)
  - `modals.rs`: النوافذ المنبثقة (Modal, Toast, AlertDialog, ConfirmDialog)
  - `widgets.rs`: الأدوات المتقدمة (DropDown, Rating, ColorPicker, DataGrid, Kanban)
  - `animations.rs`: الرسوم المتحركة (Animation, Transition, Keyframes, Easing)
  - `events.rs`: نظام الأحداث (EventManager, EventDispatcher, EventListener)
  - `styling.rs`: نظام التنسيق (Style, StyleSheet, CSSRule, Gradient, Shadow, Transform)

- ✅ **كلمات مفتاحية عربية جديدة للواجهات**
  - **التخطيط**: `صف` (Row), `عمود` (Column), `شبكة` (Grid), `مرن` (Flex), `كومة` (Stack), `التفاف` (Wrap), `فجوة` (Gap), `محاذاة` (Align), `تبرير` (Justify), `حشو` (Padding), `هامش` (Margin)
  - **المكونات**: `زر` (Button), `نص` (Text), `إدخال` (Input), `اختيار` (Select), `خانة` (Checkbox), `راديو` (Radio), `منزلق` (Slider), `تقدم` (Progress), `مؤقت` (Spinner), `بطاقة` (Card), `قائمة` (List), `جدول` (Table), `نموذج` (Form), `تسمية` (Label), `صورة` (Image), `أيقونة` (Icon), `شارة` (Badge), `صورة_شخصية` (Avatar), `تلميح` (Tooltip), `نافذة` (Modal), `تنبيه` (Toast), `منبثق` (Popup)
  - **الثيمات**: `لون` (Color), `خط` (Font), `حجم` (Size), `عرض` (Width), `ارتفاع` (Height), `حدود` (Border), `ظل` (Shadow), `خلفية` (Background)
  - **الرسوم البيانية**: `رسم` (Chart), `رسم_خطي` (LineChart), `رسم_أعمدة` (BarChart), `رسم_دائري` (PieChart), `رسم_مساحي` (AreaChart)
  - **الرسوم المتحركة**: `حرك` (Animate), `انتقال` (Transition), `مدة` (Duration), `تأخير` (Delay), `تخفيف` (Easing)
  - **الأحداث**: `نقر` (Click), `تغيير` (Change), `إرسال` (Submit), `تركيز` (Focus), `ضبابية` (Blur), `تحويم` (Hover), `تمرير` (Scroll)
  - **الربط**: `ربط` (Bind), `راقب` (Observe), `محسوب` (Computed), `راقب_التغييرات` (Watch)

### 🔴 الميزات الحرجية (الأساسية)

#### 1. نظام التخطيط التلقائي
- ✅ **Row**: تخطيط أفقي مع دعم Gap و Align و Justify
- ✅ **Column**: تخطيط عمودي مع دعم Gap و Align
- ✅ **Grid**: تخطيط شبكي مع دعم GridTrack و GridPlacement
- ✅ **FlexContainer**: دعم Flexbox كامل (FlexDirection, FlexWrap, JustifyContent, AlignItems)

#### 2. ربط البيانات التلقائي
- ✅ **ObservableValue**: قيم قابلة للملاحظة مع إشعارات التغيير
- ✅ **Binding**: ربط أحادي وثنائي الاتجاه
- ✅ **Computed**: قيم محسوبة تلقائياً
- ✅ **Watcher**: مراقبة التغييرات

#### 3. المكونات القابلة لإعادة الاستخدام
- ✅ 30+ مكون جاهز للاستخدام
- ✅ نظام Props للحفاظ على حالة المكون
- ✅ ComponentBuilder لنشاء مكونات بطريقة السلسلة

### 🟡 الميزات المهمة (الثانوية)

#### 4. التصميم المتجاوب
- ✅ **Breakpoints**: نقاط التوقف (xs, sm, md, lg, xl, xxl)
- ✅ **MediaQuery**: استعلامات الوسائط
- ✅ **ResponsiveValue**: قيم متغيرة حسب حجم الشاشة
- ✅ **DeviceType**: كشف نوع الجهاز (Mobile, Tablet, Desktop)

#### 5. نظام الثيمات
- ✅ **ThemeColors**: ألوان الثيم (Primary, Secondary, Success, Warning, Error)
- ✅ **ThemeTypography**: خطوط الثيم
- ✅ **ThemeManager**: إدارة الثيمات مع التبديل
- ✅ **ثيمات افتراضية**: Light, Dark, Arabic

#### 6. الرسوم البيانية
- ✅ **LineChart**: رسم بياني خطي
- ✅ **BarChart**: رسم بياني أعمدة
- ✅ **PieChart**: رسم بياني دائري
- ✅ **AreaChart**: رسم بياني مساحي
- ✅ تصدير SVG للرسوم البيانية

#### 7. النوافذ المنبثقة
- ✅ **Modal**: نوافذ منبثقة قابلة للتخصيص
- ✅ **AlertDialog**: حوار تنبيه
- ✅ **ConfirmDialog**: حوار تأكيد
- ✅ **Toast**: تنبيهات سريعة
- ✅ **Notification**: إشعارات النظام

### 📚 التوثيق | Documentation
- ✅ مثال UI شامل (`examples/ui_basics.mrj`)
- ✅ تحديث README.md بنظام UI
- ✅ تحديث lib.rs بالتصديرات الجديدة

### 🔧 التحسينات | Improvements
- تحديث الإصدار إلى 3.2.0
- إضافة 70+ كلمة مفتاحية عربية جديدة للواجهات
- إضافة توكنات جديدة في Lexer
- تحسين بنية المشروع

---

## [3.1.0] - 2026-01-15

### 🆕 ميزات جديدة | New Features

#### دعم ONNX الكامل | Full ONNX Support
- ✅ **وحدة ONNX جديدة** (`src/onnx/`)
  - `mod.rs`: الوحدة الرئيسية والأنواع الأساسية
  - `types.rs`: أنواع البيانات (ONNXTensor, ONNXShape, ONNXDataType)
  - `engine.rs`: محرك ONNX (ONNXEngine, ONNXSession, ONNXConfig)
  - `export.rs`: تصدير النماذج (ONNXExporter, ONNXGraphBuilder)
  - `inference.rs`: الاستدلال (ONNXInference, InferenceOptions)
  - `utils.rs`: أدوات مساعدة ونماذج مسبقة

- ✅ **كلمات مفتاحية عربية جديدة لـ ONNX**
  - `أونكس` (onnx) - الوصول لوحدة ONNX
  - `نموذج` (model) - تعريف نموذج
  - `حمّل` (load) - تحميل نموذج
  - `احفظ` (save) - حفظ نموذج
  - `استدل` (infer) - تشغيل الاستدلال
  - `موتر` (tensor) - تعريف موتر
  - `شكل` (shape) - شكل الموتر
  - `طبقة` (layer) - طبقة عصبية
  - `كثيف` (dense) - طبقة كثيفة/خطية
  - `التفاف` (conv) - طبقة التفاف
  - `تجميع` (pool) - طبقة تجميع
  - `طبع` (normalize) - طبقة تسوية
  - `إسقاط` (dropout) - طبقة إسقاط
  - `تسوية` (flatten) - تسوية الأبعاد
  - `إعادة_تشكيل` (reshape) - إعادة تشكيل الموتر
  - `تنشيط` (activation) - دالة تنشيط
  - `سوفت_ماكس` (softmax) - دالة Softmax
  - `ريلو` (relu) - دالة ReLU
  - `سيجمويد` (sigmoid) - دالة Sigmoid
  - `دفعة` (batch) - حجم الدفعة
  - `درّب` (train) - تدريب النموذج
  - `توقع` (predict) - التنبؤ
  - `محسّن` (optimizer) - محسّن التدريب
  - `خسارة` (loss) - دالة الخسارة

- ✅ **أنواع بيانات ONNX المدعومة**
  - Float32, Float64, Float16
  - Int32, Int64, Int8, Int16
  - UInt8, UInt16
  - Bool, String
  - BFloat16, Complex64, Complex128

- ✅ **الطبقات المدعومة للتصدير**
  - Dense/Linear
  - Conv2D
  - MaxPool2D, AveragePool2D
  - BatchNorm
  - Dropout
  - Flatten, Reshape
  - ReLU, Sigmoid, Tanh, Softmax

- ✅ **نماذج مسبقة متاحة**
  - ResNet-50 للتصنيف
  - BERT Base للنصوص
  - YOLOv5 Small للكشف
  - MobileNetV2 للتصنيف السريع

### 📚 التوثيق | Documentation
- ✅ دليل ONNX الشامل (`docs/ONNX_SUPPORT.md`)
- ✅ مثال ONNX كامل (`examples/onnx_basics.mrj`)
- ✅ اختبارات ONNX (`tests/onnx_tests.rs`)
- ✅ تحديث README.md بدعم ONNX

### 🔧 التحسينات | Improvements
- تحديث Cargo.toml بإضافة `ort`, `ndarray`, `zip`, `base64`
- إضافة ميزة `onnx` في features
- تحديث هيكلية المشروع في README

---

## [3.0.0] - 2025-12-01

### 🚀 ميزات رئيسية | Major Features

#### JIT Compiler المتقدم
- ✅ **5-Tier JIT Compiler**
  - Tier 0: Interpreter Baseline
  - Tier 1: Baseline JIT (50 تنفيذ)
  - Tier 2: Optimizing JIT (200 تنفيذ)
  - Tier 3: SIMD Optimizations (1000 تنفيذ)
  - Tier 4: Tracing JIT (5000 تنفيذ)

- ✅ **نتائج الأداء**
  - نسبة التسريع: 5.08x
  - العمليات الحسابية: 3.6M عملية/ثانية
  - اختبار الضغط: 19.9M عملية/ثانية

#### Bytecode VM
- ✅ 57 تعليمة bytecode
- ✅ Stack-based VM
- ✅ Garbage Collector متوازي

#### Vibe Coding
- ✅ البرمجة باللغة الطبيعية العربية
- ✅ تكامل GGUF مع نماذج LLM
- ✅ محرك موحد (UnifiedVibeEngine)

#### ميزات متقدمة
- ✅ AutoGrad للاشتقاق التلقائي
- ✅ GPU Support (wgpu)
- ✅ Package Manager لامركزي
- ✅ VS Code Extension 2.0
- ✅ LSP Server

### 📚 التوثيق
- دليل المستخدم الشامل
- مرجع API
- الهيكلية التقنية
- تقييم Vibe Coding العلمي
- نتائج الاختبارات

---

## [2.0.0] - 2025-06-15

### 🆕 ميزات جديدة
- Bytecode VM كامل
- JIT Compiler مبدئي
- AutoGrad للشبكات العصبية
- Package Manager
- VS Code Extension

---

## [1.0.0] - 2025-01-01

### 🎉 الإصدار الأولي
- Lexer عربي كامل
- Parser للمركبات النحوية
- Interpreter للمفسّر
- أنواع بيانات أساسية
- دوال مدمجة
- البرمجة الكائنية

---

## تنسيق السجل

- `🆕 ميزات جديدة` للميزات الجديدة
- `🔧 التحسينات` للتحسينات على الميزات الموجودة
- `📚 التوثيق` للتغييرات في التوثيق
- `🐛 إصلاحات` لإصلاح الأخطاء
- `🔒 أمان` للتحديثات الأمنية
- `💔 تغييرات كسرية` للتغييرات غير المتوافقة

---

© 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
جميع الحقوق محفوظة | All Rights Reserved
