// ═══════════════════════════════════════════════════════════════════════════════
// محرك الأنابيب الذكي - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// يقوم بتحويل النص العربي الطبيعي (Vibe Coding) إلى كود تنفيذي
// باستخدام نموذج Nanbeige4.1-3B-Q3_K_M المحلي
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// نتيجة تحليل النية (Intent)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// نوع الإجراء
    pub action: String,
    /// القيمة المرتبطة
    pub value: Option<String>,
    /// المعاملات الإضافية
    pub params: Option<HashMap<String, String>>,
    /// الكود المُنتج
    pub code: Option<String>,
}

/// نتيجة تنفيذ Pipeline
#[derive(Debug, Clone)]
pub struct PipelineResult {
    /// النص الأصلي
    pub input: String,
    /// النية المستخرجة
    pub intent: Intent,
    /// الكود المُنتج
    pub code: String,
    /// نتيجة التنفيذ
    pub output: String,
    /// نجح أم فشل
    pub success: bool,
}

/// محرك الأنابيب الذكي
pub struct PipelineEngine {
    /// مسار النموذج (محجوز للاستخدام المستقبلي)
    _model_path: String,
    /// هل النموذج محمّل (محجوز للاستخدام المستقبلي)
    _model_loaded: bool,
    /// قوالب الكود
    templates: HashMap<String, String>,
}

impl PipelineEngine {
    /// إنشاء محرك جديد
    pub fn new() -> Self {
        let mut engine = PipelineEngine {
            _model_path: "models/nanbeige-4.1-3b/".to_string(),
            _model_loaded: false,
            templates: HashMap::new(),
        };
        engine.load_templates();
        engine
    }

    /// إنشاء محرك مع مسار مخصص
    pub fn with_model_path(path: &str) -> Self {
        let mut engine = PipelineEngine {
            _model_path: path.to_string(),
            _model_loaded: false,
            templates: HashMap::new(),
        };
        engine.load_templates();
        engine
    }

    /// تحميل قوالب الكود
    fn load_templates(&mut self) {
        // قالب المتغير
        self.templates.insert(
            "variable".to_string(),
            "متغير {{name}} = {{value}}؛".to_string()
        );
        
        // قالب الطباعة
        self.templates.insert(
            "print".to_string(),
            "اطبع(\"{{value}}\")؛".to_string()
        );
        
        // قالب الشرط
        self.templates.insert(
            "condition".to_string(),
            "إذا {{condition}} {\n    {{body}}\n}؛".to_string()
        );
        
        // قالب الدالة
        self.templates.insert(
            "function".to_string(),
            "دالة {{name}}({{params}}) {\n    {{body}}\n}؛".to_string()
        );
        
        // قالب الحلقة
        self.templates.insert(
            "loop".to_string(),
            "كرر {{count}} مرات {\n    {{body}}\n}؛".to_string()
        );
        
        // قالب while
        self.templates.insert(
            "while".to_string(),
            "طالما {{condition}} {\n    {{body}}\n}؛".to_string()
        );
        
        // قالب الإرجاع
        self.templates.insert(
            "return".to_string(),
            "أعطِ {{value}}؛".to_string()
        );
    }

    /// تحليل النية من النص العربي
    /// هذا يستخدم محاكاة للنموذج في الوضع Offline
    /// في الإنتاج، سيتم استبداله بـ inference حقيقي للنموذج
    pub fn parse_intent(&self, text: &str) -> Intent {
        let text = text.trim();
        let text_lower = text.to_lowercase();
        
        // ═══════════════════════════════════════════════════════════════
        // تحليل أنماط Vibe Coding العربية
        // ملاحظة: الترتيب مهم - الأنماط الأكثر تحديداً أولاً
        // ═══════════════════════════════════════════════════════════════
        
        // نمط: إذا كان [شرط] [إجراء] - يجب أن يكون قبل الطباعة
        if text_lower.contains("إذا") || text_lower.starts_with("لو") || text_lower.starts_with("في حالة") {
            return self.parse_condition_intent(text);
        }
        
        // نمط: أنشئ/عرّف متغير [اسم] يساوي [قيمة]
        if text_lower.contains("متغير") || text_lower.contains("أنشئ متغير") || text_lower.contains("عرف متغير") {
            return self.parse_variable_intent(text);
        }
        
        // نمط: اطبع [نص/رسالة]
        if text_lower.contains("اطبع") || text_lower.contains("اعرض") || text_lower.contains("اكتب") {
            return self.parse_print_intent(text);
        }
        
        // نمط: أنشئ/عرّف دالة [اسم] [وصف]
        if text_lower.contains("دالة") || text_lower.contains("function") || text_lower.contains("وظيفة") {
            return self.parse_function_intent(text);
        }
        
        // نمط: كرر [عدد] مرات [إجراء]
        if text_lower.contains("كرر") || text_lower.contains("تكرار") || text_lower.contains("حلقة") {
            return self.parse_loop_intent(text);
        }
        
        // نمط: اجمع/أضف [قيمة1] و [قيمة2]
        if text_lower.contains("اجمع") || text_lower.contains("أضف") || text_lower.contains("جمع") {
            return self.parse_arithmetic_intent(text, "add");
        }
        
        // نمط: اطرح [قيمة1] من [قيمة2]
        if text_lower.contains("اطرح") || text_lower.contains("طرح") || text_lower.contains("انقص") {
            return self.parse_arithmetic_intent(text, "subtract");
        }
        
        // نمط: اضرب [قيمة1] في [قيمة2]
        if text_lower.contains("اضرب") || text_lower.contains("ضرب") || text_lower.contains("اضرب") {
            return self.parse_arithmetic_intent(text, "multiply");
        }
        
        // نمط افتراضي
        Intent {
            action: "unknown".to_string(),
            value: Some(text.to_string()),
            params: None,
            code: None,
        }
    }

    /// تحليل نية إنشاء متغير
    fn parse_variable_intent(&self, text: &str) -> Intent {
        // استخراج اسم المتغير والقيمة
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut name = "متغير".to_string();
        let mut value = "0".to_string();
        
        for (i, word) in words.iter().enumerate() {
            if *word == "متغير" || *word == "أنشئ" || *word == "عرف" {
                if let Some(n) = words.get(i + 1) {
                    name = n.to_string();
                }
            }
            if *word == "يساوي" || *word == "=" || *word == "بقيمة" {
                if let Some(v) = words.get(i + 1) {
                    value = v.to_string();
                }
            }
        }
        
        // تنظيف الاسم من "أنشئ" و "عرف"
        if name == "أنشئ" || name == "عرف" {
            name = "س".to_string();
        }
        
        let code = format!("متغير {} = {}؛", name, value);
        
        Intent {
            action: "variable".to_string(),
            value: Some(value.clone()),
            params: Some(HashMap::from([
                ("name".to_string(), name),
                ("value".to_string(), value),
            ])),
            code: Some(code),
        }
    }

    /// تحليل نية الطباعة
    fn parse_print_intent(&self, text: &str) -> Intent {
        // استخراج النص المراد طباعته
        let value = text
            .replace("اطبع", "")
            .replace("اعرض", "")
            .replace("اكتب", "")
            .replace("رسالة", "")
            .replace("نص", "")
            .trim()
            .to_string();
        
        // إزالة علامات التنصيص إذا وجدت
        let value = value.trim_matches('"').trim_matches('\'').trim();
        
        let code = format!("اطبع(\"{}\")؛", value);
        
        Intent {
            action: "print".to_string(),
            value: Some(value.to_string()),
            params: None,
            code: Some(code),
        }
    }

    /// تحليل نية الشرط
    fn parse_condition_intent(&self, text: &str) -> Intent {
        let text_lower = text.to_lowercase();
        
        // استخراج الشرط
        let condition = if text_lower.contains("أكبر من") {
            text.replace("أكبر من", ">")
                .replace("إذا كان", "")
                .replace("إذا", "")
                .trim()
                .to_string()
        } else if text_lower.contains("أصغر من") {
            text.replace("أصغر من", "<")
                .replace("إذا كان", "")
                .replace("إذا", "")
                .trim()
                .to_string()
        } else if text_lower.contains("يساوي") {
            text.replace("يساوي", "==")
                .replace("إذا كان", "")
                .replace("إذا", "")
                .trim()
                .to_string()
        } else {
            "صحيح".to_string()
        };
        
        // استخراج الإجراء
        let body = if text_lower.contains("اطبع") {
            let print_part = text.split("اطبع").last().unwrap_or("").trim();
            format!("اطبع(\"{}\")؛", print_part.replace("'", "").replace("\"", "").trim())
        } else {
            "اطبع(\"تم\")؛".to_string()
        };
        
        let code = format!("إذا {} {{\n    {}\n}}؛", condition, body);
        
        Intent {
            action: "condition".to_string(),
            value: Some(condition.clone()),
            params: Some(HashMap::from([
                ("condition".to_string(), condition),
                ("body".to_string(), body),
            ])),
            code: Some(code),
        }
    }

    /// تحليل نية الدالة
    fn parse_function_intent(&self, text: &str) -> Intent {
        // استخراج اسم الدالة
        let name = if text.contains("تضيف") || text.contains("جمع") {
            "اجمع".to_string()
        } else if text.contains("تضرب") || text.contains("ضرب") {
            "اضرب".to_string()
        } else if text.contains("تطرح") || text.contains("طرح") {
            "اطرح".to_string()
        } else {
            "دالة_جديدة".to_string()
        };
        
        // تحديد المعاملات والجسم
        let (params, body) = if name == "اجمع" {
            ("أ، ب".to_string(), "أعطِ أ + ب؛".to_string())
        } else if name == "اضرب" {
            ("أ، ب".to_string(), "أعطِ أ * ب؛".to_string())
        } else if name == "اطرح" {
            ("أ، ب".to_string(), "أعطِ أ - ب؛".to_string())
        } else {
            ("".to_string(), "أعطِ لا_شيء؛".to_string())
        };
        
        let code = format!("دالة {}({}) {{\n    {}\n}}؛", name, params, body);
        
        Intent {
            action: "function".to_string(),
            value: Some(name.clone()),
            params: Some(HashMap::from([
                ("name".to_string(), name),
                ("params".to_string(), params),
                ("body".to_string(), body),
            ])),
            code: Some(code),
        }
    }

    /// تحليل نية الحلقة
    fn parse_loop_intent(&self, text: &str) -> Intent {
        let text_lower = text.to_lowercase();
        
        // استخراج عدد التكرارات
        let count = if text_lower.contains("ثلاث") || text_lower.contains("3") {
            "3".to_string()
        } else if text_lower.contains("خمس") || text_lower.contains("5") {
            "5".to_string()
        } else if text_lower.contains("عشر") || text_lower.contains("10") {
            "10".to_string()
        } else if text_lower.contains("مرتين") || text_lower.contains("2") {
            "2".to_string()
        } else {
            // محاولة استخراج الرقم
            let num: String = text.chars()
                .filter(|c| c.is_numeric())
                .collect();
            if num.is_empty() { "1".to_string() } else { num }
        };
        
        // استخراج الإجراء
        let body = if text_lower.contains("اطبع") || text_lower.contains("مرحبا") {
            let msg = if text_lower.contains("مرحبا") {
                "مرحبا"
            } else {
                text.split("طباعة").last()
                    .unwrap_or("مرحبا")
                    .trim()
            };
            format!("اطبع(\"{}\")؛", msg)
        } else {
            "اطبع(\"تكرار\")؛".to_string()
        };
        
        let code = format!("متغير ع = 0؛\nطالما ع < {} {{\n    {}\n    ع = ع + 1؛\n}}؛", count, body);
        
        Intent {
            action: "loop".to_string(),
            value: Some(count.clone()),
            params: Some(HashMap::from([
                ("count".to_string(), count),
                ("body".to_string(), body),
            ])),
            code: Some(code),
        }
    }

    /// تحليل نية العمليات الحسابية
    fn parse_arithmetic_intent(&self, _text: &str, op: &str) -> Intent {
        let (op_symbol, op_name) = match op {
            "add" => ("+", "جمع"),
            "subtract" => ("-", "طرح"),
            "multiply" => ("*", "ضرب"),
            _ => ("+", "جمع"),
        };
        
        let code = format!("متغير نتيجة = أ {} ب؛", op_symbol);
        
        Intent {
            action: "arithmetic".to_string(),
            value: Some(op_name.to_string()),
            params: Some(HashMap::from([
                ("operator".to_string(), op_symbol.to_string()),
                ("operation".to_string(), op_name.to_string()),
            ])),
            code: Some(code),
        }
    }

    /// توليد الكود من النية
    pub fn generate_code(&self, intent: &Intent) -> String {
        if let Some(code) = &intent.code {
            code.clone()
        } else {
            // توليد من القالب
            let template = self.templates.get(&intent.action).cloned().unwrap_or_default();
            
            let mut result = template;
            if let Some(value) = &intent.value {
                result = result.replace("{{value}}", value);
            }
            if let Some(params) = &intent.params {
                for (key, val) in params {
                    result = result.replace(&format!("{{{{{}}}}}", key), val);
                }
            }
            result
        }
    }

    /// تشغيل Pipeline كامل
    pub fn run_pipeline(&self, text: &str) -> PipelineResult {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║            🚀 مرحباً بك في لغة المرجع - Vibe Coding          ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        
        // الخطوة 1: تحليل النية
        println!("\n📝 الخطوة 1: تحليل النص العربي...");
        println!("   النص الأصلي: \"{}\"", text);
        
        let intent = self.parse_intent(text);
        println!("\n🧠 الخطوة 2: استخراج النية (Intent)...");
        println!("   الإجراء: {}", intent.action);
        if let Some(value) = &intent.value {
            println!("   القيمة: {}", value);
        }
        
        // الخطوة 3: توليد الكود
        println!("\n⚙️ الخطوة 3: توليد كود المرجع...");
        let code = self.generate_code(&intent);
        println!("   الكود المُنتج:");
        for line in code.lines() {
            println!("   │ {}", line);
        }
        
        // الخطوة 4: تنفيذ الكود عبر Bytecode VM
        println!("\n▶️ الخطوة 4: تنفيذ الكود...");
        let output = self.execute_code(&code);
        
        let success = !output.starts_with("خطأ");
        if success {
            println!("   ✅ تم التنفيذ بنجاح!");
        } else {
            println!("   ❌ {}", output);
        }
        
        // الخطوة 5: عرض النتيجة
        println!("\n📊 الخطوة 5: النتيجة النهائية...");
        println!("   الخرج: {}", if output.is_empty() { "تم بنجاح" } else { &output });
        
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║              ✨ اكتمل تنفيذ Pipeline بنجاح ✨                ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");
        
        PipelineResult {
            input: text.to_string(),
            intent,
            code,
            output,
            success,
        }
    }

    /// تنفيذ الكود عبر Bytecode VM
    fn execute_code(&self, code: &str) -> String {
        // محاولة التشغيل عبر Bytecode VM
        match crate::bytecode::run_bytecode(code) {
            Ok(value) => {
                match value {
                    crate::interpreter::value::Value::Number(n) => format!("{}", n),
                    crate::interpreter::value::Value::String(s) => s,
                    crate::interpreter::value::Value::Boolean(b) => format!("{}", b),
                    crate::interpreter::value::Value::Null => "تم".to_string(),
                    _ => format!("{:?}", value),
                }
            }
            Err(e) => format!("خطأ: {}", e),
        }
    }

    /// تشغيل مثال وعرض المدخلات والمخرجات
    pub fn run_example(&self, text: &str) {
        let result = self.run_pipeline(text);
        
        println!("\n📋 ملخص المثال:");
        println!("   ┌─────────────────────────────────────");
        println!("   │ المدخل:  \"{}\"", result.input);
        println!("   │ النية:   {} {:?}", result.intent.action, result.intent.value);
        println!("   │ الكود:   {}", result.code.lines().next().unwrap_or(""));
        println!("   │ النتيجة: {}", result.output);
        println!("   └─────────────────────────────────────");
    }

    /// تشغيل دفعة من الأمثلة
    pub fn run_examples(&self, examples: &[&str]) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║            📚 تشغيل مجموعة أمثلة Vibe Coding                 ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        
        for (i, example) in examples.iter().enumerate() {
            println!("\n━━━ المثال {} ━━━", i + 1);
            self.run_example(example);
        }
        
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║            ✨ تم تشغيل {} مثال بنجاح ✨                     ║", examples.len());
        println!("╚══════════════════════════════════════════════════════════════╝");
    }
}

impl Default for PipelineEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال سهلة الاستخدام
// ═══════════════════════════════════════════════════════════════════════════════

/// تشغيل Pipeline مباشرة
pub fn run_pipeline(text: &str) -> String {
    let engine = PipelineEngine::new();
    let result = engine.run_pipeline(text);
    if result.success {
        result.output
    } else {
        format!("خطأ: {}", result.output)
    }
}

/// تشغيل مثال
pub fn run_example(text: &str) {
    let engine = PipelineEngine::new();
    engine.run_example(text);
}

/// تحليل النية فقط
pub fn parse_intent(text: &str) -> Intent {
    let engine = PipelineEngine::new();
    engine.parse_intent(text)
}

/// توليد الكود من النية
pub fn generate_code(intent: &Intent) -> String {
    let engine = PipelineEngine::new();
    engine.generate_code(intent)
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_variable() {
        let engine = PipelineEngine::new();
        let intent = engine.parse_intent("أنشئ متغير س يساوي 5");
        
        assert_eq!(intent.action, "variable");
        assert!(intent.code.is_some());
    }
    
    #[test]
    fn test_parse_print() {
        let engine = PipelineEngine::new();
        let intent = engine.parse_intent("اطبع رسالة مرحباً");
        
        assert_eq!(intent.action, "print");
        assert!(intent.code.is_some());
    }
    
    #[test]
    fn test_parse_condition() {
        let engine = PipelineEngine::new();
        let intent = engine.parse_intent("إذا كان س أكبر من 10 اطبع 'كبير'");
        
        assert_eq!(intent.action, "condition");
        assert!(intent.code.is_some());
    }
    
    #[test]
    fn test_parse_function() {
        let engine = PipelineEngine::new();
        let intent = engine.parse_intent("أنشئ دالة تضيف رقمين وتعيد النتيجة");
        
        assert_eq!(intent.action, "function");
        assert!(intent.code.is_some());
    }
    
    #[test]
    fn test_parse_loop() {
        let engine = PipelineEngine::new();
        let intent = engine.parse_intent("كرر طباعة 'مرحبا' 3 مرات");
        
        assert_eq!(intent.action, "loop");
        assert!(intent.code.is_some());
    }
    
    #[test]
    fn test_run_pipeline() {
        let engine = PipelineEngine::new();
        let result = engine.run_pipeline("اطبع مرحبا");
        
        assert!(result.success);
    }
    
    #[test]
    fn test_generate_code() {
        let engine = PipelineEngine::new();
        let intent = Intent {
            action: "print".to_string(),
            value: Some("مرحبا".to_string()),
            params: None,
            code: None,
        };
        
        let code = engine.generate_code(&intent);
        assert!(code.contains("اطبع"));
    }
}
