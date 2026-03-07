# 📚 دليل نشر إضافة VS Code على Marketplace

## دليل شامل خطوة بخطوة باللغة العربية

---

## 📋 المحتويات

1. [المتطلبات](#المتطلبات)
2. [الخطوة 1: إنشاء حساب Microsoft](#الخطوة-1-إنشاء-حساب-microsoft)
3. [الخطوة 2: إنشاء Publisher](#الخطوة-2-إنشاء-publisher)
4. [الخطوة 3: إنشاء Personal Access Token](#الخطوة-3-إنشاء-personal-access-token)
5. [الخطوة 4: إعداد البيئة](#الخطوة-4-إعداد-البيئة)
6. [الخطوة 5: التحقق من الملفات](#الخطوة-5-التحقق-من-الملفات)
7. [الخطوة 6: النشر](#الخطوة-6-النشر)
8. [بعد النشر](#بعد-النشر)
9. [تحديث الإضافة](#تحديث-الإضافة)
10. [استكشاف الأخطاء](#استكشاف-الأخطاء)

---

## المتطلبات

### ما ستحتاجه:

| المتطلب | الوصف | الحصول عليه |
|---------|-------|-------------|
| حساب Microsoft | مجاني | [signup.live.com](https://signup.live.com) |
| حساب GitHub | مجاني (اختياري) | [github.com](https://github.com) |
| Node.js | الإصدار 16 أو أحدث | [nodejs.org](https://nodejs.org) |
| VS Code | أي إصدار | [code.visualstudio.com](https://code.visualstudio.com) |

### الوقت المتوقع:
- 🕐 30-45 دقيقة (للمرة الأولى)
- 🕐 5-10 دقائق (للتحديثات)

---

## الخطوة 1: إنشاء حساب Microsoft

### إذا كان لديك حساب Microsoft/GitHub/Outlook:

1. افتح المتصفح
2. اذهب إلى: https://marketplace.visualstudio.com/
3. اضغط على **"Sign in"** في أعلى الصفحة

![Sign in](https://docs.microsoft.com/en-us/visualstudio/extensibility/images/signin.png)

4. اختر طريقة تسجيل الدخول:
   - **GitHub**: استخدم حساب GitHub
   - **Microsoft**: استخدم حساب Outlook/Hotmail
   - **Azure DevOps**: إذا كان لديك حساب

### إذا لم يكن لديك حساب:

1. اذهب إلى: https://signup.live.com/
2. املأ البيانات المطلوبة
3. أكد البريد الإلكتروني
4. اكتمل التسجيل! ✅

---

## الخطوة 2: إنشاء Publisher

الـ **Publisher** هو اسم فريد يُنسب إليك جميع إضافاتك.

### الطريقة:

1. بعد تسجيل الدخول في Marketplace
2. اضغط على **"Publish extensions"** أو:
   - اذهب إلى: https://marketplace.visualstudio.com/manage
3. سيُطلب منك إنشاء Publisher

### إدخال البيانات:

```
Publisher ID: almarjaa
             ───────────
             (يجب أن يكون فريداً، أحرف صغيرة، بدون مسافات)

Publisher Name: Al-Marjaa Language
               ─────────────────────
               (الاسم المعروض للمستخدمين)

Description: لغة المرجع - لغة برمجة عربية متكاملة
            ───────────────────────────────────────────
            (وصف مختصر)
```

### أو من سطر الأوامر:

```bash
# تثبيت vsce
npm install -g @vscode/vsce

# إنشاء publisher
vsce create-publisher almarjaa
```

---

## الخطوة 3: إنشاء Personal Access Token

الـ **Personal Access Token (PAT)** هو مفتاح أمان للنشر التلقائي.

### الخطوات بالتفصيل:

#### 1. اذهب إلى Azure DevOps
```
https://dev.azure.com/
```

#### 2. سجل الدخول
استخدم نفس حساب Microsoft الذي استخدمته في Marketplace

#### 3. افتح إعدادات المستخدم
- اضغط على أيقونة **الترس** ⚙️ في أعلى اليمين
- اختر **"Personal access tokens"**

![Settings](https://docs.microsoft.com/en-us/azure/devops/organizations/media/create-personal-access-token/user-settings.png)

#### 4. إنشاء Token جديد
- اضغط **"+ New Token"**

#### 5. إدخال البيانات:

```
┌─────────────────────────────────────────────────────────────┐
│ Name: VSCode Extension Publishing                            │
│ ────────────────────────────────────────────────            │
│ (اسم تعريفي للـ Token)                                      │
│                                                              │
│ Organization: All accessible organizations                   │
│ ────────────────────────────────────────────────            │
│ (مهم: اختر "All" للوصول لكل المنظمات)                       │
│                                                              │
│ Expiration: 90 days                                          │
│ ────────────────────────────────────────────────            │
│ (يمكن اختيار 180 يوم أو سنة مخصصة)                          │
│                                                              │
│ Scopes:                                                      │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ ☑ Marketplace                                           │ │
│ │   └─ ☑ Manage                                           │ │
│ │      (مهم جداً: هذا هو الصلاحية المطلوبة للنشر)         │ │
│ └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

#### 6. إنشاء ونسخ Token

⚠️ **تحذير مهم جداً!**
```
┌─────────────────────────────────────────────────────────────┐
│ ⚠️ انسخ الـ Token فوراً واحفظه في مكان آمن!               │
│                                                             │
│ لن تتمكن من رؤيته مرة أخرى بعد إغلاق الصفحة!               │
│                                                             │
│ مثال على شكل Token:                                         │
│ eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsIng1...               │
└─────────────────────────────────────────────────────────────┘
```

---

## الخطوة 4: إعداد البيئة

### 1. تثبيت Node.js

```bash
# التحقق من التثبيت
node --version   # يجب أن يكون v16 أو أحدث
npm --version    # يجب أن يكون 7 أو أحدث
```

### 2. تثبيت أداة vsce

```bash
npm install -g @vscode/vsce
```

### 3. التحقق من التثبيت

```bash
vsce --version
# يجب أن يظهر رقم الإصدار
```

### 4. تسجيل الدخول (اختياري)

```bash
vsce login almarjaa
# سيطلب إدخال الـ Token
```

---

## الخطوة 5: التحقق من الملفات

### الملفات المطلوبة:

```
editors/vscode/
├── package.json        ✅ مطلوب - ملف الإعدادات الرئيسي
├── README.md           ✅ مطلوب - وصف الإضافة
├── LICENSE             ✅ مطلوب - ملف الترخيص
├── CHANGELOG.md        📝 موصى به - سجل التغييرات
├── icons/
│   └── icon.png        ✅ مطلوب - أيقونة الإضافة (128x128)
├── syntaxes/
│   └── almarjaa.tmLanguage.json
├── snippets/
│   └── almarjaa.json
├── themes/
│   ├── almarjaa-dark.json
│   └── almarjaa-arabic-night.json
├── src/
│   └── extension.ts
├── out/                📁 يُنشأ عند الترجمة
└── node_modules/       📁 يُنشأ عند تثبيت الحزم
```

### التحقق من package.json:

```json
{
  "name": "almarjaa-language",
  "displayName": "Al-Marjaa Language - لغة المرجع",
  "description": "دعم شامل للغة المرجع العربية",
  "version": "2.0.0",
  "publisher": "almarjaa",
  "license": "MIT",
  "icon": "icons/icon.png",
  "repository": {
    "type": "git",
    "url": "https://github.com/.../Al-Marjaa-Language"
  },
  ...
}
```

### التحقق من الأخطاء:

```bash
# في مجلد الإضافة
cd editors/vscode

# تثبيت الحزم
npm install

# ترجمة TypeScript
npm run compile

# فحص الأخطاء
npm run lint
```

---

## الخطوة 6: النشر

### 1. تجهيز الإضافة

```bash
# الانتقال لمجلد الإضافة
cd editors/vscode

# تثبيت الحزم
npm install

# ترجمة الكود
npm run compile
```

### 2. إنشاء ملف VSIX (اختياري - للاختبار)

```bash
vsce package
```

هذا يُنشئ ملف: `almarjaa-language-2.0.0.vsix`

### 3. اختبار الملف محلياً

```bash
# في VS Code
code --install-extension almarjaa-language-2.0.0.vsix
```

### 4. النشر على Marketplace

#### الطريقة الأولى: استخدام Token مباشرة

```bash
vsce publish --pat YOUR_TOKEN_HERE
```

#### الطريقة الثانية: تسجيل دخول مسبق

```bash
# تسجيل الدخول (مرة واحدة فقط)
vsce login almarjaa
# أدخل الـ Token

# النشر
vsce publish
```

#### الطريقة الثالثة: نشر بإصدار جديد

```bash
# نشر مع زيادة رقم الإصدار تلقائياً
vsce publish patch  # 2.0.0 → 2.0.1
vsce publish minor  # 2.0.0 → 2.1.0
vsce publish major  # 2.0.0 → 3.0.0
```

### 5. رسالة النجاح

```
✅ Publishing almarjaa.almarjaa-language@2.0.0...
✅ Extension published!
✅ https://marketplace.visualstudio.com/items?itemName=almarjaa.almarjaa-language
```

---

## بعد النشر

### التحقق من النشر:

1. اذهب إلى: https://marketplace.visualstudio.com/manage
2. ستجد الإضافة في القائمة
3. اضغط عليها لرؤية التفاصيل

### ما يحدث تلقائياً:

- ✅ تظهر الإضافة في Marketplace
- ✅ يمكن البحث عنها في VS Code
- ✅ يمكن تثبيتها مباشرة
- ✅ تتوفر إحصائيات التنزيلات

### رابط الإضافة:

```
https://marketplace.visualstudio.com/items?itemName=almarjaa.almarjaa-language
```

### أمر التثبيت للمستخدمين:

```
ext install almarjaa.almarjaa-language
```

---

## تحديث الإضافة

### خطوات التحديث:

#### 1. تحديث الكود
```bash
# إجراء التغييرات على الكود
git add .
git commit -m "✨ إضافة ميزة جديدة"
git push
```

#### 2. تحديث CHANGELOG.md
```markdown
## [2.1.0] - 2024-02-01

### Added
- ميزة جديدة رقم 1
- ميزة جديدة رقم 2
```

#### 3. تحديث الإصدار في package.json
```json
{
  "version": "2.1.0"
}
```

#### 4. النشر
```bash
vsce publish
```

أو مباشرة:
```bash
vsce publish minor  # يزيد الإصدار تلقائياً
```

---

## استكشاف الأخطاء

### خطأ: "Publisher not found"

```
Error: Publisher 'almarjaa' not found
```

**الحل:**
1. تأكد من إنشاء Publisher
2. تحقق من كتابة الاسم صحيحاً (أحرف صغيرة)

---

### خطأ: "Invalid Personal Access Token"

```
Error: Invalid Personal Access Token
```

**الحل:**
1. تأكد من صلاحية الـ Token
2. تأكد من اختيار صلاحية "Marketplace > Manage"
3. أنشئ Token جديداً إذا انتهت صلاحيته

---

### خطأ: "Missing required field"

```
Error: Missing required field 'README.md'
```

**الحل:**
```bash
# تأكد من وجود الملفات
ls README.md LICENSE CHANGELOG.md icons/icon.png
```

---

### خطأ: "File too large"

```
Error: File size exceeds the limit
```

**الحل:**
- الحد الأقصى: 50 ميجابايت
- أزل الملفات غير الضرورية
- أضف ملف `.vscodeignore`:

```
.vscodeignore
.gitignore
**/*.ts
**/node_modules/**
src/**
test/**
```

---

### خطأ: "Missing icon"

```
Error: An icon is required
```

**الحل:**
1. أنشئ أيقونة PNG بحجم 128x128 أو 256x256
2. ضعها في `icons/icon.png`
3. أضف في `package.json`:
```json
{
  "icon": "icons/icon.png"
}
```

---

## 🎉 مبروك!

بعد إكمال هذه الخطوات، ستكون إضافتك منشورة على VS Code Marketplace ويمكن لملايين المطورين تثبيتها واستخدامها!

---

## 📞 المساعدة

- 📖 [توثيق VS Code Extensions](https://code.visualstudio.com/api)
- 💬 [GitHub Discussions](https://github.com/microsoft/vscode-discussions)
- 📧 [دعم Microsoft](https://support.microsoft.com)

---

## ملخص سريع

```bash
# 1. تثبيت الأدوات
npm install -g @vscode/vsce

# 2. تسجيل الدخول
vsce login almarjaa

# 3. في مجلد الإضافة
cd editors/vscode
npm install
npm run compile

# 4. النشر
vsce publish

# ✅ انتهى!
```

---

<div align="center">

**صنع بـ ❤️ للمجتمع العربي**

</div>
