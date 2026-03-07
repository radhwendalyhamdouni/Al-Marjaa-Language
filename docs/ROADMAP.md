# خطة التنفيذ المرحلية - لغة المرجع (Al-Marjaa)

**المدة:** خارطة مستمرة متعددة الأرباع (2026+)
**الإصدار المستهدف:** 3.3.0 ✅
**آخر تحديث:** 2026-02
**الحالة الزمنية:** ✅ تم إكمال المرحلة الأساسية - v3.3.0 منشور

---

## ✅ إنجازات الإصدار 3.3.0 (2026-02)

### المكتبة القياسية الشاملة
- ✅ وحدة HTTP: عميل وخادم مع WebSocket وMiddleware
- ✅ وحدة Database: MySQL, PostgreSQL, SQLite, MongoDB
- ✅ وحدة Regex: تعابير نمطية مع دعم Unicode العربي
- ✅ وحدة Crypto: SHA-256, AES, RSA, JWT, bcrypt

### نظام الحزم الكامل
- ✅ Registry مركزي للحزم
- ✅ نظام Lockfile متقدم
- ✅ التوزيع الثنائي (Linux/macOS/Windows)
- ✅ توقيع الحزم (Ed25519/RSA)
- ✅ نظام Workspace (Monorepo)
- ✅ التحقق الأمني من الحزم

---


## أولويات استراتيجية مضافة (طلب 2026)

- ✅ **سياسة توافق وإصدارات رسمية:** اعتماد `docs/RELEASE_PLAN.md` كمرجع حاكم لقرارات الإصدار بدءاً من 3.3.0
- ✅ **المكتبة القياسية الشاملة:** HTTP, Database, Regex, Crypto
- ✅ **نظام حزم كامل:** Registry, Lockfile, Signing, Workspace, Binary Distribution
- ⚠️ **LSP server كامل:** الانتقال من CLI bridge إلى خادم stateful (قيد التطوير)
- ⚠️ **Onboarding أسرع:** خفض Time-to-First-Run للمستخدم الجديد (قيد التطوير)

---

## Milestones 2026+ (قابلة للقياس)

### Q2 2026 ✅ مكتمل
- [x] M1: تثبيت quality gates بحيث تنجح 100% من أوامر `fmt/clippy/test/golden`
- [x] M2: المكتبة القياسية الشاملة (HTTP, Database, Regex, Crypto)
- [x] M2.1: نظام حزم كامل مع Lockfile وSigning وWorkspace

### Q3 2026
- [ ] M3: إطلاق LSP server MVP يدعم `diagnostics/hover/definition` بزمن استجابة P95 أقل من 120ms
- [ ] M4: دعم incremental analysis مع cache بسيط

### Q4 2026
- [ ] M5: استكمال قدرات `references/completion/formatting` داخل LSP server
- [ ] M6: إصدار 4.0.0-rc مع دليل ترحيل محدث

### 2027 (مرحلة التوسعة)
- [ ] M7: مسار الأداء المتقدم مع benchmark suite رسمي
- [ ] M8: حزمة أدوات مطور متكاملة

### 2028+ (الاستدامة)
- [ ] M9: استقرار Long-term support (LTS)
- [ ] M10: تبني مؤسسي موثق

---

## مرجع الحوكمة (مهم)

لتوحيد الرؤية بين التنفيذ والإطلاق:
- سياسة الإصدارات والتوافق الرسمية: `docs/RELEASE_PLAN.md`
- دليل الترحيل بين الإصدارات: `docs/MIGRATION_GUIDE.md`
- المسار التشغيلي للـ CI/CD: `docs/CI_CD.md`

> هذا المستند يركز على **خارطة التنفيذ الهندسية**، بينما تفاصيل التوافق والإصدار أصبحت موحدة في المراجع أعلاه.

---

## الملخص التنفيذي (2026+)

| المرحلة | الإطار الزمني | الهدف القابل للقياس | Owner |
|---------|--------------|----------------------|-------|
| Phase A+ | Q2 2026 | 4 أسابيع متتالية مع نجاح كامل `fmt/clippy/test/golden` + 20 حالة golden إضافية | Tech Lead + Tooling |
| Phase B+ | Q3 2026 | MVP لخادم LSP (`diagnostics/hover/definition`) بزمن P95 < 120ms | IDE Engineer |
| Phase C+ | Q4 2026 | استكمال `references/completion/formatting` + إصدار `3.0.0-rc` | Language + IDE Team |
| Phase D+ | 2027 | تحسين الأداء (IR/VM) بهدف 2x في benchmark suite الرسمي | Performance Engineer |

---

## الأرشيف التاريخي (للتتبع)

> الأقسام التالية توثق الـ sprints القديمة كسجل تنفيذ، وليست الجدول الزمني الحاكم بعد تحديث 2026+.

## Phase A: تثبيت الأساس (الأسبوع 1-6)

### Sprint 1: الإصلاح السريع والإعداد ✅ (مكتمل)

**الأهداف:**
- توحيد الإصدار عبر المشروع
- إعداد هيكل الاختبارات
- بناء نظام الأخطاء الأساسي

**Deliverables:**
- [x] VERSION موحد في main.rs = 2.0.0
- [x] .gitignore موجود
- [x] CHANGELOG.md موجود
- [x] هيكل docs/ موجود
- [x] src/error/mod.rs - نظام أخطاء احترافي مع spans و error codes
- [x] src/lexer/tokens.rs - إضافة span للـ Tokens
- [x] tests/lexer_tests.rs - 20+ اختبار للـ Lexer
- [x] tests/parser_tests.rs - 20+ اختبار للـ Parser
- [x] tests/interpreter_tests.rs - 78+ اختبار للمفسر
- [x] src/lib.rs - مكتبة عامة للاختبارات والتكامل
- [x] إصلاح أخطاء الـ lifetime في الـ interpreter
- [x] إصلاح مشكلة دالة "طول"

**النتائج:**
- 117 اختبار ناجح (Lexer: 20, Parser: 20, Interpreter: 77)
- 1 اختبار فاشل (匿名 functions - ميزة غير مكتملة)

**Risk:** منخفض ✅

---

### Sprint 2: نظام الأخطاء (الأسبوع 2) ✅ (مكتمل)

**الأهداف:**
- بناء professional error system
- spans + line/column + suggestions

**Deliverables:**
- [x] src/error/mod.rs - مكتمل
- [x] Error struct مع span
- [x] Error codes
- [x] Arabic error messages محسنة
- [x] SourceContext - تحسين عرض السياق
- [x] Helper functions - دمج مع Parser/Interpreter

**Dependency:** Sprint 1 (مكتمل)

---

### Sprint 3: الاختبارات الأساسية (الأسبوع 3)

**الأObjectives:**
- توسيع الاختبارات
- Golden tests

**Deliverables:**
- [x] نقل اختبارات main.rs إلى tests/
- [x] إضافة 20+ اختبار lexer
- [x] إضافة 20+ اختبار parser
- [x] إضافة 10+ اختبار interpreter

**Owner:** Senior Engineer
**Dependency:** Sprint 2

---

### Sprint 4: CI/CD (الأسبوع 4)

**الأObjectives:**
- بناء CI pipeline
- أتمتة الاختبارات

**Deliverables:**
- [x] GitHub Actions workflow
- [x] cargo test في CI
- [x] cargo clippy
- [x] cargo fmt --check
- [x] Badge في README

**Owner:** DevOps
**Dependency:** Sprint 3

---

### Sprint 5: Test Corpus (الأسبوع 5)

**الأObjectives:**
- إنشاء test corpus رسمي

**Deliverables:**
- [x] مجلد tests/corpus/
- [x] 10+ ملفات .mrj للاختبار
- [x] اختبار التكامل

**Owner:** Senior Engineer
**Dependency:** Sprint 4

---

### Sprint 6: Error Recovery + Debugging (الأسبوع 6)

**الأObjectives:**
- Error recovery في Parser
- تحسين أدوات التصحيح

**Deliverables:**
- [x] Error recovery في Parser
- [x] أفضل رسائل اقتراح (suggestions)
- [x] وضع تصحيح (debug mode)
- [x] توسيع API العتاد المدمج (PLC/CNC/HMI/ESP/Arduino) مع توحيد تحقق المعاملات

**Owner:** Tech Lead
**Dependency:** Sprint 2, 3

---

## Phase B: مواصفة اللغة (الأسبوع 7-10)

### Sprint 7: SPEC.md - Grammar (الأسبوع 7) ✅ (مكتمل)

**الأObjectives:**
- إنشاء Grammar موثقة

**Deliverables:**
- [x] EBNF grammar محدث
- [x] Tokens كاملة
- [x] Keywords مرتبة

**Owner:** Language Designer
**Dependency:** Phase A

**تحديث تنفيذي (2026-02):**
- [x] تنظيم قسم الرموز في `SPEC.md` وإصلاح أخطاء صياغة كانت تؤثر على وضوح المواصفة.
- [x] تصحيح قاعدة `ForStmt` لتستخدم الكلمة العربية `في` بدل `in`.
- [x] تغطية كلمة `طول` في Lexer ضمن الكلمات المحجوزة باختبار تلقائي.

---

### Sprint 8: SPEC.md - Semantics (الأسبوع 8)

**الأObjectives:**
- توثيق semantics

**Deliverables:**
- [x] أنواع البيانات
- [x] التعبيرات (Expressions)
- [x] التعليمات (Statements)
- [x] الوظائف (Functions)

**Owner:** Language Designer
**Dependency:** Sprint 7

---

### Sprint 9: SPEC.md - Edge Cases (الأسبوع 9)

**الأObjectives:**
- توثيق السلوك في الحالات الحدّية

**Deliverables:**
- [x] NaN semantics
- [x] Division by zero
- [x] Null semantics
- [x] Mutability rules
- [x] Type coercion

**Owner:** Language Designer
**Dependency:** Sprint 8

---

### Sprint 10: ARCHITECTURE.md + RFC Process (الأسبوع 10)

**الأObjectives:**
- توثيق البنية التقنية
- إنشاء عملية RFC

**Deliverables:**
- [x] مخطط البنية محدث
- [x] مكونات النظام موثقة
- [x] RFC template
- [x] قرارات التصميم موثقة

**Owner:** Tech Lead
**Dependency:** Sprint 9

**تحديث تنفيذي (2026-02):**
- [x] توسيع `docs/SPEC.md` إلى مرجعية تنفيذية أوضح للـ Grammar + Semantics + Edge Cases مع تفاصيل تشغيلية قابلة للاختبار.
- [x] تحديث `docs/ARCHITECTURE.md` ليتضمن طبقات النظام، حدود المسؤولية، ومصفوفة قرارات تصميم مرتبطة بالبدائل.
- [x] إضافة `docs/RFC_PROCESS.md` لتعريف دورة RFC كاملة من الاقتراح إلى الاعتماد.
- [x] إضافة قالب RFC رسمي في `docs/rfcs/0000-template.md` لتوحيد صياغة التغييرات المعمارية واللغوية.

---

## Phase C: منظومة الأدوات (الأسبوع 11-16)

### Sprint 11: Formatter - الجزء 1 (الأسبوع 11)

**الأObjectives:**
- بناء formatter رسمي

**Deliverables:**
- [x] هيكل المشروع
- [x] Indentation handling
- [x] Whitespace rules

**Owner:** Tooling Engineer
**Dependency:** Phase B

**تحديث تنفيذي (2026-02):**
- [x] إضافة وحدة `src/formatter/mod.rs` كبنية أولية للـ formatter الرسمي.
- [x] تطبيق قواعد أولية للمسافات البادئة وتطبيع الفراغات.
- [x] دمج formatter مع CLI عبر الخيار `--format`.

---

### Sprint 12: Formatter - الجزء 2 (الأسبوع 12)

**الأObjectives:**
- إكمال formatter

**Deliverables:**
- [x] Line wrapping
- [x] Comments formatting
- [x] CLI integration
- [x] tests

**Owner:** Tooling Engineer
**Dependency:** Sprint 11

**تحديث تنفيذي (2026-02):**
- [x] إضافة التفاف أسطر تلقائي في formatter للأسطر الطويلة مع احترام المسافات البادئة.
- [x] تنسيق التعليقات (`#` و`//`) عبر تطبيع المسافات داخليًا وتوحيد الشكل النهائي.
- [x] تثبيت تكامل CLI لخيار `--format` مع سلوك التفاف/تعليقات قابل للتحقق.
- [x] توسيع اختبارات الوحدة والتكامل لتغطية ميزات Sprint 12 ومنع الانحدار.

---

### Sprint 13: Linter - الجزء 1 (الأسبوع 13) ✅ (مكتمل)

**الأObjectives:**
- بناء linter رسمي

**Deliverables:**
- [x] هيكل Linter
- [x] قواعد أساسية (unused variables, duplicate declaration)
- [x] AST visitors

**Owner:** Tooling Engineer
**Dependency:** Sprint 12

---
**تحديث تنفيذي (2026-02):**
- [x] إضافة وحدة `src/linter/mod.rs` كبنية أولية للـ linter.
- [x] تطبيق قواعد lint أولية: المتغير غير المستخدم (L001) وإعادة التعريف في نفس النطاق (L002).
- [x] بناء AST Visitor لتحليل التصريحات والاستخدامات عبر النطاقات المختلفة.
- [x] دمج linter في CLI عبر الخيار `--lint` مع مخرجات عربية واضحة.
- [x] إضافة اختبارات وحدة وتكامل لتثبيت سلوك Sprint 13 ومنع الانحدار.

---

### Sprint 14: Linter - الجزء 2 (الأسبوع 14)

**الأObjectives:**
- إكمال linter

**Deliverables:**
- [x] 8/20+ قواعد (الدفعة الحالية: L003/L004/L005/L006/L007/L008 إضافة إلى L001/L002)
- [x] Configuration أولية (`LintConfig`: تعطيل قواعد + سقف التحذيرات)
- [x] CLI integration أولي (`--lint-disable` و `--lint-max`)

**Owner:** Tooling Engineer
**Dependency:** Sprint 13

**تحديث تنفيذي (2026-02):**
- [x] إضافة ست قواعد lint جديدة ضمن Sprint 14: `L003` (catch فارغ)، `L004` (self-comparison)، `L005` (constant condition)، `L006` (empty control block)، `L007` (zero division literal)، `L008` (constant assert).
- [x] تقديم `LintConfig` لتمكين تعطيل القواعد وتحديد الحد الأقصى للتحذيرات.
- [x] توسيع CLI بخياري `--lint-disable` و`--lint-max`.
- [x] توسيع اختبارات الوحدة والتكامل لتغطية السلوك الجديد ومنع الانحدار.

---

### Sprint 15: Package Manager - الجزء 1 (الأسبوع 15)

**Objectives:**
- تصميم نظام الحزم

**Deliverables:**
- [ ] mrj.toml format
- [ ] Registry design
- [ ] Dependency resolution

**Owner:** Tooling Engineer
**Dependency:** Sprint 14

---

### Sprint 16: Package Manager - الجزء 2 + Test Runner (الأسبوع 16)

**Deliverables:**
- [ ] CLI موحد
- [ ] Test runner
- [ ] Doc generator basic

**Owner:** Tooling Engineer
**Dependency:** Sprint 15

---

## Phase D: IDE & LSP (الأسبوع 17-20)

### Sprint 17: LSP Server - الجزء 1 (الأسبوع 17)

**Objectives:**
- بناء Language Server

**Deliverables:**
- [ ] LSP protocol implementation
- [ ] Text document sync
- [ ] Basic diagnostics
- [ ] تحسين تشخيص LSP على المشاريع الكبيرة (incremental diagnostics + caching)

**Owner:** IDE Engineer
**Dependency:** Phase C

---

### Sprint 18: LSP Server - الجزء 2 (الأسبوع 18)

**Deliverables:**
- [ ] Completion
- [ ] Hover
- [ ] Go to definition
- [ ] تحسين الإكمال/التعريف على قواعد كود كبيرة (symbol index + workspace scope)

**Owner:** IDE Engineer
**Dependency:** Sprint 17

---

### Sprint 19: LSP Server - الجزء 3 (الأسبوع 19)

**Deliverables:**
- [ ] Rename symbols
- [ ] Find references
- [ ] Code actions

**Owner:** IDE Engineer
**Dependency:** Sprint 18

---

### Sprint 20: VS Code Extension (الأسبوع 20)

**Deliverables:**
- [ ] Basic extension
- [ ] Syntax highlighting
- [ ] Integration with LSP

**Owner:** IDE Engineer
**Dependency:** Sprint 19

---

## Phase E: الأداء (الأسبوع 21-22)

### Sprint 21: Benchmark Suite + Path Evaluation (الأسبوع 21)

**Deliverables:**
- [ ] Benchmark suite
- [ ] Performance tests
- [ ] Before/after measurements
- [ ] تقييم: Tree-walk vs Bytecode vs JIT
- [ ] قرار مع مبررات

**Owner:** Performance Engineer
**Dependency:** Phase D

---

### Sprint 22: Optimization (الأسبوع 22)

**Deliverables:**
- [ ] تحسينات interpreter
- [ ] تقرير الأداء
- [ ] إعداد 2.x release

**Owner:** Performance Engineer
**Dependency:** Sprint 21

---

## Phase F: الإنتاجية (الأسبوع 23-24)

### Sprint 23: Standard Library Expansion (الأسبوع 23)

**Deliverables:**
- [ ] HTTP client/server
- [ ] JSON advanced
- [ ] Filesystem operations

**Owner:** Library Engineer
**Dependency:** Phase E

---

### Sprint 24: Production Readiness (الأسبوع 24)

**Deliverables:**
- [ ] Security policy محدث
- [ ] Release artifacts
- [ ] Cross-platform binaries
- [ ] تجهيز إصدارات ثنائية جاهزة (Linux/macOS/Windows) مع SHA256
- [ ] 3.0.0 release

**تحديث تنفيذي (أولويات اعتماد 2026):**
- [ ] إنشاء مسار نشر تلقائي لملفات ثنائية جاهزة على GitHub Releases (Linux/macOS/Windows).
- [ ] تحسين تجربة LSP على المشاريع الأكبر عبر فهرسة رموز المشروع وتخزين نتائج التحليل.
- [ ] نشر دليل اعتماد رسمي للمؤسسات التعليمية وفرق التدريب.

**Owner:** Tech Lead
**Dependency:** Sprint 23

---

## Dependency Map

```
Sprint 1 → Sprint 2 → Sprint 3 → Sprint 4 → Sprint 5 → Sprint 6
                                                      ↓
Phase B ← Sprint 7 ← Sprint 8 ← Sprint 9 ← Sprint 10 ←┘
                                              ↓
Phase C → Sprint 11 → Sprint 12 → Sprint 13 → Sprint 14 → Sprint 15 → Sprint 16
                                                                          ↓
Phase D ← Sprint 17 ← Sprint 18 ← Sprint 19 ← Sprint 20 ←┘
                                              ↓
Phase E ← Sprint 21 ← Sprint 22 ←┘
                                  ↓
Phase F ← Sprint 23 ← Sprint 24
```

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Scope creep | صارم في Definition of Done |
| Delays | أسبوع buffer بين المراحل |
| Resource constraints | أولويات واضحة |
| Technical debt |每周 مراجعة |

---

## Definition of Done

كل Sprint считается завершенным когда:
- ✅ الاختبارات ناجحة
- ✅ التوثيق محدث
- ✅ CHANGELOG محدث
- ✅ قياس الأثر (أداء/سهولة/استقرار)
- ✅ مراجعة backward compatibility

---

## Metrics

| المقياس | الهدف |
|---------|-------|
| Code Coverage | ≥ 80% |
| Test Duration | < 5 min |
| Flaky Tests | 0% |
| Documentation | 100% functions |
| Binary Size | < 5MB |
| Startup Time | < 100ms |
