// ═══════════════════════════════════════════════════════════════════════════════
// الإصدارات الدلالية والتكامل المستمر - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// نظام متقدم للإصدارات الدلالية والتكامل المستمر
// - تحليل التغييرات تلقائياً
// - اقتراح الإصدارات
// - CI/CD integration
// - فحص الجودة التلقائي
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// الإصدارات الدلالية
// ═══════════════════════════════════════════════════════════════════════════════

/// إصدار دلالي
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SemanticVersion {
    /// الرقم الرئيسي
    pub major: u32,
    /// الرقم الفرعي
    pub minor: u32,
    /// رقم التصحيح
    pub patch: u32,
    /// المُعرف المسبق (alpha, beta, rc)
    pub pre_release: Option<String>,
    /// بيانات البناء
    pub build: Option<String>,
}

impl SemanticVersion {
    /// إنشاء إصدار جديد
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
            build: None,
        }
    }

    /// تحليل من نص
    pub fn parse(version: &str) -> Result<Self, String> {
        let version = version.trim_start_matches('v');
        
        let parts: Vec<&str> = version.split('-').collect();
        let main_part = parts[0];
        
        let pre_release = if parts.len() > 1 {
            Some(parts[1].to_string())
        } else {
            None
        };

        let nums: Vec<&str> = main_part.split('.').collect();
        if nums.len() != 3 {
            return Err("صيغة الإصدار غير صحيحة".to_string());
        }

        Ok(Self {
            major: nums[0].parse().map_err(|_| "رقم رئيسي غير صالح")?,
            minor: nums[1].parse().map_err(|_| "رقم فرعي غير صالح")?,
            patch: nums[2].parse().map_err(|_| "رقم تصحيح غير صالح")?,
            pre_release,
            build: None,
        })
    }

    /// زيادة الإصدار الرئيسي
    pub fn bump_major(&self) -> Self {
        Self {
            major: self.major + 1,
            minor: 0,
            patch: 0,
            pre_release: None,
            build: None,
        }
    }

    /// زيادة الإصدار الفرعي
    pub fn bump_minor(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor + 1,
            patch: 0,
            pre_release: None,
            build: None,
        }
    }

    /// زيادة التصحيح
    pub fn bump_patch(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor,
            patch: self.patch + 1,
            pre_release: None,
            build: None,
        }
    }

    /// التحقق من التوافق
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        // التوافق يعني نفس الإصدار الرئيسي
        self.major == other.major && self >= other
    }

    /// مقارنة الإصدارات (تجاهل المُعرف المسبق)
    pub fn compare_main(&self, other: &Self) -> std::cmp::Ordering {
        match self.major.cmp(&other.major) {
            std::cmp::Ordering::Equal => {
                match self.minor.cmp(&other.minor) {
                    std::cmp::Ordering::Equal => self.patch.cmp(&other.patch),
                    other => other,
                }
            }
            other => other,
        }
    }
}

impl std::fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref pre) = self.pre_release {
            write!(f, "-{}", pre)?;
        }
        if let Some(ref build) = self.build {
            write!(f, "+{}", build)?;
        }
        Ok(())
    }
}

impl Default for SemanticVersion {
    fn default() -> Self {
        Self::new(0, 1, 0)
    }
}

/// نوع التغيير
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// تغيير جوهري (غير متوافق)
    Breaking,
    /// ميزة جديدة
    Feature,
    /// إصلاح خطأ
    Fix,
    /// تحسين
    Improvement,
    /// توثيق
    Documentation,
    /// إعادة هيكلة
    Refactor,
    /// أداء
    Performance,
    /// اختبارات
    Test,
    /// بنية تحتية
    Chore,
}

impl std::fmt::Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeType::Breaking => write!(f, "جوهري"),
            ChangeType::Feature => write!(f, "ميزة"),
            ChangeType::Fix => write!(f, "إصلاح"),
            ChangeType::Improvement => write!(f, "تحسين"),
            ChangeType::Documentation => write!(f, "توثيق"),
            ChangeType::Refactor => write!(f, "إعادة هيكلة"),
            ChangeType::Performance => write!(f, "أداء"),
            ChangeType::Test => write!(f, "اختبارات"),
            ChangeType::Chore => write!(f, "بنية تحتية"),
        }
    }
}

/// تغيير في الكود
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChange {
    /// نوع التغيير
    pub change_type: ChangeType,
    /// الوصف
    pub description: String,
    /// الملفات المتأثرة
    pub files: Vec<String>,
    /// رقم الـ commit
    pub commit_hash: Option<String>,
    /// المؤلف
    pub author: Option<String>,
    /// التاريخ
    pub date: String,
}

/// محلل التغييرات
pub struct ChangeAnalyzer {
    /// التغييرات المحللة
    changes: Vec<CodeChange>,
}

impl ChangeAnalyzer {
    /// إنشاء محلل جديد
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    /// تحليل رسالة commit
    pub fn analyze_commit(&mut self, message: &str, hash: &str, author: &str) -> CodeChange {
        let change_type = self.detect_change_type(message);
        
        let change = CodeChange {
            change_type,
            description: message.to_string(),
            files: vec![],
            commit_hash: Some(hash.to_string()),
            author: Some(author.to_string()),
            date: chrono::Utc::now().to_rfc3339(),
        };
        
        self.changes.push(change.clone());
        change
    }

    /// كشف نوع التغيير من الرسالة
    fn detect_change_type(&self, message: &str) -> ChangeType {
        let message_lower = message.to_lowercase();
        
        // الأنماط التقليدية
        if message_lower.starts_with("breaking") 
            || message_lower.contains("!:")
            || message_lower.contains("breaking change") {
            return ChangeType::Breaking;
        }
        
        if message_lower.starts_with("feat") 
            || message_lower.starts_with("feature")
            || message_lower.starts_with("أضف")
            || message_lower.starts_with("إضافة") {
            return ChangeType::Feature;
        }
        
        if message_lower.starts_with("fix")
            || message_lower.starts_with("أصلح")
            || message_lower.starts_with("إصلاح") {
            return ChangeType::Fix;
        }
        
        if message_lower.starts_with("docs")
            || message_lower.starts_with("توثيق") {
            return ChangeType::Documentation;
        }
        
        if message_lower.starts_with("refactor")
            || message_lower.starts_with("أعد هيكلة") {
            return ChangeType::Refactor;
        }
        
        if message_lower.starts_with("perf")
            || message_lower.starts_with("أداء") {
            return ChangeType::Performance;
        }
        
        if message_lower.starts_with("test")
            || message_lower.starts_with("اختبار") {
            return ChangeType::Test;
        }
        
        if message_lower.starts_with("chore")
            || message_lower.starts_with("بنية") {
            return ChangeType::Chore;
        }
        
        ChangeType::Improvement
    }

    /// اقتراح الإصدار التالي
    pub fn suggest_version(&self, current: &SemanticVersion) -> SemanticVersion {
        let mut has_breaking = false;
        let mut has_feature = false;
        let mut has_fix = false;

        for change in &self.changes {
            match change.change_type {
                ChangeType::Breaking => has_breaking = true,
                ChangeType::Feature => has_feature = true,
                ChangeType::Fix => has_fix = true,
                _ => {}
            }
        }

        if has_breaking {
            current.bump_major()
        } else if has_feature {
            current.bump_minor()
        } else if has_fix {
            current.bump_patch()
        } else {
            current.bump_patch()
        }
    }

    /// توليد CHANGELOG
    pub fn generate_changelog(&self, version: &SemanticVersion) -> String {
        let mut changelog = String::new();
        
        changelog.push_str(&format!("# الإصدار {}\n\n", version));
        changelog.push_str(&format!("التاريخ: {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
        
        // تجميع حسب النوع
        let mut grouped: HashMap<ChangeType, Vec<&CodeChange>> = HashMap::new();
        for change in &self.changes {
            grouped.entry(change.change_type).or_default().push(change);
        }

        // ترتيب الأنواع
        let type_order = [
            ChangeType::Breaking,
            ChangeType::Feature,
            ChangeType::Fix,
            ChangeType::Improvement,
            ChangeType::Performance,
            ChangeType::Documentation,
            ChangeType::Refactor,
            ChangeType::Test,
            ChangeType::Chore,
        ];

        for change_type in type_order {
            if let Some(changes) = grouped.get(&change_type) {
                if changes.is_empty() {
                    continue;
                }
                
                changelog.push_str(&format!("### {}\n\n", change_type));
                
                for change in changes {
                    changelog.push_str(&format!("- {}", change.description));
                    if let Some(ref hash) = change.commit_hash {
                        changelog.push_str(&format!(" ({})", &hash[..7]));
                    }
                    changelog.push('\n');
                }
                changelog.push('\n');
            }
        }

        changelog
    }

    /// مسح التغييرات
    pub fn clear(&mut self) {
        self.changes.clear();
    }
}

impl Default for ChangeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// التكامل المستمر (CI/CD)
// ═══════════════════════════════════════════════════════════════════════════════

/// إعدادات CI/CD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIConfig {
    /// تشغيل الاختبارات
    pub run_tests: bool,
    /// فحص التنسيق
    pub check_format: bool,
    /// فحص Lint
    pub run_lint: bool,
    /// فحص الأمان
    pub security_check: bool,
    /// البناء
    pub build: bool,
    /// الحد الأدنى للتغطية
    pub min_coverage: Option<f32>,
    /// أهداف البناء
    pub targets: Vec<String>,
    /// متغيرات البيئة
    pub env: HashMap<String, String>,
}

impl Default for CIConfig {
    fn default() -> Self {
        Self {
            run_tests: true,
            check_format: true,
            run_lint: true,
            security_check: true,
            build: true,
            min_coverage: Some(50.0),
            targets: vec!["native".to_string()],
            env: HashMap::new(),
        }
    }
}

/// نتيجة خطوة CI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIStepResult {
    /// اسم الخطوة
    pub name: String,
    /// نجاح أو فشل
    pub success: bool,
    /// المخرجات
    pub output: String,
    /// الأخطاء
    pub errors: Vec<String>,
    /// التحذيرات
    pub warnings: Vec<String>,
    /// الوقت (مللي ثانية)
    pub duration_ms: u64,
}

/// نتيجة CI كاملة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIRunResult {
    /// معرف التشغيل
    pub run_id: String,
    /// الفرع
    pub branch: String,
    /// الـ commit
    pub commit: String,
    /// نجاح كامل
    pub success: bool,
    /// نتائج الخطوات
    pub steps: Vec<CIStepResult>,
    /// الوقت الإجمالي
    pub total_duration_ms: u64,
    /// التاريخ
    pub timestamp: String,
}

/// نظام CI/CD
pub struct CISystem {
    /// الإعدادات
    config: CIConfig,
    /// نتائج التشغيلات
    runs: Vec<CIRunResult>,
}

impl CISystem {
    /// إنشاء نظام جديد
    pub fn new(config: CIConfig) -> Self {
        Self {
            config,
            runs: Vec::new(),
        }
    }

    /// تشغيل CI
    pub fn run(&mut self, branch: &str, commit: &str) -> CIRunResult {
        let run_id = format!("ci_{}", chrono::Utc::now().timestamp());
        let start = std::time::Instant::now();
        let mut steps = Vec::new();
        let mut all_success = true;

        println!("\n🚀 بدء تشغيل CI");
        println!("   الفرع: {}", branch);
        println!("   Commit: {}", &commit[..7]);

        // 1. فحص التنسيق
        if self.config.check_format {
            let result = self.run_format_check();
            if !result.success {
                all_success = false;
            }
            steps.push(result);
        }

        // 2. فحص Lint
        if self.config.run_lint {
            let result = self.run_lint_check();
            if !result.success {
                all_success = false;
            }
            steps.push(result);
        }

        // 3. فحص الأمان
        if self.config.security_check {
            let result = self.run_security_check();
            if !result.success {
                all_success = false;
            }
            steps.push(result);
        }

        // 4. تشغيل الاختبارات
        if self.config.run_tests {
            let result = self.run_tests();
            if !result.success {
                all_success = false;
            }
            steps.push(result);
        }

        // 5. البناء
        if self.config.build {
            let result = self.run_build();
            if !result.success {
                all_success = false;
            }
            steps.push(result);
        }

        let duration = start.elapsed().as_millis() as u64;

        println!("\n{}", if all_success { "✅ نجح التشغيل" } else { "❌ فشل التشغيل" });
        println!("   الوقت الإجمالي: {}ms", duration);

        let result = CIRunResult {
            run_id,
            branch: branch.to_string(),
            commit: commit.to_string(),
            success: all_success,
            steps,
            total_duration_ms: duration,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.runs.push(result.clone());
        result
    }

    /// فحص التنسيق
    fn run_format_check(&self) -> CIStepResult {
        println!("\n   📝 فحص التنسيق...");
        let start = std::time::Instant::now();
        
        // محاكاة فحص التنسيق
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        CIStepResult {
            name: "فحص التنسيق".to_string(),
            success: true,
            output: "جميع الملفات منسقة بشكل صحيح".to_string(),
            errors: vec![],
            warnings: vec![],
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    /// فحص Lint
    fn run_lint_check(&self) -> CIStepResult {
        println!("   🔍 فحص Lint...");
        let start = std::time::Instant::now();
        
        // محاكاة
        std::thread::sleep(std::time::Duration::from_millis(150));
        
        CIStepResult {
            name: "فحص Lint".to_string(),
            success: true,
            output: "لا توجد مشاكل Lint".to_string(),
            errors: vec![],
            warnings: vec!["استخدام متغير غير مستخدم في السطر 42".to_string()],
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    /// فحص الأمان
    fn run_security_check(&self) -> CIStepResult {
        println!("   🔒 فحص الأمان...");
        let start = std::time::Instant::now();
        
        // محاكاة
        std::thread::sleep(std::time::Duration::from_millis(200));
        
        CIStepResult {
            name: "فحص الأمان".to_string(),
            success: true,
            output: "لا توجد ثغرات أمنية معروفة".to_string(),
            errors: vec![],
            warnings: vec![],
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    /// تشغيل الاختبارات
    fn run_tests(&self) -> CIStepResult {
        println!("   🧪 تشغيل الاختبارات...");
        let start = std::time::Instant::now();
        
        // محاكاة
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        CIStepResult {
            name: "الاختبارات".to_string(),
            success: true,
            output: "43 اختبار ناجح، 0 فاشل\nتغطية: 78%".to_string(),
            errors: vec![],
            warnings: vec![],
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    /// البناء
    fn run_build(&self) -> CIStepResult {
        println!("   🔨 البناء...");
        let start = std::time::Instant::now();
        
        // محاكاة
        std::thread::sleep(std::time::Duration::from_millis(300));
        
        CIStepResult {
            name: "البناء".to_string(),
            success: true,
            output: "تم البناء بنجاح\n- target/debug/almarjaa".to_string(),
            errors: vec![],
            warnings: vec![],
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    /// توليد تكوين GitHub Actions
    pub fn generate_github_actions(&self) -> String {
        r#"name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: تثبيت Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: فحص التنسيق
      run: cargo fmt --check
    
    - name: فحص Lint
      run: cargo clippy -- -D warnings
    
    - name: تشغيل الاختبارات
      run: cargo test --verbose
    
    - name: فحص الأمان
      run: cargo audit
    
    - name: البناء
      run: cargo build --release

  publish:
    needs: build
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: نشر الحزمة
      run: almarjaa انشر
      env:
        ALMARJAA_TOKEN: ${{ secrets.ALMARJAA_TOKEN }}
"#.to_string()
    }

    /// الحصول على آخر تشغيل
    pub fn last_run(&self) -> Option<&CIRunResult> {
        self.runs.last()
    }

    /// إحصائيات التشغيلات
    pub fn stats(&self) -> CIStats {
        let total = self.runs.len() as u32;
        let successful = self.runs.iter().filter(|r| r.success).count() as u32;
        
        CIStats {
            total_runs: total,
            successful_runs: successful,
            failed_runs: total - successful,
            success_rate: if total > 0 { successful as f32 / total as f32 * 100.0 } else { 0.0 },
        }
    }
}

/// إحصائيات CI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIStats {
    /// إجمالي التشغيلات
    pub total_runs: u32,
    /// التشغيلات الناجحة
    pub successful_runs: u32,
    /// التشغيلات الفاشلة
    pub failed_runs: u32,
    /// معدل النجاح
    pub success_rate: f32,
}

impl Default for CISystem {
    fn default() -> Self {
        Self::new(CIConfig::default())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_version_parse() {
        let v = SemanticVersion::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_semantic_version_bump() {
        let v = SemanticVersion::new(1, 2, 3);
        
        assert_eq!(v.bump_major(), SemanticVersion::new(2, 0, 0));
        assert_eq!(v.bump_minor(), SemanticVersion::new(1, 3, 0));
        assert_eq!(v.bump_patch(), SemanticVersion::new(1, 2, 4));
    }

    #[test]
    fn test_semantic_version_compatibility() {
        let v1 = SemanticVersion::new(1, 0, 0);
        let v2 = SemanticVersion::new(1, 1, 0);
        let v3 = SemanticVersion::new(2, 0, 0);
        
        assert!(v2.is_compatible_with(&v1));
        assert!(!v3.is_compatible_with(&v1));
    }

    #[test]
    fn test_semantic_version_display() {
        let v = SemanticVersion::new(1, 2, 3);
        assert_eq!(format!("{}", v), "1.2.3");
    }

    #[test]
    fn test_change_analyzer() {
        let mut analyzer = ChangeAnalyzer::new();
        
        let change = analyzer.analyze_commit("feat: إضافة ميزة جديدة", "abc123", "أحمد");
        assert_eq!(change.change_type, ChangeType::Feature);
    }

    #[test]
    fn test_change_analyzer_breaking() {
        let mut analyzer = ChangeAnalyzer::new();
        
        let change = analyzer.analyze_commit("breaking!: تغيير جوهري في API", "abc123", "أحمد");
        assert_eq!(change.change_type, ChangeType::Breaking);
    }

    #[test]
    fn test_version_suggestion() {
        let mut analyzer = ChangeAnalyzer::new();
        let current = SemanticVersion::new(1, 0, 0);
        
        // ميزة جديدة
        analyzer.analyze_commit("feat: ميزة جديدة", "a", "a");
        let suggested = analyzer.suggest_version(&current);
        assert_eq!(suggested, SemanticVersion::new(1, 1, 0));
        
        analyzer.clear();
        
        // إصلاح
        analyzer.analyze_commit("fix: إصلاح خطأ", "b", "b");
        let suggested = analyzer.suggest_version(&current);
        assert_eq!(suggested, SemanticVersion::new(1, 0, 1));
    }

    #[test]
    fn test_changelog_generation() {
        let mut analyzer = ChangeAnalyzer::new();
        
        analyzer.analyze_commit("feat: ميزة 1", "a", "a");
        analyzer.analyze_commit("fix: إصلاح 1", "b", "b");
        
        let version = SemanticVersion::new(1, 0, 0);
        let changelog = analyzer.generate_changelog(&version);
        
        assert!(changelog.contains("1.0.0"));
        assert!(changelog.contains("ميزة"));
        assert!(changelog.contains("إصلاح"));
    }

    #[test]
    fn test_ci_system() {
        let mut ci = CISystem::default();
        let result = ci.run("main", "abc123def456");
        
        assert!(!result.steps.is_empty());
        assert!(result.success);
    }

    #[test]
    fn test_ci_stats() {
        let mut ci = CISystem::default();
        
        ci.run("main", "a");
        ci.run("main", "b");
        
        let stats = ci.stats();
        assert_eq!(stats.total_runs, 2);
        assert_eq!(stats.successful_runs, 2);
        assert_eq!(stats.success_rate, 100.0);
    }

    #[test]
    fn test_github_actions_generation() {
        let ci = CISystem::default();
        let yaml = ci.generate_github_actions();
        
        assert!(yaml.contains("name: CI"));
        assert!(yaml.contains("cargo test"));
    }
}
