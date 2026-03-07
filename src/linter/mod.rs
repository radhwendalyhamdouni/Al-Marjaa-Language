use crate::parser::ast::{BinaryOp, DestructuringPattern, Expr, Program, Stmt};
use crate::parser::Parser;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LintLevel {
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintDiagnostic {
    pub code: &'static str,
    pub level: LintLevel,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LintConfig {
    pub disabled_rules: HashSet<String>,
    pub max_diagnostics: Option<usize>,
}

impl LintConfig {
    pub fn is_rule_enabled(&self, code: &str) -> bool {
        !self.disabled_rules.contains(code)
    }
}

impl LintDiagnostic {
    fn warning(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            level: LintLevel::Warning,
            message: message.into(),
        }
    }
}

pub fn lint_source(input: &str) -> Result<Vec<LintDiagnostic>, String> {
    lint_source_with_config(input, &LintConfig::default())
}

pub fn lint_source_with_config(
    input: &str,
    config: &LintConfig,
) -> Result<Vec<LintDiagnostic>, String> {
    let program = Parser::parse(input).map_err(|e| e.message)?;
    Ok(lint_program_with_config(&program, config))
}

pub fn lint_program(program: &Program) -> Vec<LintDiagnostic> {
    lint_program_with_config(program, &LintConfig::default())
}

pub fn lint_program_with_config(program: &Program, config: &LintConfig) -> Vec<LintDiagnostic> {
    let mut visitor = LintVisitor::new(config.clone());
    visitor.visit_program(program);
    visitor.finish()
}

#[derive(Default)]
struct Scope {
    declarations: HashSet<String>,
    usages: HashSet<String>,
}

struct LintVisitor {
    diagnostics: Vec<LintDiagnostic>,
    scopes: Vec<Scope>,
    config: LintConfig,
}

impl LintVisitor {
    fn new(config: LintConfig) -> Self {
        Self {
            diagnostics: Vec::new(),
            scopes: vec![Scope::default()],
            config,
        }
    }

    fn finish(mut self) -> Vec<LintDiagnostic> {
        while self.scopes.len() > 1 {
            self.pop_scope();
        }
        self.pop_scope();
        self.diagnostics
    }

    fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for name in scope.declarations {
                if !scope.usages.contains(&name) {
                    self.emit_warning(
                        "L001",
                        format!(
                            "المتغير '{}' تم تعريفه لكنه غير مستخدم. احذف التعريف أو استخدم المتغير.",
                            name
                        ),
                    );
                }
            }
        }
    }

    fn emit_warning(&mut self, code: &'static str, message: String) {
        if !self.config.is_rule_enabled(code) {
            return;
        }

        if let Some(max) = self.config.max_diagnostics {
            if self.diagnostics.len() >= max {
                return;
            }
        }

        self.diagnostics
            .push(LintDiagnostic::warning(code, message));
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("must have scope")
    }

    fn declare(&mut self, name: &str) {
        if self.current_scope_mut().declarations.contains(name) {
            self.emit_warning(
                "L002",
                format!(
                    "المتغير '{}' معرّف مسبقاً في نفس النطاق. هذا قد يسبب التباساً.",
                    name
                ),
            );
        }
        self.current_scope_mut()
            .declarations
            .insert(name.to_string());
    }

    fn mark_usage(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.declarations.contains(name) {
                scope.usages.insert(name.to_string());
                return;
            }
        }
    }

    fn visit_program(&mut self, program: &Program) {
        self.visit_statements(&program.statements);
    }

    fn visit_statements(&mut self, statements: &[Stmt]) {
        let mut terminated = false;
        for statement in statements {
            if terminated {
                self.emit_warning(
                    "L009",
                    "تعليمة غير قابلة للوصول بعد تعليمة تنهي التدفق (أرجع/توقف/أكمل/ارمِ)."
                        .to_string(),
                );
                continue;
            }

            self.visit_stmt(statement);
            terminated = Self::is_flow_terminator(statement);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VariableDecl { name, value, .. } => {
                self.visit_expr(value);
                self.declare(name);
            }
            Stmt::MultiVarDecl { names, values, .. } => {
                for value in values {
                    self.visit_expr(value);
                }
                for name in names {
                    self.declare(name);
                }
            }
            Stmt::Expression(expr) => self.visit_expr(expr),
            Stmt::Block(statements) => {
                self.push_scope();
                self.visit_statements(statements);
                self.pop_scope();
            }
            Stmt::If {
                condition,
                then_branch,
                else_if_branches,
                else_branch,
            } => {
                self.warn_if_constant_condition(condition, "if");
                if Self::is_empty_block(then_branch) {
                    self.emit_warning(
                        "L006",
                        "فرع if يحتوي كتلة فارغة؛ احذف الفرع أو أضف سلوكًا واضحًا داخله.".to_string(),
                    );
                }
                self.visit_expr(condition);
                self.visit_stmt(then_branch);
                for (cond, branch) in else_if_branches {
                    self.warn_if_constant_condition(cond, "else if");
                    if Self::is_empty_block(branch) {
                        self.emit_warning(
                            "L006",
                            "فرع else if يحتوي كتلة فارغة؛ احذف الفرع أو أضف سلوكًا واضحًا داخله."
                                .to_string(),
                        );
                    }
                    self.visit_expr(cond);
                    self.visit_stmt(branch);
                }
                if let Some(branch) = else_branch {
                    if Self::is_empty_block(branch) {
                        self.emit_warning(
                            "L006",
                            "فرع else يحتوي كتلة فارغة؛ راجع منطق القرار أو أضف معالجة مناسبة."
                                .to_string(),
                        );
                    }
                    self.visit_stmt(branch);
                }
            }
            Stmt::While { condition, body } => {
                self.warn_if_constant_condition(condition, "while");
                if Self::is_empty_block(body) {
                    self.emit_warning(
                        "L006",
                        "حلقة while بكتلة فارغة؛ قد تؤدي لدوران بلا أثر أو سلوك مضلل.".to_string(),
                    );
                }
                self.visit_expr(condition);
                self.visit_stmt(body);
            }
            Stmt::For {
                variable,
                iterable,
                body,
            } => {
                self.visit_expr(iterable);
                if Self::is_empty_block(body) {
                    self.emit_warning(
                        "L006",
                        "حلقة for بكتلة فارغة؛ أضف منطقًا داخل الحلقة أو احذفها.".to_string(),
                    );
                }
                self.push_scope();
                self.declare(variable);
                self.visit_stmt(body);
                self.pop_scope();
            }
            Stmt::ForRange {
                variable,
                start,
                end,
                step,
                body,
            } => {
                self.visit_expr(start);
                self.visit_expr(end);
                if let Some(step_expr) = step {
                    self.visit_expr(step_expr);
                }
                if Self::is_empty_block(body) {
                    self.emit_warning(
                        "L006",
                        "حلقة for-range بكتلة فارغة؛ أضف سلوكًا داخل الحلقة أو احذفها.".to_string(),
                    );
                }
                self.push_scope();
                self.declare(variable);
                self.visit_stmt(body);
                self.pop_scope();
            }
            Stmt::Repeat { count, body } => {
                self.visit_expr(count);
                if Self::is_empty_block(body) {
                    self.emit_warning(
                        "L006",
                        "حلقة repeat بكتلة فارغة؛ قد تكون بقايا تجريبية غير مقصودة.".to_string(),
                    );
                }
                self.visit_stmt(body);
            }
            Stmt::FunctionDecl { params, body, .. } => {
                self.push_scope();
                for (name, default, _type_annotation) in params {
                    self.declare(name);
                    if let Some(default_expr) = default {
                        self.visit_expr(default_expr);
                    }
                }
                self.visit_stmt(body);
                self.pop_scope();
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.visit_expr(expr);
                }
            }
            Stmt::Print(exprs) => {
                for expr in exprs {
                    self.visit_expr(expr);
                }
            }
            Stmt::Input(_) | Stmt::Break | Stmt::Continue => {}
            Stmt::TryCatch {
                try_block,
                catch_var,
                catch_block,
                finally_block,
            } => {
                self.visit_stmt(try_block);
                self.push_scope();
                if let Some(name) = catch_var {
                    self.declare(name);
                }
                if matches!(catch_block.as_ref(), Stmt::Block(statements) if statements.is_empty())
                {
                    self.emit_warning(
                        "L003",
                        "كتلة catch فارغة؛ أضف معالجة للخطأ أو أعد رميه لتجنب إخفاء الأعطال."
                            .to_string(),
                    );
                }
                self.visit_stmt(catch_block);
                self.pop_scope();
                if let Some(finally) = finally_block {
                    self.visit_stmt(finally);
                }
            }
            Stmt::Throw(expr) => self.visit_expr(expr),
            Stmt::Match {
                value,
                cases,
                default,
            } => {
                self.visit_expr(value);
                for (patterns, branch) in cases {
                    for pattern in patterns {
                        self.visit_expr(pattern);
                    }
                    self.visit_stmt(branch);
                }
                if let Some(default_branch) = default {
                    self.visit_stmt(default_branch);
                }
            }
            Stmt::ClassDecl {
                methods, fields, ..
            } => {
                for (_, value) in fields {
                    if let Some(expr) = value {
                        self.visit_expr(expr);
                    }
                }
                self.push_scope();
                for method in methods {
                    self.visit_stmt(method);
                }
                self.pop_scope();
            }
            Stmt::Import { .. } => {}
            Stmt::Assert { condition, message } => {
                if let Expr::Boolean(value) = condition {
                    self.emit_warning(
                        "L008",
                        if *value {
                            "assert بشرط ثابت (صح) لن يتحقق منه فعليًا؛ قد يكون فحصًا عديم الأثر."
                        } else {
                            "assert بشرط ثابت (خطأ) سيفشل دائمًا؛ راجع منطق الشرط قبل التشغيل."
                        }
                        .to_string(),
                    );
                }
                self.visit_expr(condition);
                if let Some(message) = message {
                    self.visit_expr(message);
                }
            }
            Stmt::Delete(name) => self.mark_usage(name),
            Stmt::UiComponentDecl { name, body, .. } => {
                self.declare(name);
                self.visit_stmt(body);
            }
            Stmt::StateDecl { name, value }
            | Stmt::ThemeDecl { name, value }
            | Stmt::RouteDecl { name, value } => {
                self.visit_expr(value);
                self.declare(name);
            }
            Stmt::EventHandlerDecl { name, body, .. } => {
                self.declare(name);
                self.visit_stmt(body);
            }
            Stmt::With { resource, body, .. } => {
                self.visit_expr(resource);
                self.visit_stmt(body);
            }
            Stmt::DataClassDecl { name, fields } => {
                self.declare(name);
                for (_, default) in fields {
                    if let Some(val) = default {
                        self.visit_expr(val);
                    }
                }
            }
            Stmt::EnumDecl { name, variants } => {
                self.declare(name);
                for (_, val) in variants {
                    if let Some(v) = val {
                        self.visit_expr(v);
                    }
                }
            }
            Stmt::Decorated { decorator, target } => {
                self.visit_expr(decorator);
                self.visit_stmt(target);
            }
            Stmt::DestructuringDecl { pattern, value, .. } => {
                self.visit_expr(value);
                // Visit pattern variables
                match pattern {
                    DestructuringPattern::List(_elements) => {}
                    DestructuringPattern::Object(_fields) => {}
                    DestructuringPattern::Identifier(_name) => {}
                }
            }
            Stmt::Yield(expr) => {
                self.visit_expr(expr);
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Identifier(name) => self.mark_usage(name),
            Expr::Binary { left, op, right } => {
                if matches!(op, BinaryOp::Div | BinaryOp::Mod | BinaryOp::FloorDiv)
                    && Self::is_zero_literal(right)
                {
                    self.emit_warning(
                        "L007",
                        "عملية قسمة/باقي على صفر literal قد تسبب خطأ وقت التنفيذ.".to_string(),
                    );
                }
                self.visit_expr(left);
                self.visit_expr(right);
            }
            Expr::Logical { left, right, .. } => {
                self.visit_expr(left);
                self.visit_expr(right);
            }
            Expr::Comparison { left, right, .. } => {
                if let (Expr::Identifier(left_name), Expr::Identifier(right_name)) =
                    (left.as_ref(), right.as_ref())
                {
                    if left_name == right_name {
                        self.emit_warning(
                            "L004",
                            format!(
                                "مقارنة المتغير '{}' بنفسه دائمًا ثابتة؛ راجع الشرط لتجنب منطق غير مقصود.",
                                left_name
                            ),
                        );
                    }
                }
                self.visit_expr(left);
                self.visit_expr(right);
            }
            Expr::Unary { expr, .. } | Expr::Await(expr) => self.visit_expr(expr),
            Expr::Call { callee, args } => {
                self.visit_expr(callee);
                for arg in args {
                    self.visit_expr(arg);
                }
            }
            Expr::Property { object, .. } => self.visit_expr(object),
            Expr::Index { object, index } => {
                self.visit_expr(object);
                self.visit_expr(index);
            }
            Expr::List(items) => {
                for item in items {
                    self.visit_expr(item);
                }
            }
            Expr::Dictionary(items) => {
                for (k, v) in items {
                    self.visit_expr(k);
                    self.visit_expr(v);
                }
            }
            Expr::Assignment { target, value } => {
                self.visit_expr(target);
                self.visit_expr(value);
            }
            Expr::CompoundAssignment { name, value, .. } => {
                self.mark_usage(name);
                if matches!(
                    expr,
                    Expr::CompoundAssignment {
                        op: BinaryOp::Div | BinaryOp::Mod | BinaryOp::FloorDiv,
                        ..
                    }
                ) && Self::is_zero_literal(value)
                {
                    self.emit_warning(
                        "L007",
                        "إسناد مركب بقيمة صفر literal قد يؤدي لقسمة على صفر وقت التنفيذ."
                            .to_string(),
                    );
                }
                self.visit_expr(value);
            }
            Expr::Lambda { params, body } => {
                self.push_scope();
                for param in params {
                    self.declare(param);
                }
                self.visit_expr(body);
                self.pop_scope();
            }
            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                self.warn_if_constant_condition(condition, "ternary");
                self.visit_expr(condition);
                self.visit_expr(then_expr);
                self.visit_expr(else_expr);
            }
            Expr::Increment { name, .. } => self.mark_usage(name),
            Expr::Range { start, end, step } => {
                self.visit_expr(start);
                self.visit_expr(end);
                if let Some(step) = step {
                    self.visit_expr(step);
                }
            }
            Expr::FormatString(parts) => {
                for part in parts {
                    if let crate::parser::ast::FormatPart::Expression(expr) = part {
                        self.visit_expr(expr);
                    }
                }
            }
            // List Comprehension
            Expr::ListComprehension {
                element,
                variable,
                iterable,
                condition,
            } => {
                self.visit_expr(iterable);
                self.push_scope();
                self.declare(variable);
                self.visit_expr(element);
                if let Some(cond) = condition {
                    self.visit_expr(cond);
                }
                self.pop_scope();
            }
            // Dictionary Comprehension
            Expr::DictComprehension {
                key,
                value,
                variable,
                iterable,
                condition,
            } => {
                self.visit_expr(iterable);
                self.push_scope();
                self.declare(variable);
                self.visit_expr(key);
                self.visit_expr(value);
                if let Some(cond) = condition {
                    self.visit_expr(cond);
                }
                self.pop_scope();
            }
            // Spread
            Expr::Spread(inner) => self.visit_expr(inner),
            // Null Coalescing
            Expr::NullCoalescing { left, right } => {
                self.visit_expr(left);
                self.visit_expr(right);
            }
            // Optional Property
            Expr::OptionalProperty { object, .. } => self.visit_expr(object),
            // Optional Index
            Expr::OptionalIndex { object, index } => {
                self.visit_expr(object);
                self.visit_expr(index);
            }
            // Optional Call
            Expr::OptionalCall { callee, args } => {
                self.visit_expr(callee);
                for arg in args {
                    self.visit_expr(arg);
                }
            }
            // Pipe
            Expr::Pipe { value, function } => {
                self.visit_expr(value);
                self.visit_expr(function);
            }
            Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) | Expr::Null => {}
            // Yield
            Expr::Yield(inner) => self.visit_expr(inner),
            // Generator
            Expr::Generator { body } => self.visit_stmt(body),
        }
    }

    fn warn_if_constant_condition(&mut self, condition: &Expr, context: &str) {
        if matches!(
            condition,
            Expr::Boolean(_) | Expr::Number(_) | Expr::String(_) | Expr::Null
        ) {
            self.emit_warning(
                "L005",
                format!(
                    "شرط {context} ثابت (literal) وقد ينتج سلوكًا غير متوقع؛ راجع التعبير الشرطي."
                ),
            );
        }
    }

    fn is_empty_block(stmt: &Stmt) -> bool {
        matches!(stmt, Stmt::Block(statements) if statements.is_empty())
    }

    fn is_zero_literal(expr: &Expr) -> bool {
        matches!(expr, Expr::Number(number) if number.abs() < f64::EPSILON)
    }

    fn is_flow_terminator(stmt: &Stmt) -> bool {
        matches!(
            stmt,
            Stmt::Return(_) | Stmt::Break | Stmt::Continue | Stmt::Throw(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{lint_source, lint_source_with_config, LintConfig, LintLevel};
    use std::collections::HashSet;

    #[test]
    fn warns_for_unused_variable() {
        let source = "متغير س = ١٠؛\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "L001");
        assert_eq!(diagnostics[0].level, LintLevel::Warning);
    }

    #[test]
    fn warns_for_duplicate_declaration_in_same_scope() {
        let source = "متغير س = ١؛\nمتغير س = ٢؛\nاطبع(س)؛\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert!(diagnostics.iter().any(|d| d.code == "L002"));
    }

    #[test]
    fn does_not_warn_when_variable_is_used() {
        let source = "متغير س = ١؛\nاطبع(س)؛\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert!(
            diagnostics.is_empty(),
            "expected no diagnostics: {diagnostics:#?}"
        );
    }

    #[test]
    fn warns_for_empty_catch_block() {
        let source = "حاول {\nاطبع(١)؛\n} امسك(خ) {\n}\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert!(diagnostics.iter().any(|d| d.code == "L003"));
    }

    #[test]
    fn warns_for_self_comparison() {
        let source = "متغير س = ١؛\nإذا س == س {\nاطبع(\"نعم\")؛\n}\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert!(diagnostics.iter().any(|d| d.code == "L004"));
    }

    #[test]
    fn warns_for_constant_condition() {
        let source = "إذا صح {\nاطبع(\"ثابت\")؛\n}\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert!(diagnostics.iter().any(|d| d.code == "L005"));
    }

    #[test]
    fn supports_disabling_rules_via_config() {
        let source = "متغير س = ١٠؛\n";
        let config = LintConfig {
            disabled_rules: HashSet::from(["L001".to_string()]),
            max_diagnostics: None,
        };
        let diagnostics = lint_source_with_config(source, &config).expect("lint should parse");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn supports_limiting_diagnostics_count() {
        let source = "متغير أ = ١؛\nمتغير ب = ٢؛\n";
        let config = LintConfig {
            disabled_rules: HashSet::new(),
            max_diagnostics: Some(1),
        };
        let diagnostics = lint_source_with_config(source, &config).expect("lint should parse");
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn warns_for_empty_control_blocks() {
        let source = "إذا صحيح {}\nطالما صحيح {}\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert!(diagnostics.iter().any(|d| d.code == "L006"));
    }

    #[test]
    fn warns_for_zero_division_literals() {
        let source = "متغير س = ١٠ / ٠؛\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert!(diagnostics.iter().any(|d| d.code == "L007"));
    }

    #[test]
    fn warns_for_constant_assert_condition() {
        let source = "تأكد خطأ، \"سيفشل\"؛\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert!(diagnostics.iter().any(|d| d.code == "L008"));
    }

    #[test]
    fn warns_for_unreachable_statement_after_return() {
        let source = "دالة اختبر() {\nأرجع ١؛\nاطبع(\"لن تُنفذ\")؛\n}\n";
        let diagnostics = lint_source(source).expect("lint should parse");
        assert!(diagnostics.iter().any(|d| d.code == "L009"));
    }
}
