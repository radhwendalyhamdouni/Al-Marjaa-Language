# ═══════════════════════════════════════════════════════════════════════════════
# Chocolatey Install Script للغة المرجع
# ═══════════════════════════════════════════════════════════════════════════════

$ErrorActionPreference = 'Stop'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# معلومات الحزمة
$packageName = 'almarjaa'
$version = '3.3.0'
$binaryDir = Join-Path $toolsDir 'bin'

# ═══════════════════════════════════════════════════════════════════════════════
# تحميل Binary
# ═══════════════════════════════════════════════════════════════════════════════

$url64 = "https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v$version/almarjaa-windows-x86_64.zip"
$checksum64 = 'PLACEHOLDER_CHECKSUM'

$packageArgs = @{
    packageName    = $packageName
    unzipLocation  = $toolsDir
    url64bit       = $url64
    checksum64     = $checksum64
    checksumType64 = 'sha256'
}

Install-ChocolateyZipPackage @packageArgs

# ═══════════════════════════════════════════════════════════════════════════════
# إنشاء shim
# ═══════════════════════════════════════════════════════════════════════════════

$exePath = Join-Path $toolsDir 'almarjaa.exe'
$lspPath = Join-Path $toolsDir 'almarjaa-lsp.exe'

if (Test-Path $exePath) {
    Install-BinFile -Name 'almarjaa' -Path $exePath
}

if (Test-Path $lspPath) {
    Install-BinFile -Name 'almarjaa-lsp' -Path $lspPath
}

# ═══════════════════════════════════════════════════════════════════════════════
# إضافة إلى PATH
# ═══════════════════════════════════════════════════════════════════════════════

Install-ChocolateyPath -PathToInstall $toolsDir -PathType 'Machine'

# ═══════════════════════════════════════════════════════════════════════════════
# رسالة النجاح
# ═══════════════════════════════════════════════════════════════════════════════

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║           ✅ تم تثبيت لغة المرجع بنجاح!                                      ║" -ForegroundColor Green
Write-Host "╠═══════════════════════════════════════════════════════════════════════════════╣" -ForegroundColor Green
Write-Host "║                                                                               ║" -ForegroundColor Green
Write-Host "║  🚀 للتشغيل:                                                                  ║" -ForegroundColor Green
Write-Host "║     almarjaa              # الوضع التفاعلي                                  ║" -ForegroundColor Green
Write-Host "║     almarjaa script.mrj   # تشغيل ملف                                       ║" -ForegroundColor Green
Write-Host "║     almarjaa --help       # المساعدة                                         ║" -ForegroundColor Green
Write-Host "║                                                                               ║" -ForegroundColor Green
Write-Host "║  📖 التوثيق: https://docs.almarjaa.io                                        ║" -ForegroundColor Green
Write-Host "╚═══════════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
