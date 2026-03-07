# 📥 دليل التثبيت - Installation Guide

دليل شامل لتثبيت لغة المرجع على جميع أنظمة التشغيل.

---

## 📋 المتطلبات

| المتطلب | الإصدار المطلوب |
|---------|----------------|
| Rust | 1.70+ |
| Git | أي إصدار |
| مساحة القرص | ~500 MB |

---

## 🚀 التثبيت السريع

### 🐧 Linux (Debian/Ubuntu)

```bash
# تحميل وتشغيل سكربت التثبيت
curl -fsSL https://raw.githubusercontent.com/radhwendalyhamdouni/Al-Marjaa-Language/main/install/linux/install.sh | bash

# أو يدوياً
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language
cargo build --release
sudo ln -s $(pwd)/target/release/almarjaa /usr/local/bin/almarjaa
```

### 🪟 Windows

```powershell
# في PowerShell (Run as Administrator)
Set-ExecutionPolicy Bypass -Scope Process -Force
Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/radhwendalyhamdouni/Al-Marjaa-Language/main/install/windows/install.ps1'))
```

أو يدوياً:
1. ثبّت [Rust](https://rustup.rs/)
2. ثبّت [Git](https://git-scm.com/download/win)
3. افتح Command Prompt:
```cmd
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language
cargo build --release
```

### 🍎 macOS

```bash
# تحميل وتشغيل سكربت التثبيت
curl -fsSL https://raw.githubusercontent.com/radhwendalyhamdouni/Al-Marjaa-Language/main/install/macos/install.sh | bash

# أو يدوياً
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language
cargo build --release
sudo ln -s $(pwd)/target/release/almarjaa /usr/local/bin/almarjaa
```

---

## 📖 التثبيت التفصيلي

### 🐧 Linux (Debian/Ubuntu)

#### الخطوة 1: تحديث النظام
```bash
sudo apt update && sudo apt upgrade -y
```

#### الخطوة 2: تثبيت المتطلبات
```bash
sudo apt install -y build-essential curl wget git pkg-config libssl-dev cmake
```

#### الخطوة 3: تثبيت Rust
```bash
# تحميل rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# تفعيل Rust في الجلسة الحالية
source $HOME/.cargo/env

# التحقق من التثبيت
rustc --version
cargo --version
```

#### الخطوة 4: استنساخ المشروع
```bash
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language
```

#### الخطوة 5: البناء
```bash
# تحميل المكتبات
cargo fetch

# البناء
cargo build --release
```

#### الخطوة 6: التثبيت
```bash
# إنشاء رابط رمزي
sudo ln -sf $(pwd)/target/release/almarjaa /usr/local/bin/almarjaa

# التحقق
almarjaa --version
```

---

### 🪟 Windows

#### الخطوة 1: تثبيت Rust

1. اذهب إلى https://rustup.rs/
2. حمّل `rustup-init.exe`
3. شغّل المثبت واختر الخيارات الافتراضية
4. أعد تشغيل Terminal

#### الخطوة 2: تثبيت Git

1. اذهب إلى https://git-scm.com/download/win
2. حمّل المثبت
3. شغّل المثبت

#### الخطوة 3: استنساخ المشروع

افتح **PowerShell** أو **Command Prompt**:
```cmd
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language
```

#### الخطوة 4: البناء
```cmd
cargo build --release
```

#### الخطوة 5: إضافة إلى PATH

```powershell
# إضافة المجلد إلى PATH
$binPath = "$(Get-Location)\target\release"
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", "User") + ";$binPath",
    "User"
)
```

#### الخطوة 6: التحقق
```cmd
almarjaa --version
```

---

### 🍎 macOS

#### الخطوة 1: تثبيت Homebrew (إذا لم يكن مثبتاً)
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

#### الخطوة 2: تثبيت المتطلبات
```bash
brew install curl wget git cmake
```

#### الخطوة 3: تثبيت Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
```

#### الخطوة 4: استنساخ المشروع
```bash
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language
```

#### الخطوة 5: البناء
```bash
cargo build --release
```

#### الخطوة 6: التثبيت
```bash
sudo ln -sf $(pwd)/target/release/almarjaa /usr/local/bin/almarjaa
almarjaa --version
```

---

## 🤖 تحميل نموذج AI (اختياري)

لتفعيل ميزات Vibe Coding، حمّل نموذج AI:

```bash
# إنشاء مجلد النماذج
mkdir -p models

# تحميل النموذج الصغير (469 MB)
wget -O models/qwen2.5-0.5b-instruct-q4_k_m.gguf \
  'https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf'

# أو النموذج المتوسط (1.1 GB)
wget -O models/qwen2.5-1.5b-instruct-q4_k_m.gguf \
  'https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf'
```

---

## ✅ التحقق من التثبيت

```bash
# عرض الإصدار
almarjaa --version

# عرض المساعدة
almarjaa --help

# تشغيل مثال
almarjaa examples/hello.mrj

# الوضع التفاعلي
almarjaa
```

### الإخراج المتوقع:
```
╔═══════════════════════════════════════════════════════════════╗
║         لغة المرجع - Al-Marjaa Language                      ║
║         لغة برمجة عربية مع ذكاء اصطناعي                       ║
╠═══════════════════════════════════════════════════════════════╣
║  الإصدار: 3.0.0                                              ║
╚═══════════════════════════════════════════════════════════════╝
```

---

## 🔧 أوامر مفيدة

```bash
# تشغيل ملف
almarjaa script.mrj

# الوضع التفاعلي (REPL)
almarjaa --repl

# تنسيق الكود
almarjaa --format script.mrj

# فحص الكود
almarjaa --lint script.mrj

# Vibe Coding
almarjaa --vibe "أنشئ متغير س يساوي 10"

# التصدير
almarjaa --export myapp --platform windows
```

---

## 🐛 استكشاف الأخطاء

### خطأ: `command not found: almarjaa`

**الحل:**
```bash
# Linux/macOS
source $HOME/.cargo/env

# أو أضف إلى .bashrc / .zshrc
echo 'source $HOME/.cargo/env' >> ~/.bashrc
source ~/.bashrc
```

### خطأ: `cargo: command not found`

**الحل:**
ثبّت Rust من https://rustup.rs/

### خطأ: `linker 'cc' not found`

**الحل (Linux):**
```bash
sudo apt install build-essential
```

**الحل (macOS):**
```bash
xcode-select --install
```

### خطأ: `failed to run custom build command for openssl-sys`

**الحل (Linux):**
```bash
sudo apt install libssl-dev pkg-config
```

**الحل (macOS):**
```bash
brew install openssl@3
export OPENSSL_DIR=/opt/homebrew/opt/openssl@3
```

---

## 🔄 التحديث

```bash
cd Al-Marjaa-Language
git pull
cargo build --release
```

---

## 🗑️ إلغاء التثبيت

```bash
# Linux/macOS
rm -rf ~/almarjaa
sudo rm /usr/local/bin/almarjaa

# إلغاء تثبيت Rust (اختياري)
rustup self uninstall
```

---

## 📞 المساعدة

- 📧 البريد: almarjaa.project@hotmail.com
- 🐛 المشاكل: [GitHub Issues](https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/issues)

---

**المؤلف**: رضوان دالي حمدوني
**الإصدار**: 3.0.0
