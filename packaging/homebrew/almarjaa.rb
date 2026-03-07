# ═══════════════════════════════════════════════════════════════════════════════
# Homebrew Formula للغة المرجع
# ═══════════════════════════════════════════════════════════════════════════════
# الاستخدام: brew install almarjaa
# ═══════════════════════════════════════════════════════════════════════════════

class Almarjaa < Formula
  desc "لغة المرجع - لغة برمجة عربية متكاملة مع ذكاء اصطناعي"
  homepage "https://github.com/radhwendalyhamdouni/Al-Marjaa-Language"
  version "3.3.0"
  license "All-Rights-Reserved"
  
  # ═══════════════════════════════════════════════════════════════════════════════
  # الأنظمة المدعومة
  # ═══════════════════════════════════════════════════════════════════════════════
  
  on_macos do
    on_intel do
      url "https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v#{version}/almarjaa-macos-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_INTEL"
    end
    on_arm do
      url "https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v#{version}/almarjaa-macos-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM"
    end
  end
  
  on_linux do
    on_intel do
      url "https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v#{version}/almarjaa-linux-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_INTEL"
    end
    on_arm do
      url "https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases/download/v#{version}/almarjaa-linux-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_ARM"
    end
  end
  
  # ═══════════════════════════════════════════════════════════════════════════════
  # المتطلبات
  # ═══════════════════════════════════════════════════════════════════════════════
  
  depends_on "rust" => :build
  
  # ═══════════════════════════════════════════════════════════════════════════════
  # التثبيت من المصدر (بديل)
  # ═══════════════════════════════════════════════════════════════════════════════
  
  head do
    url "https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git", branch: "main"
    
    depends_on "rust" => :build
    depends_on "pkg-config" => :build
    depends_on "openssl"
    
    def install
      system "cargo", "install", *std_cargo_args
    end
  end
  
  # ═══════════════════════════════════════════════════════════════════════════════
  # التثبيت من Binary
  # ═══════════════════════════════════════════════════════════════════════════════
  
  def install
    bin.install "almarjaa"
    bin.install "almarjaa-lsp" if File.exist?("almarjaa-lsp")
    
    # تثبيت الوثائق
    doc.install "README.md"
    doc.install "CHANGELOG.md"
    doc.install "LICENSE"
    
    # تثبيت الأمثلة
    (pkgshare/"examples").install Dir["examples/*.mrj"]
  end
  
  # ═══════════════════════════════════════════════════════════════════════════════
  # التحقق بعد التثبيت
  # ═══════════════════════════════════════════════════════════════════════════════
  
  test do
    # اختبار الإصدار
    assert_match "3.3.0", shell_output("#{bin}/almarjaa --version")
    
    # اختبار تشغيل كود بسيط
    (testpath/"test.mrj").write <<~EOS
      اطبع("مرحباً بالعالم!")؛
    EOS
    output = shell_output("#{bin}/almarjaa test.mrj")
    assert_match "مرحباً بالعالم!", output
  end
  
  # ═══════════════════════════════════════════════════════════════════════════════
  # معلومات إضافية
  # ═══════════════════════════════════════════════════════════════════════════════
  
  def caveats
    <<~EOS
      ╔═══════════════════════════════════════════════════════════════╗
      ║         🌙 لغة المرجع - Al-Marjaa Language                   ║
      ╚═══════════════════════════════════════════════════════════════╝
      
      تم تثبيت لغة المرجع بنجاح!
      
      🚀 للتشغيل:
         almarjaa              # الوضع التفاعلي
         almarjaa script.mrj   # تشغيل ملف
         almarjaa --help       # المساعدة
      
      📚 الأمثلة:
         #{HOMEBREW_PREFIX}/share/almarjaa/examples/
      
      🔧 VS Code Extension:
         ابحث عن "Al-Marjaa" في سوق الإضافات
      
      📖 التوثيق:
         https://docs.almarjaa.io
    EOS
  end
end
