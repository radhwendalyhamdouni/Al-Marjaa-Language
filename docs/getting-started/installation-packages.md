# حزم التثبيت الرسمية | Official Installation Packages

## نظرة عامة

لغة المرجع متاحة عبر حزم تثبيت رسمية لأنظمة التشغيل المختلفة. اختر الطريقة المناسبة لنظامك.

---

## 📦 طرق التثبيت

### 1. سكربت التثبيت الموحد (موصى به)

```bash
# Linux / macOS
curl -fsSL https://install.almarjaa.io | bash

# أو
wget -qO- https://install.almarjaa.io | bash
```

---

### 2. macOS - Homebrew

```bash
# إضافة Tap
brew tap radhwendalyhamdouni/almarjaa

# التثبيت
brew install almarjaa

# التشغيل
almarjaa --version
```

**الميزات:**
- تحديث تلقائي مع `brew upgrade`
- تثبيت Binary مُجمّع مسبقاً
- دعم Intel و Apple Silicon

---

### 3. Windows - Chocolatey

```powershell
# التثبيت
choco install almarjaa

# التشغيل
almarjaa --version

# التحديث
choco upgrade almarjaa
```

**الميزات:**
- تكامل مع Windows
- تحديثات تلقائية
- إضافة تلقائية لـ PATH

---

### 4. Arch Linux - AUR

```bash
# باستخدام yay
yay -S almarjaa

# أو باستخدام paru
paru -S almarjaa

# أو يدوياً
git clone https://aur.archlinux.org/almarjaa.git
cd almarjaa
makepkg -si
```

---

### 5. Debian/Ubuntu - APT

```bash
# إضافة المستودع
curl -fsSL https://deb.almarjaa.io/almarjaa.gpg | sudo gpg --dearmor -o /usr/share/keyrings/almarjaa.gpg

echo "deb [signed-by=/usr/share/keyrings/almarjaa.gpg] https://deb.almarjaa.io stable main" | sudo tee /etc/apt/sources.list.d/almarjaa.list

# التثبيت
sudo apt-get update
sudo apt-get install almarjaa
```

---

### 6. Fedora/RHEL - RPM

```bash
# إضافة المستودع
sudo dnf config-manager --add-repo https://rpm.almarjaa.io/almarjaa.repo

# التثبيت
sudo dnf install almarjaa

# أو لـ RHEL/CentOS
sudo yum install almarjaa
```

---

### 7. Snap

```bash
# التثبيت
sudo snap install almarjaa

# التشغيل
almarjaa --version

# التحديث
sudo snap refresh almarjaa
```

**الميزات:**
- تحديثات تلقائية
- عزل آمن
- دعم Linux distributions المتعددة

---

### 8. Flatpak

```bash
# إضافة المستودع
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# التثبيت
flatpak install io.almarjaa.Almarjaa

# التشغيل
flatpak run io.almarjaa.Almarjaa
```

---

### 9. Binary مباشر

#### Linux x86_64
```bash
curl -fsSL https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v3.3.0/almarjaa-linux-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv almarjaa /usr/local/bin/
```

#### Linux ARM64
```bash
curl -fsSL https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v3.3.0/almarjaa-linux-aarch64-unknown-linux-gnu.tar.gz | tar xz
sudo mv almarjaa /usr/local/bin/
```

#### macOS Intel
```bash
curl -fsSL https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v3.3.0/almarjaa-macos-x86_64-apple-darwin.tar.gz | tar xz
sudo mv almarjaa /usr/local/bin/
```

#### macOS Apple Silicon
```bash
curl -fsSL https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v3.3.0/almarjaa-macos-aarch64-apple-darwin.tar.gz | tar xz
sudo mv almarjaa /usr/local/bin/
```

#### Windows
```powershell
# تحميل
Invoke-WebRequest -Uri "https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v3.3.0/almarjaa-windows-x86_64.zip" -OutFile "almarjaa.zip"

# استخراج
Expand-Archive -Path "almarjaa.zip" -DestinationPath "C:\Program Files\AlMarjaa"

# إضافة إلى PATH
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\AlMarjaa", "User")
```

---

### 10. من المصدر

```bash
# استنساخ
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language

# تثبيت Rust (إذا لم يكن مثبتاً)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# البناء
cargo build --release

# التثبيت
sudo cp target/release/almarjaa /usr/local/bin/
```

---

## 📊 مقارنة طرق التثبيت

| الطريقة | السهولة | التحديثات | الأداء |
|---------|---------|-----------|--------|
| سكربت موحد | ⭐⭐⭐⭐⭐ | يدوي | ⭐⭐⭐⭐⭐ |
| Homebrew | ⭐⭐⭐⭐⭐ | تلقائي | ⭐⭐⭐⭐⭐ |
| Chocolatey | ⭐⭐⭐⭐⭐ | تلقائي | ⭐⭐⭐⭐⭐ |
| AUR | ⭐⭐⭐⭐ | تلقائي | ⭐⭐⭐⭐⭐ |
| APT/RPM | ⭐⭐⭐⭐ | تلقائي | ⭐⭐⭐⭐⭐ |
| Snap | ⭐⭐⭐⭐⭐ | تلقائي | ⭐⭐⭐⭐ |
| Flatpak | ⭐⭐⭐⭐ | تلقائي | ⭐⭐⭐⭐ |
| Binary | ⭐⭐⭐ | يدوي | ⭐⭐⭐⭐⭐ |
| من المصدر | ⭐⭐ | يدوي | ⭐⭐⭐⭐⭐ |

---

## 🔧 التحقق من التثبيت

```bash
# التحقق من الإصدار
almarjaa --version

# تشغيل اختبار سريع
almarjaa -e 'اطبع("مرحباً بالعالم!")؛'

# عرض المساعدة
almarjaa --help
```

---

## 🗑️ إلغاء التثبيت

### Homebrew
```bash
brew uninstall almarjaa
brew untap radhwendalyhamdouni/almarjaa
```

### Chocolatey
```powershell
choco uninstall almarjaa
```

### AUR
```bash
yay -R almarjaa
```

### APT
```bash
sudo apt-get remove almarjaa
```

### Snap
```bash
sudo snap remove almarjaa
```

### Flatpak
```bash
flatpak uninstall io.almarjaa.Almarjaa
```

### Binary يدوي
```bash
sudo rm /usr/local/bin/almarjaa
rm -rf ~/.almarjaa
```

---

## 📝 ملاحظات

### متطلبات النظام

| النظام | الحد الأدنى | الموصى به |
|--------|-------------|-----------|
| Linux | glibc 2.27+ | Ubuntu 20.04+ |
| macOS | 10.15+ | 12.0+ |
| Windows | 10 | 11 |

### التبعيات

- **Linux**: glibc, OpenSSL
- **macOS**: لا توجد تبعيات إضافية
- **Windows**: Visual C++ Redistributable

---

© 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
جميع الحقوق محفوظة | All Rights Reserved
