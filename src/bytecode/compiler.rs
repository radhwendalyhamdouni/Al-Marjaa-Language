// ═══════════════════════════════════════════════════════════════════════════════
// مترجم الـ Bytecode - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// يحول AST إلى تعليمات bytecode للآلة الافتراضية
// ═══════════════════════════════════════════════════════════════════════════════

use super::opcodes::{Chunk, OpCode};
use crate::parser::ast::{BinaryOp, ComparisonOp, Expr, LogicalOp, Program, Stmt, UnaryOp};

/// نتيجة الترجمة
#[derive(Debug)]
pub struct CompileResult {
    /// الـ chunk المترجم
    pub chunk: Chunk,
    /// الأخطاء إن وجدت
    pub errors: Vec<String>,
}

/// جدول الرموز المحلي
#[derive(Clone, Debug)]
struct LocalVar {
    name: String,
    slot: u16,
    /// هل المتغير ثابت (محجوز للاستخدام المستقبلي)
    _is_const: bool,
}

/// نطاق محلي
#[derive(Clone, Debug)]
struct Scope {
    locals: Vec<LocalVar>,
    /// عمق النطاق (محجوز للاستخدام المستقبلي)
    _depth: usize,
}

/// المترجم
pub struct Compiler {
    /// الـ chunk الناتج
    chunk: Chunk,
    /// الأخطاء
    errors: Vec<String>,
    /// النطاقات
    scopes: Vec<Scope>,
    /// العمق الحالي
    scope_depth: usize,
    /// الفتحة المحلية التالية
    next_local_slot: u16,
}

impl Compiler {
    /// إنشاء مترجم جديد
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            errors: Vec::new(),
            scopes: vec![Scope {
                locals: Vec::new(),
                _depth: 0,
            }],
            scope_depth: 0,
            next_local_slot: 0,
        }
    }
    
    /// ترجمة برنامج كامل
    pub fn compile(program: &Program) -> CompileResult {
        let mut compiler = Compiler::new();
        compiler.compile_program(program);
        
        CompileResult {
            chunk: compiler.chunk,
            errors: compiler.errors,
        }
    }
    
    /// ترجمة من كود مصدري
    pub fn compile_source(source: &str) -> Result<Chunk, String> {
        let program = crate::parser::Parser::parse(source)
            .map_err(|e| e.message)?;
        
        let result = Self::compile(&program);
        
        if !result.errors.is_empty() {
            return Err(result.errors.join("\n"));
        }
        
        Ok(result.chunk)
    }
    
    // ═══════════════════════════════════════════════════════════════
    // ترجمة البرنامج
    // ═══════════════════════════════════════════════════════════════
    
    fn compile_program(&mut self, program: &Program) {
        let last_idx = program.statements.len().saturating_sub(1);
        
        for (i, stmt) in program.statements.iter().enumerate() {
            // للجملة الأخيرة، نستخدم compile_stmt_last
            if i == last_idx {
                self.compile_stmt_last(stmt);
            } else {
                self.compile_stmt(stmt);
            }
        }
        self.emit(OpCode::Halt);
    }
    
    // ═══════════════════════════════════════════════════════════════
    // ترجمة الجمل
    // ═══════════════════════════════════════════════════════════════
    
    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VariableDecl { name, value, is_const } => {
                self.compile_expr(value);
                
                if self.scope_depth == 0 {
                    // متغير عام
                    let idx = self.chunk.add_string(name);
                    if *is_const {
                        self.emit(OpCode::DefineConst(idx));
                    } else {
                        self.emit(OpCode::StoreGlobal(idx));
                    }
                } else {
                    // متغير محلي
                    self.declare_local(name, *is_const);
                }
            }
            
            Stmt::Expression(expr) => {
                self.compile_expr(expr);
                self.emit(OpCode::Pop);
            }
            
            Stmt::Print(exprs) => {
                for expr in exprs {
                    self.compile_expr(expr);
                }
                if !exprs.is_empty() {
                    self.emit(OpCode::Print);
                }
            }
            
            Stmt::If { condition, then_branch, else_if_branches, else_branch } => {
                self.compile_expr(condition);
                let then_jump = self.emit_placeholder(OpCode::JumpIfFalse(0));
                self.compile_stmt(then_branch);
                let mut end_jumps = vec![self.emit_placeholder(OpCode::Jump(0))];
                
                for (elif_cond, elif_body) in else_if_branches {
                    self.patch_jump(then_jump);
                    self.compile_expr(elif_cond);
                    let _next_jump = self.emit_placeholder(OpCode::JumpIfFalse(0));
                    self.compile_stmt(elif_body);
                    end_jumps.push(self.emit_placeholder(OpCode::Jump(0)));
                }
                
                if let Some(else_stmt) = else_branch {
                    self.patch_jump(then_jump);
                    self.compile_stmt(else_stmt);
                }
                
                let end = self.chunk.len();
                for jump in end_jumps {
                    self.patch_jump_to(jump, end);
                }
            }
            
            Stmt::While { condition, body } => {
                let loop_start = self.chunk.len();
                self.compile_expr(condition);
                let exit_jump = self.emit_placeholder(OpCode::JumpIfFalse(0));
                self.compile_stmt(body);
                // القفز للخلف: من الموضع الحالي إلى loop_start
                // offset = current - loop_start + 1 (لتعويض الزيادة في step)
                self.emit(OpCode::JumpBack((self.chunk.len() - loop_start) as i32));
                self.patch_jump(exit_jump);
            }
            
            Stmt::For { variable, iterable, body } => {
                self.begin_scope();
                self.compile_expr(iterable);
                let _var_slot = self.next_local_slot;
                self.declare_local(variable, false);
                // TODO: تنفيذ حلقة for بشكل كامل
                self.compile_stmt(body);
                self.end_scope();
            }
            
            Stmt::ForRange { variable, start, end, step, body } => {
                self.begin_scope();
                self.compile_expr(start);
                let var_slot = self.declare_local(variable, false);
                
                let loop_start = self.chunk.len();
                self.emit(OpCode::LoadLocal(var_slot));
                self.compile_expr(end);
                
                let exit_jump = if step.is_some() {
                    self.emit(OpCode::Less); // مبسط
                    self.emit_placeholder(OpCode::JumpIfFalse(0))
                } else {
                    self.emit(OpCode::Less);
                    self.emit_placeholder(OpCode::JumpIfFalse(0))
                };
                
                self.compile_stmt(body);
                
                self.emit(OpCode::LoadLocal(var_slot));
                self.emit(OpCode::PushNumber(1.0));
                self.emit(OpCode::Add);
                self.emit(OpCode::StoreLocal(var_slot));
                self.emit(OpCode::JumpBack((self.chunk.len() - loop_start) as i32));
                
                self.patch_jump(exit_jump);
                self.end_scope();
            }
            
            Stmt::Return(value) => {
                if let Some(expr) = value {
                    self.compile_expr(expr);
                    self.emit(OpCode::ReturnValue);
                } else {
                    self.emit(OpCode::Return);
                }
            }
            
            Stmt::Break => { self.emit(OpCode::Break); }
            Stmt::Continue => { self.emit(OpCode::Continue); }
            
            Stmt::Block(statements) => {
                self.begin_scope();
                for s in statements {
                    self.compile_stmt(s);
                }
                self.end_scope();
            }
            
            Stmt::Throw(expr) => {
                self.compile_expr(expr);
                self.emit(OpCode::Throw);
            }
            
            _ => {}
        }
    }
    
    /// ترجمة الجملة الأخيرة (تبقي القيمة على المكدس)
    fn compile_stmt_last(&mut self, stmt: &Stmt) {
        match stmt {
            // للتعابير، لا نضيف Pop لكي تبقى القيمة على المكدس
            Stmt::Expression(expr) => {
                self.compile_expr(expr);
                // لا نضيف Pop - القيمة تبقى على المكدس
            }
            
            // المتغيرات تعيد قيمة Null
            Stmt::VariableDecl { name, value, is_const } => {
                self.compile_expr(value);
                
                if self.scope_depth == 0 {
                    let idx = self.chunk.add_string(name);
                    if *is_const {
                        self.emit(OpCode::DefineConst(idx));
                    } else {
                        self.emit(OpCode::StoreGlobal(idx));
                    }
                } else {
                    self.declare_local(name, *is_const);
                }
                self.emit(OpCode::PushNull);
            }
            
            // للطباعة، نعيد Null
            Stmt::Print(exprs) => {
                for expr in exprs {
                    self.compile_expr(expr);
                }
                if !exprs.is_empty() {
                    self.emit(OpCode::Print);
                }
                self.emit(OpCode::PushNull);
            }
            
            // الحلقات تعيد Null
            Stmt::While { condition, body } => {
                let loop_start = self.chunk.len();
                self.compile_expr(condition);
                let exit_jump = self.emit_placeholder(OpCode::JumpIfFalse(0));
                self.compile_stmt(body);
                self.emit(OpCode::JumpBack((self.chunk.len() - loop_start + 1) as i32));
                self.patch_jump(exit_jump);
                self.emit(OpCode::PushNull);
            }
            
            // باقي الجمل تستخدم compile_stmt العادي
            _ => self.compile_stmt(stmt),
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // ترجمة التعبيرات
    // ═══════════════════════════════════════════════════════════════
    
    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => {
                self.emit(OpCode::PushNumber(*n));
            }
            
            Expr::String(s) => {
                let idx = self.chunk.add_string(s);
                self.emit(OpCode::PushString(idx));
            }
            
            Expr::Boolean(b) => {
                self.emit(OpCode::PushBool(*b));
            }
            
            Expr::Null => {
                self.emit(OpCode::PushNull);
            }
            
            Expr::Identifier(name) => {
                if let Some(slot) = self.resolve_local(name) {
                    self.emit(OpCode::LoadLocal(slot));
                } else {
                    let idx = self.chunk.add_string(name);
                    self.emit(OpCode::LoadGlobal(idx));
                }
            }
            
            Expr::Binary { left, op, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                
                match op {
                    BinaryOp::Add => self.emit(OpCode::Add),
                    BinaryOp::Sub => self.emit(OpCode::Sub),
                    BinaryOp::Mul => self.emit(OpCode::Mul),
                    BinaryOp::Div => self.emit(OpCode::Div),
                    BinaryOp::Mod => self.emit(OpCode::Mod),
                    BinaryOp::Pow => self.emit(OpCode::Pow),
                    BinaryOp::FloorDiv => self.emit(OpCode::Div), // TODO: Add FloorDiv opcode
                    BinaryOp::BitAnd => self.emit(OpCode::And),
                    BinaryOp::BitOr => self.emit(OpCode::Or),
                    BinaryOp::BitXor => self.emit(OpCode::Xor),
                    BinaryOp::ShiftLeft => self.emit(OpCode::Shl),
                    BinaryOp::ShiftRight => self.emit(OpCode::Shr),
                }
            }
            
            Expr::Comparison { left, op, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                
                match op {
                    ComparisonOp::Equal => self.emit(OpCode::Equal),
                    ComparisonOp::NotEqual => self.emit(OpCode::NotEqual),
                    ComparisonOp::Less => self.emit(OpCode::Less),
                    ComparisonOp::Greater => self.emit(OpCode::Greater),
                    ComparisonOp::LessEqual => self.emit(OpCode::LessEqual),
                    ComparisonOp::GreaterEqual => self.emit(OpCode::GreaterEqual),
                }
            }
            
            Expr::Logical { left, op, right } => {
                match op {
                    LogicalOp::And => {
                        self.compile_expr(left);
                        let end_jump = self.emit_placeholder(OpCode::JumpIfFalse(0));
                        self.emit(OpCode::Pop);
                        self.compile_expr(right);
                        self.patch_jump(end_jump);
                    }
                    LogicalOp::Or => {
                        self.compile_expr(left);
                        let else_jump = self.emit_placeholder(OpCode::JumpIfTrue(0));
                        self.emit(OpCode::Pop);
                        self.compile_expr(right);
                        self.patch_jump(else_jump);
                    }
                }
            }
            
            Expr::Unary { op, expr } => {
                self.compile_expr(expr);
                match op {
                    UnaryOp::Neg => self.emit(OpCode::Neg),
                    UnaryOp::Not => self.emit(OpCode::Not),
                    _ => {}
                }
            }
            
            Expr::Ternary { condition, then_expr, else_expr } => {
                self.compile_expr(condition);
                let else_jump = self.emit_placeholder(OpCode::JumpIfFalse(0));
                self.compile_expr(then_expr);
                let end_jump = self.emit_placeholder(OpCode::Jump(0));
                self.patch_jump(else_jump);
                self.compile_expr(else_expr);
                self.patch_jump(end_jump);
            }
            
            Expr::Call { callee, args } => {
                self.compile_expr(callee);
                for arg in args {
                    self.compile_expr(arg);
                }
                self.emit(OpCode::Call(args.len() as u8));
            }
            
            Expr::List(elements) => {
                for elem in elements {
                    self.compile_expr(elem);
                }
                self.emit(OpCode::BuildList(elements.len() as u16));
            }
            
            Expr::Dictionary(entries) => {
                for (key, value) in entries {
                    self.compile_expr(key);
                    self.compile_expr(value);
                }
                self.emit(OpCode::BuildDict(entries.len() as u16));
            }
            
            Expr::Index { object, index } => {
                self.compile_expr(object);
                self.compile_expr(index);
                self.emit(OpCode::Index);
            }
            
            Expr::Property { object, property } => {
                self.compile_expr(object);
                let idx = self.chunk.add_string(property);
                self.emit(OpCode::GetProperty(idx));
            }
            
            Expr::Assignment { target, value } => {
                self.compile_expr(value);
                
                match target.as_ref() {
                    Expr::Identifier(name) => {
                        if let Some(slot) = self.resolve_local(name) {
                            self.emit(OpCode::StoreLocal(slot));
                        } else {
                            let idx = self.chunk.add_string(name);
                            self.emit(OpCode::StoreGlobal(idx));
                        }
                    }
                    Expr::Index { object, index } => {
                        self.compile_expr(object);
                        self.compile_expr(index);
                        self.emit(OpCode::IndexSet);
                    }
                    Expr::Property { object, property } => {
                        self.compile_expr(object);
                        let idx = self.chunk.add_string(property);
                        self.emit(OpCode::SetProperty(idx));
                    }
                    _ => {
                        self.errors.push("هدف الإسناد غير صالح".into());
                    }
                }
            }
            
            Expr::Range { start, end, step } => {
                self.compile_expr(start);
                self.compile_expr(end);
                if step.is_some() {
                    if let Some(s) = step {
                        self.compile_expr(s);
                    }
                    self.emit(OpCode::RangeStep);
                } else {
                    self.emit(OpCode::Range);
                }
            }
            
            _ => {
                self.emit(OpCode::PushNull);
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // إدارة النطاقات
    // ═══════════════════════════════════════════════════════════════
    
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
        self.scopes.push(Scope {
            locals: Vec::new(),
            _depth: self.scope_depth,
        });
    }
    
    fn end_scope(&mut self) {
        if self.scope_depth > 0 {
            self.scope_depth -= 1;
            let scope = self.scopes.pop().unwrap();
            self.next_local_slot -= scope.locals.len() as u16;
        }
    }
    
    fn declare_local(&mut self, name: &str, is_const: bool) -> u16 {
        let slot = self.next_local_slot;
        self.next_local_slot += 1;
        
        if let Some(scope) = self.scopes.last_mut() {
            scope.locals.push(LocalVar {
                name: name.to_string(),
                slot,
                _is_const: is_const,
            });
        }
        
        self.emit(OpCode::StoreLocal(slot));
        slot
    }
    
    fn resolve_local(&self, name: &str) -> Option<u16> {
        for scope in self.scopes.iter().rev() {
            for local in &scope.locals {
                if local.name == name {
                    return Some(local.slot);
                }
            }
        }
        None
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال مساعدة للإصدار
    // ═══════════════════════════════════════════════════════════════
    
    fn emit(&mut self, opcode: OpCode) {
        self.chunk.emit(opcode);
    }
    
    fn emit_placeholder(&mut self, opcode: OpCode) -> usize {
        let index = self.chunk.len();
        self.chunk.emit(opcode);
        index
    }
    
    fn patch_jump(&mut self, index: usize) {
        let current = self.chunk.len();
        let offset = (current as i32) - (index as i32);
        self.chunk.patch_jump(index, offset);
    }
    
    fn patch_jump_to(&mut self, index: usize, target: usize) {
        let offset = (target as i32) - (index as i32);
        self.chunk.patch_jump(index, offset);
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compile_number() {
        let result = Compiler::compile_source("اطبع(42)؛");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_compile_arithmetic() {
        let result = Compiler::compile_source("اطبع(5 + 3 * 2)؛");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_compile_variables() {
        let result = Compiler::compile_source(r#"
            متغير س = 10؛
            متجر ص = 20؛
            اطبع(س + ص)؛
        "#);
        assert!(result.is_ok());
    }
}
