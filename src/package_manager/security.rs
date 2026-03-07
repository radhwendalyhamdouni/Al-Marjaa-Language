// ═══════════════════════════════════════════════════════════════════════════════
// مدقق الأمان - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use crate::package_manager::registry::PackageInfo;

/// تقرير الأمان
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityReport {
    /// آمن أم لا
    pub safe: bool,
    /// التحذيرات
    pub warning: String,
    /// الثغرات المكتشفة
    pub vulnerabilities: Vec<Vulnerability>,
    /// نتيجة الأمان (0-100)
    pub score: u8,
    /// التوصيات
    pub recommendations: Vec<String>,
}

/// ثغرة أمنية
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// المعرف
    pub id: String,
    /// مستوى الخطورة
    pub severity: Severity,
    /// الوصف
    pub description: String,
    /// الحل المقترح
    pub fix: String,
    /// الرابط
    pub reference: Option<String>,
}

/// مستوى الخطورة
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// منخفض
    Low,
    /// متوسط
    Medium,
    /// عالي
    High,
    /// حرج
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Low => write!(f, "منخفض"),
            Severity::Medium => write!(f, "متوسط"),
            Severity::High => write!(f, "عالي"),
            Severity::Critical => write!(f, "حرج"),
        }
    }
}

/// مدقق الأمان
pub struct SecurityChecker {
    /// قاعدة بيانات الثغرات المعروفة
    known_vulnerabilities: Vec<KnownVulnerability>,
    /// الأنماط الخطرة
    dangerous_patterns: Vec<(String, String, Severity)>,
}

/// ثغرة معروفة
#[derive(Debug, Clone)]
struct KnownVulnerability {
    /// اسم الحزمة
    package: String,
    /// الإصدارات المتأثرة
    affected_versions: String,
    /// الثغرة
    vulnerability: Vulnerability,
}

impl SecurityChecker {
    /// إنشاء مدقق جديد
    pub fn new() -> Self {
        Self {
            known_vulnerabilities: Self::load_vulnerability_db(),
            dangerous_patterns: Self::load_dangerous_patterns(),
        }
    }

    /// فحص حزمة
    pub fn check(&self, package: &PackageInfo) -> Result<SecurityReport, String> {
        let mut vulnerabilities = Vec::new();
        let mut recommendations = Vec::new();
        let mut warnings = Vec::new();
        
        // 1. فحص الثغرات المعروفة
        for known in &self.known_vulnerabilities {
            if known.package == package.name {
                if Self::version_matches(&package.latest_version, &known.affected_versions) {
                    vulnerabilities.push(known.vulnerability.clone());
                }
            }
        }
        
        // 2. فحص الرخصة
        let license_score = self.check_license(&package.license);
        if license_score < 50 {
            warnings.push(format!("الرخصة '{}' قد تحتوي قيوداً", package.license));
            recommendations.push("تحقق من توافق الرخصة مع مشروعك".to_string());
        }
        
        // 3. فحص عدد التبعيات
        if package.dependencies.len() > 15 {
            warnings.push("عدد كبير من التبعيات يزيد سطح الهجوم".to_string());
            recommendations.push("راجع الحزم غير الضرورية".to_string());
        }
        
        // 4. فحص المؤلف
        if package.author.is_empty() || package.author == "unknown" {
            warnings.push("مؤلف غير معروف".to_string());
            recommendations.push("تحقق من مصدر الحزمة".to_string());
        }
        
        // حساب النتيجة
        let score = self.calculate_score(&vulnerabilities, &warnings);
        
        Ok(SecurityReport {
            safe: vulnerabilities.is_empty() && score >= 70,
            warning: warnings.join(" | "),
            vulnerabilities,
            score,
            recommendations,
        })
    }

    /// فحص محتوى ملف
    pub fn scan_content(&self, content: &str) -> Result<SecurityReport, String> {
        let mut vulnerabilities = Vec::new();
        let mut warnings = Vec::new();
        
        for (pattern, desc, severity) in &self.dangerous_patterns {
            if content.contains(pattern) {
                vulnerabilities.push(Vulnerability {
                    id: format!("CODE-{}", pattern),
                    severity: *severity,
                    description: desc.clone(),
                    fix: "تجنب استخدام هذه الدالة أو التحقق من المدخلات".to_string(),
                    reference: None,
                });
            }
        }
        
        let score = self.calculate_score(&vulnerabilities, &warnings);
        
        Ok(SecurityReport {
            safe: vulnerabilities.is_empty(),
            warning: warnings.join(" | "),
            vulnerabilities,
            score,
            recommendations: vec![],
        })
    }

    /// فحص التبعيات
    pub fn check_dependencies(&self, dependencies: &std::collections::HashMap<String, String>) -> Result<SecurityReport, String> {
        let mut vulnerabilities = Vec::new();
        let mut recommendations = Vec::new();
        
        for (name, version) in dependencies {
            // فحص الثغرات المعروفة
            for known in &self.known_vulnerabilities {
                if known.package == *name {
                    if Self::version_matches(version, &known.affected_versions) {
                        vulnerabilities.push(Vulnerability {
                            id: format!("DEP-{}", known.vulnerability.id),
                            severity: known.vulnerability.severity,
                            description: format!("{}: {}", name, known.vulnerability.description),
                            fix: known.vulnerability.fix.clone(),
                            reference: known.vulnerability.reference.clone(),
                        });
                    }
                }
            }
        }
        
        if !vulnerabilities.is_empty() {
            recommendations.push("حدث الحزم المتأثرة لإصلاح الثغرات".to_string());
        }
        
        let score = self.calculate_score(&vulnerabilities, &[]);
        
        Ok(SecurityReport {
            safe: vulnerabilities.is_empty(),
            warning: String::new(),
            vulnerabilities,
            score,
            recommendations,
        })
    }

    /// حساب النتيجة
    fn calculate_score(&self, vulnerabilities: &[Vulnerability], warnings: &[String]) -> u8 {
        let mut score = 100u8;
        
        for vuln in vulnerabilities {
            match vuln.severity {
                Severity::Low => score = score.saturating_sub(5),
                Severity::Medium => score = score.saturating_sub(15),
                Severity::High => score = score.saturating_sub(30),
                Severity::Critical => score = score.saturating_sub(50),
            }
        }
        
        for _ in warnings {
            score = score.saturating_sub(5);
        }
        
        score
    }

    /// فحص الرخصة
    fn check_license(&self, license: &str) -> u8 {
        let permissive = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC"];
        let copyleft = ["GPL-3.0", "GPL-2.0", "LGPL-3.0", "MPL-2.0"];
        
        if permissive.iter().any(|l| license.contains(l)) {
            100
        } else if copyleft.iter().any(|l| license.contains(l)) {
            70
        } else if license == "UNKNOWN" || license.is_empty() {
            30
        } else {
            50
        }
    }

    /// مطابقة الإصدار
    fn version_matches(version: &str, pattern: &str) -> bool {
        // تبسيط - في التطبيق الحقيقي سنستخدم semver
        pattern == "*" || pattern.contains(version)
    }

    /// تحميل قاعدة بيانات الثغرات
    fn load_vulnerability_db() -> Vec<KnownVulnerability> {
        vec![
            KnownVulnerability {
                package: "example-vuln".to_string(),
                affected_versions: "*".to_string(),
                vulnerability: Vulnerability {
                    id: "ALM-001".to_string(),
                    severity: Severity::High,
                    description: "ثغرة اختبارية".to_string(),
                    fix: "حدث الحزمة".to_string(),
                    reference: None,
                },
            },
        ]
    }

    /// تحميل الأنماط الخطرة
    fn load_dangerous_patterns() -> Vec<(String, String, Severity)> {
        vec![
            ("نفّذ(".to_string(), "تنفيذ أوامر خارجية".to_string(), Severity::High),
            ("eval(".to_string(), "تنفيذ كود ديناميكي".to_string(), Severity::Medium),
            ("كلمة_السر".to_string(), "كلمات مرور في الكود".to_string(), Severity::Critical),
            ("مفتاح_سري".to_string(), "مفاتيح سرية في الكود".to_string(), Severity::Critical),
        ]
    }
}

impl Default for SecurityChecker {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_creation() {
        let checker = SecurityChecker::new();
        assert!(!checker.known_vulnerabilities.is_empty());
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(format!("{}", Severity::Low), "منخفض");
        assert_eq!(format!("{}", Severity::Critical), "حرج");
    }

    #[test]
    fn test_scan_content_safe() {
        let checker = SecurityChecker::new();
        let report = checker.scan_content("اطبع(\"مرحبا\")").unwrap();
        
        assert!(report.safe);
        assert_eq!(report.score, 100);
    }

    #[test]
    fn test_scan_content_unsafe() {
        let checker = SecurityChecker::new();
        let report = checker.scan_content("نفّذ(\"rm -rf /\")").unwrap();
        
        assert!(!report.safe);
        assert!(!report.vulnerabilities.is_empty());
    }

    #[test]
    fn test_license_check() {
        let checker = SecurityChecker::new();
        
        assert_eq!(checker.check_license("MIT"), 100);
        assert_eq!(checker.check_license("GPL-3.0"), 70);
        assert_eq!(checker.check_license("UNKNOWN"), 30);
    }
}
