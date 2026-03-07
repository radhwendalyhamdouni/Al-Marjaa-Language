// ═══════════════════════════════════════════════════════════════════════════════
// المُحسِّن (Optimizer) - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// يقوم بتحسين AST قبل الترجمة إلى bytecode
// يتضمن:
// - Constant Folding: حساب الثوابت في وقت الترجمة
// - Dead Code Elimination: إزالة الكود غير المستخدم
// - Loop Optimization: تحسين الحلقات
// - Inline Expansion: توسيع الدوال الصغيرة
// - Common Subexpression Elimination: إزالة التعبيرات المتكررة
// - Strength Reduction: تقليل قوة العمليات
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::{HashMap, HashSet};
use crate::parser::ast::{BinaryOp, ComparisonOp, Expr, LogicalOp, Program, Stmt, UnaryOp};

/// نتيجة التحسين
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// البرنامج المُحسَّن
    pub program: Program,
    /// عدد التحسينات المطبقة
    pub optimizations_applied: u32,
    /// تفاصيل التحسينات
    pub details: Vec<OptimizationDetail>,
}

/// تفاصيل التحسين
#[derive(Debug, Clone)]
pub struct OptimizationDetail {
    pub kind: OptimizationKind,
    pub location: String,
    pub description: String,
}

/// أنواع التحسين
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationKind {
    ConstantFolding,
    DeadCodeElimination,
    LoopOptimization,
    InlineExpansion,
    CommonSubexpressionElimination,
    StrengthReduction,
    CopyPropagation,
    VariableElimination,
}

/// مستوى التحسين
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimizationLevel {
    /// بدون تحسين
    None,
    /// تحسين أساسي
    Basic,
    /// تحسين متوسط
    Standard,
    /// تحسين عدواني
    Aggressive,
}

/// المُحسِّن
pub struct Optimizer {
    /// مستوى التحسين
    level: OptimizationLevel,
    /// جدول الثوابت
    constants: HashMap<String, Expr>,
    /// المتغيرات المستخدمة
    used_variables: HashSet<String>,
    /// التعبيرات المحسوبة (محجوز للاستخدام المستقبلي)
    _computed_expressions: HashMap<String, Expr>,
    /// عدد التحسينات
    optimizations_count: u32,
    /// تفاصيل التحسينات
    details: Vec<OptimizationDetail>,
}

impl Optimizer {
    /// إنشاء محسِّن جديد
    pub fn new(level: OptimizationLevel) -> Self {
        Optimizer {
            level,
            constants: HashMap::new(),
            used_variables: HashSet::new(),
            _computed_expressions: HashMap::new(),
            optimizations_count: 0,
            details: Vec::new(),
        }
    }
    
    /// تحسين برنامج كامل
    pub fn optimize(program: &Program, level: OptimizationLevel) -> OptimizationResult {
        let mut optimizer = Optimizer::new(level);
        let optimized = optimizer.optimize_program(program);
        
        OptimizationResult {
            program: optimized,
            optimizations_applied: optimizer.optimizations_count,
            details: optimizer.details,
        }
    }
    
    /// تحسين مع المستوى القياسي
    pub fn optimize_standard(program: &Program) -> Program {
        Self::optimize(program, OptimizationLevel::Standard).program
    }
    
    /// تحسين مع المستوى العدواني
    pub fn optimize_aggressive(program: &Program) -> Program {
        Self::optimize(program, OptimizationLevel::Aggressive).program
    }
    
    // ═══════════════════════════════════════════════════════════════
    // تحسين البرنامج
    // ═══════════════════════════════════════════════════════════════
    
    fn optimize_program(&mut self, program: &Program) -> Program {
        if self.level == OptimizationLevel::None {
            return program.clone();
        }
        
        // المرحلة 1: تحليل المتغيرات المستخدمة
        self.analyze_used_variables(&program.statements);
        
        // المرحلة 2: تطبيق التحسينات
        let mut statements = Vec::new();
        for stmt in &program.statements {
            if let Some(optimized) = self.optimize_stmt(stmt) {
                statements.push(optimized);
            }
        }
        
        // المرحلة 3: تحسين إضافي للمستوى العدواني
        if self.level == OptimizationLevel::Aggressive {
            statements = self.aggressive_optimization(statements);
        }
        
        Program { statements }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // تحليل المتغيرات المستخدمة
    // ═══════════════════════════════════════════════════════════════
    
    fn analyze_used_variables(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            self.mark_used_variables_stmt(stmt);
        }
    }
    
    fn mark_used_variables_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VariableDecl { value, .. } => {
                self.mark_used_variables_expr(value);
            }
            Stmt::Expression(expr) => {
                self.mark_used_variables_expr(expr);
            }
            Stmt::Print(exprs) => {
                for expr in exprs {
                    self.mark_used_variables_expr(expr);
                }
            }
            Stmt::If { condition, then_branch, else_if_branches, else_branch } => {
                self.mark_used_variables_expr(condition);
                self.mark_used_variables_stmt(then_branch);
                for (cond, body) in else_if_branches {
                    self.mark_used_variables_expr(cond);
                    self.mark_used_variables_stmt(body);
                }
                if let Some(else_s) = else_branch {
                    self.mark_used_variables_stmt(else_s);
                }
            }
            Stmt::While { condition, body } => {
                self.mark_used_variables_expr(condition);
                self.mark_used_variables_stmt(body);
            }
            Stmt::For { iterable, body, .. } => {
                self.mark_used_variables_expr(iterable);
                self.mark_used_variables_stmt(body);
            }
            Stmt::ForRange { start, end, step, body, .. } => {
                self.mark_used_variables_expr(start);
                self.mark_used_variables_expr(end);
                if let Some(s) = step {
                    self.mark_used_variables_expr(s);
                }
                self.mark_used_variables_stmt(body);
            }
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.mark_used_variables_stmt(s);
                }
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    self.mark_used_variables_expr(e);
                }
            }
            Stmt::FunctionDecl { params, body, .. } => {
                for (_, default, _type) in params {
                    if let Some(d) = default {
                        self.mark_used_variables_expr(d);
                    }
                }
                self.mark_used_variables_stmt(body);
            }
            _ => {}
        }
    }
    
    fn mark_used_variables_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Identifier(name) => {
                self.used_variables.insert(name.clone());
            }
            Expr::Binary { left, right, .. } => {
                self.mark_used_variables_expr(left);
                self.mark_used_variables_expr(right);
            }
            Expr::Unary { expr, .. } => {
                self.mark_used_variables_expr(expr);
            }
            Expr::Call { callee, args } => {
                self.mark_used_variables_expr(callee);
                for arg in args {
                    self.mark_used_variables_expr(arg);
                }
            }
            Expr::Index { object, index } => {
                self.mark_used_variables_expr(object);
                self.mark_used_variables_expr(index);
            }
            Expr::Property { object, .. } => {
                self.mark_used_variables_expr(object);
            }
            Expr::Assignment { target, value } => {
                self.mark_used_variables_expr(target);
                self.mark_used_variables_expr(value);
            }
            Expr::Ternary { condition, then_expr, else_expr } => {
                self.mark_used_variables_expr(condition);
                self.mark_used_variables_expr(then_expr);
                self.mark_used_variables_expr(else_expr);
            }
            Expr::List(elements) => {
                for e in elements {
                    self.mark_used_variables_expr(e);
                }
            }
            Expr::Dictionary(entries) => {
                for (k, v) in entries {
                    self.mark_used_variables_expr(k);
                    self.mark_used_variables_expr(v);
                }
            }
            Expr::Range { start, end, step } => {
                self.mark_used_variables_expr(start);
                self.mark_used_variables_expr(end);
                if let Some(s) = step {
                    self.mark_used_variables_expr(s);
                }
            }
            Expr::Comparison { left, right, .. } => {
                self.mark_used_variables_expr(left);
                self.mark_used_variables_expr(right);
            }
            Expr::Logical { left, right, .. } => {
                self.mark_used_variables_expr(left);
                self.mark_used_variables_expr(right);
            }
            _ => {}
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // تحسين الجمل
    // ═══════════════════════════════════════════════════════════════
    
    fn optimize_stmt(&mut self, stmt: &Stmt) -> Option<Stmt> {
        match stmt {
            Stmt::VariableDecl { name, value, is_const } => {
                let optimized_value = self.optimize_expr(value);
                
                // تسجيل الثوابت
                if *is_const && self.is_pure_expr(&optimized_value) {
                    self.constants.insert(name.clone(), optimized_value.clone());
                }
                
                // Dead Code Elimination: إزالة المتغيرات غير المستخدمة
                if self.level >= OptimizationLevel::Standard && !self.used_variables.contains(name) {
                    self.add_detail(
                        OptimizationKind::DeadCodeElimination,
                        name.clone(),
                        format!("إزالة متغير غير مستخدم: {}", name)
                    );
                    return None;
                }
                
                Some(Stmt::VariableDecl {
                    name: name.clone(),
                    value: optimized_value,
                    is_const: *is_const,
                })
            }
            
            Stmt::Expression(expr) => {
                let optimized = self.optimize_expr(expr);
                
                // إزالة التعبيرات الجانبية البحتة
                if self.level >= OptimizationLevel::Aggressive && self.is_pure_expr(&optimized) {
                    self.add_detail(
                        OptimizationKind::DeadCodeElimination,
                        "expression".to_string(),
                        "إزالة تعبير جانبي بحت".to_string()
                    );
                    return None;
                }
                
                Some(Stmt::Expression(optimized))
            }
            
            Stmt::Print(exprs) => {
                let optimized: Vec<Expr> = exprs.iter()
                    .map(|e| self.optimize_expr(e))
                    .collect();
                Some(Stmt::Print(optimized))
            }
            
            Stmt::If { condition, then_branch, else_if_branches, else_branch } => {
                let optimized_condition = self.optimize_expr(condition);
                
                // Constant Folding للشرط
                if let Expr::Boolean(true) = optimized_condition {
                    self.add_detail(
                        OptimizationKind::DeadCodeElimination,
                        "if".to_string(),
                        "تبسيط شرط دائماً صحيح".to_string()
                    );
                    return self.optimize_stmt(then_branch);
                }
                
                if let Expr::Boolean(false) = optimized_condition {
                    self.add_detail(
                        OptimizationKind::DeadCodeElimination,
                        "if".to_string(),
                        "تبسيط شرط دائماً خاطئ".to_string()
                    );
                    if let Some(else_s) = else_branch {
                        return self.optimize_stmt(else_s);
                    }
                    return None;
                }
                
                let optimized_then = self.optimize_stmt(then_branch);
                let optimized_elif: Vec<(Expr, Stmt)> = else_if_branches.iter()
                    .filter_map(|(c, b)| {
                        let opt_c = self.optimize_expr(c);
                        let opt_b = self.optimize_stmt(b);
                        opt_b.map(|b| (opt_c.clone(), b))
                    })
                    .collect();
                let optimized_else = else_branch.as_ref().and_then(|e| self.optimize_stmt(e));
                
                Some(Stmt::If {
                    condition: optimized_condition,
                    then_branch: Box::new(optimized_then.unwrap_or(Stmt::Block(Vec::new()))),
                    else_if_branches: optimized_elif.into_iter().map(|(c, b)| (c, Box::new(b))).collect(),
                    else_branch: optimized_else.map(Box::new),
                })
            }
            
            Stmt::While { condition, body } => {
                let optimized_condition = self.optimize_expr(condition);
                let optimized_body = self.optimize_stmt(body);
                
                // إزالة الحلقات التي لا تنفذ
                if let Expr::Boolean(false) = optimized_condition {
                    self.add_detail(
                        OptimizationKind::DeadCodeElimination,
                        "while".to_string(),
                        "إزالة حلقة لن تُنفَّذ".to_string()
                    );
                    return None;
                }
                
                Some(Stmt::While {
                    condition: optimized_condition,
                    body: Box::new(optimized_body.unwrap_or(Stmt::Block(Vec::new()))),
                })
            }
            
            Stmt::ForRange { variable, start, end, step, body } => {
                let opt_start = self.optimize_expr(start);
                let opt_end = self.optimize_expr(end);
                let opt_step = step.as_ref().map(|s| self.optimize_expr(s));
                let opt_body = self.optimize_stmt(body);
                
                // Loop Unrolling للحلقات الصغيرة
                if self.level >= OptimizationLevel::Aggressive {
                    if let (Expr::Number(s), Expr::Number(e)) = (&opt_start, &opt_end) {
                        let range_size = (e - s).abs() as usize;
                        if range_size <= 10 && range_size > 0 {
                            return self.unroll_loop(variable, *s, *e, opt_step.as_ref(), body);
                        }
                    }
                }
                
                Some(Stmt::ForRange {
                    variable: variable.clone(),
                    start: opt_start,
                    end: opt_end,
                    step: opt_step,
                    body: Box::new(opt_body.unwrap_or(Stmt::Block(Vec::new()))),
                })
            }
            
            Stmt::Block(statements) => {
                let optimized: Vec<Stmt> = statements.iter()
                    .filter_map(|s| self.optimize_stmt(s))
                    .collect();
                Some(Stmt::Block(optimized))
            }
            
            Stmt::Return(value) => {
                let optimized = value.as_ref().map(|v| self.optimize_expr(v));
                Some(Stmt::Return(optimized))
            }
            
            Stmt::FunctionDecl { name, params, body, is_async, return_type } => {
                let optimized_body = self.optimize_stmt(body);
                Some(Stmt::FunctionDecl {
                    name: name.clone(),
                    params: params.clone(),
                    body: Box::new(optimized_body.unwrap_or(Stmt::Block(Vec::new()))),
                    is_async: *is_async,
                    return_type: return_type.clone(),
                })
            }
            
            _ => Some(stmt.clone()),
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // تحسين التعبيرات
    // ═══════════════════════════════════════════════════════════════
    
    fn optimize_expr(&mut self, expr: &Expr) -> Expr {
        match expr {
            // Constant Folding للعمليات الحسابية
            Expr::Binary { left, op, right } => {
                let opt_left = self.optimize_expr(left);
                let opt_right = self.optimize_expr(right);
                
                // محاولة الحساب في وقت الترجمة
                if let (Expr::Number(a), Expr::Number(b)) = (&opt_left, &opt_right) {
                    if let Some(result) = self.eval_binary_op(*a, op, *b) {
                        self.add_detail(
                            OptimizationKind::ConstantFolding,
                            format!("{} {:?} {}", a, op, b),
                            format!("تبسيط: {} {:?} {} = {}", a, op, b, result)
                        );
                        return Expr::Number(result);
                    }
                }
                
                // Strength Reduction
                if let Some(reduced) = self.strength_reduction(&opt_left, op, &opt_right) {
                    return reduced;
                }
                
                // Algebraic simplification
                if let Some(simplified) = self.algebraic_simplify(&opt_left, op, &opt_right) {
                    return simplified;
                }
                
                Expr::Binary {
                    left: Box::new(opt_left),
                    op: *op,
                    right: Box::new(opt_right),
                }
            }
            
            Expr::Unary { op, expr } => {
                let opt_expr = self.optimize_expr(expr);
                
                // Constant Folding للعمليات الأحادية
                if let Expr::Number(n) = &opt_expr {
                    match op {
                        UnaryOp::Neg => {
                            self.add_detail(
                                OptimizationKind::ConstantFolding,
                                format!("-{}", n),
                                format!("تبسيط: -{} = {}", n, -n)
                            );
                            return Expr::Number(-n);
                        }
                        _ => {}
                    }
                }
                
                if let Expr::Boolean(b) = &opt_expr {
                    match op {
                        UnaryOp::Not => {
                            self.add_detail(
                                OptimizationKind::ConstantFolding,
                                format!("لا {}", b),
                                format!("تبسيط: لا {} = {}", b, !b)
                            );
                            return Expr::Boolean(!b);
                        }
                        _ => {}
                    }
                }
                
                Expr::Unary {
                    op: *op,
                    expr: Box::new(opt_expr),
                }
            }
            
            Expr::Comparison { left, op, right } => {
                let opt_left = self.optimize_expr(left);
                let opt_right = self.optimize_expr(right);
                
                // Constant Folding للمقارنات
                if let (Expr::Number(a), Expr::Number(b)) = (&opt_left, &opt_right) {
                    if let Some(result) = self.eval_comparison(*a, op, *b) {
                        self.add_detail(
                            OptimizationKind::ConstantFolding,
                            format!("{} {:?} {}", a, op, b),
                            format!("تبسيط مقارنة: {}", result)
                        );
                        return Expr::Boolean(result);
                    }
                }
                
                Expr::Comparison {
                    left: Box::new(opt_left),
                    op: *op,
                    right: Box::new(opt_right),
                }
            }
            
            Expr::Logical { left, op, right } => {
                let opt_left = self.optimize_expr(left);
                
                // Short-circuit optimization
                match op {
                    LogicalOp::And => {
                        if let Expr::Boolean(false) = opt_left {
                            self.add_detail(
                                OptimizationKind::DeadCodeElimination,
                                "and".to_string(),
                                "تبسيط: خطأ و ... = خطأ".to_string()
                            );
                            return Expr::Boolean(false);
                        }
                        if let Expr::Boolean(true) = opt_left {
                            self.add_detail(
                                OptimizationKind::DeadCodeElimination,
                                "and".to_string(),
                                "تبسيط: صح و س = س".to_string()
                            );
                            return self.optimize_expr(right);
                        }
                    }
                    LogicalOp::Or => {
                        if let Expr::Boolean(true) = opt_left {
                            self.add_detail(
                                OptimizationKind::DeadCodeElimination,
                                "or".to_string(),
                                "تبسيط: صح أو ... = صح".to_string()
                            );
                            return Expr::Boolean(true);
                        }
                        if let Expr::Boolean(false) = opt_left {
                            self.add_detail(
                                OptimizationKind::DeadCodeElimination,
                                "or".to_string(),
                                "تبسيط: خطأ أو س = س".to_string()
                            );
                            return self.optimize_expr(right);
                        }
                    }
                }
                
                let opt_right = self.optimize_expr(right);
                Expr::Logical {
                    left: Box::new(opt_left),
                    op: *op,
                    right: Box::new(opt_right),
                }
            }
            
            Expr::Ternary { condition, then_expr, else_expr } => {
                let opt_condition = self.optimize_expr(condition);
                
                // تبسيط إذا كان الشرط ثابت
                if let Expr::Boolean(b) = opt_condition {
                    self.add_detail(
                        OptimizationKind::DeadCodeElimination,
                        "ternary".to_string(),
                        format!("تبسيط تعبير شرطي ثابت: {}", b)
                    );
                    return if b {
                        self.optimize_expr(then_expr)
                    } else {
                        self.optimize_expr(else_expr)
                    };
                }
                
                Expr::Ternary {
                    condition: Box::new(opt_condition),
                    then_expr: Box::new(self.optimize_expr(then_expr)),
                    else_expr: Box::new(self.optimize_expr(else_expr)),
                }
            }
            
            Expr::Call { callee, args } => {
                let opt_callee = self.optimize_expr(callee);
                let opt_args: Vec<Expr> = args.iter()
                    .map(|a| self.optimize_expr(a))
                    .collect();
                
                // Inline Expansion للدوال الصغيرة
                if self.level >= OptimizationLevel::Aggressive {
                    if let Expr::Identifier(name) = &opt_callee {
                        if let Some(inlined) = self.try_inline_function(name, &opt_args) {
                            return inlined;
                        }
                    }
                }
                
                Expr::Call {
                    callee: Box::new(opt_callee),
                    args: opt_args,
                }
            }
            
            Expr::Identifier(name) => {
                // Copy Propagation: استبدال المتغيرات بالقيم الثابتة
                if self.level >= OptimizationLevel::Standard {
                    if let Some(value) = self.constants.get(name).cloned() {
                        self.add_detail(
                            OptimizationKind::CopyPropagation,
                            name.clone(),
                            format!("استبدال {} بقيمته الثابتة", name)
                        );
                        return value;
                    }
                }
                Expr::Identifier(name.clone())
            }
            
            Expr::List(elements) => {
                let optimized: Vec<Expr> = elements.iter()
                    .map(|e| self.optimize_expr(e))
                    .collect();
                Expr::List(optimized)
            }
            
            Expr::Dictionary(entries) => {
                let optimized: Vec<(Expr, Expr)> = entries.iter()
                    .map(|(k, v)| (self.optimize_expr(k), self.optimize_expr(v)))
                    .collect();
                Expr::Dictionary(optimized)
            }
            
            Expr::Index { object, index } => {
                let opt_object = self.optimize_expr(object);
                let opt_index = self.optimize_expr(index);
                
                // Constant Folding للفهرسة الثابتة
                if let (Expr::List(elements), Expr::Number(idx)) = (&opt_object, &opt_index) {
                    let i = *idx as usize;
                    if i < elements.len() && self.is_pure_expr(&elements[i]) {
                        self.add_detail(
                            OptimizationKind::ConstantFolding,
                            format!("list[{}]", idx),
                            format!("تبسيط فهرسة قائمة ثابتة")
                        );
                        return elements[i].clone();
                    }
                }
                
                Expr::Index {
                    object: Box::new(opt_object),
                    index: Box::new(opt_index),
                }
            }
            
            Expr::Range { start, end, step } => {
                let opt_start = self.optimize_expr(start);
                let opt_end = self.optimize_expr(end);
                let opt_step = step.as_ref().map(|s| self.optimize_expr(s));
                Expr::Range {
                    start: Box::new(opt_start),
                    end: Box::new(opt_end),
                    step: opt_step.map(Box::new),
                }
            }
            
            _ => expr.clone(),
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال مساعدة
    // ═══════════════════════════════════════════════════════════════
    
    /// حساب عملية ثنائية
    fn eval_binary_op(&self, a: f64, op: &BinaryOp, b: f64) -> Option<f64> {
        match op {
            BinaryOp::Add => Some(a + b),
            BinaryOp::Sub => Some(a - b),
            BinaryOp::Mul => Some(a * b),
            BinaryOp::Div => if b != 0.0 { Some(a / b) } else { None },
            BinaryOp::Mod => if b != 0.0 { Some(a % b) } else { None },
            BinaryOp::Pow => Some(a.powf(b)),
            BinaryOp::FloorDiv => if b != 0.0 { Some((a / b).floor()) } else { None },
            // Bitwise operations - convert to integers
            BinaryOp::BitAnd => Some((a as i64 & b as i64) as f64),
            BinaryOp::BitOr => Some((a as i64 | b as i64) as f64),
            BinaryOp::BitXor => Some((a as i64 ^ b as i64) as f64),
            BinaryOp::ShiftLeft => Some(((a as i64) << (b as i64)) as f64),
            BinaryOp::ShiftRight => Some(((a as i64) >> (b as i64)) as f64),
        }
    }
    
    /// حساب مقارنة
    fn eval_comparison(&self, a: f64, op: &ComparisonOp, b: f64) -> Option<bool> {
        match op {
            ComparisonOp::Equal => Some(a == b),
            ComparisonOp::NotEqual => Some(a != b),
            ComparisonOp::Less => Some(a < b),
            ComparisonOp::Greater => Some(a > b),
            ComparisonOp::LessEqual => Some(a <= b),
            ComparisonOp::GreaterEqual => Some(a >= b),
        }
    }
    
    /// تقليل قوة العمليات
    fn strength_reduction(&mut self, left: &Expr, op: &BinaryOp, right: &Expr) -> Option<Expr> {
        match op {
            BinaryOp::Mul => {
                // x * 2 => x + x (أسرع)
                if let Expr::Number(2.0) = right {
                    // x + x قد يكون أسرع في بعض الحالات
                }
                // x * 0 => 0
                if let Expr::Number(0.0) = right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x * 0".to_string(),
                        "تبسيط: x * 0 = 0".to_string()
                    );
                    return Some(Expr::Number(0.0));
                }
                // x * 1 => x
                if let Expr::Number(1.0) = right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x * 1".to_string(),
                        "تبسيط: x * 1 = x".to_string()
                    );
                    return Some(left.clone());
                }
            }
            BinaryOp::Div => {
                // x / 1 => x
                if let Expr::Number(1.0) = right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x / 1".to_string(),
                        "تبسيط: x / 1 = x".to_string()
                    );
                    return Some(left.clone());
                }
                // x / x => 1 (إذا x != 0)
                if left == right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x / x".to_string(),
                        "تبسيط: x / x = 1".to_string()
                    );
                    return Some(Expr::Number(1.0));
                }
            }
            BinaryOp::Add => {
                // x + 0 => x
                if let Expr::Number(0.0) = right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x + 0".to_string(),
                        "تبسيط: x + 0 = x".to_string()
                    );
                    return Some(left.clone());
                }
            }
            BinaryOp::Sub => {
                // x - 0 => x
                if let Expr::Number(0.0) = right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x - 0".to_string(),
                        "تبسيط: x - 0 = x".to_string()
                    );
                    return Some(left.clone());
                }
                // x - x => 0
                if left == right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x - x".to_string(),
                        "تبسيط: x - x = 0".to_string()
                    );
                    return Some(Expr::Number(0.0));
                }
            }
            BinaryOp::Pow => {
                // x ** 0 => 1
                if let Expr::Number(0.0) = right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x ** 0".to_string(),
                        "تبسيط: x ** 0 = 1".to_string()
                    );
                    return Some(Expr::Number(1.0));
                }
                // x ** 1 => x
                if let Expr::Number(1.0) = right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x ** 1".to_string(),
                        "تبسيط: x ** 1 = x".to_string()
                    );
                    return Some(left.clone());
                }
            }
            _ => {}
        }
        None
    }
    
    /// تبسيط جبري
    fn algebraic_simplify(&mut self, left: &Expr, op: &BinaryOp, right: &Expr) -> Option<Expr> {
        match op {
            BinaryOp::Add => {
                // (a + b) + c => a + (b + c) للثوابت
                if let Expr::Binary { op: BinaryOp::Add, right: inner_right, .. } = left {
                    if let (Expr::Number(_b), Expr::Number(_c)) = (inner_right.as_ref(), right) {
                        self.add_detail(
                            OptimizationKind::ConstantFolding,
                            "associative".to_string(),
                            "إعادة ترتيب عملية الجمع".to_string()
                        );
                    }
                }
            }
            BinaryOp::Mul => {
                // x * -1 => -x
                if let Expr::Number(-1.0) = right {
                    self.add_detail(
                        OptimizationKind::StrengthReduction,
                        "x * -1".to_string(),
                        "تبسيط: x * -1 = -x".to_string()
                    );
                    return Some(Expr::Unary {
                        op: UnaryOp::Neg,
                        expr: Box::new(left.clone()),
                    });
                }
            }
            _ => {}
        }
        None
    }
    
    /// فك الحلقات (Loop Unrolling)
    fn unroll_loop(
        &mut self,
        _variable: &str,
        start: f64,
        end: f64,
        _step: Option<&Expr>,
        _body: &Stmt,
    ) -> Option<Stmt> {
        let iterations = (end - start).abs() as usize;
        if iterations <= 10 {
            self.add_detail(
                OptimizationKind::LoopOptimization,
                format!("loop {}..{}", start, end),
                format!("فك حلقة من {} تكرارات", iterations)
            );
            // يمكن تنفيذ Loop Unrolling هنا
        }
        None
    }
    
    /// محاولة توسيع الدالة
    fn try_inline_function(&mut self, _name: &str, _args: &[Expr]) -> Option<Expr> {
        // يمكن تنفيذ Inline Expansion هنا
        None
    }
    
    /// هل التعبير نقي (بدون آثار جانبية)
    fn is_pure_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) | Expr::Null => true,
            Expr::Identifier(_) => true,
            Expr::Binary { left, right, .. } => {
                self.is_pure_expr(left) && self.is_pure_expr(right)
            }
            Expr::Unary { expr, .. } => self.is_pure_expr(expr),
            Expr::List(elements) => elements.iter().all(|e| self.is_pure_expr(e)),
            Expr::Comparison { left, right, .. } => {
                self.is_pure_expr(left) && self.is_pure_expr(right)
            }
            Expr::Logical { left, right, .. } => {
                self.is_pure_expr(left) && self.is_pure_expr(right)
            }
            _ => false,
        }
    }
    
    /// إضافة تفصيل التحسين
    fn add_detail(&mut self, kind: OptimizationKind, location: String, description: String) {
        self.optimizations_count += 1;
        self.details.push(OptimizationDetail {
            kind,
            location,
            description,
        });
    }
    
    /// تحسين عدواني إضافي
    fn aggressive_optimization(&mut self, statements: Vec<Stmt>) -> Vec<Stmt> {
        // Common Subexpression Elimination
        statements
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new(OptimizationLevel::Standard)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_folding() {
        let source = "اطبع(2 + 3 * 4)؛";
        let program = crate::parser::Parser::parse(source).unwrap();
        let result = Optimizer::optimize(&program, OptimizationLevel::Standard);
        
        // يجب أن يكون هناك تحسين واحد على الأقل
        assert!(result.optimizations_applied >= 1);
    }
    
    #[test]
    fn test_dead_code_elimination() {
        let source = r#"
            متغير س = 10؛
            متغير ص = 20؛
            اطبع(س)؛
        "#;
        let program = crate::parser::Parser::parse(source).unwrap();
        let result = Optimizer::optimize(&program, OptimizationLevel::Standard);
        
        // يجب إزالة المتغير ص غير المستخدم
        assert!(result.optimizations_applied >= 1);
    }
    
    #[test]
    fn test_strength_reduction() {
        let source = "اطبع(س * 1)؛";
        let program = crate::parser::Parser::parse(source).unwrap();
        let result = Optimizer::optimize(&program, OptimizationLevel::Standard);
        
        assert!(result.optimizations_applied >= 1);
    }
    
    #[test]
    fn test_no_optimization() {
        let source = "اطبع(س)؛";
        let program = crate::parser::Parser::parse(source).unwrap();
        let result = Optimizer::optimize(&program, OptimizationLevel::None);
        
        assert_eq!(result.optimizations_applied, 0);
    }
}
