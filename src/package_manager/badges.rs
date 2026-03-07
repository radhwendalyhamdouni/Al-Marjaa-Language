// ═══════════════════════════════════════════════════════════════════════════════
// نظام الشارات والتقييم - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// نظام متقدم لتقييم الحزم ومنح شارات الجودة
// - شارات متنوعة للجودة والأمان والشعبية
// - تقييم شامل بناءً على معايير متعددة
// - توصيات لتحسين جودة الحزمة
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// شارة الجودة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    /// معرف الشارة
    pub id: String,
    /// اسم الشارة
    pub name: String,
    /// الوصف
    pub description: String,
    /// مستوى الشارة (برونزي، فضي، ذهبي، بلاتيني)
    pub level: BadgeLevel,
    /// الأيقونة (emoji)
    pub icon: String,
    /// النقاط الممنوحة
    pub points: u32,
    /// تاريخ الحصول عليها
    pub earned_at: Option<String>,
}

/// مستوى الشارة
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeLevel {
    /// برونزي
    Bronze,
    /// فضي
    Silver,
    /// ذهبي
    Gold,
    /// بلاتيني
    Platinum,
}

impl std::fmt::Display for BadgeLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeLevel::Bronze => write!(f, "برونزي"),
            BadgeLevel::Silver => write!(f, "فضي"),
            BadgeLevel::Gold => write!(f, "ذهبي"),
            BadgeLevel::Platinum => write!(f, "بلاتيني"),
        }
    }
}

/// تقييم الحزمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRating {
    /// النتيجة الإجمالية (0-100)
    pub overall_score: u8,
    /// نتيجة الجودة
    pub quality_score: u8,
    /// نتيجة الأمان
    pub security_score: u8,
    /// نتيجة الشعبية
    pub popularity_score: u8,
    /// نتيجة الصيانة
    pub maintenance_score: u8,
    /// نتيجة التوثيق
    pub documentation_score: u8,
    /// الشارات الممنوحة
    pub badges: Vec<Badge>,
    /// التوصيات
    pub recommendations: Vec<Recommendation>,
    /// تاريخ التقييم
    pub evaluated_at: String,
    /// تفاصيل التقييم
    pub details: HashMap<String, f32>,
}

/// توصية لتحسين الحزمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// الأولوية
    pub priority: RecommendationPriority,
    /// الفئة
    pub category: String,
    /// العنوان
    pub title: String,
    /// الوصف
    pub description: String,
    /// كيفية التنفيذ
    pub how_to_fix: String,
    /// تأثير التطبيق على النتيجة
    pub impact: u8,
}

/// أولوية التوصية
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendationPriority {
    /// حرج
    Critical,
    /// عالي
    High,
    /// متوسط
    Medium,
    /// منخفض
    Low,
}

impl std::fmt::Display for RecommendationPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecommendationPriority::Critical => write!(f, "حرج"),
            RecommendationPriority::High => write!(f, "عالي"),
            RecommendationPriority::Medium => write!(f, "متوسط"),
            RecommendationPriority::Low => write!(f, "منخفض"),
        }
    }
}

/// معايير التقييم
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCriteria {
    /// الحد الأدنى للتوثيق
    pub min_documentation_coverage: f32,
    /// الحد الأقصى للثغرات الأمنية الحرجة
    pub max_critical_vulnerabilities: u32,
    /// الحد الأدنى للتنزيلات للشعبية
    pub min_downloads_for_popularity: u64,
    /// الحد الأقصى لعمر آخر تحديث (أيام)
    pub max_days_since_update: u32,
    /// الحد الأدنى للاختبارات
    pub min_test_coverage: f32,
}

impl Default for EvaluationCriteria {
    fn default() -> Self {
        Self {
            min_documentation_coverage: 60.0,
            max_critical_vulnerabilities: 0,
            min_downloads_for_popularity: 1000,
            max_days_since_update: 90,
            min_test_coverage: 50.0,
        }
    }
}

/// مُقيّم الحزم
pub struct PackageEvaluator {
    /// معايير التقييم
    criteria: EvaluationCriteria,
    /// الشارات المتاحة
    available_badges: Vec<BadgeDefinition>,
}

/// تعريف الشارة
#[derive(Debug, Clone)]
struct BadgeDefinition {
    id: String,
    name: String,
    description: String,
    icon: String,
    points: u32,
    check: fn(&EvaluationInput) -> Option<BadgeLevel>,
}

/// مدخلات التقييم
#[derive(Debug, Clone)]
pub struct EvaluationInput {
    /// اسم الحزمة
    pub name: String,
    /// الإصدار
    pub version: String,
    /// الوصف
    pub description: String,
    /// عدد التنزيلات
    pub downloads: u64,
    /// عدد النجوم
    pub stars: u64,
    /// عدد الفروع
    pub forks: u64,
    /// عدد المشاكل المفتوحة
    pub open_issues: u64,
    /// عدد التبعيات
    pub dependencies_count: usize,
    /// حجم الحزمة (بايت)
    pub size: u64,
    /// لديها توثيق
    pub has_documentation: bool,
    /// لديها اختبارات
    pub has_tests: bool,
    /// لديها README
    pub has_readme: bool,
    /// لديها CHANGELOG
    pub has_changelog: bool,
    /// لديها ملف رخصة
    pub has_license: bool,
    /// نسبة تغطية التوثيق
    pub documentation_coverage: f32,
    /// نسبة تغطية الاختبارات
    pub test_coverage: f32,
    /// عدد الثغرات الأمنية
    pub vulnerabilities_count: u32,
    /// عدد الثغرات الحرجة
    pub critical_vulnerabilities: u32,
    /// أيام منذ آخر تحديث
    pub days_since_update: u32,
    /// عدد الإصدارات
    pub versions_count: u32,
    /// عدد المساهمين
    pub contributors_count: u32,
    /// مؤلف موثوق
    pub is_trusted_author: bool,
    /// لديها CI/CD
    pub has_ci: bool,
    /// مكتوبة بالكامل بالعربية
    pub is_pure_arabic: bool,
}

impl PackageEvaluator {
    /// إنشاء مُقيّم جديد
    pub fn new() -> Self {
        Self {
            criteria: EvaluationCriteria::default(),
            available_badges: Self::define_badges(),
        }
    }

    /// إنشاء مُقيّم بمعايير مخصصة
    pub fn with_criteria(criteria: EvaluationCriteria) -> Self {
        Self {
            criteria,
            available_badges: Self::define_badges(),
        }
    }

    /// تقييم حزمة
    pub fn evaluate(&self, input: &EvaluationInput) -> PackageRating {
        let now = chrono::Utc::now().to_rfc3339();

        // حساب النتائج الفرعية
        let quality_score = self.calculate_quality_score(input);
        let security_score = self.calculate_security_score(input);
        let popularity_score = self.calculate_popularity_score(input);
        let maintenance_score = self.calculate_maintenance_score(input);
        let documentation_score = self.calculate_documentation_score(input);

        // المتوسط المرجح
        let overall_score = (
            quality_score as u32 * 25 +
            security_score as u32 * 25 +
            popularity_score as u32 * 20 +
            maintenance_score as u32 * 15 +
            documentation_score as u32 * 15
        ) as u8 / 100;

        // الحصول على الشارات
        let badges = self.evaluate_badges(input);

        // توليد التوصيات
        let recommendations = self.generate_recommendations(input);

        // تفاصيل إضافية
        let mut details = HashMap::new();
        details.insert("نسخة_للنجوم".to_string(), Self::calculate_stars_score(input));
        details.insert("نسخة_للتنزيلات".to_string(), Self::calculate_downloads_score(input));
        details.insert("نسخة_للاختبارات".to_string(), input.test_coverage);
        details.insert("نسخة_للتوثيق".to_string(), input.documentation_coverage);
        details.insert("العمر_بالأيام".to_string(), input.days_since_update as f32);

        PackageRating {
            overall_score,
            quality_score,
            security_score,
            popularity_score,
            maintenance_score,
            documentation_score,
            badges,
            recommendations,
            evaluated_at: now,
            details,
        }
    }

    /// حساب نتيجة الجودة
    fn calculate_quality_score(&self, input: &EvaluationInput) -> u8 {
        let mut score = 50.0;

        // وجود README
        if input.has_readme {
            score += 10.0;
        }

        // وجود اختبارات
        if input.has_tests {
            score += 15.0;
            score += input.test_coverage * 0.1;
        }

        // تغطية الاختبارات
        if input.test_coverage >= self.criteria.min_test_coverage {
            score += 10.0;
        }

        // عدد التبعيات (أقل أفضل)
        if input.dependencies_count <= 5 {
            score += 10.0;
        } else if input.dependencies_count <= 10 {
            score += 5.0;
        } else {
            score -= (input.dependencies_count - 10) as f32 * 0.5;
        }

        // CI/CD
        if input.has_ci {
            score += 10.0;
        }

        score.clamp(0.0, 100.0) as u8
    }

    /// حساب نتيجة الأمان
    fn calculate_security_score(&self, input: &EvaluationInput) -> u8 {
        let mut score = 100.0;

        // خصم للثغرات
        score -= input.vulnerabilities_count as f32 * 5.0;
        score -= input.critical_vulnerabilities as f32 * 25.0;

        // مكافأة للتحديث المستمر
        if input.days_since_update <= 30 {
            score += 5.0;
        }

        // مكافأة للمؤلف الموثوق
        if input.is_trusted_author {
            score += 5.0;
        }

        score.clamp(0.0, 100.0) as u8
    }

    /// حساب نتيجة الشعبية
    fn calculate_popularity_score(&self, input: &EvaluationInput) -> u8 {
        let stars_score = Self::calculate_stars_score(input);
        let downloads_score = Self::calculate_downloads_score(input);
        let forks_score = Self::calculate_forks_score(input);

        let score = (stars_score * 0.4 + downloads_score * 0.4 + forks_score * 0.2);

        score.clamp(0.0, 100.0) as u8
    }

    /// حساب نتيجة الصيانة
    fn calculate_maintenance_score(&self, input: &EvaluationInput) -> u8 {
        let mut score = 50.0;

        // آخر تحديث
        if input.days_since_update <= 30 {
            score += 20.0;
        } else if input.days_since_update <= 60 {
            score += 10.0;
        } else if input.days_since_update > self.criteria.max_days_since_update {
            score -= 20.0;
        }

        // عدد الإصدارات
        if input.versions_count >= 5 {
            score += 10.0;
        } else if input.versions_count >= 3 {
            score += 5.0;
        }

        // عدد المساهمين
        if input.contributors_count >= 5 {
            score += 10.0;
        } else if input.contributors_count >= 2 {
            score += 5.0;
        }

        // المشاكل المفتوحة
        if input.open_issues <= 5 {
            score += 10.0;
        } else if input.open_issues > 20 {
            score -= 5.0;
        }

        score.clamp(0.0, 100.0) as u8
    }

    /// حساب نتيجة التوثيق
    fn calculate_documentation_score(&self, input: &EvaluationInput) -> u8 {
        let mut score = 30.0;

        // وجود توثيق
        if input.has_documentation {
            score += 20.0;
        }

        // وجود README
        if input.has_readme {
            score += 15.0;
        }

        // وجود CHANGELOG
        if input.has_changelog {
            score += 10.0;
        }

        // تغطية التوثيق
        if input.documentation_coverage >= self.criteria.min_documentation_coverage {
            score += 15.0;
        } else {
            score += input.documentation_coverage * 0.15;
        }

        // وجود رخصة
        if input.has_license {
            score += 10.0;
        }

        score.clamp(0.0, 100.0) as u8
    }

    /// حساب نتيجة النجوم
    fn calculate_stars_score(input: &EvaluationInput) -> f32 {
        if input.stars >= 10000 {
            100.0
        } else if input.stars >= 5000 {
            90.0
        } else if input.stars >= 1000 {
            80.0
        } else if input.stars >= 500 {
            70.0
        } else if input.stars >= 100 {
            60.0
        } else if input.stars >= 50 {
            50.0
        } else if input.stars >= 10 {
            40.0
        } else {
            input.stars as f32 * 3.0
        }
    }

    /// حساب نتيجة التنزيلات
    fn calculate_downloads_score(input: &EvaluationInput) -> f32 {
        if input.downloads >= 1000000 {
            100.0
        } else if input.downloads >= 500000 {
            90.0
        } else if input.downloads >= 100000 {
            80.0
        } else if input.downloads >= 50000 {
            70.0
        } else if input.downloads >= 10000 {
            60.0
        } else if input.downloads >= 1000 {
            50.0
        } else if input.downloads >= 100 {
            40.0
        } else {
            input.downloads as f32 * 0.3
        }
    }

    /// حساب نتيجة الفروع
    fn calculate_forks_score(input: &EvaluationInput) -> f32 {
        if input.forks >= 1000 {
            100.0
        } else if input.forks >= 500 {
            80.0
        } else if input.forks >= 100 {
            60.0
        } else if input.forks >= 50 {
            50.0
        } else if input.forks >= 10 {
            40.0
        } else {
            input.forks as f32 * 3.0
        }
    }

    /// تقييم الشارات
    fn evaluate_badges(&self, input: &EvaluationInput) -> Vec<Badge> {
        let mut badges = Vec::new();
        let now = chrono::Utc::now().to_rfc3339();

        for badge_def in &self.available_badges {
            if let Some(level) = (badge_def.check)(input) {
                badges.push(Badge {
                    id: badge_def.id.clone(),
                    name: badge_def.name.clone(),
                    description: badge_def.description.clone(),
                    level,
                    icon: badge_def.icon.clone(),
                    points: badge_def.points,
                    earned_at: Some(now.clone()),
                });
            }
        }

        // ترتيب حسب النقاط
        badges.sort_by(|a, b| b.points.cmp(&a.points));
        badges
    }

    /// توليد التوصيات
    fn generate_recommendations(&self, input: &EvaluationInput) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        // التحقق من الثغرات الأمنية
        if input.critical_vulnerabilities > 0 {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::Critical,
                category: "أمان".to_string(),
                title: "ثغرات أمنية حرجة".to_string(),
                description: format!("الحزمة تحتوي على {} ثغرة أمنية حرجة", input.critical_vulnerabilities),
                how_to_fix: "قم بتحديث التبعيات المتأثرة أو ترقية الحزمة".to_string(),
                impact: 25,
            });
        }

        // التحقق من الاختبارات
        if !input.has_tests {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::High,
                category: "جودة".to_string(),
                title: "لا توجد اختبارات".to_string(),
                description: "الحزمة لا تحتوي على اختبارات وحدة".to_string(),
                how_to_fix: "أضف اختبارات وحدة للدوال الأساسية".to_string(),
                impact: 15,
            });
        } else if input.test_coverage < self.criteria.min_test_coverage {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::Medium,
                category: "جودة".to_string(),
                title: "تغطية اختبارات منخفضة".to_string(),
                description: format!("تغطية الاختبارات {:.1}% أقل من الحد الأدنى", input.test_coverage),
                how_to_fix: "أضف المزيد من الاختبارات لرفع التغطية".to_string(),
                impact: 10,
            });
        }

        // التحقق من التوثيق
        if !input.has_readme {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::High,
                category: "توثيق".to_string(),
                title: "ملف README مفقود".to_string(),
                description: "الحزمة لا تحتوي على ملف README".to_string(),
                how_to_fix: "أضف ملف README.md يشرح كيفية استخدام الحزمة".to_string(),
                impact: 15,
            });
        }

        if input.documentation_coverage < self.criteria.min_documentation_coverage {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::Medium,
                category: "توثيق".to_string(),
                title: "تغطية توثيق منخفضة".to_string(),
                description: format!("تغطية التوثيق {:.1}% أقل من الحد الأدنى", input.documentation_coverage),
                how_to_fix: "أضف تعليقات توثيقية للدوال العامة".to_string(),
                impact: 10,
            });
        }

        // التحقق من التحديث
        if input.days_since_update > self.criteria.max_days_since_update {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::Medium,
                category: "صيانة".to_string(),
                title: "الحزمة غير محدثة".to_string(),
                description: format!("مضى {} يوم على آخر تحديث", input.days_since_update),
                how_to_fix: "قم بتحديث الحزمة وإصلاح المشاكل المفتوحة".to_string(),
                impact: 10,
            });
        }

        // التحقق من التبعيات
        if input.dependencies_count > 15 {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::Low,
                category: "جودة".to_string(),
                title: "عدد كبير من التبعيات".to_string(),
                description: format!("الحزمة تعتمد على {} حزمة أخرى", input.dependencies_count),
                how_to_fix: "راجع التبعيات وأزل غير الضرورية".to_string(),
                impact: 5,
            });
        }

        // التحقق من الرخصة
        if !input.has_license {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::Medium,
                category: "قانوني".to_string(),
                title: "لا يوجد ملف رخصة".to_string(),
                description: "الحزمة لا تحتوي على ملف رخصة".to_string(),
                how_to_fix: "أضف ملف LICENSE مع رخصة مناسبة (MIT, Apache-2.0, ...)".to_string(),
                impact: 10,
            });
        }

        // التحقق من CI/CD
        if !input.has_ci {
            recommendations.push(Recommendation {
                priority: RecommendationPriority::Low,
                category: "جودة".to_string(),
                title: "لا يوجد CI/CD".to_string(),
                description: "الحزمة لا تحتوي على تكامل مستمر".to_string(),
                how_to_fix: "أضف GitHub Actions أو Travis CI للتشغيل التلقائي للاختبارات".to_string(),
                impact: 5,
            });
        }

        recommendations
    }

    /// تعريف الشارات المتاحة
    fn define_badges() -> Vec<BadgeDefinition> {
        vec![
            // شارات الأمان
            BadgeDefinition {
                id: "secure".to_string(),
                name: "آمن".to_string(),
                description: "خالية من الثغرات الأمنية".to_string(),
                icon: "🔒".to_string(),
                points: 30,
                check: |input: &EvaluationInput| {
                    if input.vulnerabilities_count == 0 && input.critical_vulnerabilities == 0 {
                        Some(BadgeLevel::Gold)
                    } else if input.critical_vulnerabilities == 0 {
                        Some(BadgeLevel::Silver)
                    } else {
                        None
                    }
                },
            },
            BadgeDefinition {
                id: "verified".to_string(),
                name: "موثق".to_string(),
                description: "مؤلف موثوق ومتحقق".to_string(),
                icon: "✅".to_string(),
                points: 25,
                check: |input: &EvaluationInput| {
                    if input.is_trusted_author {
                        Some(BadgeLevel::Gold)
                    } else {
                        None
                    }
                },
            },

            // شارات الجودة
            BadgeDefinition {
                id: "tested".to_string(),
                name: "مختبر".to_string(),
                description: "اختبارات شاملة".to_string(),
                icon: "🧪".to_string(),
                points: 20,
                check: |input: &EvaluationInput| {
                    if input.test_coverage >= 90.0 {
                        Some(BadgeLevel::Platinum)
                    } else if input.test_coverage >= 75.0 {
                        Some(BadgeLevel::Gold)
                    } else if input.test_coverage >= 50.0 {
                        Some(BadgeLevel::Silver)
                    } else if input.has_tests {
                        Some(BadgeLevel::Bronze)
                    } else {
                        None
                    }
                },
            },
            BadgeDefinition {
                id: "documented".to_string(),
                name: "موثق".to_string(),
                description: "توثيق شامل".to_string(),
                icon: "📚".to_string(),
                points: 15,
                check: |input: &EvaluationInput| {
                    if input.documentation_coverage >= 90.0 && input.has_readme && input.has_changelog {
                        Some(BadgeLevel::Platinum)
                    } else if input.documentation_coverage >= 75.0 && input.has_readme {
                        Some(BadgeLevel::Gold)
                    } else if input.documentation_coverage >= 50.0 {
                        Some(BadgeLevel::Silver)
                    } else if input.has_readme {
                        Some(BadgeLevel::Bronze)
                    } else {
                        None
                    }
                },
            },
            BadgeDefinition {
                id: "minimal".to_string(),
                name: "بسيط".to_string(),
                description: "تبعيات قليلة".to_string(),
                icon: "📦".to_string(),
                points: 10,
                check: |input: &EvaluationInput| {
                    if input.dependencies_count == 0 {
                        Some(BadgeLevel::Platinum)
                    } else if input.dependencies_count <= 3 {
                        Some(BadgeLevel::Gold)
                    } else if input.dependencies_count <= 5 {
                        Some(BadgeLevel::Silver)
                    } else if input.dependencies_count <= 10 {
                        Some(BadgeLevel::Bronze)
                    } else {
                        None
                    }
                },
            },

            // شارات الشعبية
            BadgeDefinition {
                id: "popular".to_string(),
                name: "شائع".to_string(),
                description: "محبوب من المطورين".to_string(),
                icon: "⭐".to_string(),
                points: 20,
                check: |input: &EvaluationInput| {
                    if input.stars >= 10000 || input.downloads >= 1000000 {
                        Some(BadgeLevel::Platinum)
                    } else if input.stars >= 5000 || input.downloads >= 500000 {
                        Some(BadgeLevel::Gold)
                    } else if input.stars >= 1000 || input.downloads >= 100000 {
                        Some(BadgeLevel::Silver)
                    } else if input.stars >= 100 || input.downloads >= 10000 {
                        Some(BadgeLevel::Bronze)
                    } else {
                        None
                    }
                },
            },
            BadgeDefinition {
                id: "trending".to_string(),
                name: "رائج".to_string(),
                description: "نمو سريع في الاستخدام".to_string(),
                icon: "📈".to_string(),
                points: 15,
                check: |input: &EvaluationInput| {
                    if input.stars >= 1000 && input.days_since_update <= 7 {
                        Some(BadgeLevel::Gold)
                    } else if input.stars >= 500 && input.days_since_update <= 14 {
                        Some(BadgeLevel::Silver)
                    } else {
                        None
                    }
                },
            },

            // شارات الصيانة
            BadgeDefinition {
                id: "maintained".to_string(),
                name: "مُصان".to_string(),
                description: "محدث بانتظام".to_string(),
                icon: "🔧".to_string(),
                points: 15,
                check: |input: &EvaluationInput| {
                    if input.days_since_update <= 30 && input.versions_count >= 5 {
                        Some(BadgeLevel::Gold)
                    } else if input.days_since_update <= 60 {
                        Some(BadgeLevel::Silver)
                    } else if input.days_since_update <= 90 {
                        Some(BadgeLevel::Bronze)
                    } else {
                        None
                    }
                },
            },
            BadgeDefinition {
                id: "community".to_string(),
                name: "مجتمع".to_string(),
                description: "مساهمون متعددون".to_string(),
                icon: "👥".to_string(),
                points: 15,
                check: |input: &EvaluationInput| {
                    if input.contributors_count >= 20 {
                        Some(BadgeLevel::Platinum)
                    } else if input.contributors_count >= 10 {
                        Some(BadgeLevel::Gold)
                    } else if input.contributors_count >= 5 {
                        Some(BadgeLevel::Silver)
                    } else if input.contributors_count >= 2 {
                        Some(BadgeLevel::Bronze)
                    } else {
                        None
                    }
                },
            },

            // شارات خاصة
            BadgeDefinition {
                id: "arabic".to_string(),
                name: "عربي".to_string(),
                description: "مكتوب بالكامل بالعربية".to_string(),
                icon: "🌙".to_string(),
                points: 25,
                check: |input: &EvaluationInput| {
                    if input.is_pure_arabic {
                        Some(BadgeLevel::Gold)
                    } else {
                        None
                    }
                },
            },
            BadgeDefinition {
                id: "ci".to_string(),
                name: "CI/CD".to_string(),
                description: "تكامل مستمر".to_string(),
                icon: "🔄".to_string(),
                points: 10,
                check: |input: &EvaluationInput| {
                    if input.has_ci {
                        Some(BadgeLevel::Silver)
                    } else {
                        None
                    }
                },
            },
        ]
    }

    /// الحصول على مستوى الحزمة
    pub fn get_package_level(rating: &PackageRating) -> PackageLevel {
        match rating.overall_score {
            90..=100 => PackageLevel::Platinum,
            75..=89 => PackageLevel::Gold,
            60..=74 => PackageLevel::Silver,
            40..=59 => PackageLevel::Bronze,
            _ => PackageLevel::Basic,
        }
    }

    /// توليد تقرير Markdown
    pub fn generate_report(&self, rating: &PackageRating, package_name: &str) -> String {
        let level = Self::get_package_level(rating);
        let level_emoji = match level {
            PackageLevel::Platinum => "💎",
            PackageLevel::Gold => "🥇",
            PackageLevel::Silver => "🥈",
            PackageLevel::Bronze => "🥉",
            PackageLevel::Basic => "📦",
        };

        let mut report = String::new();

        report.push_str(&format!("# تقرير تقييم الحزمة: {}\n\n", package_name));
        report.push_str(&format!("## {} المستوى: {:?}\n\n", level_emoji, level));
        report.push_str(&format!("**النتيجة الإجمالية: {} / 100**\n\n", rating.overall_score));

        // النتائج الفرعية
        report.push_str("### تفاصيل التقييم\n\n");
        report.push_str("| المعيار | النتيجة |\n");
        report.push_str("|---------|--------|\n");
        report.push_str(&format!("| 🔒 الأمان | {} / 100 |\n", rating.security_score));
        report.push_str(&format!("| ⭐ الجودة | {} / 100 |\n", rating.quality_score));
        report.push_str(&format!("| 📈 الشعبية | {} / 100 |\n", rating.popularity_score));
        report.push_str(&format!("| 🔧 الصيانة | {} / 100 |\n", rating.maintenance_score));
        report.push_str(&format!("| 📚 التوثيق | {} / 100 |\n", rating.documentation_score));
        report.push_str("\n");

        // الشارات
        if !rating.badges.is_empty() {
            report.push_str("### الشارات الممنوحة\n\n");
            for badge in &rating.badges {
                report.push_str(&format!(
                    "- {} **{}** ({}) - {} نقطة\n",
                    badge.icon, badge.name, badge.level, badge.points
                ));
            }
            report.push_str("\n");
        }

        // التوصيات
        if !rating.recommendations.is_empty() {
            report.push_str("### التوصيات\n\n");
            for rec in &rating.recommendations {
                let priority_emoji = match rec.priority {
                    RecommendationPriority::Critical => "🔴",
                    RecommendationPriority::High => "🟠",
                    RecommendationPriority::Medium => "🟡",
                    RecommendationPriority::Low => "🟢",
                };
                report.push_str(&format!(
                    "{} **[{}] {}**\n  - {}\n  - الحل: {}\n  - التأثير: +{} نقطة\n\n",
                    priority_emoji, rec.category, rec.title, rec.description, rec.how_to_fix, rec.impact
                ));
            }
        }

        report.push_str(&format!("\n---\n*تم التقييم في: {}*\n", rating.evaluated_at));

        report
    }
}

/// مستوى الحزمة
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageLevel {
    /// بلاتيني
    Platinum,
    /// ذهبي
    Gold,
    /// فضي
    Silver,
    /// برونزي
    Bronze,
    /// أساسي
    Basic,
}

impl Default for PackageEvaluator {
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

    fn create_test_input() -> EvaluationInput {
        EvaluationInput {
            name: "حزمة_اختبار".to_string(),
            version: "1.0.0".to_string(),
            description: "حزمة للاختبار".to_string(),
            downloads: 5000,
            stars: 100,
            forks: 20,
            open_issues: 3,
            dependencies_count: 5,
            size: 50000,
            has_documentation: true,
            has_tests: true,
            has_readme: true,
            has_changelog: true,
            has_license: true,
            documentation_coverage: 80.0,
            test_coverage: 75.0,
            vulnerabilities_count: 0,
            critical_vulnerabilities: 0,
            days_since_update: 15,
            versions_count: 5,
            contributors_count: 3,
            is_trusted_author: true,
            has_ci: true,
            is_pure_arabic: true,
        }
    }

    #[test]
    fn test_evaluator_creation() {
        let evaluator = PackageEvaluator::new();
        assert!(!evaluator.available_badges.is_empty());
    }

    #[test]
    fn test_evaluate_package() {
        let evaluator = PackageEvaluator::new();
        let input = create_test_input();
        let rating = evaluator.evaluate(&input);

        assert!(rating.overall_score > 0);
        assert!(!rating.badges.is_empty());
    }

    #[test]
    fn test_security_score() {
        let evaluator = PackageEvaluator::new();
        let mut input = create_test_input();
        input.critical_vulnerabilities = 2;

        let rating = evaluator.evaluate(&input);
        assert!(rating.security_score < 100);
    }

    #[test]
    fn test_badge_earning() {
        let evaluator = PackageEvaluator::new();
        let input = create_test_input();
        let rating = evaluator.evaluate(&input);

        // يجب أن تحصل على شارة "آمن" لعدم وجود ثغرات
        assert!(rating.badges.iter().any(|b| b.id == "secure"));
    }

    #[test]
    fn test_recommendations() {
        let evaluator = PackageEvaluator::new();
        let mut input = create_test_input();
        input.has_tests = false;
        input.has_readme = false;

        let rating = evaluator.evaluate(&input);
        assert!(!rating.recommendations.is_empty());
    }

    #[test]
    fn test_package_level() {
        let evaluator = PackageEvaluator::new();
        let input = create_test_input();
        let rating = evaluator.evaluate(&input);
        let level = PackageEvaluator::get_package_level(&rating);

        assert!(matches!(level, PackageLevel::Silver | PackageLevel::Gold | PackageLevel::Platinum));
    }

    #[test]
    fn test_report_generation() {
        let evaluator = PackageEvaluator::new();
        let input = create_test_input();
        let rating = evaluator.evaluate(&input);
        let report = evaluator.generate_report(&rating, "حزمة_اختبار");

        assert!(report.contains("تقرير تقييم"));
        assert!(report.contains("الشارات"));
    }
}
