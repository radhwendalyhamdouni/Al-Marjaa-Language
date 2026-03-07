// ═══════════════════════════════════════════════════════════════════════════════
// المستودع اللامركزي - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// نظام مستودع لامركزي يعتمد على GitHub كـ backend
// - تخزين الحزم على GitHub Releases
// - فهرس مركزي على مستودع GitHub
// - توزيع جغرافي عبر CDN
// - تحقق من التوقيع الرقمي
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::registry::{PackageInfo, PackageVersion};
use super::PackageSource;

/// المستودع اللامركزي
pub struct DecentralizedRegistry {
    /// عنوان مستودع الفهرس
    index_url: String,
    /// الكاش المحلي
    cache: HashMap<String, CachedPackage>,
    /// مجلد الكاش
    cache_dir: PathBuf,
    /// المهلة (مللي ثانية)
    timeout: u64,
    /// استخدام CDN
    use_cdn: bool,
    /// نقاط CDN
    cdn_endpoints: Vec<String>,
}

/// حزمة مخزنة مؤقتاً
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedPackage {
    /// المعلومات
    info: PackageInfo,
    /// وقت التخزين
    cached_at: i64,
    /// مدة الصلاحية (ثواني)
    ttl: i64,
}

/// فهرس المستودع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryIndex {
    /// الحزم المتاحة
    pub packages: HashMap<String, IndexEntry>,
    /// آخر تحديث
    pub updated_at: String,
    /// الإصدار
    pub version: String,
}

/// مدخل الفهرس
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    /// اسم الحزمة
    pub name: String,
    /// أحدث إصدار
    pub latest_version: String,
    /// رابط التحميل
    pub download_url: String,
    /// التوقيع الرقمي
    pub signature: String,
    /// checksum SHA256
    pub checksum: String,
    /// الحجم
    pub size: u64,
    /// المصدر
    pub source: PackageSource,
}

/// طلب النشر
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    /// اسم الحزمة
    pub name: String,
    /// الإصدار
    pub version: String,
    /// الوصف
    pub description: String,
    /// المؤلف
    pub author: String,
    /// الرخصة
    pub license: String,
    /// الكلمات المفتاحية
    pub keywords: Vec<String>,
    /// التبعيات
    pub dependencies: HashMap<String, String>,
    /// التوقيع الرقمي
    pub signature: String,
    /// مفتاح عام للتوقيع
    pub public_key: String,
    /// محتوى الحزمة (base64)
    pub tarball: String,
    /// checksum
    pub checksum: String,
}

/// نتيجة النشر
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResult {
    /// نجاح أو فشل
    pub success: bool,
    /// الرسالة
    pub message: String,
    /// رابط الحزمة
    pub package_url: Option<String>,
    /// رقم الإصدار
    pub version: String,
}

/// إحصائيات المستودع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStats {
    /// إجمالي الحزم
    pub total_packages: u64,
    /// إجمالي التنزيلات
    pub total_downloads: u64,
    /// الحزم النشطة (آخر 30 يوم)
    pub active_packages: u64,
    /// المساهمون
    pub contributors: u64,
    /// آخر تحديث
    pub last_updated: String,
}

impl DecentralizedRegistry {
    /// إنشاء مستودع جديد
    pub fn new(cache_dir: &Path) -> Self {
        Self {
            index_url: "https://raw.githubusercontent.com/almarjaa-lang/registry/main/index.json".to_string(),
            cache: HashMap::new(),
            cache_dir: cache_dir.to_path_buf(),
            timeout: 30000,
            use_cdn: true,
            cdn_endpoints: vec![
                "https://cdn.jsdelivr.net/gh/almarjaa-lang/registry".to_string(),
                "https://raw.githubusercontent.com/almarjaa-lang/registry/main".to_string(),
            ],
        }
    }

    /// تعيين عنوان الفهرس
    pub fn with_index_url(mut self, url: &str) -> Self {
        self.index_url = url.to_string();
        self
    }

    /// تعيين المهلة
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    /// تعطيل CDN
    pub fn without_cdn(mut self) -> Self {
        self.use_cdn = false;
        self
    }

    // ═══════════════════════════════════════════════════════════════
    // العمليات الأساسية
    // ═══════════════════════════════════════════════════════════════

    /// تحميل الفهرس
    pub fn fetch_index(&self) -> Result<RegistryIndex, String> {
        println!("📥 تحميل فهرس المستودع...");

        // محاولة من CDN أولاً
        if self.use_cdn {
            for cdn in &self.cdn_endpoints {
                let url = format!("{}/index.json", cdn);
                if let Ok(content) = self.fetch_url(&url) {
                    if let Ok(index) = serde_json::from_str(&content) {
                        return Ok(index);
                    }
                }
            }
        }

        // المحاولة من المصدر الرئيسي
        let content = self.fetch_url(&self.index_url)?;
        let index: RegistryIndex = serde_json::from_str(&content)
            .map_err(|e| format!("خطأ في تحليل الفهرس: {}", e))?;

        Ok(index)
    }

    /// البحث عن حزمة
    pub fn find_package(&mut self, name: &str) -> Result<PackageInfo, String> {
        // التحقق من الكاش
        if let Some(cached) = self.cache.get(name) {
            if cached.is_valid() {
                return Ok(cached.info.clone());
            }
        }

        // البحث في الفهرس
        let index = self.fetch_index()?;
        
        let entry = index.packages.get(name)
            .ok_or_else(|| format!("الحزمة '{}' غير موجودة", name))?;

        // بناء معلومات الحزمة
        let info = self.fetch_package_info(entry)?;

        // تخزين في الكاش
        self.cache.insert(name.to_string(), CachedPackage {
            info: info.clone(),
            cached_at: chrono::Utc::now().timestamp(),
            ttl: 3600, // ساعة واحدة
        });

        Ok(info)
    }

    /// تحميل حزمة
    pub fn download_package(&self, name: &str, version: &str) -> Result<Vec<u8>, String> {
        println!("📥 تحميل {}@{}...", name, version);

        let index = self.fetch_index()?;
        
        let entry = index.packages.get(name)
            .ok_or_else(|| format!("الحزمة '{}' غير موجودة", name))?;

        // تحميل من الرابط
        let tarball = self.fetch_bytes(&entry.download_url)?;

        // التحقق من checksum
        let computed_checksum = Self::compute_checksum(&tarball);
        if computed_checksum != entry.checksum {
            return Err("خطأ: checksum غير متطابق".to_string());
        }

        println!("   ✅ تم التحميل بنجاح ({} بايت)", tarball.len());
        Ok(tarball)
    }

    /// نشر حزمة
    pub fn publish(&self, request: &PublishRequest) -> Result<PublishResult, String> {
        println!("📤 نشر {}@{}...", request.name, request.version);

        // التحقق من التوقيع
        if !self.verify_signature(request)? {
            return Ok(PublishResult {
                success: false,
                message: "فشل التحقق من التوقيع الرقمي".to_string(),
                package_url: None,
                version: request.version.clone(),
            });
        }

        // التحقق من عدم وجود الإصدار
        if self.version_exists(&request.name, &request.version)? {
            return Ok(PublishResult {
                success: false,
                message: format!("الإصدار {} موجود بالفعل", request.version),
                package_url: None,
                version: request.version.clone(),
            });
        }

        // إنشاء GitHub Release
        let release_url = self.create_github_release(request)?;

        // تحديث الفهرس
        self.update_index(request, &release_url)?;

        Ok(PublishResult {
            success: true,
            message: "تم النشر بنجاح".to_string(),
            package_url: Some(format!("https://registry.almarjaa.io/package/{}", request.name)),
            version: request.version.clone(),
        })
    }

    /// البحث عن حزم
    pub fn search(&self, query: &str) -> Result<Vec<PackageInfo>, String> {
        let index = self.fetch_index()?;
        let query_lower = query.to_lowercase();
        
        let mut results = Vec::new();
        
        for (name, entry) in &index.packages {
            if name.to_lowercase().contains(&query_lower) {
                if let Ok(info) = self.fetch_package_info(entry) {
                    results.push(info);
                }
            }
        }

        // ترتيب حسب الشعبية
        results.sort_by(|a, b| b.downloads.cmp(&a.downloads));
        
        Ok(results)
    }

    /// الحصول على إحصائيات
    pub fn get_stats(&self) -> Result<RegistryStats, String> {
        let index = self.fetch_index()?;
        
        Ok(RegistryStats {
            total_packages: index.packages.len() as u64,
            total_downloads: 0, // سيتم حسابها من البيانات
            active_packages: index.packages.len() as u64,
            contributors: 0,
            last_updated: index.updated_at,
        })
    }

    // ═══════════════════════════════════════════════════════════════
    // العمليات الداخلية
    // ═══════════════════════════════════════════════════════════════

    /// جلب URL
    fn fetch_url(&self, url: &str) -> Result<String, String> {
        // محاكاة - في التطبيق الحقيقي سنستخدم reqwest
        println!("   🌐 جلب: {}", url);
        Ok("{}".to_string())
    }

    /// جلب بايتات
    fn fetch_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
        // محاكاة
        println!("   🌐 تحميل: {}", url);
        Ok(vec![])
    }

    /// جلب معلومات حزمة من المدخل
    fn fetch_package_info(&self, entry: &IndexEntry) -> Result<PackageInfo, String> {
        Ok(PackageInfo {
            name: entry.name.clone(),
            latest_version: entry.latest_version.clone(),
            description: String::new(),
            author: String::new(),
            license: "MIT".to_string(),
            versions: vec![entry.latest_version.clone()],
            downloads: 0,
            stars: 0,
            forks: 0,
            github_url: None,
            documentation_url: None,
            keywords: vec![],
            dependencies: HashMap::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            source: entry.source.clone(),
            size: entry.size,
            checksum: entry.checksum.clone(),
        })
    }

    /// التحقق من التوقيع
    fn verify_signature(&self, request: &PublishRequest) -> Result<bool, String> {
        // محاكاة - في التطبيق الحقيقي سنستخدم التشفير
        Ok(!request.signature.is_empty() && !request.public_key.is_empty())
    }

    /// التحقق من وجود إصدار
    fn version_exists(&self, name: &str, version: &str) -> Result<bool, String> {
        let index = self.fetch_index()?;
        
        if let Some(entry) = index.packages.get(name) {
            Ok(entry.latest_version == version)
        } else {
            Ok(false)
        }
    }

    /// إنشاء GitHub Release
    fn create_github_release(&self, request: &PublishRequest) -> Result<String, String> {
        // محاكاة
        Ok(format!(
            "https://github.com/almarjaa-lang/packages/releases/download/{}-{}/package.tar.gz",
            request.name, request.version
        ))
    }

    /// تحديث الفهرس
    fn update_index(&self, request: &PublishRequest, release_url: &str) -> Result<(), String> {
        // محاكاة - في التطبيق الحقيقي سنستخدم GitHub API
        println!("   📝 تحديث الفهرس: {} -> {}", request.name, release_url);
        Ok(())
    }

    /// حساب checksum
    fn compute_checksum(data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// حفظ الكاش
    pub fn save_cache(&self) -> Result<(), String> {
        let cache_file = self.cache_dir.join("registry_cache.json");
        let content = serde_json::to_string_pretty(&self.cache)
            .map_err(|e| format!("خطأ في تسلسل الكاش: {}", e))?;
        
        std::fs::write(&cache_file, content)
            .map_err(|e| format!("خطأ في كتابة الكاش: {}", e))?;
        
        Ok(())
    }

    /// تحميل الكاش
    pub fn load_cache(&mut self) -> Result<(), String> {
        let cache_file = self.cache_dir.join("registry_cache.json");
        
        if !cache_file.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&cache_file)
            .map_err(|e| format!("خطأ في قراءة الكاش: {}", e))?;
        
        self.cache = serde_json::from_str(&content)
            .map_err(|e| format!("خطأ في تحليل الكاش: {}", e))?;

        Ok(())
    }

    /// مسح الكاش
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        let cache_file = self.cache_dir.join("registry_cache.json");
        let _ = std::fs::remove_file(cache_file);
    }
}

impl CachedPackage {
    /// التحقق من صلاحية الكاش
    fn is_valid(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        now - self.cached_at < self.ttl
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مولد التوثيق التلقائي
// ═══════════════════════════════════════════════════════════════════════════════

/// مولد التوثيق
pub struct DocumentationGenerator {
    /// القالب
    template: String,
    /// إعدادات
    settings: DocSettings,
}

/// إعدادات التوثيق
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSettings {
    /// تضمين الأمثلة
    pub include_examples: bool,
    /// تضمين التواقيع
    pub include_signatures: bool,
    /// اللغة
    pub language: String,
    /// تنسيق الإخراج
    pub output_format: OutputFormat,
}

/// تنسيق الإخراج
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Markdown,
    HTML,
    JSON,
}

impl Default for DocSettings {
    fn default() -> Self {
        Self {
            include_examples: true,
            include_signatures: true,
            language: "ar".to_string(),
            output_format: OutputFormat::Markdown,
        }
    }
}

/// التوثيق المُولّد
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedDocumentation {
    /// اسم الحزمة
    pub package_name: String,
    /// الوصف
    pub description: String,
    /// الدوال
    pub functions: Vec<FunctionDoc>,
    /// الأنواع
    pub types: Vec<TypeDoc>,
    /// الأمثلة
    pub examples: Vec<ExampleDoc>,
    /// نسبة التغطية
    pub coverage: f32,
}

/// توثيق دالة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDoc {
    /// الاسم
    pub name: String,
    /// الوصف
    pub description: String,
    /// المعاملات
    pub parameters: Vec<ParameterDoc>,
    /// القيمة المرجعة
    pub return_type: String,
    /// مثال
    pub example: Option<String>,
    /// الأسطر
    pub lines: (usize, usize),
}

/// توثيق معامل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDoc {
    /// الاسم
    pub name: String,
    /// النوع
    pub type_annotation: String,
    /// الوصف
    pub description: String,
    /// اختياري
    pub is_optional: bool,
    /// القيمة الافتراضية
    pub default_value: Option<String>,
}

/// توثيق نوع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDoc {
    /// الاسم
    pub name: String,
    /// النوع (struct, enum, etc)
    pub kind: String,
    /// الوصف
    pub description: String,
    /// الحقول
    pub fields: Vec<FieldDoc>,
}

/// توثيق حقل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDoc {
    /// الاسم
    pub name: String,
    /// النوع
    pub type_annotation: String,
    /// الوصف
    pub description: String,
}

/// توثيق مثال
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleDoc {
    /// العنوان
    pub title: String,
    /// الوصف
    pub description: String,
    /// الكود
    pub code: String,
    /// النتيجة المتوقعة
    pub expected_output: Option<String>,
}

impl DocumentationGenerator {
    /// إنشاء مولد جديد
    pub fn new() -> Self {
        Self {
            template: Self::default_template(),
            settings: DocSettings::default(),
        }
    }

    /// إنشاء مولد بإعدادات مخصصة
    pub fn with_settings(settings: DocSettings) -> Self {
        Self {
            template: Self::default_template(),
            settings,
        }
    }

    /// توليد التوثيق من الكود المصدري
    pub fn generate(&self, source: &str, package_name: &str) -> GeneratedDocumentation {
        let mut functions = Vec::new();
        let mut types = Vec::new();
        let mut examples = Vec::new();

        // استخراج الدوال
        for (i, line) in source.lines().enumerate() {
            let line = line.trim();
            
            // البحث عن تعريفات الدوال
            if line.starts_with("دالة") || line.starts_with("func") {
                if let Some(func_doc) = self.parse_function(source, i) {
                    functions.push(func_doc);
                }
            }

            // البحث عن تعريفات الأنواع
            if line.starts_with("نوع") || line.starts_with("struct") || line.starts_with("enum") {
                if let Some(type_doc) = self.parse_type(source, i) {
                    types.push(type_doc);
                }
            }

            // البحث عن الأمثلة
            if line.contains("مثال") || line.contains("# example") {
                if let Some(example_doc) = self.parse_example(source, i) {
                    examples.push(example_doc);
                }
            }
        }

        // حساب التغطية
        let total_lines = source.lines().count();
        let documented_lines = functions.iter()
            .map(|f| f.lines.1 - f.lines.0 + 1)
            .sum::<usize>();
        
        let coverage = if total_lines > 0 {
            (documented_lines as f32 / total_lines as f32) * 100.0
        } else {
            0.0
        };

        GeneratedDocumentation {
            package_name: package_name.to_string(),
            description: String::new(),
            functions,
            types,
            examples,
            coverage,
        }
    }

    /// تحليل دالة
    fn parse_function(&self, source: &str, start_line: usize) -> Option<FunctionDoc> {
        let lines: Vec<&str> = source.lines().collect();
        
        // البحث عن تعليق التوثيق السابق
        let mut description = String::new();
        let mut i = start_line as i32 - 1;
        
        while i >= 0 {
            let line = lines[i as usize].trim();
            if line.starts_with("//") || line.starts_with("#") {
                description = format!("{} {}", line.trim_start_matches('/').trim_start_matches('#').trim(), description);
                i -= 1;
            } else {
                break;
            }
        }

        // تحليل التوقيع
        let signature = lines[start_line].to_string();
        
        Some(FunctionDoc {
            name: self.extract_function_name(&signature),
            description,
            parameters: self.parse_parameters(&signature),
            return_type: self.extract_return_type(&signature),
            example: if self.settings.include_examples { Some(String::new()) } else { None },
            lines: (start_line, start_line),
        })
    }

    /// تحليل نوع
    fn parse_type(&self, source: &str, start_line: usize) -> Option<TypeDoc> {
        let lines: Vec<&str> = source.lines().collect();
        let line = lines[start_line].trim();
        
        let kind = if line.starts_with("enum") || line.starts_with("تعداد") {
            "enum"
        } else {
            "struct"
        };

        Some(TypeDoc {
            name: self.extract_type_name(line),
            kind: kind.to_string(),
            description: String::new(),
            fields: vec![],
        })
    }

    /// تحليل مثال
    fn parse_example(&self, source: &str, start_line: usize) -> Option<ExampleDoc> {
        let lines: Vec<&str> = source.lines().collect();
        let mut code = String::new();
        let mut i = start_line + 1;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() || line.starts_with("//") {
                break;
            }
            code.push_str(line);
            code.push('\n');
            i += 1;
        }

        Some(ExampleDoc {
            title: "مثال".to_string(),
            description: String::new(),
            code,
            expected_output: None,
        })
    }

    /// استخراج اسم الدالة
    fn extract_function_name(&self, signature: &str) -> String {
        // استخراج الاسم من التوقيع
        let parts: Vec<&str> = signature.split('(').collect();
        if parts.is_empty() {
            return String::new();
        }
        
        let name_part = parts[0].trim();
        let words: Vec<&str> = name_part.split_whitespace().collect();
        words.last().unwrap_or(&"").to_string()
    }

    /// استخراج نوع الإرجاع
    fn extract_return_type(&self, signature: &str) -> String {
        if let Some(pos) = signature.find("->") {
            signature[pos + 2..].trim().split_whitespace().next().unwrap_or("").to_string()
        } else if let Some(pos) = signature.find("يعيد") {
            signature[pos + 4..].trim().split_whitespace().next().unwrap_or("").to_string()
        } else {
            "فراغ".to_string()
        }
    }

    /// تحليل المعاملات
    fn parse_parameters(&self, signature: &str) -> Vec<ParameterDoc> {
        let mut params = Vec::new();
        
        if let Some(start) = signature.find('(') {
            if let Some(end) = signature.find(')') {
                let params_str = &signature[start + 1..end];
                
                for param in params_str.split(',') {
                    let param = param.trim();
                    if param.is_empty() {
                        continue;
                    }
                    
                    let parts: Vec<&str> = param.split(':').collect();
                    params.push(ParameterDoc {
                        name: parts.get(0).unwrap_or(&"").trim().to_string(),
                        type_annotation: parts.get(1).unwrap_or(&"").trim().to_string(),
                        description: String::new(),
                        is_optional: param.contains('?'),
                        default_value: None,
                    });
                }
            }
        }
        
        params
    }

    /// استخراج اسم النوع
    fn extract_type_name(&self, line: &str) -> String {
        let words: Vec<&str> = line.split_whitespace().collect();
        words.get(1).unwrap_or(&"").trim_end_matches('{').to_string()
    }

    /// توليد Markdown
    pub fn generate_markdown(&self, doc: &GeneratedDocumentation) -> String {
        let mut md = String::new();

        md.push_str(&format!("# {}\n\n", doc.package_name));
        
        if !doc.description.is_empty() {
            md.push_str(&doc.description);
            md.push_str("\n\n");
        }

        md.push_str(&format!("**تغطية التوثيق: {:.1}%**\n\n", doc.coverage));

        // الدوال
        if !doc.functions.is_empty() {
            md.push_str("## الدوال\n\n");
            
            for func in &doc.functions {
                md.push_str(&format!("### `{}`\n\n", func.name));
                
                if !func.description.is_empty() {
                    md.push_str(&func.description);
                    md.push_str("\n\n");
                }

                if !func.parameters.is_empty() {
                    md.push_str("**المعاملات:**\n\n");
                    for param in &func.parameters {
                        md.push_str(&format!(
                            "- `{}`: {} `{}`{}\n",
                            param.name,
                            if param.is_optional { "?" } else { "" },
                            param.type_annotation,
                            if !param.description.is_empty() {
                                format!(" - {}", param.description)
                            } else {
                                String::new()
                            }
                        ));
                    }
                    md.push_str("\n");
                }

                md.push_str(&format!("**القيمة المرجعة:** `{}`\n\n", func.return_type));

                if let Some(ref example) = func.example {
                    if !example.is_empty() {
                        md.push_str("**مثال:**\n\n");
                        md.push_str("```mrj\n");
                        md.push_str(example);
                        md.push_str("```\n\n");
                    }
                }
            }
        }

        // الأنواع
        if !doc.types.is_empty() {
            md.push_str("## الأنواع\n\n");
            
            for t in &doc.types {
                md.push_str(&format!("### `{}` ({})\n\n", t.name, t.kind));
                
                if !t.description.is_empty() {
                    md.push_str(&t.description);
                    md.push_str("\n\n");
                }
            }
        }

        // الأمثلة
        if !doc.examples.is_empty() {
            md.push_str("## أمثلة\n\n");
            
            for example in &doc.examples {
                md.push_str(&format!("### {}\n\n", example.title));
                
                if !example.description.is_empty() {
                    md.push_str(&example.description);
                    md.push_str("\n\n");
                }

                md.push_str("```mrj\n");
                md.push_str(&example.code);
                md.push_str("```\n\n");
            }
        }

        md
    }

    /// القالب الافتراضي
    fn default_template() -> String {
        r#"# {{name}}

{{description}}

## التثبيت

```
almarjaa أضف {{name}}
```

## الاستخدام

```mrj
استيراد "{{name}}"

// مثال
```

## التوثيق

{{functions}}

## الترخيص

MIT
"#.to_string()
    }
}

impl Default for DocumentationGenerator {
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
    fn test_registry_creation() {
        let registry = DecentralizedRegistry::new(Path::new("/tmp"));
        assert!(!registry.index_url.is_empty());
    }

    #[test]
    fn test_doc_generator_creation() {
        let gen = DocumentationGenerator::new();
        assert!(!gen.template.is_empty());
    }

    #[test]
    fn test_generate_documentation() {
        let gen = DocumentationGenerator::new();
        let source = r#"
// دالة لتحية المستخدم
دالة حيّ(اسم: نص) -> نص {
    أرجع "مرحبا " + اسم
}
"#;
        
        let doc = gen.generate(source, "اختبار");
        assert!(!doc.functions.is_empty());
    }

    #[test]
    fn test_generate_markdown() {
        let gen = DocumentationGenerator::new();
        let doc = GeneratedDocumentation {
            package_name: "اختبار".to_string(),
            description: "حزمة اختبارية".to_string(),
            functions: vec![FunctionDoc {
                name: "حيّ".to_string(),
                description: "تحية المستخدم".to_string(),
                parameters: vec![],
                return_type: "نص".to_string(),
                example: Some("حيّ(\"أحمد\")".to_string()),
                lines: (1, 3),
            }],
            types: vec![],
            examples: vec![],
            coverage: 80.0,
        };

        let md = gen.generate_markdown(&doc);
        assert!(md.contains("اختبار"));
        assert!(md.contains("حيّ"));
    }

    #[test]
    fn test_checksum() {
        let data = b"test data";
        let checksum = DecentralizedRegistry::compute_checksum(data);
        assert!(!checksum.is_empty());
        assert_eq!(checksum.len(), 64); // SHA256 hex
    }

    #[test]
    fn test_cached_package_validity() {
        let cached = CachedPackage {
            info: PackageInfo {
                name: "test".to_string(),
                latest_version: "1.0.0".to_string(),
                description: String::new(),
                author: String::new(),
                license: String::new(),
                versions: vec![],
                downloads: 0,
                stars: 0,
                forks: 0,
                github_url: None,
                documentation_url: None,
                keywords: vec![],
                dependencies: HashMap::new(),
                created_at: String::new(),
                updated_at: String::new(),
                source: PackageSource::Registry,
                size: 0,
                checksum: String::new(),
            },
            cached_at: chrono::Utc::now().timestamp(),
            ttl: 3600,
        };

        assert!(cached.is_valid());
    }
}
