// ═══════════════════════════════════════════════════════════════════════════════
// نظام السمعة والمؤلفين - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// نظام متقدم لتقييم المؤلفين وبناء السمعة
// - ملف تعريف المؤلف
// - نظام السمعة والنقاط
// - شارات الإنجاز للمؤلفين
// - التحقق من الهوية
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ملف تعريف المؤلف
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorProfile {
    /// المعرف الفريد
    pub id: String,
    /// اسم المستخدم
    pub username: String,
    /// الاسم الحقيقي
    pub real_name: Option<String>,
    /// البريد الإلكتروني
    pub email: Option<String>,
    /// الرابط في GitHub
    pub github: Option<String>,
    /// الرابط في Twitter
    pub twitter: Option<String>,
    /// الموقع الشخصي
    pub website: Option<String>,
    /// السيرة الذاتية
    pub bio: String,
    /// الدولة
    pub country: Option<String>,
    /// تاريخ التسجيل
    pub joined_at: String,
    /// آخر نشاط
    pub last_active: String,
    /// حالة التحقق
    pub verification_status: VerificationStatus,
    /// نقاط السمعة
    pub reputation_points: u64,
    /// المستوى
    pub level: AuthorLevel,
    /// الشارات
    pub badges: Vec<AuthorBadge>,
    /// الإحصائيات
    pub stats: AuthorStats,
    /// الحزم المنشورة
    pub packages: Vec<String>,
    /// المساهمات
    pub contributions: Vec<Contribution>,
}

/// حالة التحقق
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    /// غير موثق
    Unverified,
    /// قيد التحقق
    Pending,
    /// موثق بالبريد
    EmailVerified,
    /// موثق بـ GitHub
    GithubVerified,
    /// موثق بالكامل
    FullyVerified,
}

impl std::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationStatus::Unverified => write!(f, "غير موثق"),
            VerificationStatus::Pending => write!(f, "قيد التحقق"),
            VerificationStatus::EmailVerified => write!(f, "موثق بالبريد"),
            VerificationStatus::GithubVerified => write!(f, "موثق بـ GitHub"),
            VerificationStatus::FullyVerified => write!(f, "موثق بالكامل"),
        }
    }
}

/// مستوى المؤلف
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorLevel {
    /// مبتدئ
    Beginner,
    /// مساهم
    Contributor,
    /// مطور
    Developer,
    /// خبير
    Expert,
    /// محترف
    Professional,
    /// أسطورة
    Legend,
}

impl std::fmt::Display for AuthorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorLevel::Beginner => write!(f, "مبتدئ"),
            AuthorLevel::Contributor => write!(f, "مساهم"),
            AuthorLevel::Developer => write!(f, "مطور"),
            AuthorLevel::Expert => write!(f, "خبير"),
            AuthorLevel::Professional => write!(f, "محترف"),
            AuthorLevel::Legend => write!(f, "أسطورة"),
        }
    }
}

impl AuthorLevel {
    /// النقاط المطلوبة للمستوى
    pub fn required_points(&self) -> u64 {
        match self {
            AuthorLevel::Beginner => 0,
            AuthorLevel::Contributor => 100,
            AuthorLevel::Developer => 500,
            AuthorLevel::Expert => 2000,
            AuthorLevel::Professional => 10000,
            AuthorLevel::Legend => 50000,
        }
    }

    /// المستوى التالي
    pub fn next(&self) -> Option<AuthorLevel> {
        match self {
            AuthorLevel::Beginner => Some(AuthorLevel::Contributor),
            AuthorLevel::Contributor => Some(AuthorLevel::Developer),
            AuthorLevel::Developer => Some(AuthorLevel::Expert),
            AuthorLevel::Expert => Some(AuthorLevel::Professional),
            AuthorLevel::Professional => Some(AuthorLevel::Legend),
            AuthorLevel::Legend => None,
        }
    }

    /// الأيقونة
    pub fn icon(&self) -> &str {
        match self {
            AuthorLevel::Beginner => "🌱",
            AuthorLevel::Contributor => "🤝",
            AuthorLevel::Developer => "💻",
            AuthorLevel::Expert => "⭐",
            AuthorLevel::Professional => "🏆",
            AuthorLevel::Legend => "👑",
        }
    }
}

/// إحصائيات المؤلف
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorStats {
    /// عدد الحزم
    pub packages_count: u32,
    /// إجمالي التنزيلات
    pub total_downloads: u64,
    /// إجمالي النجوم
    pub total_stars: u64,
    /// عدد المساهمات
    pub contributions_count: u32,
    /// عدد المتابعين
    pub followers_count: u32,
    /// عدد المتابعات
    pub following_count: u32,
    /// المتوسط التقييم
    pub average_rating: f32,
    /// أفضل تقييم
    pub best_rating: u8,
    /// شهور النشاط
    pub active_months: u32,
}

impl Default for AuthorStats {
    fn default() -> Self {
        Self {
            packages_count: 0,
            total_downloads: 0,
            total_stars: 0,
            contributions_count: 0,
            followers_count: 0,
            following_count: 0,
            average_rating: 0.0,
            best_rating: 0,
            active_months: 0,
        }
    }
}

/// شارة المؤلف
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorBadge {
    /// المعرف
    pub id: String,
    /// الاسم
    pub name: String,
    /// الوصف
    pub description: String,
    /// الأيقونة
    pub icon: String,
    /// نادر
    pub is_rare: bool,
    /// تاريخ الحصول
    pub earned_at: String,
}

/// مساهمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    /// نوع المساهمة
    pub kind: ContributionKind,
    /// الحزمة
    pub package: String,
    /// الوصف
    pub description: String,
    /// النقاط
    pub points: u32,
    /// التاريخ
    pub date: String,
}

/// نوع المساهمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionKind {
    /// نشر حزمة
    PackagePublished,
    /// تحديث حزمة
    PackageUpdated,
    /// إصلاح خطأ
    BugFix,
    /// إضافة ميزة
    FeatureAdded,
    /// تحسين التوثيق
    DocumentationImproved,
    /// مساهمة في كود
    CodeContribution,
    /// مراجعة كود
    CodeReview,
    /// تقرير خطأ
    BugReport,
    /// اقتراح ميزة
    FeatureSuggestion,
    /// مساعدة مجتمع
    CommunityHelp,
}

/// نظام السمعة
pub struct ReputationSystem {
    /// المؤلفون المسجلون
    authors: HashMap<String, AuthorProfile>,
    /// قواعد النقاط
    point_rules: HashMap<String, u32>,
    /// الشارات المتاحة
    available_badges: Vec<AuthorBadgeDefinition>,
}

/// تعريف شارة المؤلف
#[derive(Debug, Clone)]
struct AuthorBadgeDefinition {
    id: String,
    name: String,
    description: String,
    icon: String,
    is_rare: bool,
    check: fn(&AuthorProfile) -> bool,
}

impl ReputationSystem {
    /// إنشاء نظام سمعة جديد
    pub fn new() -> Self {
        Self {
            authors: HashMap::new(),
            point_rules: Self::default_point_rules(),
            available_badges: Self::define_badges(),
        }
    }

    /// تسجيل مؤلف جديد
    pub fn register_author(&mut self, username: &str, email: Option<&str>) -> AuthorProfile {
        let now = chrono::Utc::now().to_rfc3339();
        
        let profile = AuthorProfile {
            id: Self::generate_id(),
            username: username.to_string(),
            real_name: None,
            email: email.map(|e| e.to_string()),
            github: None,
            twitter: None,
            website: None,
            bio: String::new(),
            country: None,
            joined_at: now.clone(),
            last_active: now,
            verification_status: VerificationStatus::Unverified,
            reputation_points: 0,
            level: AuthorLevel::Beginner,
            badges: vec![],
            stats: AuthorStats::default(),
            packages: vec![],
            contributions: vec![],
        };
        
        self.authors.insert(username.to_string(), profile.clone());
        profile
    }

    /// الحصول على ملف المؤلف
    pub fn get_author(&self, username: &str) -> Option<&AuthorProfile> {
        self.authors.get(username)
    }

    /// تحديث ملف المؤلف
    pub fn update_author(&mut self, username: &str, updates: AuthorUpdates) -> Result<(), String> {
        let profile = self.authors.get_mut(username)
            .ok_or("المؤلف غير موجود")?;

        if let Some(bio) = updates.bio {
            profile.bio = bio;
        }
        if let Some(country) = updates.country {
            profile.country = Some(country);
        }
        if let Some(github) = updates.github {
            profile.github = Some(github);
        }
        if let Some(website) = updates.website {
            profile.website = Some(website);
        }

        profile.last_active = chrono::Utc::now().to_rfc3339();
        Ok(())
    }

    /// إضافة مساهمة
    pub fn add_contribution(&mut self, username: &str, contribution: Contribution) -> Result<(), String> {
        let profile = self.authors.get_mut(username)
            .ok_or("المؤلف غير موجود")?;

        // إضافة النقاط
        profile.reputation_points += contribution.points as u64;

        // إضافة المساهمة
        profile.contributions.push(contribution.clone());

        // تحديث الإحصائيات
        match contribution.kind {
            ContributionKind::PackagePublished => {
                profile.stats.packages_count += 1;
            }
            ContributionKind::CodeContribution | ContributionKind::BugFix => {
                profile.stats.contributions_count += 1;
            }
            _ => {}
        }

        // تحديث المستوى
        self.update_level(username);

        // التحقق من الشارات الجديدة
        self.check_badges(username);

        Ok(())
    }

    /// تحديث المستوى
    fn update_level(&mut self, username: &str) {
        let profile = self.authors.get_mut(username).unwrap();
        
        let new_level = Self::calculate_level(profile.reputation_points);
        if new_level != profile.level {
            profile.level = new_level;
        }
    }

    /// حساب المستوى من النقاط
    fn calculate_level(points: u64) -> AuthorLevel {
        if points >= AuthorLevel::Legend.required_points() {
            AuthorLevel::Legend
        } else if points >= AuthorLevel::Professional.required_points() {
            AuthorLevel::Professional
        } else if points >= AuthorLevel::Expert.required_points() {
            AuthorLevel::Expert
        } else if points >= AuthorLevel::Developer.required_points() {
            AuthorLevel::Developer
        } else if points >= AuthorLevel::Contributor.required_points() {
            AuthorLevel::Contributor
        } else {
            AuthorLevel::Beginner
        }
    }

    /// التحقق من الشارات
    fn check_badges(&mut self, username: &str) {
        let profile = self.authors.get(username).unwrap();
        let existing_badge_ids: Vec<&String> = profile.badges.iter().map(|b| &b.id).collect();
        
        let mut new_badges = Vec::new();
        
        for badge_def in &self.available_badges {
            if existing_badge_ids.contains(&badge_def.id) {
                continue;
            }
            
            if (badge_def.check)(profile) {
                new_badges.push(AuthorBadge {
                    id: badge_def.id.clone(),
                    name: badge_def.name.clone(),
                    description: badge_def.description.clone(),
                    icon: badge_def.icon.clone(),
                    is_rare: badge_def.is_rare,
                    earned_at: chrono::Utc::now().to_rfc3339(),
                });
            }
        }
        
        // إضافة الشارات الجديدة
        if let Some(p) = self.authors.get_mut(username) {
            p.badges.extend(new_badges);
        }
    }

    /// التحقق من المؤلف
    pub fn verify_author(&mut self, username: &str, method: VerificationMethod) -> Result<VerificationStatus, String> {
        let profile = self.authors.get_mut(username)
            .ok_or("المؤلف غير موجود")?;

        let new_status = match method {
            VerificationMethod::Email => {
                if profile.email.is_some() {
                    VerificationStatus::EmailVerified
                } else {
                    return Err("البريد الإلكتروني غير محدد".to_string());
                }
            }
            VerificationMethod::Github => {
                if profile.github.is_some() {
                    VerificationStatus::GithubVerified
                } else {
                    return Err("حساب GitHub غير مربوط".to_string());
                }
            }
        };

        // إذا كان موثق بالبريد و GitHub
        if matches!(new_status, VerificationStatus::EmailVerified) 
            && matches!(profile.verification_status, VerificationStatus::GithubVerified)
            || matches!(new_status, VerificationStatus::GithubVerified) 
            && matches!(profile.verification_status, VerificationStatus::EmailVerified) {
            profile.verification_status = VerificationStatus::FullyVerified;
        } else {
            profile.verification_status = new_status;
        }

        Ok(profile.verification_status)
    }

    /// إضافة متابع
    pub fn follow(&mut self, follower: &str, following: &str) -> Result<(), String> {
        if follower == following {
            return Err("لا يمكنك متابعة نفسك".to_string());
        }

        let follower_profile = self.authors.get_mut(follower)
            .ok_or("المتابع غير موجود")?;
        follower_profile.stats.following_count += 1;

        let following_profile = self.authors.get_mut(following)
            .ok_or("المتابَع غير موجود")?;
        following_profile.stats.followers_count += 1;

        Ok(())
    }

    /// الحصول على لوحة المتصدرين
    pub fn get_leaderboard(&self, limit: usize) -> Vec<AuthorProfile> {
        let mut authors: Vec<_> = self.authors.values().cloned().collect();
        authors.sort_by(|a, b| b.reputation_points.cmp(&a.reputation_points));
        authors.into_iter().take(limit).collect()
    }

    /// البحث عن مؤلفين
    pub fn search_authors(&self, query: &str) -> Vec<&AuthorProfile> {
        let query_lower = query.to_lowercase();
        
        self.authors.values()
            .filter(|a| {
                a.username.to_lowercase().contains(&query_lower) ||
                a.bio.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// توليد معرف فريد
    fn generate_id() -> String {
        format!("author_{}", uuid::Uuid::new_v4())
    }

    /// قواعد النقاط الافتراضية
    fn default_point_rules() -> HashMap<String, u32> {
        let mut rules = HashMap::new();
        
        rules.insert("نشر_حزمة".to_string(), 50);
        rules.insert("تحديث_حزمة".to_string(), 10);
        rules.insert("إصلاح_خطأ".to_string(), 30);
        rules.insert("إضافة_ميزة".to_string(), 40);
        rules.insert("تحسين_توثيق".to_string(), 15);
        rules.insert("مراجعة_كود".to_string(), 20);
        rules.insert("تقرير_خطأ".to_string(), 10);
        rules.insert("اقتراح_ميزة".to_string(), 5);
        rules.insert("مساعدة_مجتمع".to_string(), 25);
        
        rules
    }

    /// تعريف الشارات
    fn define_badges() -> Vec<AuthorBadgeDefinition> {
        vec![
            AuthorBadgeDefinition {
                id: "first_package".to_string(),
                name: "البداية".to_string(),
                description: "نشر أول حزمة".to_string(),
                icon: "🎁".to_string(),
                is_rare: false,
                check: |p: &AuthorProfile| p.stats.packages_count >= 1,
            },
            AuthorBadgeDefinition {
                id: "prolific".to_string(),
                name: "مُنتِج".to_string(),
                description: "نشر 10 حزم".to_string(),
                icon: "📦".to_string(),
                is_rare: false,
                check: |p: &AuthorProfile| p.stats.packages_count >= 10,
            },
            AuthorBadgeDefinition {
                id: "popular".to_string(),
                name: "مشهور".to_string(),
                description: "1000 متابع".to_string(),
                icon: "🌟".to_string(),
                is_rare: true,
                check: |p: &AuthorProfile| p.stats.followers_count >= 1000,
            },
            AuthorBadgeDefinition {
                id: "helpful".to_string(),
                name: "مُفيد".to_string(),
                description: "50 مساهمة مجتمع".to_string(),
                icon: "🤝".to_string(),
                is_rare: false,
                check: |p: &AuthorProfile| {
                    p.contributions.iter()
                        .filter(|c| matches!(c.kind, ContributionKind::CommunityHelp))
                        .count() >= 50
                },
            },
            AuthorBadgeDefinition {
                id: "bug_hunter".to_string(),
                name: "صياد الأخطاء".to_string(),
                description: "أبلغ عن 20 خطأ".to_string(),
                icon: "🐛".to_string(),
                is_rare: false,
                check: |p: &AuthorProfile| {
                    p.contributions.iter()
                        .filter(|c| matches!(c.kind, ContributionKind::BugReport))
                        .count() >= 20
                },
            },
            AuthorBadgeDefinition {
                id: "verified".to_string(),
                name: "موثق".to_string(),
                description: "تم التحقق من الحساب".to_string(),
                icon: "✅".to_string(),
                is_rare: false,
                check: |p: &AuthorProfile| {
                    matches!(p.verification_status, VerificationStatus::FullyVerified)
                },
            },
            AuthorBadgeDefinition {
                id: "star_creator".to_string(),
                name: "صانع النجوم".to_string(),
                description: "1000 نجمة على الحزم".to_string(),
                icon: "⭐".to_string(),
                is_rare: true,
                check: |p: &AuthorProfile| p.stats.total_stars >= 1000,
            },
            AuthorBadgeDefinition {
                id: "download_king".to_string(),
                name: "ملك التنزيلات".to_string(),
                description: "10000 تنزيل".to_string(),
                icon: "👑".to_string(),
                is_rare: true,
                check: |p: &AuthorProfile| p.stats.total_downloads >= 10000,
            },
            AuthorBadgeDefinition {
                id: "expert".to_string(),
                name: "خبير".to_string(),
                description: "وصول مستوى خبير".to_string(),
                icon: "🎓".to_string(),
                is_rare: false,
                check: |p: &AuthorProfile| {
                    matches!(p.level, AuthorLevel::Expert | AuthorLevel::Professional | AuthorLevel::Legend)
                },
            },
            AuthorBadgeDefinition {
                id: "legend".to_string(),
                name: "أسطورة".to_string(),
                description: "وصول مستوى أسطورة".to_string(),
                icon: "🏆".to_string(),
                is_rare: true,
                check: |p: &AuthorProfile| matches!(p.level, AuthorLevel::Legend),
            },
        ]
    }
}

/// تحديثات المؤلف
#[derive(Debug, Clone, Default)]
pub struct AuthorUpdates {
    pub bio: Option<String>,
    pub country: Option<String>,
    pub github: Option<String>,
    pub website: Option<String>,
}

/// طريقة التحقق
#[derive(Debug, Clone)]
pub enum VerificationMethod {
    Email,
    Github,
}

impl Default for ReputationSystem {
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
    fn test_register_author() {
        let mut system = ReputationSystem::new();
        let profile = system.register_author("أحمد", Some("ahmed@example.com"));
        
        assert_eq!(profile.username, "أحمد");
        assert!(profile.email.is_some());
        assert_eq!(profile.level, AuthorLevel::Beginner);
    }

    #[test]
    fn test_add_contribution() {
        let mut system = ReputationSystem::new();
        system.register_author("أحمد", None);
        
        let contribution = Contribution {
            kind: ContributionKind::PackagePublished,
            package: "حزمة_اختبار".to_string(),
            description: "نشر حزمة جديدة".to_string(),
            points: 50,
            date: chrono::Utc::now().to_rfc3339(),
        };
        
        let result = system.add_contribution("أحمد", contribution);
        assert!(result.is_ok());
        
        let profile = system.get_author("أحمد").unwrap();
        assert_eq!(profile.reputation_points, 50);
    }

    #[test]
    fn test_level_progression() {
        let mut system = ReputationSystem::new();
        system.register_author("أحمد", None);
        
        // إضافة 100 نقطة
        for _ in 0..2 {
            let contribution = Contribution {
                kind: ContributionKind::PackagePublished,
                package: "test".to_string(),
                description: String::new(),
                points: 50,
                date: chrono::Utc::now().to_rfc3339(),
            };
            system.add_contribution("أحمد", contribution).unwrap();
        }
        
        let profile = system.get_author("أحمد").unwrap();
        assert_eq!(profile.level, AuthorLevel::Contributor);
    }

    #[test]
    fn test_verification() {
        let mut system = ReputationSystem::new();
        system.register_author("أحمد", Some("ahmed@example.com"));
        
        // تحديث GitHub
        let updates = AuthorUpdates {
            github: Some("ahmed".to_string()),
            ..Default::default()
        };
        system.update_author("أحمد", updates).unwrap();
        
        // التحقق بالبريد
        let status = system.verify_author("أحمد", VerificationMethod::Email);
        assert!(matches!(status.unwrap(), VerificationStatus::EmailVerified));
    }

    #[test]
    fn test_first_package_badge() {
        let mut system = ReputationSystem::new();
        system.register_author("أحمد", None);
        
        // نشر حزمة
        let contribution = Contribution {
            kind: ContributionKind::PackagePublished,
            package: "test".to_string(),
            description: String::new(),
            points: 50,
            date: chrono::Utc::now().to_rfc3339(),
        };
        system.add_contribution("أحمد", contribution).unwrap();
        
        let profile = system.get_author("أحمد").unwrap();
        assert!(profile.badges.iter().any(|b| b.id == "first_package"));
    }

    #[test]
    fn test_leaderboard() {
        let mut system = ReputationSystem::new();
        
        system.register_author("أحمد", None);
        system.register_author("محمد", None);
        
        // أحمد لديه نقاط أكثر
        let contribution = Contribution {
            kind: ContributionKind::PackagePublished,
            package: "test".to_string(),
            description: String::new(),
            points: 50,
            date: chrono::Utc::now().to_rfc3339(),
        };
        system.add_contribution("أحمد", contribution).unwrap();
        
        let leaderboard = system.get_leaderboard(10);
        assert_eq!(leaderboard[0].username, "أحمد");
    }

    #[test]
    fn test_level_requirements() {
        assert_eq!(AuthorLevel::Beginner.required_points(), 0);
        assert_eq!(AuthorLevel::Contributor.required_points(), 100);
        assert_eq!(AuthorLevel::Developer.required_points(), 500);
        assert_eq!(AuthorLevel::Expert.required_points(), 2000);
        assert_eq!(AuthorLevel::Professional.required_points(), 10000);
        assert_eq!(AuthorLevel::Legend.required_points(), 50000);
    }

    #[test]
    fn test_level_next() {
        assert_eq!(AuthorLevel::Beginner.next(), Some(AuthorLevel::Contributor));
        assert_eq!(AuthorLevel::Legend.next(), None);
    }
}
