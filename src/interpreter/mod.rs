mod native_io;
mod native_stdlib;
pub mod value;
pub mod autograd;
pub mod gpu;
pub mod jit;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;

use crate::parser::ast::{
    BinaryOp, ComparisonOp, DestructuringPattern, Expr, FormatPart, LogicalOp, Program, Stmt, UnaryOp,
};
use crate::parser::Parser;
use value::{Environment, SharedValue, Value};

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        {
            let mut env = globals.borrow_mut();
            // الدوال الأساسية
            native_io::define_io(&mut env);
            native_io::define_file_funcs(&mut env);
            native_io::define_system_funcs(&mut env);
            native_io::define_time_funcs(&mut env);
            native_io::define_network_funcs(&mut env);
            native_io::define_hardware_funcs(&mut env);
            
            // الدوال الرياضية والنصية
            native_stdlib::define_math(&mut env);
            native_stdlib::define_string_funcs(&mut env);
            native_stdlib::define_list_funcs(&mut env);
            native_stdlib::define_dict_funcs(&mut env);
            native_stdlib::define_type_funcs(&mut env);
            native_stdlib::define_random_funcs(&mut env);
            native_stdlib::define_json_funcs(&mut env);
            
            // الدوال الجديدة المختصرة
            native_stdlib::define_datetime_funcs(&mut env);
            native_stdlib::define_test_funcs(&mut env);
            native_stdlib::define_utility_funcs(&mut env);
            
            // دوال الذكاء الاصطناعي
            native_stdlib::define_tensor_funcs(&mut env);
            native_stdlib::define_ai_funcs(&mut env);
            native_stdlib::define_matrix_funcs(&mut env);
            native_stdlib::define_gradient_deriv_funcs(&mut env);
            native_stdlib::define_autograd_funcs(&mut env);
            native_stdlib::define_http_funcs(&mut env);
            native_stdlib::define_data_funcs(&mut env);
            
            // دوال التدريب والمحسنات
            native_stdlib::define_optimizer_funcs(&mut env);
            native_stdlib::define_training_funcs(&mut env);
            
            // نظام الوحدات
            native_stdlib::define_module_funcs(&mut env);
            
            // حفظ وتحميل النماذج
            native_stdlib::define_model_io_funcs(&mut env);
            
            // الشبكات العصبية الجاهزة
            native_stdlib::define_neural_network_funcs(&mut env);
            
            // DataLoader
            native_stdlib::define_dataloader_funcs(&mut env);
            
            // Regularization
            native_stdlib::define_regularization_funcs(&mut env);
            
            // GPU Support
            native_stdlib::define_gpu_funcs(&mut env);
            
            // JIT Compilation
            jit::define_jit_funcs(&mut env);
            jit::define_performance_funcs(&mut env);
            
            // تصدير/استيراد النماذج المتقدمة
            native_stdlib::define_advanced_model_io_funcs(&mut env);
            
            // الثوابت
            native_stdlib::define_constants(&mut env);
        }
        Interpreter {
            environment: Rc::clone(&globals),
            globals: Rc::clone(&globals),
        }
    }

    pub fn interpret(&mut self, program: &Program) -> Result<SharedValue, RuntimeError> {
        let mut last_value = Rc::new(RefCell::new(Value::Null));
        for stmt in &program.statements {
            last_value = self.execute_statement(stmt)?;
            match &*last_value.borrow() {
                Value::Return(_) | Value::Break | Value::Continue => break,
                _ => {}
            }
        }
        Ok(last_value)
    }

    pub fn run(&mut self, source: &str) -> Result<SharedValue, RuntimeError> {
        let program = Parser::parse(source).map_err(|e| RuntimeError {
            message: format!("{} (السطر {}، العمود {})", e.message, e.line, e.column),
        })?;
        self.interpret(&program)
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Result<SharedValue, RuntimeError> {
        match stmt {
            Stmt::VariableDecl {
                name,
                value,
                is_const,
            } => {
                let val = self.evaluate_expression(value)?;
                self.environment
                    .borrow_mut()
                    .define(name, (*val.borrow()).clone(), *is_const);
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            // تنفيذ التفكيك: متغير [أ، ب، ج] = قائمة؛ أو متغير {اسم، عمر} = كائن؛
            Stmt::DestructuringDecl {
                pattern,
                value,
                is_const,
            } => {
                let val = self.evaluate_expression(value)?;
                let val_clone = (*val.borrow()).clone();
                
                match pattern {
                    DestructuringPattern::List(names) => {
                        // تفكيك القائمة
                        if let Value::List(list) = val_clone {
                            if names.len() > list.len() {
                                return Err(RuntimeError {
                                    message: format!(
                                        "عدد العناصر ({}) أقل من عدد المتغيرات ({})",
                                        list.len(),
                                        names.len()
                                    ),
                                });
                            }
                            for (name, item) in names.iter().zip(list.iter()) {
                                self.environment
                                    .borrow_mut()
                                    .define(name, (*item.borrow()).clone(), *is_const);
                            }
                        } else {
                            return Err(RuntimeError {
                                message: format!("لا يمكن تفكيك {} كقائمة", val_clone.type_name()),
                            });
                        }
                    }
                    DestructuringPattern::Object(fields) => {
                        // تفكيك الكائن
                        if let Value::Dictionary(dict) = val_clone {
                            for (prop, alias) in fields {
                                let var_name = alias.as_ref().unwrap_or(prop);
                                if let Some(item) = dict.get(prop) {
                                    self.environment
                                        .borrow_mut()
                                        .define(var_name, (*item.borrow()).clone(), *is_const);
                                } else {
                                    return Err(RuntimeError {
                                        message: format!("الخاصية '{}' غير موجودة", prop),
                                    });
                                }
                            }
                        } else if let Value::Instance { fields: inst_fields, .. } = val_clone {
                            for (prop, alias) in fields {
                                let var_name = alias.as_ref().unwrap_or(prop);
                                if let Some(item) = inst_fields.borrow().get(prop) {
                                    self.environment
                                        .borrow_mut()
                                        .define(var_name, (*item.borrow()).clone(), *is_const);
                                } else {
                                    return Err(RuntimeError {
                                        message: format!("الخاصية '{}' غير موجودة", prop),
                                    });
                                }
                            }
                        } else {
                            return Err(RuntimeError {
                                message: format!("لا يمكن تفكيك {} ككائن", val_clone.type_name()),
                            });
                        }
                    }
                    DestructuringPattern::Identifier(name) => {
                        self.environment
                            .borrow_mut()
                            .define(name, val_clone, *is_const);
                    }
                }
                
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::MultiVarDecl {
                names,
                values,
                is_const,
            } => {
                let mut vals = Vec::new();
                for v in values {
                    vals.push(self.evaluate_expression(v)?);
                }
                for (name, val) in names.iter().zip(vals.iter()) {
                    self.environment
                        .borrow_mut()
                        .define(name, (*val.borrow()).clone(), *is_const);
                }
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::Expression(expr) => self.evaluate_expression(expr),

            Stmt::Block(statements) => {
                let previous = Rc::clone(&self.environment);
                self.environment =
                    Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&previous))));
                let mut result = Rc::new(RefCell::new(Value::Null));
                for s in statements {
                    result = self.execute_statement(s)?;
                    match &*result.borrow() {
                        Value::Return(_) | Value::Break | Value::Continue => break,
                        _ => {}
                    }
                }
                self.environment = previous;
                Ok(result)
            }

            Stmt::If {
                condition,
                then_branch,
                else_if_branches,
                else_branch,
            } => {
                let cv = self.evaluate_expression(condition)?;
                if cv.borrow().is_truthy() {
                    return self.execute_statement(then_branch);
                }
                for (elif_cond, elif_body) in else_if_branches {
                    let ec = self.evaluate_expression(elif_cond)?;
                    if ec.borrow().is_truthy() {
                        return self.execute_statement(elif_body);
                    }
                }
                if let Some(else_stmt) = else_branch {
                    self.execute_statement(else_stmt)
                } else {
                    Ok(Rc::new(RefCell::new(Value::Null)))
                }
            }

            Stmt::While { condition, body } => {
                loop {
                    let cond = self.evaluate_expression(condition)?;
                    if !cond.borrow().is_truthy() {
                        break;
                    }
                    let result = self.execute_statement(body)?;
                    let rv = (*result.borrow()).clone();
                    match rv {
                        Value::Return(_) => return Ok(result),
                        Value::Break => break,
                        Value::Continue => continue,
                        _ => {}
                    }
                }
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::For {
                variable,
                iterable,
                body,
            } => {
                let iter_val = self.evaluate_expression(iterable)?;
                let iter_clone = (*iter_val.borrow()).clone();
                let items = self.collect_iterable(iter_clone)?;
                for item in items {
                    self.environment
                        .borrow_mut()
                        .define(variable, (*item.borrow()).clone(), false);
                    let result = self.execute_statement(body)?;
                    let rv = (*result.borrow()).clone();
                    match rv {
                        Value::Return(_) => return Ok(result),
                        Value::Break => break,
                        Value::Continue => continue,
                        _ => {}
                    }
                }
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::ForRange {
                variable,
                start,
                end,
                step,
                body,
            } => {
                let start_val = self.evaluate_expression(start)?;
                let s = start_val
                    .borrow()
                    .to_number()
                    .map_err(|e| RuntimeError { message: e })?;
                let end_val = self.evaluate_expression(end)?;
                let e = end_val
                    .borrow()
                    .to_number()
                    .map_err(|e| RuntimeError { message: e })?;
                let st = match step {
                    Some(expr) => {
                        let sv = self.evaluate_expression(expr)?;
                        let num = sv
                            .borrow()
                            .to_number()
                            .map_err(|e| RuntimeError { message: e })?;
                        num
                    }
                    None => {
                        if s <= e {
                            1.0
                        } else {
                            -1.0
                        }
                    }
                };
                if st == 0.0 {
                    return Err(RuntimeError {
                        message: "الخطوة لا يمكن أن تكون صفر".to_string(),
                    });
                }
                let mut i = s;
                while (st > 0.0 && i < e) || (st < 0.0 && i > e) {
                    self.environment
                        .borrow_mut()
                        .define(variable, Value::Number(i), false);
                    let result = self.execute_statement(body)?;
                    let rv = (*result.borrow()).clone();
                    match rv {
                        Value::Return(_) => return Ok(result),
                        Value::Break => break,
                        _ => {}
                    }
                    i += st;
                }
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::Repeat { count, body } => {
                let count_val = self.evaluate_expression(count)?;
                let n = count_val
                    .borrow()
                    .to_number()
                    .map_err(|e| RuntimeError { message: e })? as usize;
                for _ in 0..n {
                    let result = self.execute_statement(body)?;
                    let rv = (*result.borrow()).clone();
                    match rv {
                        Value::Return(_) => return Ok(result),
                        Value::Break => break,
                        Value::Continue => continue,
                        _ => {}
                    }
                }
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::FunctionDecl {
                name,
                params,
                body,
                is_async,
                return_type: _, // التعليقات التوضيحية للأنواع لا تؤثر في التنفيذ حالياً
            } => {
                // تحويل المعاملات مع التعليقات التوضيحية إلى صيغة بسيطة
                let simple_params: Vec<(String, Option<Expr>)> = params
                    .iter()
                    .map(|(n, d, _t)| (n.clone(), d.clone()))
                    .collect();
                    
                let func = Value::Function {
                    name: name.clone(),
                    params: simple_params,
                    body: Box::new((**body).clone()),
                    closure: Some(Rc::clone(&self.environment)),
                    is_async: *is_async,
                };
                self.environment.borrow_mut().define(name, func, false);
                Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // تنفيذ جملة أعطِ (Yield) للمولدات
            Stmt::Yield(expr) => {
                let value = self.evaluate_expression(expr)?;
                let val = (*value.borrow()).clone();
                Ok(Rc::new(RefCell::new(Value::Yield(Box::new(val)))))
            }

            Stmt::Return(value) => {
                let return_value = match value {
                    Some(expr) => self.evaluate_expression(expr)?,
                    None => Rc::new(RefCell::new(Value::Null)),
                };
                let ret = (*return_value.borrow()).clone();
                Ok(Rc::new(RefCell::new(Value::Return(Box::new(ret)))))
            }

            Stmt::Break => Ok(Rc::new(RefCell::new(Value::Break))),
            Stmt::Continue => Ok(Rc::new(RefCell::new(Value::Continue))),

            Stmt::Print(exprs) => {
                let mut parts = Vec::new();
                for e in exprs {
                    parts.push(self.evaluate_expression(e)?.borrow().to_string());
                }
                println!("{}", parts.join(" "));
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::Input(_prompt) => {
                if !_prompt.is_empty() {
                    print!("{} ", _prompt);
                    io::stdout().flush().ok();
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap_or(0);
                Ok(Rc::new(RefCell::new(Value::String(
                    input
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string(),
                ))))
            }

            Stmt::TryCatch {
                try_block,
                catch_var,
                catch_block,
                finally_block,
            } => {
                let try_result = self.execute_statement(try_block);
                let result = match try_result {
                    Ok(v) => {
                        let vv = (*v.borrow()).clone();
                        match vv {
                            Value::Error(e) => {
                                if let Some(var) = catch_var {
                                    self.environment.borrow_mut().define(
                                        var,
                                        Value::String(e),
                                        false,
                                    );
                                }
                                self.execute_statement(catch_block)?
                            }
                            _ => v,
                        }
                    }
                    Err(e) => {
                        if let Some(var) = catch_var {
                            self.environment.borrow_mut().define(
                                var,
                                Value::String(e.message.clone()),
                                false,
                            );
                        }
                        self.execute_statement(catch_block)?
                    }
                };
                if let Some(finally) = finally_block {
                    self.execute_statement(finally)?;
                }
                Ok(result)
            }

            Stmt::Throw(expr) => {
                let val = self.evaluate_expression(expr)?;
                let msg = val.borrow().to_string_value();
                Err(RuntimeError { message: msg })
            }

            Stmt::Match {
                value,
                cases,
                default,
            } => {
                let val = self.evaluate_expression(value)?;
                let val_clone = (*val.borrow()).clone();
                for (patterns, body) in cases {
                    for pattern in patterns {
                        let p = self.evaluate_expression(pattern)?;
                        if *p.borrow() == val_clone {
                            return self.execute_statement(body);
                        }
                    }
                }
                if let Some(def) = default {
                    self.execute_statement(def)
                } else {
                    Ok(Rc::new(RefCell::new(Value::Null)))
                }
            }

            Stmt::ClassDecl {
                name,
                parent,
                methods,
                fields,
            } => {
                let mut method_map = HashMap::new();
                for method in methods {
                    if let Stmt::FunctionDecl {
                        name: mname,
                        params,
                        body,
                        is_async,
                        ..
                    } = method
                    {
                        // تحويل params من 3 عناصر إلى عنصرين (بدون type annotation)
                        let params_simplified: Vec<(String, Option<Expr>)> = params
                            .iter()
                            .map(|(name, default, _)| (name.clone(), default.clone()))
                            .collect();
                        method_map.insert(
                            mname.clone(),
                            Value::Function {
                                name: mname.clone(),
                                params: params_simplified,
                                body: Box::new((**body).clone()),
                                closure: Some(Rc::clone(&self.environment)),
                                is_async: *is_async,
                            },
                        );
                    }
                }
                let class = Value::Class {
                    name: name.clone(),
                    parent: parent.clone(),
                    methods: method_map,
                    fields: fields.clone(),
                };
                self.environment.borrow_mut().define(name, class, false);
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::Import {
                path,
                alias,
                items: _,
            } => {
                let name = alias.as_ref().unwrap_or(path);
                self.environment
                    .borrow_mut()
                    .define(name, Value::String(path.clone()), false);
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::Assert { condition, message } => {
                let cond = self.evaluate_expression(condition)?;
                if !cond.borrow().is_truthy() {
                    let msg = match message {
                        Some(m) => self.evaluate_expression(m)?.borrow().to_string_value(),
                        None => "فشل التأكيد".to_string(),
                    };
                    return Err(RuntimeError { message: msg });
                }
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::Delete(name) => {
                self.environment.borrow_mut().delete(name);
                Ok(Rc::new(RefCell::new(Value::Null)))
            }

            Stmt::UiComponentDecl { .. }
            | Stmt::StateDecl { .. }
            | Stmt::ThemeDecl { .. }
            | Stmt::RouteDecl { .. }
            | Stmt::EventHandlerDecl { .. }
            | Stmt::With { .. }
            | Stmt::DataClassDecl { .. }
            | Stmt::EnumDecl { .. }
            | Stmt::Decorated { .. } => Ok(Rc::new(RefCell::new(Value::Null))),
        }
    }

    fn collect_iterable(&self, val: Value) -> Result<Vec<SharedValue>, RuntimeError> {
        match val {
            Value::List(l) => Ok(l),
            Value::String(s) => Ok(s
                .chars()
                .map(|c| Rc::new(RefCell::new(Value::String(c.to_string()))))
                .collect()),
            Value::Dictionary(d) => Ok(d
                .into_keys()
                .map(|k| Rc::new(RefCell::new(Value::String(k))))
                .collect()),
            _ => Err(RuntimeError {
                message: format!("نوع {} لا يمكن تكراره", val.type_name()),
            }),
        }
    }

    fn evaluate_expression(&mut self, expr: &Expr) -> Result<SharedValue, RuntimeError> {
        match expr {
            Expr::Number(n) => Ok(Rc::new(RefCell::new(Value::Number(*n)))),
            Expr::String(s) => Ok(Rc::new(RefCell::new(Value::String(s.clone())))),
            Expr::Boolean(b) => Ok(Rc::new(RefCell::new(Value::Boolean(*b)))),
            Expr::Null => Ok(Rc::new(RefCell::new(Value::Null))),

            Expr::FormatString(parts) => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        FormatPart::Literal(s) => result.push_str(s),
                        FormatPart::Expression(e) => {
                            let v = self.evaluate_expression(e)?;
                            result.push_str(&v.borrow().to_string_value());
                        }
                    }
                }
                Ok(Rc::new(RefCell::new(Value::String(result))))
            }

            Expr::Identifier(name) => match self.environment.borrow().get(name) {
                Some(value) => Ok(value),
                None => Err(RuntimeError {
                    message: format!("'{}' غير معرف", name),
                }),
            },

            Expr::Binary { left, op, right } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let lc = (*l.borrow()).clone();
                let rc = (*r.borrow()).clone();
                self.eval_binary_op(lc, op, rc)
            }

            Expr::Logical { left, op, right } => {
                let lv = self.evaluate_expression(left)?;
                let lc = (*lv.borrow()).clone();
                match op {
                    LogicalOp::And => {
                        if !lc.is_truthy() {
                            return Ok(lv);
                        }
                        self.evaluate_expression(right)
                    }
                    LogicalOp::Or => {
                        if lc.is_truthy() {
                            return Ok(lv);
                        }
                        self.evaluate_expression(right)
                    }
                }
            }

            Expr::Comparison { left, op, right } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let lc = (*l.borrow()).clone();
                let rc = (*r.borrow()).clone();
                self.eval_comparison(lc, op, rc)
            }

            Expr::Unary { op, expr } => {
                let v = self.evaluate_expression(expr)?;
                let vc = (*v.borrow()).clone();
                self.eval_unary_op(op, vc)
            }

            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                let cond = self.evaluate_expression(condition)?;
                if cond.borrow().is_truthy() {
                    self.evaluate_expression(then_expr)
                } else {
                    self.evaluate_expression(else_expr)
                }
            }

            Expr::Call { callee, args } => {
                let func_val = self.evaluate_expression(callee)?;
                let func_clone = (*func_val.borrow()).clone();
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate_expression(arg)?);
                }
                self.call_function(func_clone, arg_values)
            }

            Expr::Property { object, property } => {
                let obj_val = self.evaluate_expression(object)?;
                let obj_clone = (*obj_val.borrow()).clone();
                self.get_property(obj_clone, property)
            }

            Expr::Index { object, index } => {
                let obj_val = self.evaluate_expression(object)?;
                let idx_val = self.evaluate_expression(index)?;
                let obj_clone = (*obj_val.borrow()).clone();
                let idx_clone = (*idx_val.borrow()).clone();
                self.get_index(obj_clone, idx_clone)
            }

            Expr::List(elements) => {
                let mut list = Vec::new();
                for elem in elements {
                    // Check if this is a spread expression
                    if let Expr::Spread(spread_expr) = elem {
                        let val = self.evaluate_expression(spread_expr)?;
                        let borrowed = val.borrow();
                        if let Value::List(inner_list) = &*borrowed {
                            list.extend(inner_list.clone());
                        }
                    } else {
                        let val = self.evaluate_expression(elem)?;
                        list.push(val);
                    }
                }
                Ok(Rc::new(RefCell::new(Value::List(list))))
            }

            Expr::Dictionary(entries) => {
                let mut dict = HashMap::new();
                for (key_expr, value_expr) in entries {
                    let key = self
                        .evaluate_expression(key_expr)?
                        .borrow()
                        .to_string_value();
                    let val = self.evaluate_expression(value_expr)?;
                    dict.insert(key, val);
                }
                Ok(Rc::new(RefCell::new(Value::Dictionary(dict))))
            }

            Expr::Assignment { target, value } => {
                let val = self.evaluate_expression(value)?;
                self.perform_assignment(target, val.clone())?;
                Ok(val)
            }

            Expr::CompoundAssignment { name, op, value } => {
                let current = self
                    .environment
                    .borrow()
                    .get(name)
                    .ok_or_else(|| RuntimeError {
                        message: format!("'{}' غير معرف", name),
                    })?;
                let new_val = self.evaluate_expression(value)?;
                let cc = (*current.borrow()).clone();
                let nc = (*new_val.borrow()).clone();
                let result = self.eval_binary_op(cc, op, nc)?;
                self.environment
                    .borrow_mut()
                    .assign(name, (*result.borrow()).clone())
                    .map_err(|e| RuntimeError { message: e })?;
                Ok(result)
            }

            Expr::Lambda { params, body } => Ok(Rc::new(RefCell::new(Value::Lambda {
                params: params.clone(),
                body: Box::new((**body).clone()),
                closure: Rc::clone(&self.environment),
            }))),

            Expr::Increment {
                name,
                is_prefix,
                delta,
            } => {
                let current = self
                    .environment
                    .borrow()
                    .get(name)
                    .ok_or_else(|| RuntimeError {
                        message: format!("'{}' غير معرف", name),
                    })?;
                let old_num = current
                    .borrow()
                    .to_number()
                    .map_err(|e| RuntimeError { message: e })?;
                let new_num = old_num + delta;
                self.environment
                    .borrow_mut()
                    .assign(name, Value::Number(new_num))
                    .map_err(|e| RuntimeError { message: e })?;
                if *is_prefix {
                    Ok(Rc::new(RefCell::new(Value::Number(new_num))))
                } else {
                    Ok(Rc::new(RefCell::new(Value::Number(old_num))))
                }
            }

            Expr::Range { start, end, step } => {
                let s = self
                    .evaluate_expression(start)?
                    .borrow()
                    .to_number()
                    .map_err(|e| RuntimeError { message: e })?;
                let e = self
                    .evaluate_expression(end)?
                    .borrow()
                    .to_number()
                    .map_err(|e| RuntimeError { message: e })?;
                let st = match step {
                    Some(expr) => self
                        .evaluate_expression(expr)?
                        .borrow()
                        .to_number()
                        .map_err(|e| RuntimeError { message: e })?,
                    None => 1.0,
                };
                let mut list = Vec::new();
                let mut i = s;
                while (st > 0.0 && i < e) || (st < 0.0 && i > e) {
                    list.push(Rc::new(RefCell::new(Value::Number(i))));
                    i += st;
                    if list.len() > 1_000_000 {
                        return Err(RuntimeError {
                            message: "النطاق كبير جداً".to_string(),
                        });
                    }
                }
                Ok(Rc::new(RefCell::new(Value::List(list))))
            }

            Expr::Await(inner) => self.evaluate_expression(inner),

            // List Comprehension: [تعبير لكل عنصر في قابل_التكرار إذا شرط]
            Expr::ListComprehension {
                element,
                variable,
                iterable,
                condition,
            } => {
                let iter_val = self.evaluate_expression(iterable)?;
                let iter_clone = (*iter_val.borrow()).clone();
                let items = self.collect_iterable(iter_clone)?;
                let mut result = Vec::new();
                
                for item in items {
                    self.environment
                        .borrow_mut()
                        .define(variable, (*item.borrow()).clone(), false);
                    
                    if let Some(cond) = condition {
                        let cond_val = self.evaluate_expression(cond)?;
                        if !cond_val.borrow().is_truthy() {
                            continue;
                        }
                    }
                    
                    let elem_val = self.evaluate_expression(element)?;
                    result.push(elem_val);
                }
                
                Ok(Rc::new(RefCell::new(Value::List(result))))
            }

            // Dictionary Comprehension: {مفتاح: قيمة لكل عنصر في قابل_التكرار إذا شرط}
            Expr::DictComprehension {
                key,
                value,
                variable,
                iterable,
                condition,
            } => {
                let iter_val = self.evaluate_expression(iterable)?;
                let iter_clone = (*iter_val.borrow()).clone();
                let items = self.collect_iterable(iter_clone)?;
                let mut result = HashMap::new();
                
                for item in items {
                    self.environment
                        .borrow_mut()
                        .define(variable, (*item.borrow()).clone(), false);
                    
                    if let Some(cond) = condition {
                        let cond_val = self.evaluate_expression(cond)?;
                        if !cond_val.borrow().is_truthy() {
                            continue;
                        }
                    }
                    
                    let key_val = self.evaluate_expression(key)?;
                    let val_val = self.evaluate_expression(value)?;
                    result.insert(key_val.borrow().to_string_value(), val_val);
                }
                
                Ok(Rc::new(RefCell::new(Value::Dictionary(result))))
            }

            // Spread Operator: ...قائمة
            Expr::Spread(inner) => {
                // Spread returns the inner value for use in list/dict construction
                self.evaluate_expression(inner)
            }

            // Null Coalescing: أ ؟؟ ب
            Expr::NullCoalescing { left, right } => {
                let left_val = self.evaluate_expression(left)?;
                if matches!(*left_val.borrow(), Value::Null) {
                    self.evaluate_expression(right)
                } else {
                    Ok(left_val)
                }
            }

            // Optional Property: كائن؟.خاصية
            Expr::OptionalProperty { object, property } => {
                let obj_val = self.evaluate_expression(object)?;
                if matches!(*obj_val.borrow(), Value::Null) {
                    return Ok(Rc::new(RefCell::new(Value::Null)));
                }
                let obj_clone = (*obj_val.borrow()).clone();
                self.get_property(obj_clone, property)
            }

            // Optional Index: كائن؟?[فهرس]
            Expr::OptionalIndex { object, index } => {
                let obj_val = self.evaluate_expression(object)?;
                if matches!(*obj_val.borrow(), Value::Null) {
                    return Ok(Rc::new(RefCell::new(Value::Null)));
                }
                let idx_val = self.evaluate_expression(index)?;
                let obj_clone = (*obj_val.borrow()).clone();
                let idx_clone = (*idx_val.borrow()).clone();
                self.get_index(obj_clone, idx_clone)
            }

            // Optional Call: دالة؟?(معاملات)
            Expr::OptionalCall { callee, args } => {
                let func_val = self.evaluate_expression(callee)?;
                if matches!(*func_val.borrow(), Value::Null) {
                    return Ok(Rc::new(RefCell::new(Value::Null)));
                }
                let func_clone = (*func_val.borrow()).clone();
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate_expression(arg)?);
                }
                self.call_function(func_clone, arg_values)
            }

            // Pipe Operator: بيانات |> دالة
            Expr::Pipe { value, function } => {
                let val = self.evaluate_expression(value)?;
                let func_val = self.evaluate_expression(function)?;
                let func_clone = (*func_val.borrow()).clone();
                self.call_function(func_clone, vec![val])
            }

            // Yield: إنتاج قيمة
            Expr::Yield(inner) => {
                let val = self.evaluate_expression(inner)?;
                let cloned_val = (*val.borrow()).clone();
                Ok(Rc::new(RefCell::new(Value::Yield(Box::new(cloned_val)))))
            }

            // Generator: مولد
            Expr::Generator { body } => {
                Ok(Rc::new(RefCell::new(Value::Generator {
                    name: String::new(),
                    params: vec![],
                    body: Box::new((**body).clone()),
                    closure: Rc::clone(&self.environment),
                })))
            }
        }
    }

    fn perform_assignment(&mut self, target: &Expr, val: SharedValue) -> Result<(), RuntimeError> {
        match target {
            Expr::Identifier(name) => {
                if self.environment.borrow().is_defined(name) {
                    self.environment
                        .borrow_mut()
                        .assign(name, (*val.borrow()).clone())
                        .map_err(|e| RuntimeError { message: e })?;
                } else {
                    self.environment
                        .borrow_mut()
                        .define(name, (*val.borrow()).clone(), false);
                }
                Ok(())
            }
            Expr::Index { object, index } => {
                let obj_val = self.evaluate_expression(object)?;
                let idx_val = self.evaluate_expression(index)?;
                let new_val = (*val.borrow()).clone();
                let idx_num = idx_val.borrow().to_number().ok();
                let idx_str = idx_val.borrow().to_string_value();
                {
                    let mut obj_ref = obj_val.borrow_mut();
                    match &mut *obj_ref {
                        Value::List(l) => {
                            let idx = idx_num.ok_or_else(|| RuntimeError {
                                message: "الفهرس يجب أن يكون رقماً".to_string(),
                            })? as i64;
                            let len = l.len() as i64;
                            let actual = if idx < 0 { len + idx } else { idx };
                            if actual < 0 || actual >= len {
                                return Err(RuntimeError {
                                    message: format!("الفهرس {} خارج النطاق", idx),
                                });
                            }
                            l[actual as usize] = Rc::new(RefCell::new(new_val));
                        }
                        Value::Dictionary(d) => {
                            d.insert(idx_str, Rc::new(RefCell::new(new_val)));
                        }
                        _ => {
                            return Err(RuntimeError {
                                message: "لا يمكن تعيين عنصر في هذا النوع".to_string(),
                            })
                        }
                    }
                }
                Ok(())
            }
            Expr::Property { object, property } => {
                let obj_val = self.evaluate_expression(object)?;
                let new_val = (*val.borrow()).clone();
                let prop = property.clone();
                {
                    let mut obj_ref = obj_val.borrow_mut();
                    match &mut *obj_ref {
                        Value::Dictionary(d) => {
                            d.insert(prop, Rc::new(RefCell::new(new_val)));
                        }
                        Value::Instance { fields, .. } => {
                            fields
                                .borrow_mut()
                                .insert(prop, Rc::new(RefCell::new(new_val)));
                        }
                        _ => {
                            return Err(RuntimeError {
                                message: "لا يمكن تعيين خاصية في هذا النوع".to_string(),
                            })
                        }
                    }
                }
                Ok(())
            }
            _ => Err(RuntimeError {
                message: "هدف التعيين غير صالح".to_string(),
            }),
        }
    }

    fn eval_binary_op(
        &self,
        left: Value,
        op: &BinaryOp,
        right: Value,
    ) -> Result<SharedValue, RuntimeError> {
        match op {
            BinaryOp::Add => match (&left, &right) {
                (Value::Number(l), Value::Number(r)) => {
                    Ok(Rc::new(RefCell::new(Value::Number(l + r))))
                }
                (Value::String(l), Value::String(r)) => {
                    Ok(Rc::new(RefCell::new(Value::String(format!("{}{}", l, r)))))
                }
                (Value::String(l), r) => {
                    Ok(Rc::new(RefCell::new(Value::String(format!("{}{}", l, r)))))
                }
                (l, Value::String(r)) => {
                    Ok(Rc::new(RefCell::new(Value::String(format!("{}{}", l, r)))))
                }
                (Value::List(l), Value::List(r)) => {
                    let mut new_l = l.clone();
                    new_l.extend(
                        r.iter()
                            .map(|v| Rc::new(RefCell::new((*v.borrow()).clone()))),
                    );
                    Ok(Rc::new(RefCell::new(Value::List(new_l))))
                }
                _ => Err(RuntimeError {
                    message: format!("لا يمكن جمع {} و {}", left.type_name(), right.type_name()),
                }),
            },
            BinaryOp::Sub => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })?;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })?;
                Ok(Rc::new(RefCell::new(Value::Number(l - r))))
            }
            BinaryOp::Mul => match (&left, &right) {
                (Value::Number(l), Value::Number(r)) => {
                    Ok(Rc::new(RefCell::new(Value::Number(l * r))))
                }
                (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
                    Ok(Rc::new(RefCell::new(Value::String(s.repeat(*n as usize)))))
                }
                (Value::List(l), Value::Number(n)) => {
                    let mut result = Vec::new();
                    for _ in 0..(*n as usize) {
                        result.extend(
                            l.iter()
                                .map(|v| Rc::new(RefCell::new((*v.borrow()).clone()))),
                        );
                    }
                    Ok(Rc::new(RefCell::new(Value::List(result))))
                }
                _ => Err(RuntimeError {
                    message: "الضرب غير مدعوم لهذين النوعين".to_string(),
                }),
            },
            BinaryOp::Div => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })?;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })?;
                if r == 0.0 {
                    return Err(RuntimeError {
                        message: "القسمة على صفر".to_string(),
                    });
                }
                Ok(Rc::new(RefCell::new(Value::Number(l / r))))
            }
            BinaryOp::FloorDiv => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })?;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })?;
                if r == 0.0 {
                    return Err(RuntimeError {
                        message: "القسمة على صفر".to_string(),
                    });
                }
                Ok(Rc::new(RefCell::new(Value::Number((l / r).floor()))))
            }
            BinaryOp::Mod => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })?;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })?;
                Ok(Rc::new(RefCell::new(Value::Number(l % r))))
            }
            BinaryOp::Pow => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })?;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })?;
                Ok(Rc::new(RefCell::new(Value::Number(l.powf(r)))))
            }
            BinaryOp::BitAnd => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })? as i64;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })? as i64;
                Ok(Rc::new(RefCell::new(Value::Number((l & r) as f64))))
            }
            BinaryOp::BitOr => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })? as i64;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })? as i64;
                Ok(Rc::new(RefCell::new(Value::Number((l | r) as f64))))
            }
            BinaryOp::BitXor => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })? as i64;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })? as i64;
                Ok(Rc::new(RefCell::new(Value::Number((l ^ r) as f64))))
            }
            BinaryOp::ShiftLeft => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })? as i64;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })? as u32;
                Ok(Rc::new(RefCell::new(Value::Number((l << r) as f64))))
            }
            BinaryOp::ShiftRight => {
                let l = left.to_number().map_err(|e| RuntimeError { message: e })? as i64;
                let r = right.to_number().map_err(|e| RuntimeError { message: e })? as u32;
                Ok(Rc::new(RefCell::new(Value::Number((l >> r) as f64))))
            }
        }
    }

    fn eval_comparison(
        &self,
        left: Value,
        op: &ComparisonOp,
        right: Value,
    ) -> Result<SharedValue, RuntimeError> {
        let result = match (&left, &right) {
            (Value::Number(l), Value::Number(r)) => match op {
                ComparisonOp::Equal => l == r,
                ComparisonOp::NotEqual => l != r,
                ComparisonOp::Less => l < r,
                ComparisonOp::Greater => l > r,
                ComparisonOp::LessEqual => l <= r,
                ComparisonOp::GreaterEqual => l >= r,
            },
            (Value::String(l), Value::String(r)) => match op {
                ComparisonOp::Equal => l == r,
                ComparisonOp::NotEqual => l != r,
                ComparisonOp::Less => l < r,
                ComparisonOp::Greater => l > r,
                ComparisonOp::LessEqual => l <= r,
                ComparisonOp::GreaterEqual => l >= r,
            },
            (Value::Boolean(l), Value::Boolean(r)) => match op {
                ComparisonOp::Equal => l == r,
                ComparisonOp::NotEqual => l != r,
                _ => {
                    return Err(RuntimeError {
                        message: "مقارنة غير صالحة للقيم المنطقية".to_string(),
                    })
                }
            },
            (Value::Null, Value::Null) => matches!(op, ComparisonOp::Equal),
            (Value::Null, _) | (_, Value::Null) => matches!(op, ComparisonOp::NotEqual),
            _ => match op {
                ComparisonOp::Equal => left == right,
                ComparisonOp::NotEqual => left != right,
                _ => {
                    return Err(RuntimeError {
                        message: format!(
                            "لا يمكن مقارنة {} و {}",
                            left.type_name(),
                            right.type_name()
                        ),
                    })
                }
            },
        };
        Ok(Rc::new(RefCell::new(Value::Boolean(result))))
    }

    fn eval_unary_op(&self, op: &UnaryOp, val: Value) -> Result<SharedValue, RuntimeError> {
        match op {
            UnaryOp::Neg => {
                let n = val.to_number().map_err(|e| RuntimeError { message: e })?;
                Ok(Rc::new(RefCell::new(Value::Number(-n))))
            }
            UnaryOp::Not => Ok(Rc::new(RefCell::new(Value::Boolean(!val.is_truthy())))),
            UnaryOp::BitNot => {
                let n = val.to_number().map_err(|e| RuntimeError { message: e })? as i64;
                Ok(Rc::new(RefCell::new(Value::Number((!n) as f64))))
            }
        }
    }

    fn call_function(
        &mut self,
        func: Value,
        args: Vec<SharedValue>,
    ) -> Result<SharedValue, RuntimeError> {
        match func {
            Value::NativeFunction {
                func: native_func,
                name,
            } => native_func(&args).map_err(|e| RuntimeError {
                message: format!("[{}] {}", name, e),
            }),

            Value::Function {
                params,
                body,
                closure,
                ..
            } => {
                let previous = Rc::clone(&self.environment);
                let parent_env = closure.unwrap_or_else(|| Rc::clone(&self.globals));
                let mut new_env = Environment::with_parent(parent_env);

                for (i, (param_name, default)) in params.iter().enumerate() {
                    let val = if i < args.len() {
                        (*args[i].borrow()).clone()
                    } else if let Some(def_expr) = default {
                        let saved = Rc::clone(&self.environment);
                        self.environment = Rc::clone(&previous);
                        let def_val = self
                            .evaluate_expression(def_expr)
                            .map(|v| (*v.borrow()).clone())
                            .unwrap_or(Value::Null);
                        self.environment = saved;
                        def_val
                    } else {
                        Value::Null
                    };
                    new_env.define(param_name, val, false);
                }

                self.environment = Rc::new(RefCell::new(new_env));
                let result = self.execute_statement(&body)?;
                let result_val = (*result.borrow()).clone();
                self.environment = previous;

                match result_val {
                    Value::Return(v) => Ok(Rc::new(RefCell::new(*v))),
                    _ => Ok(Rc::new(RefCell::new(Value::Null))),
                }
            }

            Value::Lambda {
                params,
                body,
                closure,
            } => {
                let previous = Rc::clone(&self.environment);
                let mut new_env = Environment::with_parent(closure);
                for (i, param) in params.iter().enumerate() {
                    let val = args
                        .get(i)
                        .map(|v| (*v.borrow()).clone())
                        .unwrap_or(Value::Null);
                    new_env.define(param, val, false);
                }
                self.environment = Rc::new(RefCell::new(new_env));
                let result = self.evaluate_expression(&body)?;
                self.environment = previous;
                Ok(result)
            }

            Value::Class {
                name,
                methods,
                fields,
                ..
            } => {
                let instance_fields = Rc::new(RefCell::new(HashMap::new()));
                for (fname, default_expr) in &fields {
                    let default_val = match default_expr {
                        Some(e) => (*self.evaluate_expression(e)?.borrow()).clone(),
                        None => Value::Null,
                    };
                    instance_fields
                        .borrow_mut()
                        .insert(fname.clone(), Rc::new(RefCell::new(default_val)));
                }

                let instance = Value::Instance {
                    class_name: name.clone(),
                    fields: Rc::clone(&instance_fields),
                    methods: methods.clone(),
                };
                let instance_ref = Rc::new(RefCell::new(instance));

                if let Some(init_func) = methods.get("هيئ").or_else(|| methods.get("init")) {
                    let init_clone = init_func.clone();
                    if let Value::Function {
                        params,
                        body,
                        closure,
                        ..
                    } = init_clone
                    {
                        let previous = Rc::clone(&self.environment);
                        let parent = closure.unwrap_or_else(|| Rc::clone(&self.globals));
                        let mut new_env = Environment::with_parent(parent);
                        new_env.define("هذا", (*instance_ref.borrow()).clone(), false);
                        for (i, (param, _)) in params.iter().enumerate() {
                            let val = args
                                .get(i)
                                .map(|v| (*v.borrow()).clone())
                                .unwrap_or(Value::Null);
                            new_env.define(param, val, false);
                        }
                        self.environment = Rc::new(RefCell::new(new_env));
                        self.execute_statement(&body)?;
                        self.environment = previous;
                    }
                }

                Ok(instance_ref)
            }

            _ => Err(RuntimeError {
                message: "النوع ليس دالة قابلة للاستدعاء".to_string(),
            }),
        }
    }

    fn get_property(&mut self, obj: Value, property: &str) -> Result<SharedValue, RuntimeError> {
        match obj {
            Value::Dictionary(ref dict) => Ok(dict
                .get(property)
                .map(Rc::clone)
                .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))),
            Value::Instance {
                fields, methods, ..
            } => {
                if let Some(v) = fields.borrow().get(property) {
                    return Ok(Rc::clone(v));
                }
                if let Some(method) = methods.get(property) {
                    return Ok(Rc::new(RefCell::new(method.clone())));
                }
                Ok(Rc::new(RefCell::new(Value::Null)))
            }
            Value::String(ref s) => match property {
                "طول" => Ok(Rc::new(RefCell::new(Value::Number(
                    s.chars().count() as f64
                )))),
                "كبير" => Ok(Rc::new(RefCell::new(Value::String(s.to_uppercase())))),
                "صغير" => Ok(Rc::new(RefCell::new(Value::String(s.to_lowercase())))),
                "مقلوب" => Ok(Rc::new(RefCell::new(Value::String(
                    s.chars().rev().collect(),
                )))),
                "قص" => Ok(Rc::new(RefCell::new(Value::String(s.trim().to_string())))),
                _ => Err(RuntimeError {
                    message: format!("النص ليس له خاصية '{}'", property),
                }),
            },
            Value::List(ref l) => match property {
                "طول" => Ok(Rc::new(RefCell::new(Value::Number(l.len() as f64)))),
                "أول" => Ok(l
                    .first()
                    .map(Rc::clone)
                    .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))),
                "آخر" => Ok(l
                    .last()
                    .map(Rc::clone)
                    .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))),
                "فارغة" => Ok(Rc::new(RefCell::new(Value::Boolean(l.is_empty())))),
                _ => Err(RuntimeError {
                    message: format!("القائمة ليس لها خاصية '{}'", property),
                }),
            },
            _ => Err(RuntimeError {
                message: format!(
                    "لا يمكن الوصول للخاصية '{}' في {}",
                    property,
                    obj.type_name()
                ),
            }),
        }
    }

    fn get_index(&self, obj: Value, index: Value) -> Result<SharedValue, RuntimeError> {
        match (&obj, &index) {
            (Value::List(list), Value::Number(n)) => {
                let idx = *n as i64;
                let len = list.len() as i64;
                let actual = if idx < 0 { len + idx } else { idx };
                if actual < 0 || actual >= len {
                    return Err(RuntimeError {
                        message: format!("الفهرس {} خارج النطاق [0..{}]", idx, len - 1),
                    });
                }
                Ok(Rc::clone(&list[actual as usize]))
            }
            (Value::String(s), Value::Number(n)) => {
                let idx = *n as i64;
                let chars: Vec<char> = s.chars().collect();
                let len = chars.len() as i64;
                let actual = if idx < 0 { len + idx } else { idx };
                if actual < 0 || actual >= len {
                    return Err(RuntimeError {
                        message: format!("الفهرس {} خارج النطاق", idx),
                    });
                }
                Ok(Rc::new(RefCell::new(Value::String(
                    chars[actual as usize].to_string(),
                ))))
            }
            (Value::Dictionary(dict), Value::String(key)) => Ok(dict
                .get(key)
                .map(Rc::clone)
                .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))),
            (Value::Dictionary(dict), k) => {
                let key = k.to_string_value();
                Ok(dict
                    .get(&key)
                    .map(Rc::clone)
                    .unwrap_or_else(|| Rc::new(RefCell::new(Value::Null))))
            }
            _ => Err(RuntimeError {
                message: format!("لا يمكن فهرسة {} بـ {}", obj.type_name(), index.type_name()),
            }),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let mut interp = Interpreter::new();
        let result = interp.run("١٠ + ٥").unwrap();
        assert!(matches!(*result.borrow(), Value::Number(15.0)));
    }

    #[test]
    fn test_variable() {
        let mut interp = Interpreter::new();
        interp.run("متغير س = ١٠؛").unwrap();
        let result = interp.run("س").unwrap();
        assert!(matches!(*result.borrow(), Value::Number(10.0)));
    }

    #[test]
    fn test_function() {
        let mut interp = Interpreter::new();
        let code = r#"
            دالة جمع(أ، ب) {
                أرجع أ + ب؛
            }
            جمع(٣، ٤)
        "#;
        let result = interp.run(code).unwrap();
        assert!(matches!(*result.borrow(), Value::Number(7.0)));
    }

    #[test]
    fn test_repeat() {
        let mut interp = Interpreter::new();
        let code = r#"
            متغير عداد = ٠؛
            كرر ٥ مرات { عداد += ١؛ }
            عداد
        "#;
        let result = interp.run(code).unwrap();
        assert!(matches!(*result.borrow(), Value::Number(5.0)));
    }

    #[test]
    fn test_short_var() {
        let mut interp = Interpreter::new();
        let result = interp.run("م س = ٥؛ س").unwrap();
        assert!(matches!(*result.borrow(), Value::Number(5.0)));
    }

    #[test]
    fn test_for_range() {
        let mut interp = Interpreter::new();
        let result = interp
            .run(
                r#"
            متغير مجموع = ٠؛
            لكل ي في مدى(١، ٦) { مجموع += ي؛ }
            مجموع
        "#,
            )
            .unwrap();
        assert!(matches!(*result.borrow(), Value::Number(15.0)));
    }

    #[test]
    fn test_try_catch() {
        let mut interp = Interpreter::new();
        let result = interp
            .run(
                r#"
            متغير نتيجة = "لا"؛
            حاول {
                ألقِ "خطأ اختبار"؛
            } امسك(خ) {
                نتيجة = "نعم"؛
            }
            نتيجة
        "#,
            )
            .unwrap();
        assert_eq!(result.borrow().to_string_value(), "نعم");
    }

    #[test]
    fn test_match() {
        let mut interp = Interpreter::new();
        let result = interp
            .run(
                r#"
            متغير نتيجة = ""؛
            طابق ٢ {
                حالة ١: نتيجة = "واحد"؛
                حالة ٢: نتيجة = "اثنان"؛
                افتراضي: نتيجة = "أخرى"؛
            }
            نتيجة
        "#,
            )
            .unwrap();
        assert_eq!(result.borrow().to_string_value(), "اثنان");
    }
}
