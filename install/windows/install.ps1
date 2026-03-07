# ═══════════════════════════════════════════════════════════════════════════════
# لغة المرجع - سكربت التثبيت لنظام Windows
# Al-Marjaa Language - Windows Installation Script
# ═══════════════════════════════════════════════════════════════════════════════
# كيفية التشغيل: انقر بزر الأيمن ثم "Run with PowerShell" أو:
#powershell -ExecutionPolicy Bypass -File install.ps1

# ألوان للإخراج
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

Write-ColorOutput Cyan @"
╔══════════════════════════════════════════════════════════════════════════════╗
║         لغة المرجع - Al-Marjaa Language v3.0.0                              ║
║         سكربت التثبيت لنظام Windows                                         ║
╚══════════════════════════════════════════════════════════════════════════════╝
"@

# متغيرات
$InstallDir = "$env:USERPROFILE\almarjaa"
$BinaryPath = "$InstallDir\target\release\almarjaa.exe"

# 1. تثبيت Rust
Write-ColorOutput Yellow "[1/5] التحقق من Rust..."
if (Get-Command rustc -ErrorAction SilentlyContinue) {
    $rustVersion = rustc --version
    Write-ColorOutput Green "✅ Rust مثبت: $rustVersion"
} else {
    Write-ColorOutput Yellow "   جاري تثبيت Rust..."

    # تحميل rustup-init
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"

    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath

    # تثبيت Rust
    Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait

    # تحديث PATH
    $env:Path += ";$env:USERPROFILE\.cargo\bin"

    # إضافة PATH بشكل دائم
    [Environment]::SetEnvironmentVariable(
        "Path",
        [Environment]::GetEnvironmentVariable("Path", "User") + ";$env:USERPROFILE\.cargo\bin",
        "User"
    )

    Write-ColorOutput Green "✅ تم تثبيت Rust"
}

# 2. تثبيت Git
Write-ColorOutput Yellow "[2/5] التحقق من Git..."
if (Get-Command git -ErrorAction SilentlyContinue) {
    Write-ColorOutput Green "✅ Git مثبت"
} else {
    Write-ColorOutput Yellow "   جاري تثبيت Git..."

    # تحميل Git
    $gitUrl = "https://github.com/git-for-windows/git/releases/download/v2.43.0.windows.1/Git-2.43.0-64-bit.exe"
    $gitPath = "$env:TEMP\git-installer.exe"

    Invoke-WebRequest -Uri $gitUrl -OutFile $gitPath
    Start-Process -FilePath $gitPath -ArgumentList "/VERYSILENT", "/NORESTART" -Wait

    # تحديث PATH
    $env:Path += ";C:\Program Files\Git\cmd"

    Write-ColorOutput Green "✅ تم تثبيت Git"
}

# 3. استنساخ المشروع
Write-ColorOutput Yellow "[3/5] استنساخ المشروع..."

if (Test-Path $InstallDir) {
    Write-ColorOutput Yellow "   المجلد موجود، جاري التحديث..."
    Set-Location $InstallDir
    git pull
} else {
    Write-ColorOutput Yellow "   جاري استنساخ المشروع..."
    git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git $InstallDir
}

Set-Location $InstallDir
Write-ColorOutput Green "✅ تم استنساخ المشروع في: $InstallDir"

# 4. بناء المشروع
Write-ColorOutput Yellow "[4/5] بناء المشروع..."

# تحديث PATH للتأكد
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")

# تحميل المكتبات
cargo fetch

# البناء
cargo build --release

Write-ColorOutput Green "✅ تم بناء المشروع"

# 5. إضافة إلى PATH
Write-ColorOutput Yellow "[5/5] إضافة إلى PATH..."

$binDir = Split-Path $BinaryPath -Parent
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($currentPath -notlike "*$binDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        $currentPath + ";$binDir",
        "User"
    )
    Write-ColorOutput Green "✅ تمت الإضافة إلى PATH"
} else {
    Write-ColorOutput Green "✅ المسار موجود بالفعل في PATH"
}

# تحميل النموذج (اختياري)
Write-ColorOutput Yellow ""
$response = Read-Host "هل تريد تحميل نموذج AI؟ (اختياري) [y/N]"

if ($response -eq "y" -or $response -eq "Y") {
    Write-ColorOutput Cyan "   تحميل نموذج Qwen 2.5 (469 MB)..."

    $modelsDir = "$InstallDir\models"
    New-Item -ItemType Directory -Force -Path $modelsDir | Out-Null

    $modelUrl = "https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf"
    $modelPath = "$modelsDir\qwen2.5-0.5b-instruct-q4_k_m.gguf"

    Invoke-WebRequest -Uri $modelUrl -OutFile $modelPath

    Write-ColorOutput Green "✅ تم تحميل النموذج"
}

# رسالة النجاح
Write-ColorOutput Green @"
╔══════════════════════════════════════════════════════════════════════════════╗
║                      ✅ تم التثبيت بنجاح!                                   ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  📂 موقع التثبيت: $InstallDir
║                                                                              ║
║  🚀 للتشغيل (افتح Terminal جديد):                                           ║
║     almarjaa                    # الوضع التفاعلي                            ║
║     almarjaa script.mrj         # تشغيل ملف                                 ║
║     almarjaa --help             # المساعدة                                   ║
║                                                                              ║
║  📚 الأمثلة:                                                                 ║
║     $InstallDir\examples\                                                   ║
║                                                                              ║
║  📖 التوثيق:                                                                 ║
║     $InstallDir\docs\                                                       ║
║                                                                              ║
║  ⚠️  أعد تشغيل Terminal لتفعيل PATH                                         ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
"@

# اختبار التشغيل
Write-ColorOutput Yellow ""
$response = Read-Host "هل تريد اختبار التشغيل؟ [Y/n]"

if ($response -ne "n" -and $response -ne "N") {
    Write-ColorOutput Cyan "   تشغيل البرنامج..."
    & $BinaryPath --help
}
