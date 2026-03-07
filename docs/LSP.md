# LSP Plan - بناء LSP مستقرة

## الوضع الحالي (2026-03)
يوجد baseline عملي عبر واجهة CLI (`--lsp-diag` و `lsp complete/hover/definition/references`) لكنه ليس خادماً LSP stateful بعد.

**الحالة التنفيذية:** Baseline مكتمل، والمرحلة التالية هي التحويل إلى server تدريجياً دون كسر التكامل الحالي.

الهدف في هذه الخطة هو نقل القدرات الحالية إلى LSP server كامل مع معايير نجاح قابلة للقياس.


## تعريف “LSP server كامل”

يُعتبر الخادم **كاملاً (Full LSP Server)** عندما يحقق الحد الأدنى التالي بصورة stateful عبر JSON-RPC/LSP (وليس CLI bridge فقط):
- `initialize` + `initialized` + إدارة قدرات العميل (capability negotiation).
- `textDocument/didOpen` + `didChange` + `didClose` + تخزين مستندات داخلية.
- `textDocument/publishDiagnostics` بشكل incremental.
- `textDocument/hover`, `definition`, `references`, `completion`, `formatting`.
- `shutdown` + `exit` مع إغلاق نظيف وبدون تسرب حالة.

**معيار الإعلان Stable:** لا يُعلن أي مستوى Stable قبل اجتياز اختبارات تكامل متعددة المحررات (VS Code/Neovim/Helix) وتوثيق قياسات latency والدقة.

## Milestones تنفيذية (2026+)

| Milestone | الإطار الزمني | القدرات المستهدفة | الحالة |
|-----------|---------------|-------------------|--------|
| M1 | Q2 2026 / Sprint 1-2 | Diagnostics | Planned |
| M2 | Q2 2026 / Sprint 3-4 | Hover + Definition | Planned |
| M3 | Q3 2026 / Sprint 1-2 | References + Completion | Planned |
| M4 | Q3 2026 / Sprint 3 | Formatting + Schema Stability | Planned |
| M5 | Q4 2026 | Hardening (incremental + cache + observability) | Planned |
| M6 | Q1 2027 | Full protocol parity + packaging for editors | Planned |

### M1 — Diagnostics Channel
**النطاق:** ربط parser/linter بقناة `textDocument/publishDiagnostics` على خادم LSP.
**معيار النجاح:**
- 95% من حالات أخطاء parser/linter المغطاة حالياً في CLI تظهر عبر LSP بنفس الكود والرسالة.
- جميع اختبارات diagnostics التكاملية (VS Code + Neovim + Helix bridge) تمر دون regressions.

### M2 — Symbol Intelligence (Hover/Definition)
**النطاق:** symbol table موحد يخدم `hover` و`definition`.
**معيار النجاح:**
- دقة `definition` لا تقل عن 90% على corpus مرجعي داخلي.
- `hover` يعيد نوع الرمز/الوصف في أقل من 120ms (P95) للملف المتوسط.

### M3 — References + Completion
**النطاق:** دعم `references` و`completion` مع نفس semantics الحالية في CLI.
**معيار النجاح:**
- دقة `references` ≥ 90% على اختبارات التكامل.
- `completion` يعيد اقتراحات سياقية (keywords + symbols) بزمن P95 < 150ms.

### M4 — Formatting + Stability
**النطاق:** توصيل `textDocument/formatting` إلى `format_source` مع سياسة استقرار payloads.
**معيار النجاح:**
- تطابق ناتج formatting بين CLI وLSP بنسبة 100% على golden tests الخاصة بالتنسيق.
- لا توجد breaking changes في schema بدون RFC + migration note.

### M5 — Production Hardening
**النطاق:** incremental diagnostics + caching + observability.
**معيار النجاح:**
- تقليل إعادة التحليل الكامل بنسبة 40% على مشاريع مرجعية.
- تقرير استقرار Sprintين متتاليين بدون incidents حرجة (S1/S2).


## مصفوفة القياس (Execution Scorecard)

| Capability | Metric رئيسي | هدف القبول | أداة القياس |
|------------|---------------|------------|-------------|
| Diagnostics | Parity مع CLI | ≥95% تطابق في code/message/span | integration snapshots |
| Hover | Latency P95 | <120ms | benchmark harness |
| Definition | Accuracy | ≥90% على corpus المرجعي | navigation test suite |
| References | Precision/Recall | ≥90% مع deduplication ثابت | references integration tests |
| Completion | Relevance + Latency | Top-5 مفيد + P95 <150ms | completion benchmarks |
| Formatting | Determinism | 100% تطابق مع formatter CLI | golden tests |

## Definition of Done لكل capability
- **Diagnostics:** parity واضح مع CLI + telemetry لعدد الأخطاء/الزمن.
- **Hover/Definition:** دعم symbols المحلية والمستوردة مع fallback واضح.
- **References:** نتائج دقيقة مع deduplication وترتيب ثابت.
- **Completion:** اقتراحات سياقية + ترتيب relevance + عدم كسر RTL display.
- **Formatting:** output deterministic ومتوافق 100% مع formatter الحالي.

## سياسة الاستقرار
- أي capability تُعلن "stable" يجب أن تُدعم باختبارات تكامل LSP مخصصة.
- توحيد schema بين CLI bridge وLSP server خلال فترة الانتقال لتقليل كلفة التبني.
- منع breaking changes في payloads دون RFC داخلي وتوثيق migration.


## بوابة القبول لكل Milestone
- لا يُغلق أي milestone قبل توثيق القياس (latency/accuracy) في تقرير Sprint.
- أي اختلاف سلوكي بين CLI bridge وLSP server يجب أن يُوثق مع rationale وخطة migration.


### M6 — Full Server GA
**النطاق:** اكتمال بروتوكول LSP المستهدف + توزيع server binary/plugins المبدئية للمحررات الشائعة.
**معيار النجاح:**
- دعم end-to-end لمسار جلسة LSP كاملة من `initialize` حتى `shutdown`.
- نجاح smoke tests عبر VS Code + Neovim + Helix على نفس corpus المرجعي.
- إصدار artifact رسمي لخادم LSP ضمن GitHub Release مع checksum.
