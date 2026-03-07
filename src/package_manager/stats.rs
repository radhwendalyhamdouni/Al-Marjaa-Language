// ═══════════════════════════════════════════════════════════════════════════════
// إحصائيات الحزم - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// إحصائيات الحزمة
pub struct PackageStats {
    /// التنزيلات
    downloads: HashMap<String, DownloadStats>,
    /// الاستخدام
    usage: HashMap<String, UsageStats>,
    /// التقييمات
    ratings: HashMap<String, PackageRating>,
}

/// إحصائيات التنزيل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStats {
    /// اسم الحزمة
    pub package: String,
    /// تنزيلات اليوم
    pub daily: u64,
    /// تنزيلات الأسبوع
    pub weekly: u64,
    /// تنزيلات الشهر
    pub monthly: u64,
    /// تنزيلات السنة
    pub yearly: u64,
    /// الإجمالي
    pub total: u64,
    /// اتجاه النمو
    pub trend: f64,
}

/// إحصائيات الاستخدام
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    /// اسم الحزمة
    pub package: String,
    /// المشاريع المستخدمة
    pub projects_count: u64,
    /// المشاريع المباشرة
    pub direct_dependencies: u64,
    /// المشاريع غير المباشرة
    pub indirect_dependencies: u64,
    /// آخر استخدام
    pub last_used: String,
}

/// تقييم الحزمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRating {
    /// اسم الحزمة
    pub package: String,
    /// متوسط التقييم
    pub average: f64,
    /// عدد التقييمات
    pub count: u64,
    /// التوزيع (5 نجوم)
    pub distribution: [u64; 5],
}

/// تقرير إحصائي
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsReport {
    /// الحزم الأكثر تنزيلاً
    pub top_downloads: Vec<DownloadStats>,
    /// الحزم الأكثر استخداماً
    pub top_used: Vec<UsageStats>,
    /// الحزم الأعلى تقييماً
    pub top_rated: Vec<PackageRating>,
    /// إحصائيات عامة
    pub summary: GlobalStats,
}

/// إحصائيات عامة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStats {
    /// إجمالي الحزم
    pub total_packages: u64,
    /// إجمالي التنزيلات
    pub total_downloads: u64,
    /// الحزم النشطة
    pub active_packages: u64,
    /// المتوسط اليومي
    pub daily_average: u64,
}

impl PackageStats {
    /// إنشاء إحصائيات جديدة
    pub fn new() -> Self {
        Self {
            downloads: HashMap::new(),
            usage: HashMap::new(),
            ratings: HashMap::new(),
        }
    }

    /// تسجيل تنزيل
    pub fn record_download(&mut self, package: &str) {
        let stats = self.downloads.entry(package.to_string()).or_insert(DownloadStats {
            package: package.to_string(),
            daily: 0,
            weekly: 0,
            monthly: 0,
            yearly: 0,
            total: 0,
            trend: 0.0,
        });
        
        stats.daily += 1;
        stats.weekly += 1;
        stats.monthly += 1;
        stats.yearly += 1;
        stats.total += 1;
    }

    /// الحصول على إحصائيات التنزيل
    pub fn get_download_stats(&self, package: &str) -> Option<&DownloadStats> {
        self.downloads.get(package)
    }

    /// الحزم الأكثر تنزيلاً
    pub fn top_downloads(&self, limit: usize) -> Vec<DownloadStats> {
        let mut packages: Vec<_> = self.downloads.values().cloned().collect();
        packages.sort_by(|a, b| b.total.cmp(&a.total));
        packages.into_iter().take(limit).collect()
    }

    /// الحزم الأكثر استخداماً
    pub fn top_used(&self, limit: usize) -> Vec<UsageStats> {
        let mut packages: Vec<_> = self.usage.values().cloned().collect();
        packages.sort_by(|a, b| b.projects_count.cmp(&a.projects_count));
        packages.into_iter().take(limit).collect()
    }

    /// الحزم الأعلى تقييماً
    pub fn top_rated(&self, limit: usize) -> Vec<PackageRating> {
        let mut packages: Vec<_> = self.ratings.values().cloned().collect();
        packages.sort_by(|a, b| b.average.partial_cmp(&a.average).unwrap_or(std::cmp::Ordering::Equal));
        packages.into_iter().take(limit).collect()
    }

    /// إنشاء تقرير
    pub fn generate_report(&self) -> StatsReport {
        let total_downloads: u64 = self.downloads.values().map(|s| s.total).sum();
        let daily_average = total_downloads / 365.max(1);
        
        StatsReport {
            top_downloads: self.top_downloads(10),
            top_used: self.top_used(10),
            top_rated: self.top_rated(10),
            summary: GlobalStats {
                total_packages: self.downloads.len() as u64,
                total_downloads,
                active_packages: self.usage.values().filter(|u| u.projects_count > 0).count() as u64,
                daily_average,
            },
        }
    }

    /// طباعة التقرير
    pub fn print_report(&self) {
        let report = self.generate_report();
        
        println!("\n📊 ═════════════════════════════════════════════");
        println!("   تقرير إحصائيات المستودع");
        println!("📊 ═════════════════════════════════════════════\n");
        
        println!("📈 ملخص عام:");
        println!("   • إجمالي الحزم: {}", report.summary.total_packages);
        println!("   • إجمالي التنزيلات: {}", Self::format_number(report.summary.total_downloads));
        println!("   • المتوسط اليومي: {}", Self::format_number(report.summary.daily_average));
        
        println!("\n🏆 أكثر الحزم تنزيلاً:");
        for (i, stats) in report.top_downloads.iter().enumerate().take(5) {
            println!("   {}. {} - {}", i + 1, stats.package, Self::format_number(stats.total));
        }
        
        println!("\n⭐ أعلى الحزم تقييماً:");
        for (i, rating) in report.top_rated.iter().enumerate().take(5) {
            println!("   {}. {} - ⭐ {:.1}", i + 1, rating.package, rating.average);
        }
    }

    /// تنسيق الأرقام
    fn format_number(n: u64) -> String {
        if n >= 1_000_000 {
            format!("{:.1}M", n as f64 / 1_000_000.0)
        } else if n >= 1_000 {
            format!("{:.1}K", n as f64 / 1_000.0)
        } else {
            n.to_string()
        }
    }

    /// تحميل الإحصائيات الافتراضية
    pub fn load_default_stats() -> Self {
        let mut stats = Self::new();
        
        // إضافة إحصائيات افتراضية للحزم المدمجة
        let defaults = [
            ("json", 50000),
            ("http", 35000),
            ("قاعدة_بيانات", 28000),
            ("ملفات", 25000),
            ("رياضيات", 20000),
            ("تاريخ", 15000),
            ("تشفير", 12000),
            ("ويب", 10000),
        ];
        
        for (name, downloads) in defaults {
            stats.downloads.insert(name.to_string(), DownloadStats {
                package: name.to_string(),
                daily: downloads / 365,
                weekly: downloads / 52,
                monthly: downloads / 12,
                yearly: downloads,
                total: downloads,
                trend: 0.0,
            });
            
            stats.ratings.insert(name.to_string(), PackageRating {
                package: name.to_string(),
                average: 4.5,
                count: 100,
                distribution: [5, 10, 20, 30, 35],
            });
        }
        
        stats
    }
}

impl Default for PackageStats {
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
    fn test_stats_creation() {
        let stats = PackageStats::new();
        assert!(stats.downloads.is_empty());
    }

    #[test]
    fn test_record_download() {
        let mut stats = PackageStats::new();
        stats.record_download("test");
        
        let dl = stats.get_download_stats("test").unwrap();
        assert_eq!(dl.daily, 1);
        assert_eq!(dl.total, 1);
    }

    #[test]
    fn test_top_downloads() {
        let mut stats = PackageStats::new();
        stats.record_download("a");
        stats.record_download("a");
        stats.record_download("b");
        
        let top = stats.top_downloads(10);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].package, "a");
    }

    #[test]
    fn test_format_number() {
        assert_eq!(PackageStats::format_number(500), "500");
        assert_eq!(PackageStats::format_number(5000), "5.0K");
        assert_eq!(PackageStats::format_number(5000000), "5.0M");
    }

    #[test]
    fn test_default_stats() {
        let stats = PackageStats::load_default_stats();
        
        assert!(!stats.downloads.is_empty());
        assert!(stats.get_download_stats("json").is_some());
    }
}
