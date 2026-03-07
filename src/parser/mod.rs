pub mod ast;

use crate::error::{helpers as error_helpers, Position, Span};
use crate::lexer::tokens::{Token, TokenType};
use crate::lexer::Lexer;
use ast::{BinaryOp, ComparisonOp, DestructuringPattern, Expr, FormatPart, LogicalOp, Program, Stmt, TypeAnnotation, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    source_lines: Vec<String>,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Parser {
    fn combine_errors(errors: Vec<ParseError>) -> ParseError {
        let mut combined = errors[0].message.clone();

        if errors.len() > 1 {
            combined.push_str(&format!(
                "\n\nتم اكتشاف {} أخطاء إضافية أثناء التحليل:\n",
                errors.len() - 1
            ));

            for (index, error) in errors.iter().skip(1).enumerate() {
                combined.push_str(&format!(
                    "  {}. السطر {}، العمود {}: {}\n",
                    index + 1,
                    error.line,
                    error.column,
                    error.message.lines().next().unwrap_or("خطأ نحوي")
                ));
            }
        }

        ParseError {
            message: combined,
            line: errors[0].line,
            column: errors[0].column,
        }
    }

    fn token_suggestion(expected: &TokenType) -> Option<&'static str> {
        match expected {
            TokenType::Semicolon => Some("أضف الفاصلة المنقوطة العربية '؛' في نهاية التعليمة"),
            TokenType::RParen => Some("تحقق من إغلاق القوس ')' بعد التعبير أو المعاملات"),
            TokenType::RBrace => Some("تحقق من إغلاق الكتلة بالقوس '}'"),
            TokenType::RBracket => Some("تحقق من إغلاق القائمة أو الفهرسة بالقوس ']'"),
            TokenType::Comma => Some("افصل العناصر بالفاصلة العربية '،' أو ','"),
            TokenType::Assign => Some("استخدم '=' لإسناد قيمة للمتغير"),
            _ => None,
        }
    }

    pub fn new(tokens: Vec<Token>, source: &str) -> Self {
        Parser {
            tokens,
            current: 0,
            source_lines: source.lines().map(str::to_string).collect(),
        }
    }

    pub fn parse(input: &str) -> Result<Program, ParseError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().map_err(|e| ParseError {
            message: e,
            line: 1,
            column: 1,
        })?;

        let mut parser = Parser::new(tokens, input);
        parser.parse_program()
    }

    fn source_line(&self, line: usize) -> Option<&str> {
        if line == 0 {
            return None;
        }
        self.source_lines.get(line - 1).map(String::as_str)
    }

    fn token_span(&self, token: &Token) -> Span {
        let token_repr = token.token_type.to_string();
        let width = token_repr.chars().count().max(1);
        let start = Position::new(token.line, token.column, token.span_start);
        let end = Position::new(token.line, token.column + width, token.span_end);
        Span::new(start, end)
    }

    fn current_token(&self) -> &Token {
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| self.tokens.last().unwrap())
    }

    #[allow(dead_code)]
    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
        self.current_token()
    }

    fn expect(&mut self, expected: TokenType) -> Result<&Token, ParseError> {
        let token = self.current_token();
        if std::mem::discriminant(&token.token_type) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            let span = self.token_span(token);
            let mut rich_error = error_helpers::expected_token(
                &token.token_type.to_string(),
                &expected.to_string(),
                span,
            );

            if let Some(context) = self.source_line(token.line) {
                rich_error = rich_error.with_source_context(context.to_string());
            }

            if let Some(suggestion) = Self::token_suggestion(&expected) {
                rich_error = rich_error.with_suggestion(suggestion.to_string());
            }

            Err(ParseError {
                message: rich_error.format("<النص>"),
                line: token.line,
                column: token.column,
            })
        }
    }

    fn match_token(&self, types: &[TokenType]) -> bool {
        let current = self.current_token();
        types
            .iter()
            .any(|t| std::mem::discriminant(&current.token_type) == std::mem::discriminant(t))
    }

    fn consume_semicolon(&mut self) -> Result<(), ParseError> {
        if self.match_token(&[TokenType::Semicolon]) {
            self.advance();
        }
        Ok(())
    }

    fn synchronize(&mut self) {
        while !matches!(self.current_token().token_type, TokenType::EOF) {
            if matches!(self.current_token().token_type, TokenType::Semicolon) {
                self.advance();
                return;
            }

            match self.current_token().token_type {
                TokenType::Let
                | TokenType::Const
                | TokenType::Function
                | TokenType::If
                | TokenType::While
                | TokenType::For
                | TokenType::Repeat
                | TokenType::Return
                | TokenType::Print
                | TokenType::Try
                | TokenType::Throw
                | TokenType::Match
                | TokenType::Class
                | TokenType::Import
                | TokenType::Assert
                | TokenType::Delete
                | TokenType::Interface
                | TokenType::Theme
                | TokenType::Router
                | TokenType::Event => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        let mut errors: Vec<ParseError> = Vec::new();

        while !matches!(self.current_token().token_type, TokenType::EOF) {
            if matches!(self.current_token().token_type, TokenType::Comment(_)) {
                self.advance();
                continue;
            }

            let cursor_before_parse = self.current;

            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    errors.push(err);
                    self.synchronize();

                    if self.current == cursor_before_parse
                        && !matches!(self.current_token().token_type, TokenType::EOF)
                    {
                        self.advance();
                    }
                }
            }
        }

        if !errors.is_empty() {
            Err(Self::combine_errors(errors))
        } else {
            Ok(Program::new(statements))
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let token = self.current_token();

        match &token.token_type {
            TokenType::Let => self.parse_variable_decl(false),
            TokenType::Const => self.parse_variable_decl(true),
            TokenType::Function => self.parse_function_decl(false),
            TokenType::Async => {
                self.advance();
                self.parse_function_decl(true)
            }
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Repeat => self.parse_repeat_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Yield => self.parse_yield_statement(),
            TokenType::Break => {
                self.advance();
                self.consume_semicolon()?;
                Ok(Stmt::Break)
            }
            TokenType::Continue => {
                self.advance();
                self.consume_semicolon()?;
                Ok(Stmt::Continue)
            }
            TokenType::Print => self.parse_print_statement(),
            TokenType::LBrace => self.parse_block(),
            TokenType::Try => self.parse_try_catch(),
            TokenType::Throw => self.parse_throw_statement(),
            TokenType::Match => self.parse_match_statement(),
            TokenType::Class => self.parse_class_decl(),
            TokenType::Import => self.parse_import(),
            TokenType::Assert => self.parse_assert(),
            TokenType::Delete => {
                self.advance();
                let name = self.parse_identifier()?;
                self.consume_semicolon()?;
                Ok(Stmt::Delete(name))
            }
            TokenType::Interface => self.parse_ui_component_decl(),
            TokenType::Case => self.parse_state_decl(),
            TokenType::Theme => self.parse_theme_decl(),
            TokenType::Router => self.parse_route_decl(),
            TokenType::Event => self.parse_event_handler_decl(),
            // Context Manager: مع مورد كـ اسم { ... }
            TokenType::With => self.parse_with_statement(),
            // Data Class: بيانات اسم { حقل: نوع، ... }
            TokenType::Data => self.parse_data_class_decl(),
            // Enum: تعداد اسم { قيمة، ... }
            TokenType::Enum => self.parse_enum_decl(),
            // Decorator: @زخرفة
            TokenType::At => self.parse_decorator(),
            TokenType::Comment(_) => {
                self.advance();
                self.parse_statement()
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        match &self.current_token().token_type {
            TokenType::Identifier(n) => {
                let name = n.clone();
                self.advance();
                Ok(name)
            }
            _ => {
                let token = self.current_token();
                let span = self.token_span(token);
                let mut rich_error =
                    error_helpers::unexpected_token(&token.token_type.to_string(), "معرف", span)
                        .with_help("يجب أن يبدأ المعرف بحرف عربي أو لاتيني".to_string());

                if let Some(context) = self.source_line(token.line) {
                    rich_error = rich_error.with_source_context(context.to_string());
                }

                Err(ParseError {
                    message: rich_error.format("<النص>"),
                    line: token.line,
                    column: token.column,
                })
            }
        }
    }

    fn parse_variable_decl(&mut self, is_const: bool) -> Result<Stmt, ParseError> {
        self.advance();

        // دعم التفكيك: متغير [أ، ب، ج] = قائمة؛ أو متغير {اسم، عمر} = كائن؛
        if self.match_token(&[TokenType::LBracket]) {
            return self.parse_destructuring_decl(is_const);
        }
        
        if self.match_token(&[TokenType::LBrace]) {
            // تحقق من أنه تفكيك كائن وليس قاموس
            // تفكيك الكائن: {أ، ب} أو {أ: اسم_جديد}
            return self.parse_object_destructuring_decl(is_const);
        }

        // دعم تعريف متغيرات متعددة: متغير أ = ١، ب = ٢؛
        let mut names = Vec::new();
        let mut values = Vec::new();

        loop {
            let name = self.parse_identifier()?;
            
            // دعم التعليقات التوضيحية: متغير س: رقم = ١٠؛
            let _type_annotation = if self.match_token(&[TokenType::Colon]) {
                self.advance();
                Some(self.parse_type_annotation()?)
            } else {
                None
            };

            if self.match_token(&[TokenType::Assign]) {
                self.advance();
                let value = self.parse_expression()?;
                names.push(name);
                values.push(value);
            } else {
                names.push(name);
                values.push(Expr::Null);
            }

            if self.match_token(&[TokenType::Comma]) {
                self.advance();
            } else {
                break;
            }
        }

        self.consume_semicolon()?;

        if names.len() == 1 {
            Ok(Stmt::VariableDecl {
                name: names.remove(0),
                value: values.remove(0),
                is_const,
            })
        } else {
            Ok(Stmt::MultiVarDecl {
                names,
                values,
                is_const,
            })
        }
    }
    
    /// تحليل تفكيك القائمة: متغير [أ، ب، ج] = قائمة؛
    fn parse_destructuring_decl(&mut self, is_const: bool) -> Result<Stmt, ParseError> {
        self.advance(); // تجاوز '['
        
        let mut names = Vec::new();
        while !self.match_token(&[TokenType::RBracket, TokenType::EOF]) {
            names.push(self.parse_identifier()?);
            if self.match_token(&[TokenType::Comma]) {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenType::RBracket)?;
        
        self.expect(TokenType::Assign)?;
        let value = self.parse_expression()?;
        self.consume_semicolon()?;
        
        Ok(Stmt::DestructuringDecl {
            pattern: DestructuringPattern::List(names),
            value,
            is_const,
        })
    }
    
    /// تحليل تفكيك الكائن: متغير {اسم، عمر: ع} = كائن؛
    fn parse_object_destructuring_decl(&mut self, is_const: bool) -> Result<Stmt, ParseError> {
        self.advance(); // تجاوز '{'
        
        let mut fields = Vec::new();
        while !self.match_token(&[TokenType::RBrace, TokenType::EOF]) {
            let prop = self.parse_identifier()?;
            let alias = if self.match_token(&[TokenType::Colon]) {
                self.advance();
                Some(self.parse_identifier()?)
            } else {
                None
            };
            fields.push((prop, alias));
            
            if self.match_token(&[TokenType::Comma]) {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenType::RBrace)?;
        
        self.expect(TokenType::Assign)?;
        let value = self.parse_expression()?;
        self.consume_semicolon()?;
        
        Ok(Stmt::DestructuringDecl {
            pattern: DestructuringPattern::Object(fields),
            value,
            is_const,
        })
    }
    
    /// تحليل التعليقات التوضيحية للأنواع
    fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ParseError> {
        // تحليل النوع الأساسي
        let base_type = self.parse_simple_type()?;
        
        // دعم النوع الاختياري: نوع؟
        if self.match_token(&[TokenType::Question]) || self.match_token(&[TokenType::QuestionQuestion]) {
            self.advance();
            return Ok(TypeAnnotation::Optional(Box::new(base_type)));
        }
        
        Ok(base_type)
    }
    
    fn parse_simple_type(&mut self) -> Result<TypeAnnotation, ParseError> {
        let token = self.current_token();
        
        match &token.token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                
                // أنواع مدمجة
                match name.as_str() {
                    "رقم" | "عدد" | "number" => Ok(TypeAnnotation::Simple("رقم".to_string())),
                    "نص" | "string" => Ok(TypeAnnotation::Simple("نص".to_string())),
                    "منطقي" | "bool" | "boolean" => Ok(TypeAnnotation::Simple("منطقي".to_string())),
                    "قائمة" | "list" => {
                        // [نوع] - نوع عناصر القائمة
                        if self.match_token(&[TokenType::LBracket]) {
                            self.advance();
                            let inner = self.parse_type_annotation()?;
                            self.expect(TokenType::RBracket)?;
                            Ok(TypeAnnotation::List(Box::new(inner)))
                        } else {
                            Ok(TypeAnnotation::Simple("قائمة".to_string()))
                        }
                    }
                    "قاموس" | "dict" | "dictionary" => {
                        // {مفتاح: قيمة}
                        if self.match_token(&[TokenType::LBrace]) {
                            self.advance();
                            let key = self.parse_type_annotation()?;
                            self.expect(TokenType::Colon)?;
                            let val = self.parse_type_annotation()?;
                            self.expect(TokenType::RBrace)?;
                            Ok(TypeAnnotation::Dict(Box::new(key), Box::new(val)))
                        } else {
                            Ok(TypeAnnotation::Simple("قاموس".to_string()))
                        }
                    }
                    _ => Ok(TypeAnnotation::Simple(name)),
                }
            }
            TokenType::LBracket => {
                // [نوع] - قائمة
                self.advance();
                let inner = self.parse_type_annotation()?;
                self.expect(TokenType::RBracket)?;
                Ok(TypeAnnotation::List(Box::new(inner)))
            }
            TokenType::LParen => {
                // (معاملات) -> نتيجة - دالة
                self.advance();
                let mut params = Vec::new();
                while !self.match_token(&[TokenType::RParen, TokenType::EOF]) {
                    params.push(self.parse_type_annotation()?);
                    if self.match_token(&[TokenType::Comma]) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(TokenType::RParen)?;
                
                if self.match_token(&[TokenType::Arrow]) {
                    self.advance();
                    let ret = self.parse_type_annotation()?;
                    Ok(TypeAnnotation::Function(params, Box::new(ret)))
                } else {
                    Ok(TypeAnnotation::Function(params, Box::new(TypeAnnotation::Simple("لا_شيء".to_string()))))
                }
            }
            _ => Err(ParseError {
                message: format!("توقع نوع، وجد {}", token.token_type),
                line: token.line,
                column: token.column,
            }),
        }
    }

    fn parse_function_decl(&mut self, is_async: bool) -> Result<Stmt, ParseError> {
        self.advance();

        let name = self.parse_identifier()?;

        self.expect(TokenType::LParen)?;
        let params = self.parse_parameters_with_types()?;
        self.expect(TokenType::RParen)?;
        
        // دعم نوع الإرجاع: -> نوع
        let return_type = if self.match_token(&[TokenType::Arrow]) {
            self.advance();
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        let body = Box::new(self.parse_block()?);

        Ok(Stmt::FunctionDecl {
            name,
            params,
            body,
            is_async,
            return_type,
        })
    }
    
    /// تحليل معاملات الدالة مع دعم التعليقات التوضيحية
    fn parse_parameters_with_types(
        &mut self,
    ) -> Result<Vec<(String, Option<Expr>, Option<TypeAnnotation>)>, ParseError> {
        let mut params = Vec::new();

        if self.match_token(&[TokenType::RParen]) {
            return Ok(params);
        }

        loop {
            let name = self.parse_identifier()?;
            
            // دعم التعليقات التوضيحية: معامل: نوع
            let type_annotation = if self.match_token(&[TokenType::Colon]) {
                self.advance();
                Some(self.parse_type_annotation()?)
            } else {
                None
            };
            
            // دعم القيم الافتراضية: معامل = قيمة
            let default = if self.match_token(&[TokenType::Assign]) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            params.push((name, default, type_annotation));

            if self.match_token(&[TokenType::Comma]) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(params)
    }

    fn parse_named_callable_signature(&mut self) -> Result<(String, Vec<String>), ParseError> {
        self.advance();
        let name = self.parse_identifier()?;
        self.expect(TokenType::LParen)?;

        let mut params = Vec::new();
        while !self.match_token(&[TokenType::RParen]) {
            params.push(self.parse_identifier()?);

            if self.match_token(&[TokenType::Comma]) {
                self.advance();
            } else {
                break;
            }
        }

        self.expect(TokenType::RParen)?;
        Ok((name, params))
    }

    fn parse_ui_component_decl(&mut self) -> Result<Stmt, ParseError> {
        let (name, params) = self.parse_named_callable_signature()?;
        let body = Box::new(self.parse_block()?);

        Ok(Stmt::UiComponentDecl { name, params, body })
    }

    fn parse_event_handler_decl(&mut self) -> Result<Stmt, ParseError> {
        let (name, params) = self.parse_named_callable_signature()?;
        let body = Box::new(self.parse_block()?);

        Ok(Stmt::EventHandlerDecl { name, params, body })
    }

    fn parse_state_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name = self.parse_identifier()?;
        self.expect(TokenType::Assign)?;
        let value = self.parse_expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::StateDecl { name, value })
    }

    fn parse_theme_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name = self.parse_identifier()?;
        self.expect(TokenType::Assign)?;
        let value = self.parse_expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::ThemeDecl { name, value })
    }

    fn parse_route_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name = self.parse_identifier()?;
        self.expect(TokenType::Assign)?;
        let value = self.parse_expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::RouteDecl { name, value })
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let condition = self.parse_expression()?;
        let then_branch = Box::new(self.parse_statement()?);

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;

        loop {
            if self.match_token(&[TokenType::ElseIf]) {
                self.advance();
                let elif_cond = self.parse_expression()?;
                let elif_body = Box::new(self.parse_statement()?);
                else_if_branches.push((elif_cond, elif_body));
            } else if self.match_token(&[TokenType::Else]) {
                self.advance();
                else_branch = Some(Box::new(self.parse_statement()?));
                break;
            } else {
                break;
            }
        }

        Ok(Stmt::If {
            condition,
            then_branch,
            else_if_branches,
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let condition = self.parse_expression()?;
        let body = Box::new(self.parse_statement()?);

        Ok(Stmt::While { condition, body })
    }

    fn parse_for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let variable = self.parse_identifier()?;

        if self.match_token(&[TokenType::In]) {
            self.advance();

            if self.match_token(&[TokenType::Range]) {
                self.advance();
                self.expect(TokenType::LParen)?;
                let start = self.parse_expression()?;
                self.expect(TokenType::Comma)?;
                let end = self.parse_expression()?;
                let step = if self.match_token(&[TokenType::Comma]) {
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.expect(TokenType::RParen)?;
                let body = Box::new(self.parse_statement()?);
                return Ok(Stmt::ForRange {
                    variable,
                    start,
                    end,
                    step,
                    body,
                });
            }

            let iterable = self.parse_expression()?;
            let body = Box::new(self.parse_statement()?);
            return Ok(Stmt::For {
                variable,
                iterable,
                body,
            });
        }

        let token = self.current_token();
        Err(ParseError {
            message: "توقع 'في' في حلقة لكل".to_string(),
            line: token.line,
            column: token.column,
        })
    }

    fn parse_repeat_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let count = self.parse_expression()?;

        if self.match_token(&[TokenType::Times]) {
            self.advance();
        }

        let body = Box::new(self.parse_statement()?);

        Ok(Stmt::Repeat { count, body })
    }

    fn parse_return_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let value = if self.match_token(&[TokenType::Semicolon, TokenType::EOF, TokenType::RBrace])
        {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume_semicolon()?;
        Ok(Stmt::Return(value))
    }

    fn parse_print_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let mut args = Vec::new();

        if self.match_token(&[TokenType::LParen]) {
            self.advance();
            if !self.match_token(&[TokenType::RParen]) {
                loop {
                    args.push(self.parse_expression()?);
                    if self.match_token(&[TokenType::Comma]) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            self.expect(TokenType::RParen)?;
        } else {
            args.push(self.parse_expression()?);
        }

        self.consume_semicolon()?;
        Ok(Stmt::Print(args))
    }

    fn parse_block(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenType::LBrace)?;

        let mut statements = Vec::new();

        while !self.match_token(&[TokenType::RBrace, TokenType::EOF]) {
            if matches!(self.current_token().token_type, TokenType::Comment(_)) {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement()?);
        }

        self.expect(TokenType::RBrace)?;
        Ok(Stmt::Block(statements))
    }

    fn parse_try_catch(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let try_block = Box::new(self.parse_block()?);

        let (catch_var, catch_block) = if self.match_token(&[TokenType::Catch]) {
            self.advance();
            let var = if self.match_token(&[TokenType::LParen]) {
                self.advance();
                let name = self.parse_identifier()?;
                self.expect(TokenType::RParen)?;
                Some(name)
            } else {
                None
            };
            let block = Box::new(self.parse_block()?);
            (var, block)
        } else {
            let token = self.current_token();
            return Err(ParseError {
                message: "توقع 'امسك' بعد 'حاول'".to_string(),
                line: token.line,
                column: token.column,
            });
        };

        let finally_block = if self.match_token(&[TokenType::Finally]) {
            self.advance();
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };

        Ok(Stmt::TryCatch {
            try_block,
            catch_var,
            catch_block,
            finally_block,
        })
    }

    fn parse_throw_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let expr = self.parse_expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::Throw(expr))
    }

    fn parse_match_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let value = self.parse_expression()?;

        self.expect(TokenType::LBrace)?;

        let mut cases = Vec::new();
        let mut default = None;

        while !self.match_token(&[TokenType::RBrace, TokenType::EOF]) {
            if matches!(self.current_token().token_type, TokenType::Comment(_)) {
                self.advance();
                continue;
            }

            if self.match_token(&[TokenType::Default]) {
                self.advance();
                self.expect(TokenType::Colon)?;
                default = Some(Box::new(self.parse_statement()?));
            } else if self.match_token(&[TokenType::Case]) {
                self.advance();
                let mut patterns = Vec::new();
                loop {
                    patterns.push(self.parse_expression()?);
                    if self.match_token(&[TokenType::Comma]) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(TokenType::Colon)?;
                let body = Box::new(self.parse_statement()?);
                cases.push((patterns, body));
            } else {
                break;
            }
        }

        self.expect(TokenType::RBrace)?;

        Ok(Stmt::Match {
            value,
            cases,
            default,
        })
    }

    fn parse_class_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let name = self.parse_identifier()?;

        let parent = if self.match_token(&[TokenType::Colon]) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.expect(TokenType::LBrace)?;

        let mut methods = Vec::new();
        let mut fields = Vec::new();

        while !self.match_token(&[TokenType::RBrace, TokenType::EOF]) {
            if matches!(self.current_token().token_type, TokenType::Comment(_)) {
                self.advance();
                continue;
            }

            if self.match_token(&[TokenType::Function]) {
                methods.push(self.parse_function_decl(false)?);
            } else if self.match_token(&[TokenType::Let]) || self.match_token(&[TokenType::Const]) {
                let is_const = self.match_token(&[TokenType::Const]);
                self.advance();
                let fname = self.parse_identifier()?;
                let default = if self.match_token(&[TokenType::Assign]) {
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.consume_semicolon()?;
                let _ = is_const;
                fields.push((fname, default));
            } else {
                self.advance();
            }
        }

        self.expect(TokenType::RBrace)?;

        Ok(Stmt::ClassDecl {
            name,
            parent,
            methods,
            fields,
        })
    }

    fn parse_import(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let mut items = Vec::new();
        let mut path = String::new();
        let mut alias = None;

        if self.match_token(&[TokenType::LBrace]) {
            self.advance();
            while !self.match_token(&[TokenType::RBrace, TokenType::EOF]) {
                items.push(self.parse_import_item_name()?);
                if self.match_token(&[TokenType::Comma]) {
                    self.advance();
                }
            }
            self.expect(TokenType::RBrace)?;
            if self.match_token(&[TokenType::From]) {
                self.advance();
            }
        }

        match &self.current_token().token_type {
            TokenType::String(s) => {
                path = s.clone();
                self.advance();
            }
            TokenType::Identifier(n) => {
                path = n.clone();
                self.advance();
            }
            _ => {}
        }

        if !items.is_empty() && path.is_empty() {
            let token = self.current_token();
            let span = self.token_span(token);
            let mut rich_error = error_helpers::unexpected_token(
                &token.token_type.to_string(),
                "مسار الاستيراد",
                span,
            )
            .with_help("صيغة الاستيراد الجزئي تتطلب: استورد { عنصر } من \"module\"؛".to_string());

            if let Some(context) = self.source_line(token.line) {
                rich_error = rich_error.with_source_context(context.to_string());
            }

            return Err(ParseError {
                message: rich_error.format("<النص>"),
                line: token.line,
                column: token.column,
            });
        }

        if self.match_token(&[TokenType::As]) {
            self.advance();
            alias = Some(self.parse_identifier()?);
        }

        self.consume_semicolon()?;

        Ok(Stmt::Import { path, alias, items })
    }

    fn parse_import_item_name(&mut self) -> Result<String, ParseError> {
        let current = self.current_token().clone();

        let name = match current.token_type {
            TokenType::Identifier(name) => name,
            TokenType::Function
            | TokenType::Class
            | TokenType::Let
            | TokenType::Const
            | TokenType::Match
            | TokenType::Case
            | TokenType::Default
            | TokenType::Delete
            | TokenType::Print
            | TokenType::Input
            | TokenType::TypeOf
            | TokenType::Typeof
            | TokenType::Length
            | TokenType::New
            | TokenType::This
            | TokenType::Super => current.token_type.to_string(),
            _ => {
                return self.parse_identifier();
            }
        };

        self.advance();
        Ok(name)
    }

    fn parse_assert(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let condition = self.parse_expression()?;
        let message = if self.match_token(&[TokenType::Comma]) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume_semicolon()?;
        Ok(Stmt::Assert { condition, message })
    }
    
    /// تحليل جملة أعطِ: أعطِ قيمة؛
    fn parse_yield_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let value = self.parse_expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::Yield(value))
    }
    
    /// تحليل جملة مع (Context Manager): مع مورد كـ اسم { ... }
    fn parse_with_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        
        let resource = self.parse_expression()?;
        
        let alias = if self.match_token(&[TokenType::AsKeyword]) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            None
        };
        
        let body = Box::new(self.parse_block()?);
        
        Ok(Stmt::With {
            resource,
            alias,
            body,
        })
    }
    
    /// تحليل فئة البيانات: بيانات اسم { حقل: نوع، ... }
    fn parse_data_class_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        
        let name = self.parse_identifier()?;
        
        self.expect(TokenType::LBrace)?;
        
        let mut fields = Vec::new();
        
        while !self.match_token(&[TokenType::RBrace, TokenType::EOF]) {
            if matches!(self.current_token().token_type, TokenType::Comment(_)) {
                self.advance();
                continue;
            }
            
            let field_name = self.parse_identifier()?;
            
            // دعم النوع: حقل: نوع
            let _field_type = if self.match_token(&[TokenType::Colon]) {
                self.advance();
                Some(self.parse_type_annotation()?)
            } else {
                None
            };
            
            // دعم القيمة الافتراضية: حقل = قيمة
            let default = if self.match_token(&[TokenType::Assign]) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            fields.push((field_name, default));
            
            if self.match_token(&[TokenType::Comma]) {
                self.advance();
            } else if !self.match_token(&[TokenType::RBrace]) {
                break;
            }
        }
        
        self.expect(TokenType::RBrace)?;
        
        Ok(Stmt::DataClassDecl { name, fields })
    }
    
    /// تحليل التعداد: تعداد اسم { قيمة، ... }
    fn parse_enum_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        
        let name = self.parse_identifier()?;
        
        self.expect(TokenType::LBrace)?;
        
        let mut variants = Vec::new();
        let mut auto_value = 0.0;
        
        while !self.match_token(&[TokenType::RBrace, TokenType::EOF]) {
            if matches!(self.current_token().token_type, TokenType::Comment(_)) {
                self.advance();
                continue;
            }
            
            let variant_name = self.parse_identifier()?;
            
            // دعم القيمة الصريحة: قيمة = رقم
            let value = if self.match_token(&[TokenType::Assign]) {
                self.advance();
                let expr = self.parse_expression()?;
                Some(expr)
            } else {
                // قيمة تلقائية
                let val = Some(Expr::Number(auto_value));
                auto_value += 1.0;
                val
            };
            
            variants.push((variant_name, value));
            
            if self.match_token(&[TokenType::Comma]) {
                self.advance();
            } else if !self.match_token(&[TokenType::RBrace]) {
                break;
            }
        }
        
        self.expect(TokenType::RBrace)?;
        
        Ok(Stmt::EnumDecl { name, variants })
    }
    
    /// تحليل المزخرف: @زخرفة
    fn parse_decorator(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // تجاوز '@'
        
        // قراءة اسم المزخرف (قد يكون مع استدعاء)
        let decorator = self.parse_expression()?;
        
        // التالي يجب أن يكون دالة أو صنف
        let target = self.parse_statement()?;
        
        Ok(Stmt::Decorated {
            decorator,
            target: Box::new(target),
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::Expression(expr))
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_ternary()?;

        let compound_ops = [
            TokenType::PlusAssign,
            TokenType::MinusAssign,
            TokenType::MultAssign,
            TokenType::DivAssign,
            TokenType::ModAssign,
            TokenType::PowAssign,
        ];

        if self.match_token(&[TokenType::Assign]) {
            self.advance();
            let value = self.parse_assignment()?;
            return Ok(Expr::Assignment {
                target: Box::new(expr),
                value: Box::new(value),
            });
        } else if self.match_token(&compound_ops) {
            if let Expr::Identifier(name) = &expr {
                let op = match self.current_token().token_type {
                    TokenType::PlusAssign => BinaryOp::Add,
                    TokenType::MinusAssign => BinaryOp::Sub,
                    TokenType::MultAssign => BinaryOp::Mul,
                    TokenType::DivAssign => BinaryOp::Div,
                    TokenType::ModAssign => BinaryOp::Mod,
                    TokenType::PowAssign => BinaryOp::Pow,
                    _ => unreachable!(),
                };
                let name = name.clone();
                self.advance();
                let value = self.parse_assignment()?;
                return Ok(Expr::CompoundAssignment {
                    name,
                    op,
                    value: Box::new(value),
                });
            }
        }

        Ok(expr)
    }

    fn parse_ternary(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_null_coalescing()?;

        if self.match_token(&[TokenType::Question]) {
            self.advance();
            let then_expr = self.parse_expression()?;
            self.expect(TokenType::Colon)?;
            let else_expr = self.parse_ternary()?;
            return Ok(Expr::Ternary {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            });
        }

        Ok(expr)
    }

    fn parse_null_coalescing(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_or()?;

        while self.match_token(&[TokenType::QuestionQuestion]) {
            self.advance();
            let right = self.parse_or()?;
            left = Expr::NullCoalescing {
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;

        while self.match_token(&[TokenType::Or]) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Logical {
                left: Box::new(left),
                op: LogicalOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_equality()?;

        while self.match_token(&[TokenType::And]) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Logical {
                left: Box::new(left),
                op: LogicalOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;

        while self.match_token(&[TokenType::Equal, TokenType::NotEqual]) {
            let op = match self.current_token().token_type {
                TokenType::Equal => ComparisonOp::Equal,
                TokenType::NotEqual => ComparisonOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Comparison {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_bitwise()?;

        while self.match_token(&[
            TokenType::Less,
            TokenType::Greater,
            TokenType::LessEqual,
            TokenType::GreaterEqual,
        ]) {
            let op = match self.current_token().token_type {
                TokenType::Less => ComparisonOp::Less,
                TokenType::Greater => ComparisonOp::Greater,
                TokenType::LessEqual => ComparisonOp::LessEqual,
                TokenType::GreaterEqual => ComparisonOp::GreaterEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_bitwise()?;
            left = Expr::Comparison {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_bitwise(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_shift()?;

        while self.match_token(&[TokenType::BitAnd, TokenType::BitOr, TokenType::BitXor]) {
            let op = match self.current_token().token_type {
                TokenType::BitAnd => BinaryOp::BitAnd,
                TokenType::BitOr => BinaryOp::BitOr,
                TokenType::BitXor => BinaryOp::BitXor,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_shift()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_additive()?;

        while self.match_token(&[TokenType::ShiftLeft, TokenType::ShiftRight]) {
            let op = match self.current_token().token_type {
                TokenType::ShiftLeft => BinaryOp::ShiftLeft,
                TokenType::ShiftRight => BinaryOp::ShiftRight,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let op = match self.current_token().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_power()?;

        while self.match_token(&[
            TokenType::Multiply,
            TokenType::Divide,
            TokenType::Modulo,
            TokenType::FloorDiv,
        ]) {
            let op = match self.current_token().token_type {
                TokenType::Multiply => BinaryOp::Mul,
                TokenType::Divide => BinaryOp::Div,
                TokenType::Modulo => BinaryOp::Mod,
                TokenType::FloorDiv => BinaryOp::FloorDiv,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_unary()?;

        if self.match_token(&[TokenType::Power]) {
            self.advance();
            let right = self.parse_power()?;
            Ok(Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Pow,
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Minus]) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            });
        }
        if self.match_token(&[TokenType::Not]) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            });
        }
        if self.match_token(&[TokenType::BitNot]) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::BitNot,
                expr: Box::new(expr),
            });
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_call()?;

        if self.match_token(&[TokenType::Increment]) {
            self.advance();
            if let Expr::Identifier(name) = expr {
                return Ok(Expr::Increment {
                    name,
                    is_prefix: false,
                    delta: 1.0,
                });
            }
        } else if self.match_token(&[TokenType::Decrement]) {
            self.advance();
            if let Expr::Identifier(name) = expr {
                return Ok(Expr::Increment {
                    name,
                    is_prefix: false,
                    delta: -1.0,
                });
            }
        }

        Ok(expr)
    }

    fn parse_call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&[TokenType::LParen]) {
                self.advance();
                let args = self.parse_arguments()?;
                self.expect(TokenType::RParen)?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.match_token(&[TokenType::Dot]) {
                self.advance();
                match &self.current_token().token_type.clone() {
                    TokenType::Identifier(name) => {
                        let name = name.clone();
                        self.advance();
                        expr = Expr::Property {
                            object: Box::new(expr),
                            property: name,
                        };
                    }
                    _ => {
                        let token = self.current_token();
                        return Err(ParseError {
                            message: "توقع اسم خاصية بعد '.'".to_string(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                }
            } else if self.match_token(&[TokenType::QuestionDot]) {
                // Optional chaining: obj?.property
                self.advance();
                match &self.current_token().token_type.clone() {
                    TokenType::Identifier(name) => {
                        let name = name.clone();
                        self.advance();
                        expr = Expr::OptionalProperty {
                            object: Box::new(expr),
                            property: name,
                        };
                    }
                    TokenType::LBracket => {
                        // obj?.[index]
                        self.advance();
                        let index = self.parse_expression()?;
                        self.expect(TokenType::RBracket)?;
                        expr = Expr::OptionalIndex {
                            object: Box::new(expr),
                            index: Box::new(index),
                        };
                    }
                    TokenType::LParen => {
                        // obj?.(args)
                        self.advance();
                        let args = self.parse_arguments()?;
                        self.expect(TokenType::RParen)?;
                        expr = Expr::OptionalCall {
                            callee: Box::new(expr),
                            args,
                        };
                    }
                    _ => {
                        let token = self.current_token();
                        return Err(ParseError {
                            message: "توقع اسم خاصية أو فهرس أو استدعاء بعد '؟.'".to_string(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                }
            } else if self.match_token(&[TokenType::LBracket]) {
                self.advance();
                let index = self.parse_expression()?;
                self.expect(TokenType::RBracket)?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&[TokenType::Pipe]) {
                // Pipe operator: value |> function
                self.advance();
                let func = self.parse_call()?; // Right-associative
                expr = Expr::Pipe {
                    value: Box::new(expr),
                    function: Box::new(func),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();

        if self.match_token(&[TokenType::RParen]) {
            return Ok(args);
        }

        loop {
            args.push(self.parse_expression()?);

            if self.match_token(&[TokenType::Comma]) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(args)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.current_token().clone();

        match &token.token_type {
            TokenType::Number(n) => {
                let n = *n;
                self.advance();
                Ok(Expr::Number(n))
            }
            TokenType::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(self.parse_format_string_if_needed(s))
            }
            TokenType::Boolean(b) => {
                let b = *b;
                self.advance();
                Ok(Expr::Boolean(b))
            }
            TokenType::Null | TokenType::NullKeyword => {
                self.advance();
                Ok(Expr::Null)
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Boolean(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Boolean(false))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();

                // دعم ++ و -- قبل
                Ok(Expr::Identifier(name))
            }
            TokenType::TypeOf | TokenType::Length => {
                let name = token.token_type.to_string();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            TokenType::Function => self.parse_function_expression(),
            TokenType::Increment => {
                self.advance();
                let name = self.parse_identifier()?;
                Ok(Expr::Increment {
                    name,
                    is_prefix: true,
                    delta: 1.0,
                })
            }
            TokenType::Decrement => {
                self.advance();
                let name = self.parse_identifier()?;
                Ok(Expr::Increment {
                    name,
                    is_prefix: true,
                    delta: -1.0,
                })
            }
            TokenType::LParen => {
                self.advance();

                // تحقق من لامدا: (أ, ب) => ...
                if self.is_lambda() {
                    return self.parse_lambda_from_params();
                }

                let expr = self.parse_expression()?;
                self.expect(TokenType::RParen)?;
                Ok(expr)
            }
            TokenType::LBracket => self.parse_list(),
            TokenType::LBrace => self.parse_dictionary(),
            TokenType::Lambda => self.parse_lambda(),
            TokenType::Await => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Expr::Await(Box::new(expr)))
            }
            TokenType::This => {
                self.advance();
                Ok(Expr::Identifier("هذا".to_string()))
            }
            TokenType::New => {
                self.advance();
                let name = self.parse_identifier()?;
                self.expect(TokenType::LParen)?;
                let args = self.parse_arguments()?;
                self.expect(TokenType::RParen)?;
                Ok(Expr::Call {
                    callee: Box::new(Expr::Identifier(format!("__جديد_{}", name))),
                    args,
                })
            }
            _ => Err(ParseError {
                message: format!("تعبير غير متوقع: {}", token.token_type),
                line: token.line,
                column: token.column,
            }),
        }
    }

    fn parse_format_string_if_needed(&self, s: String) -> Expr {
        if !s.contains('{') {
            return Expr::String(s);
        }

        let mut parts = Vec::new();
        let mut current_literal = String::new();
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                if chars.peek() == Some(&'{') {
                    chars.next();
                    current_literal.push('{');
                    continue;
                }

                let mut expr_str = String::new();
                for ec in chars.by_ref() {
                    if ec == '}' {
                        break;
                    }
                    expr_str.push(ec);
                }

                if !current_literal.is_empty() {
                    parts.push(FormatPart::Literal(current_literal.clone()));
                    current_literal.clear();
                }

                if let Ok(program) = Parser::parse(&expr_str) {
                    if let Some(crate::parser::ast::Stmt::Expression(expr)) =
                        program.statements.into_iter().next()
                    {
                        parts.push(FormatPart::Expression(expr));
                        continue;
                    }
                }
                parts.push(FormatPart::Literal(format!("{{{}}}", expr_str)));
            } else if ch == '}' && chars.peek() == Some(&'}') {
                chars.next();
                current_literal.push('}');
            } else {
                current_literal.push(ch);
            }
        }

        if !current_literal.is_empty() {
            parts.push(FormatPart::Literal(current_literal));
        }

        if parts.iter().all(|p| matches!(p, FormatPart::Literal(_))) {
            Expr::String(s)
        } else {
            Expr::FormatString(parts)
        }
    }

    fn is_lambda(&self) -> bool {
        let mut pos = self.current;
        let mut depth = 0i32;

        while pos < self.tokens.len() {
            match &self.tokens[pos].token_type {
                TokenType::LParen => depth += 1,
                TokenType::RParen => {
                    depth -= 1;
                    if depth == 0 {
                        if let Some(next) = self.tokens.get(pos + 1) {
                            return matches!(
                                next.token_type,
                                TokenType::FatArrow | TokenType::Arrow
                            );
                        }
                        return false;
                    }
                }
                TokenType::EOF => return false,
                _ => {}
            }
            pos += 1;
        }
        false
    }

    fn parse_lambda_from_params(&mut self) -> Result<Expr, ParseError> {
        let mut params = Vec::new();

        if !self.match_token(&[TokenType::RParen]) {
            loop {
                params.push(self.parse_identifier()?);
                if self.match_token(&[TokenType::Comma]) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.expect(TokenType::RParen)?;

        if self.match_token(&[TokenType::FatArrow]) || self.match_token(&[TokenType::Arrow]) {
            self.advance();
        }

        let body = self.parse_expression()?;
        Ok(Expr::Lambda {
            params,
            body: Box::new(body),
        })
    }

    fn parse_function_expression(&mut self) -> Result<Expr, ParseError> {
        self.advance();

        self.expect(TokenType::LParen)?;
        let mut params = Vec::new();

        if !self.match_token(&[TokenType::RParen]) {
            loop {
                params.push(self.parse_identifier()?);
                if self.match_token(&[TokenType::Comma]) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.expect(TokenType::RParen)?;

        if !self.match_token(&[TokenType::LBrace]) {
            let token = self.current_token();
            let span = self.token_span(token);
            let mut rich_error = error_helpers::unexpected_token(
                &token.token_type.to_string(),
                &TokenType::LBrace.to_string(),
                span,
            )
            .with_help("الدالة المجهولة تتطلب جسماً بين أقواس معقوفة".to_string());

            if let Some(context) = self.source_line(token.line) {
                rich_error = rich_error.with_source_context(context.to_string());
            }

            return Err(ParseError {
                message: rich_error.format("<النص>"),
                line: token.line,
                column: token.column,
            });
        }

        let block = self.parse_block()?;
        let body = self.extract_lambda_body_from_block(block)?;

        Ok(Expr::Lambda {
            params,
            body: Box::new(body),
        })
    }

    fn extract_lambda_body_from_block(&self, block: Stmt) -> Result<Expr, ParseError> {
        if let Stmt::Block(stmts) = block {
            match stmts.as_slice() {
                [Stmt::Return(Some(expr))] => Ok(expr.clone()),
                [Stmt::Expression(expr)] => Ok(expr.clone()),
                _ => Err(ParseError {
                    message: "جسم الدالة المجهولة يجب أن يحتوي تعبيراً واحداً أو أرجع تعبيراً"
                        .to_string(),
                    line: 1,
                    column: 1,
                }),
            }
        } else {
            Err(ParseError {
                message: "خطأ داخلي: متوقع كتلة".to_string(),
                line: 1,
                column: 1,
            })
        }
    }

    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {
        self.advance();

        let mut params = Vec::new();
        if self.match_token(&[TokenType::LParen]) {
            self.advance();
            if !self.match_token(&[TokenType::RParen]) {
                loop {
                    params.push(self.parse_identifier()?);
                    if self.match_token(&[TokenType::Comma]) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            self.expect(TokenType::RParen)?;
        } else {
            params.push(self.parse_identifier()?);
        }

        if self.match_token(&[TokenType::Arrow]) || self.match_token(&[TokenType::FatArrow]) {
            self.advance();
        }

        let body = if self.match_token(&[TokenType::LBrace]) {
            let block = self.parse_block()?;
            self.extract_lambda_body_from_block(block)?
        } else {
            self.parse_expression()?
        };

        Ok(Expr::Lambda {
            params,
            body: Box::new(body),
        })
    }

    fn parse_list(&mut self) -> Result<Expr, ParseError> {
        self.expect(TokenType::LBracket)?;
        
        // Check for empty list
        if self.match_token(&[TokenType::RBracket]) {
            self.advance();
            return Ok(Expr::List(Vec::new()));
        }
        
        // Check for spread operator at the beginning
        if self.match_token(&[TokenType::DotDotDot]) {
            self.advance();
            let spread_expr = self.parse_expression()?;
            let mut elements = vec![Expr::Spread(Box::new(spread_expr))];
            
            if self.match_token(&[TokenType::Comma]) {
                self.advance();
                while !self.match_token(&[TokenType::RBracket]) {
                    if self.match_token(&[TokenType::DotDotDot]) {
                        self.advance();
                        elements.push(Expr::Spread(Box::new(self.parse_expression()?)));
                    } else {
                        elements.push(self.parse_expression()?);
                    }
                    if self.match_token(&[TokenType::Comma]) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            
            self.expect(TokenType::RBracket)?;
            return Ok(Expr::List(elements));
        }
        
        // Parse first expression
        let first = self.parse_expression()?;
        
        // Check for list comprehension: [تعبير لكل عنصر في قابل_التكرار إذا شرط]
        if self.match_token(&[TokenType::For]) {
            self.advance();
            let variable = self.parse_identifier()?;
            self.expect(TokenType::In)?;
            let iterable = self.parse_expression()?;
            
            let condition = if self.match_token(&[TokenType::If]) {
                self.advance();
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };
            
            self.expect(TokenType::RBracket)?;
            return Ok(Expr::ListComprehension {
                element: Box::new(first),
                variable,
                iterable: Box::new(iterable),
                condition,
            });
        }
        
        // Regular list
        let mut elements = vec![first];
        
        if self.match_token(&[TokenType::Comma]) {
            self.advance();
            while !self.match_token(&[TokenType::RBracket]) {
                if self.match_token(&[TokenType::DotDotDot]) {
                    self.advance();
                    elements.push(Expr::Spread(Box::new(self.parse_expression()?)));
                } else {
                    elements.push(self.parse_expression()?);
                }
                if self.match_token(&[TokenType::Comma]) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.expect(TokenType::RBracket)?;
        Ok(Expr::List(elements))
    }

    fn parse_dictionary(&mut self) -> Result<Expr, ParseError> {
        self.expect(TokenType::LBrace)?;
        
        // Check for empty dictionary
        if self.match_token(&[TokenType::RBrace]) {
            self.advance();
            return Ok(Expr::Dictionary(Vec::new()));
        }

        // Parse first key
        let first_key = match &self.current_token().token_type.clone() {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Expr::String(name)
            }
            TokenType::String(name) => {
                let name = name.clone();
                self.advance();
                Expr::String(name)
            }
            TokenType::Number(n) => {
                let n = *n;
                self.advance();
                Expr::Number(n)
            }
            TokenType::LBracket => {
                self.advance();
                let e = self.parse_expression()?;
                self.expect(TokenType::RBracket)?;
                e
            }
            _ => {
                let token = self.current_token();
                return Err(ParseError {
                    message: "توقع مفتاح القاموس".to_string(),
                    line: token.line,
                    column: token.column,
                });
            }
        };

        // Expect colon
        self.expect(TokenType::Colon)?;
        let first_value = self.parse_expression()?;

        // Check for dictionary comprehension: {key: value لكل عنصر في قابل_التكرار إذا شرط}
        if self.match_token(&[TokenType::For]) {
            self.advance();
            let variable = self.parse_identifier()?;
            self.expect(TokenType::In)?;
            let iterable = self.parse_expression()?;

            let condition = if self.match_token(&[TokenType::If]) {
                self.advance();
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };

            self.expect(TokenType::RBrace)?;
            return Ok(Expr::DictComprehension {
                key: Box::new(first_key),
                value: Box::new(first_value),
                variable,
                iterable: Box::new(iterable),
                condition,
            });
        }

        // Regular dictionary
        let mut entries = vec![(first_key, first_value)];

        loop {
            if matches!(self.current_token().token_type, TokenType::Comment(_)) {
                self.advance();
                continue;
            }

            if self.match_token(&[TokenType::Comma]) {
                self.advance();
                if self.match_token(&[TokenType::RBrace]) {
                    break;
                }
            } else {
                break;
            }

            let key = match &self.current_token().token_type.clone() {
                TokenType::Identifier(name) => {
                    let name = name.clone();
                    self.advance();
                    Expr::String(name)
                }
                TokenType::String(name) => {
                    let name = name.clone();
                    self.advance();
                    Expr::String(name)
                }
                TokenType::Number(n) => {
                    let n = *n;
                    self.advance();
                    Expr::Number(n)
                }
                TokenType::LBracket => {
                    self.advance();
                    let e = self.parse_expression()?;
                    self.expect(TokenType::RBracket)?;
                    e
                }
                _ => {
                    let token = self.current_token();
                    return Err(ParseError {
                        message: "توقع مفتاح القاموس".to_string(),
                        line: token.line,
                        column: token.column,
                    });
                }
            };

            self.expect(TokenType::Colon)?;
            let value = self.parse_expression()?;
            entries.push((key, value));
        }

        self.expect(TokenType::RBrace)?;
        Ok(Expr::Dictionary(entries))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expression() {
        let input = "١ + ٢";
        let result = Parser::parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_variable_declaration() {
        let input = "متغير س = ١٠؛";
        let result = Parser::parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function() {
        let input = r#"
            دالة جمع(أ، ب) {
                أرجع أ + ب؛
            }
        "#;
        let result = Parser::parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_repeat() {
        let input = r#"كرر ٥ مرات { اطبع("مرحبا")؛ }"#;
        let result = Parser::parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_catch() {
        let input = r#"حاول { اطبع("حاول")؛ } امسك(خ) { اطبع(خ)؛ }"#;
        let result = Parser::parse(input);
        assert!(result.is_ok());
    }
}
