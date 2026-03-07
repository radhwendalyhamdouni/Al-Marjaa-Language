// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات نظام الحزم المحسّن - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::package_manager::*;

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الشارات والتقييم
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_package_evaluation() {
    let evaluator = PackageEvaluator::new();
    
    let input = EvaluationInput {
        name: "json".to_string(),
        version: "1.0.0".to_string(),
        description: "مكتبة JSON".to_string(),
        downloads: 10000,
        stars: 500,
        forks: 50,
        open_issues: 5,
        dependencies_count: 2,
        size: 30000,
        has_documentation: true,
        has_tests: true,
        has_readme: true,
        has_changelog: true,
        has_license: true,
        documentation_coverage: 85.0,
        test_coverage: 80.0,
        vulnerabilities_count: 0,
        critical_vulnerabilities: 0,
        days_since_update: 10,
        versions_count: 8,
        contributors_count: 5,
        is_trusted_author: true,
        has_ci: true,
        is_pure_arabic: true,
    };
    
    let rating = evaluator.evaluate(&input);
    
    // يجب أن تحصل على نتيجة عالية
    assert!(rating.overall_score >= 70, "النتيجة الإجمالية يجب أن تكون >= 70");
    
    // يجب أن تحصل على شارات
    assert!(!rating.badges.is_empty(), "يجب الحصول على شارات");
    
    // يجب أن تحصل على شارة "آمن"
    assert!(rating.badges.iter().any(|b| b.id == "secure"), "يجب الحصول على شارة آمن");
    
    // يجب أن تحصل على شارة "عربي"
    assert!(rating.badges.iter().any(|b| b.id == "arabic"), "يجب الحصول على شارة عربي");
}

#[test]
fn test_badge_levels() {
    let evaluator = PackageEvaluator::new();
    
    // حزمة بلاتينية
    let platinum_input = EvaluationInput {
        name: "super_package".to_string(),
        version: "1.0.0".to_string(),
        description: "حزمة ممتازة".to_string(),
        downloads: 500000,
        stars: 10000,
        forks: 500,
        open_issues: 2,
        dependencies_count: 1,
        size: 20000,
        has_documentation: true,
        has_tests: true,
        has_readme: true,
        has_changelog: true,
        has_license: true,
        documentation_coverage: 95.0,
        test_coverage: 95.0,
        vulnerabilities_count: 0,
        critical_vulnerabilities: 0,
        days_since_update: 5,
        versions_count: 20,
        contributors_count: 30,
        is_trusted_author: true,
        has_ci: true,
        is_pure_arabic: true,
    };
    
    let rating = evaluator.evaluate(&platinum_input);
    
    // التحقق من وجود شارات بلاتينية
    assert!(rating.badges.iter().any(|b| matches!(b.level, BadgeLevel::Platinum)));
}

#[test]
fn test_recommendations() {
    let evaluator = PackageEvaluator::new();
    
    // حزمة تحتاج تحسينات
    let poor_input = EvaluationInput {
        name: "poor_package".to_string(),
        version: "0.1.0".to_string(),
        description: "".to_string(),
        downloads: 10,
        stars: 0,
        forks: 0,
        open_issues: 50,
        dependencies_count: 25,
        size: 500000,
        has_documentation: false,
        has_tests: false,
        has_readme: false,
        has_changelog: false,
        has_license: false,
        documentation_coverage: 0.0,
        test_coverage: 0.0,
        vulnerabilities_count: 3,
        critical_vulnerabilities: 1,
        days_since_update: 365,
        versions_count: 1,
        contributors_count: 1,
        is_trusted_author: false,
        has_ci: false,
        is_pure_arabic: false,
    };
    
    let rating = evaluator.evaluate(&poor_input);
    
    // يجب أن تحصل على توصيات
    assert!(!rating.recommendations.is_empty(), "يجب الحصول على توصيات");
    
    // يجب أن تكون النتيجة منخفضة
    assert!(rating.overall_score < 50, "النتيجة يجب أن تكون منخفضة");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الإصدارات الدلالية
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_version_parsing() {
    // إصدارات صالحة
    assert!(SemanticVersion::parse("1.0.0").is_ok());
    assert!(SemanticVersion::parse("0.1.0").is_ok());
    assert!(SemanticVersion::parse("10.20.30").is_ok());
    assert!(SemanticVersion::parse("v1.0.0").is_ok());
    
    // إصدارات غير صالحة
    assert!(SemanticVersion::parse("1.0").is_err());
    assert!(SemanticVersion::parse("abc").is_err());
}

#[test]
fn test_version_bumping() {
    let v = SemanticVersion::new(1, 2, 3);
    
    // زيادة رئيسية
    let major = v.bump_major();
    assert_eq!(major.major, 2);
    assert_eq!(major.minor, 0);
    assert_eq!(major.patch, 0);
    
    // زيادة فرعية
    let minor = v.bump_minor();
    assert_eq!(minor.major, 1);
    assert_eq!(minor.minor, 3);
    assert_eq!(minor.patch, 0);
    
    // زيادة تصحيح
    let patch = v.bump_patch();
    assert_eq!(patch.major, 1);
    assert_eq!(patch.minor, 2);
    assert_eq!(patch.patch, 4);
}

#[test]
fn test_version_compatibility() {
    let v1 = SemanticVersion::new(1, 0, 0);
    let v2 = SemanticVersion::new(1, 5, 0);
    let v3 = SemanticVersion::new(2, 0, 0);
    
    // v2 متوافق مع v1
    assert!(v2.is_compatible_with(&v1));
    
    // v3 غير متوافق مع v1
    assert!(!v3.is_compatible_with(&v1));
}

#[test]
fn test_change_analyzer() {
    let mut analyzer = ChangeAnalyzer::new();
    let current = SemanticVersion::new(1, 0, 0);
    
    // إصلاح -> patch bump
    analyzer.analyze_commit("fix: إصلاح خطأ", "abc123", "developer");
    let v1 = analyzer.suggest_version(&current);
    assert_eq!(v1.patch, 1);
    
    analyzer.clear();
    
    // ميزة -> minor bump
    analyzer.analyze_commit("feat: ميزة جديدة", "def456", "developer");
    let v2 = analyzer.suggest_version(&current);
    assert_eq!(v2.minor, 1);
    assert_eq!(v2.patch, 0);
    
    analyzer.clear();
    
    // breaking -> major bump
    analyzer.analyze_commit("breaking!: تغيير جوهري", "ghi789", "developer");
    let v3 = analyzer.suggest_version(&current);
    assert_eq!(v3.major, 2);
    assert_eq!(v3.minor, 0);
    assert_eq!(v3.patch, 0);
}

#[test]
fn test_changelog_generation() {
    let mut analyzer = ChangeAnalyzer::new();
    
    analyzer.analyze_commit("feat: إضافة دالة جديدة", "a1", "dev");
    analyzer.analyze_commit("fix: إصلاح خطأ في التحويل", "b2", "dev");
    analyzer.analyze_commit("docs: تحديث التوثيق", "c3", "dev");
    
    let version = SemanticVersion::new(1, 1, 0);
    let changelog = analyzer.generate_changelog(&version);
    
    assert!(changelog.contains("1.1.0"));
    assert!(changelog.contains("إضافة دالة"));
    assert!(changelog.contains("إصلاح خطأ"));
    assert!(changelog.contains("تحديث التوثيق"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات نظام السمعة
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_author_registration() {
    let mut system = ReputationSystem::new();
    
    let profile = system.register_author("مطور_عربي", Some("dev@example.com"));
    
    assert_eq!(profile.username, "مطور_عربي");
    assert!(profile.email.is_some());
    assert_eq!(profile.level, AuthorLevel::Beginner);
    assert_eq!(profile.verification_status, VerificationStatus::Unverified);
}

#[test]
fn test_reputation_points() {
    let mut system = ReputationSystem::new();
    system.register_author("dev", None);
    
    // إضافة مساهمة
    let contribution = reputation::Contribution {
        kind: reputation::ContributionKind::PackagePublished,
        package: "json".to_string(),
        description: "نشر حزمة JSON".to_string(),
        points: 50,
        date: chrono::Utc::now().to_rfc3339(),
    };
    
    system.add_contribution("dev", contribution).unwrap();
    
    let profile = system.get_author("dev").unwrap();
    assert_eq!(profile.reputation_points, 50);
    assert_eq!(profile.stats.packages_count, 1);
}

#[test]
fn test_level_progression() {
    let mut system = ReputationSystem::new();
    system.register_author("dev", None);
    
    // إضافة 100 نقطة للوصول لمستوى مساهم
    for i in 0..2 {
        let contribution = reputation::Contribution {
            kind: reputation::ContributionKind::PackagePublished,
            package: format!("pkg_{}", i),
            description: String::new(),
            points: 50,
            date: chrono::Utc::now().to_rfc3339(),
        };
        system.add_contribution("dev", contribution).unwrap();
    }
    
    let profile = system.get_author("dev").unwrap();
    assert_eq!(profile.level, AuthorLevel::Contributor);
}

#[test]
fn test_author_badges() {
    let mut system = ReputationSystem::new();
    system.register_author("dev", None);
    
    // نشر أول حزمة
    let contribution = reputation::Contribution {
        kind: reputation::ContributionKind::PackagePublished,
        package: "first_pkg".to_string(),
        description: String::new(),
        points: 50,
        date: chrono::Utc::now().to_rfc3339(),
    };
    system.add_contribution("dev", contribution).unwrap();
    
    let profile = system.get_author("dev").unwrap();
    
    // يجب الحصول على شارة "البداية"
    assert!(profile.badges.iter().any(|b| b.id == "first_package"));
}

#[test]
fn test_leaderboard() {
    let mut system = ReputationSystem::new();
    
    system.register_author("dev1", None);
    system.register_author("dev2", None);
    system.register_author("dev3", None);
    
    // إضافة نقاط لـ dev1
    let c = reputation::Contribution {
        kind: reputation::ContributionKind::PackagePublished,
        package: "pkg".to_string(),
        description: String::new(),
        points: 100,
        date: chrono::Utc::now().to_rfc3339(),
    };
    system.add_contribution("dev1", c).unwrap();
    
    let leaderboard = system.get_leaderboard(10);
    
    assert_eq!(leaderboard[0].username, "dev1");
    assert_eq!(leaderboard[0].reputation_points, 100);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التوثيق التلقائي
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_documentation_generator() {
    let gen = DocumentationGenerator::new();
    
    let source = r#"
// دالة لتحية المستخدم
// تأخذ اسم المستخدم وتعيد رسالة ترحيب
دالة حيّ(الاسم: نص) -> نص {
    أرجع "مرحبا " + الاسم
}

// دالة لجمع رقمين
دالة اجمع(أ: رقم، ب: رقم) -> رقم {
    أرجع أ + ب
}
"#;
    
    let doc = gen.generate(source, "حزمة_اختبار");
    
    assert!(!doc.functions.is_empty());
    assert!(doc.coverage > 0.0);
}

#[test]
fn test_markdown_generation() {
    let gen = DocumentationGenerator::new();
    
    let doc = GeneratedDocumentation {
        package_name: "اختبار".to_string(),
        description: "حزمة اختبارية".to_string(),
        functions: vec![
            versioning::FunctionDoc {
                name: "حيّ".to_string(),
                description: "تحية المستخدم".to_string(),
                parameters: vec![
                    versioning::ParameterDoc {
                        name: "الاسم".to_string(),
                        type_annotation: "نص".to_string(),
                        description: "اسم المستخدم".to_string(),
                        is_optional: false,
                        default_value: None,
                    }
                ],
                return_type: "نص".to_string(),
                example: Some("حيّ(\"أحمد\")".to_string()),
                lines: (1, 3),
            }
        ],
        types: vec![],
        examples: vec![],
        coverage: 75.0,
    };
    
    let md = gen.generate_markdown(&doc);
    
    assert!(md.contains("# اختبار"));
    assert!(md.contains("حيّ"));
    assert!(md.contains("نص"));
    assert!(md.contains("75"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات CI/CD
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ci_run() {
    let mut ci = CISystem::default();
    
    let result = ci.run("main", "abc123def456");
    
    assert!(result.success);
    assert!(!result.steps.is_empty());
    assert!(result.total_duration_ms > 0);
}

#[test]
fn test_ci_steps() {
    let mut ci = CISystem::default();
    
    let result = ci.run("develop", "feature123");
    
    // التحقق من وجود جميع الخطوات
    let step_names: Vec<&str> = result.steps.iter().map(|s| s.name.as_str()).collect();
    
    assert!(step_names.iter().any(|&n| n.contains("تنسيق")));
    assert!(step_names.iter().any(|&n| n.contains("Lint")));
    assert!(step_names.iter().any(|&n| n.contains("أمان")));
    assert!(step_names.iter().any(|&n| n.contains("اختبار")));
    assert!(step_names.iter().any(|&n| n.contains("بناء")));
}

#[test]
fn test_ci_stats() {
    let mut ci = CISystem::default();
    
    // تشغيلات متعددة
    ci.run("main", "a");
    ci.run("main", "b");
    ci.run("develop", "c");
    
    let stats = ci.stats();
    
    assert_eq!(stats.total_runs, 3);
    assert_eq!(stats.successful_runs, 3);
    assert_eq!(stats.success_rate, 100.0);
}

#[test]
fn test_github_actions_generation() {
    let ci = CISystem::default();
    
    let yaml = ci.generate_github_actions();
    
    assert!(yaml.contains("name: CI"));
    assert!(yaml.contains("cargo fmt"));
    assert!(yaml.contains("cargo clippy"));
    assert!(yaml.contains("cargo test"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات متكاملة
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_full_package_workflow() {
    // 1. إنشاء نظام السمعة وتسجيل مؤلف
    let mut reputation_system = ReputationSystem::new();
    let author = reputation_system.register_author("المطور", Some("dev@example.com"));
    
    // 2. تقييم الحزمة
    let evaluator = PackageEvaluator::new();
    let input = EvaluationInput {
        name: "حزمة_متقدمة".to_string(),
        version: "1.0.0".to_string(),
        description: "حزمة متقدمة للاختبار".to_string(),
        downloads: 1000,
        stars: 100,
        forks: 10,
        open_issues: 2,
        dependencies_count: 3,
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
        days_since_update: 7,
        versions_count: 5,
        contributors_count: 3,
        is_trusted_author: false,
        has_ci: true,
        is_pure_arabic: true,
    };
    
    let rating = evaluator.evaluate(&input);
    
    // 3. تحليل التغييرات
    let mut analyzer = ChangeAnalyzer::new();
    analyzer.analyze_commit("feat: إضافة ميزة جديدة", "a1", "المطور");
    
    let version = SemanticVersion::new(1, 0, 0);
    let next_version = analyzer.suggest_version(&version);
    
    // 4. تشغيل CI
    let mut ci = CISystem::default();
    let ci_result = ci.run("main", "new_feature");
    
    // التحققات
    assert!(rating.overall_score >= 60);
    assert_eq!(next_version.minor, 1); // minor bump للـ feature
    assert!(ci_result.success);
    
    // 5. إضافة مساهمة للمؤلف
    let contribution = reputation::Contribution {
        kind: reputation::ContributionKind::PackagePublished,
        package: "حزمة_متقدمة".to_string(),
        description: "نشر حزمة جديدة".to_string(),
        points: 50,
        date: chrono::Utc::now().to_rfc3339(),
    };
    reputation_system.add_contribution("المطور", contribution).unwrap();
    
    let updated_author = reputation_system.get_author("المطور").unwrap();
    assert_eq!(updated_author.reputation_points, 50);
    assert!(updated_author.badges.iter().any(|b| b.id == "first_package"));
}

#[test]
fn test_package_quality_pipeline() {
    // محاكاة pipeline كامل لفحص جودة الحزمة
    
    let source = r#"
// حزمة رياضيات
// توفر دوال رياضية أساسية

/// دالة لحساب المضروب
دالة مضروب(ن: رقم) -> رقم {
    إذا ن <= 1 {
        أرجع 1
    }
    أرجع ن * مضروب(ن - 1)
}

/// دالة للتحقق من أولية الرقم
دالة هل_أولي(ن: رقم) -> منطقي {
    إذا ن < 2 {
        أرجع خطأ
    }
    لكل i من 2 إلى ن - 1 {
        إذا ن % i == 0 {
            أرجع خطأ
        }
    }
    أرجع صحيح
}
"#;
    
    // 1. توليد التوثيق
    let doc_gen = DocumentationGenerator::new();
    let doc = doc_gen.generate(source, "رياضيات");
    assert!(!doc.functions.is_empty());
    
    // 2. تحليل الإصدار
    let mut analyzer = ChangeAnalyzer::new();
    analyzer.analyze_commit("feat: إضافة دوال رياضية", "v1", "dev");
    let version = analyzer.suggest_version(&SemanticVersion::new(0, 1, 0));
    assert_eq!(version.minor, 1);
    
    // 3. تشغيل CI
    let ci = CISystem::default();
    let yaml = ci.generate_github_actions();
    assert!(yaml.contains("cargo test"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأداء
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_evaluation_performance() {
    use std::time::Instant;
    
    let evaluator = PackageEvaluator::new();
    let input = EvaluationInput {
        name: "test".to_string(),
        version: "1.0.0".to_string(),
        description: String::new(),
        downloads: 1000,
        stars: 100,
        forks: 10,
        open_issues: 5,
        dependencies_count: 5,
        size: 50000,
        has_documentation: true,
        has_tests: true,
        has_readme: true,
        has_changelog: false,
        has_license: true,
        documentation_coverage: 70.0,
        test_coverage: 60.0,
        vulnerabilities_count: 0,
        critical_vulnerabilities: 0,
        days_since_update: 30,
        versions_count: 5,
        contributors_count: 3,
        is_trusted_author: false,
        has_ci: true,
        is_pure_arabic: false,
    };
    
    let start = Instant::now();
    
    // تقييم 100 حزمة
    for _ in 0..100 {
        let _ = evaluator.evaluate(&input);
    }
    
    let duration = start.elapsed();
    
    // يجب أن يكون أقل من ثانية واحدة
    assert!(duration.as_millis() < 1000, "تقييم 100 حزمة يجب أن يكون أقل من ثانية");
}

#[test]
fn test_version_operations_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // 1000 عملية إصدار
    for i in 0..1000 {
        let v = SemanticVersion::new(i / 100, (i % 100) / 10, i % 10);
        let _ = v.bump_major();
        let _ = v.bump_minor();
        let _ = v.bump_patch();
    }
    
    let duration = start.elapsed();
    
    // يجب أن يكون سريعاً جداً
    assert!(duration.as_millis() < 100, "عمليات الإصدار يجب أن تكون سريعة");
}
