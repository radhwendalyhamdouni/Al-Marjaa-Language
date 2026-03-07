pub mod tokens;

use tokens::{Token, TokenType};
use unicode_segmentation::UnicodeSegmentation;

/// محسن للـ Lexer مع String Interning
/// يحافظ على الأداء العالي ويقلل من استهلاك الذاكرة
pub struct Lexer {
    position: usize,
    line: usize,
    column: usize,
    current_char: Option<char>,
    current_grapheme: Option<String>,
    graphemes: Vec<String>,
    /// كاشش للكلمات المفتاحية العربية
    keywords_cache: std::collections::HashMap<&'static str, TokenType>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let graphemes: Vec<String> = input.graphemes(true).map(|g| g.to_string()).collect();
        let first_grapheme = graphemes.first().cloned();
        let first_char = first_grapheme.as_ref().and_then(|g| g.chars().next());
        
        // إنشاء كاشش للكلمات المفتاحية
        let mut keywords_cache = std::collections::HashMap::with_capacity(100);
        
        // الكلمات المفتاحية العربية
        let arabic_keywords = [
            ("دالة", TokenType::Function),
            ("أرجع", TokenType::Return), ("ارجع", TokenType::Return),
            ("إذا", TokenType::If), ("اذا", TokenType::If),
            ("وإلا", TokenType::Else), ("والا", TokenType::Else), ("غير_ذلك", TokenType::Else),
            ("وإذا", TokenType::ElseIf), ("واذا", TokenType::ElseIf),
            ("طالما", TokenType::While),
            ("لكل", TokenType::For),
            ("في", TokenType::In),
            ("توقف", TokenType::Break),
            ("أكمل", TokenType::Continue), ("اكمل", TokenType::Continue),
            ("متغير", TokenType::Let), ("م", TokenType::Let),
            ("ثابت", TokenType::Const), ("ث", TokenType::Const),
            ("صح", TokenType::True),
            ("خطأ", TokenType::False), ("خطا", TokenType::False),
            ("لا_شيء", TokenType::NullKeyword), ("لاشيء", TokenType::NullKeyword),
            ("صنف", TokenType::Class),
            ("هذا", TokenType::This),
            ("جديد", TokenType::New),
            ("استورد", TokenType::Import),
            ("صدر", TokenType::Export),
            ("حاول", TokenType::Try),
            ("امسك", TokenType::Catch),
            ("أخيراً", TokenType::Finally), ("اخيراً", TokenType::Finally),
            ("أخيرا", TokenType::Finally), ("اخيرا", TokenType::Finally),
            ("ألقِ", TokenType::Throw), ("الق", TokenType::Throw),
            ("اطبع", TokenType::Print), ("طبع", TokenType::Print), ("أطبع", TokenType::Print),
            ("ادخل", TokenType::Input), ("دخل", TokenType::Input),
            ("نوع", TokenType::TypeOf),
            ("طول", TokenType::Length),
            ("و", TokenType::And),
            ("أو", TokenType::Or), ("او", TokenType::Or),
            ("ليس", TokenType::Not),
            ("احذف", TokenType::Delete),
            ("كرر", TokenType::Repeat),
            ("مرة", TokenType::Times), ("مرات", TokenType::Times),
            ("طابق", TokenType::Match),
            ("حالة", TokenType::Case),
            ("افتراضي", TokenType::Default),
            ("لامدا", TokenType::Lambda), ("دالة_صغيرة", TokenType::Lambda),
            ("غير_متزامن", TokenType::Async),
            ("انتظر", TokenType::Await),
            ("أعطِ", TokenType::Yield), ("اعط", TokenType::Yield),
            ("وحدة", TokenType::Module),
            ("استخدم", TokenType::Use),
            ("بوصف", TokenType::As),
            ("من", TokenType::From),
            ("مدى", TokenType::Range),
            ("خطوة", TokenType::Step),
            ("تأكد", TokenType::Assert), ("تاكد", TokenType::Assert),
            ("نوع_من", TokenType::Typeof),
            ("نوع_مثل", TokenType::Instanceof),
            ("أصل", TokenType::Super), ("اصل", TokenType::Super),
            ("مع", TokenType::With),
            ("كـ", TokenType::AsKeyword), ("ك", TokenType::AsKeyword),
            ("تعداد", TokenType::Enum),
            ("بيانات", TokenType::Data),
            ("واجهة", TokenType::Interface),
            ("ثيم", TokenType::Theme),
            ("موجه", TokenType::Router), ("موجّه", TokenType::Router), ("مسار", TokenType::Router),
            ("حدث", TokenType::Event),
            // ONNX والذكاء الاصطناعي المتقدم
            ("أونكس", TokenType::ONNX), ("onnx", TokenType::ONNX),
            ("نموذج", TokenType::Model),
            ("حمّل", TokenType::Load), ("حمل", TokenType::Load),
            ("احفظ", TokenType::Save), ("حفظ", TokenType::Save),
            ("استدل", TokenType::Infer), ("استدلال", TokenType::Infer),
            ("موتر", TokenType::Tensor),
            ("شكل", TokenType::Shape),
            ("مخرج", TokenType::Output),
            ("طبقة", TokenType::Layer),
            ("كثيف", TokenType::Dense), ("خطي", TokenType::Dense),
            ("التفاف", TokenType::Conv), ("conv", TokenType::Conv),
            ("تجميع", TokenType::Pool), ("pool", TokenType::Pool),
            ("طبع", TokenType::Normalize), ("تسوية_قيم", TokenType::Normalize),
            ("إسقاط", TokenType::Dropout), ("اسقاط", TokenType::Dropout),
            ("تسوية", TokenType::Flatten), ("flatten", TokenType::Flatten),
            ("إعادة_تشكيل", TokenType::Reshape), ("reshape", TokenType::Reshape),
            ("تنشيط", TokenType::Activation),
            ("سوفت_ماكس", TokenType::Softmax), ("softmax", TokenType::Softmax),
            ("ريلو", TokenType::Relu), ("relu", TokenType::Relu),
            ("سيجمويد", TokenType::Sigmoid), ("sigmoid", TokenType::Sigmoid),
            ("دفعة", TokenType::Batch), ("batch", TokenType::Batch),
            ("درّب", TokenType::Train), ("درب", TokenType::Train), ("تدريب", TokenType::Train),
            ("توقع", TokenType::Predict), ("تنبؤ", TokenType::Predict),
            ("محسّن", TokenType::Optimizer), ("محقق", TokenType::Optimizer),
            ("خسارة", TokenType::Loss),
            // واجهات المستخدم المتقدمة
            // التخطيط
            ("صف", TokenType::Row), ("صف_أفقي", TokenType::Row),
            ("عمود", TokenType::Column), ("صف_عمودي", TokenType::Column),
            ("شبكة", TokenType::Grid),
            ("مرن", TokenType::Flex),
            ("كومة", TokenType::Stack),
            ("التفاف", TokenType::Wrap),
            ("فجوة", TokenType::Gap),
            ("محاذاة", TokenType::Align),
            ("تبرير", TokenType::Justify),
            ("حشو", TokenType::Padding),
            ("هامش", TokenType::Margin),
            // المكونات
            ("زر", TokenType::Button),
            ("نص", TokenType::Text),
            ("إدخال", TokenType::Input), ("ادخال", TokenType::Input),
            ("اختيار", TokenType::Select),
            ("خانة", TokenType::Checkbox),
            ("راديو", TokenType::Radio),
            ("منزلق", TokenType::Slider),
            ("تقدم", TokenType::Progress),
            ("مؤقت", TokenType::Spinner),
            ("بطاقة", TokenType::Card),
            ("قائمة", TokenType::List),
            ("جدول", TokenType::Table),
            ("نموذج", TokenType::Form),
            ("تسمية", TokenType::Label),
            ("صورة", TokenType::Image),
            ("أيقونة", TokenType::Icon), ("ايقونة", TokenType::Icon),
            ("شارة", TokenType::Badge),
            ("صورة_شخصية", TokenType::Avatar),
            ("تلميح", TokenType::Tooltip),
            ("نافذة", TokenType::Modal), ("نافذة_منبثقة", TokenType::Modal),
            ("تنبيه", TokenType::Toast),
            ("منبثق", TokenType::Popup),
            // الثيمات
            ("لون", TokenType::Color),
            ("خط", TokenType::Font),
            ("حجم", TokenType::Size),
            ("عرض", TokenType::Width),
            ("ارتفاع", TokenType::Height),
            ("حدود", TokenType::Border),
            ("ظل", TokenType::Shadow),
            ("خلفية", TokenType::Background),
            // الرسوم البيانية
            ("رسم", TokenType::Chart), ("رسم_بياني", TokenType::Chart),
            ("رسم_خطي", TokenType::LineChart),
            ("رسم_أعمدة", TokenType::BarChart),
            ("رسم_دائري", TokenType::PieChart),
            ("رسم_مساحي", TokenType::AreaChart),
            // الرسوم المتحركة
            ("حرك", TokenType::Animate), ("حركة", TokenType::Animate),
            ("انتقال", TokenType::Transition),
            ("مدة", TokenType::Duration),
            ("تأخير", TokenType::Delay),
            ("تخفيف", TokenType::Easing),
            // الأحداث
            ("نقر", TokenType::Click),
            ("تغيير", TokenType::Change),
            ("إرسال", TokenType::Submit), ("ارسال", TokenType::Submit),
            ("تركيز", TokenType::Focus),
            ("ضبابية", TokenType::Blur),
            ("تحويم", TokenType::Hover),
            ("تمرير", TokenType::Scroll),
            // الربط
            ("ربط", TokenType::Bind),
            ("راقب", TokenType::Observe),
            ("محسوب", TokenType::Computed),
            ("راقب_التغييرات", TokenType::Watch),
            // الكلمات المفتاحية الإنجليزية
            ("fn", TokenType::Function),
            ("true", TokenType::True),
            ("false", TokenType::False),
            ("null", TokenType::NullKeyword),
        ];
        
        for (keyword, token_type) in arabic_keywords {
            keywords_cache.insert(keyword, token_type);
        }
        
        Lexer {
            position: 0,
            line: 1,
            column: 1,
            current_char: first_char,
            current_grapheme: first_grapheme,
            graphemes,
            keywords_cache,
        }
    }

    #[inline(always)]
    fn advance(&mut self) {
        if let Some(ch) = self.current_char {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.position += 1;
        let next_grapheme = self.graphemes.get(self.position).cloned();
        self.current_char = next_grapheme.as_ref().and_then(|g| g.chars().next());
        self.current_grapheme = next_grapheme;
    }

    #[inline(always)]
    fn peek_char(&self) -> Option<char> {
        self.graphemes
            .get(self.position + 1)
            .map(|g| g.chars().next().unwrap())
    }

    #[inline(always)]
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_slash_comment(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;
        let mut comment = String::new();

        // تجاوز //
        self.advance();
        self.advance();

        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            comment.push(ch);
            self.advance();
        }

        Token::new(TokenType::Comment(comment), start_line, start_col)
    }

    fn skip_block_comment(&mut self) -> Result<(), String> {
        self.advance();
        self.advance();
        loop {
            match self.current_char {
                None => return Err(format!("تعليق غير مغلق في السطر {}", self.line)),
                Some('*') => {
                    self.advance();
                    if self.current_char == Some('/') {
                        self.advance();
                        return Ok(());
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn read_string(&mut self) -> Result<String, String> {
        let quote = self.current_char.unwrap();
        self.advance();

        let mut result = String::with_capacity(32);

        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance();
                return Ok(result);
            } else if ch == '\\' {
                self.advance();
                if let Some(escape_ch) = self.current_char {
                    match escape_ch {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        '\'' => result.push('\''),
                        '0' => result.push('\0'),
                        _ => {
                            result.push('\\');
                            result.push(escape_ch);
                        }
                    }
                    self.advance();
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err(format!("خطأ في السطر {}: نص غير مغلق", self.line))
    }

    fn read_multiline_string(&mut self) -> Result<String, String> {
        self.advance();
        self.advance();
        self.advance();

        let mut result = String::with_capacity(128);
        loop {
            match self.current_char {
                None => return Err(format!("نص متعدد الأسطر غير مغلق في السطر {}", self.line)),
                Some('`') => {
                    self.advance();
                    return Ok(result);
                }
                Some(ch) => {
                    result.push(ch);
                    self.advance();
                }
            }
        }
    }

    #[inline(always)]
    fn read_number(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;
        let mut num_str = String::with_capacity(32);
        let mut has_dot = false;

        // دالة تحويل الأرقام العربية محسّنة
        #[inline(always)]
        fn arabic_to_ascii(ch: char) -> char {
            match ch {
                '٠' => '0', '١' => '1', '٢' => '2', '٣' => '3', '٤' => '4',
                '٥' => '5', '٦' => '6', '٧' => '7', '٨' => '8', '٩' => '9',
                _ => ch,
            }
        }

        // دعم النظام الست عشري: 0x...
        if self.current_char == Some('0') {
            if let Some(next) = self.peek_char() {
                if next == 'x' || next == 'X' {
                    self.advance();
                    self.advance();
                    let mut hex_str = String::from("0x");
                    while let Some(ch) = self.current_char {
                        if ch.is_ascii_hexdigit() {
                            hex_str.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let num = i64::from_str_radix(&hex_str[2..], 16).unwrap_or(0) as f64;
                    return Token::new(TokenType::Number(num), start_line, start_col);
                } else if next == 'b' || next == 'B' {
                    self.advance();
                    self.advance();
                    let mut bin_str = String::new();
                    while let Some(ch) = self.current_char {
                        if ch == '0' || ch == '1' {
                            bin_str.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let num = i64::from_str_radix(&bin_str, 2).unwrap_or(0) as f64;
                    return Token::new(TokenType::Number(num), start_line, start_col);
                }
            }
        }

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ('\u{0660}'..='\u{0669}').contains(&ch) {
                num_str.push(arabic_to_ascii(ch));
                self.advance();
            } else if (ch == '.' || ch == '٫') && !has_dot {
                if let Some(next) = self.peek_char() {
                    if next.is_ascii_digit() || ('\u{0660}'..='\u{0669}').contains(&next) {
                        has_dot = true;
                        num_str.push('.');
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else if ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let num = num_str.parse::<f64>().unwrap_or(0.0);
        Token::new(TokenType::Number(num), start_line, start_col)
    }

    fn read_identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;
        let mut ident = String::with_capacity(32);

        // دالة مساعدة للتحقق من أحرف التشكيل العربية
        #[inline(always)]
        fn is_arabic_diacritic(ch: char) -> bool {
            matches!(ch, '\u{064B}'..='\u{065F}' | '\u{0670}')
        }

        // دالة للتحقق مما إذا كان الـ grapheme هو جزء من معرف
        #[inline(always)]
        fn is_identifier_grapheme(g: &str) -> bool {
            if let Some(first_char) = g.chars().next() {
                if first_char.is_alphanumeric() || first_char == '_' || first_char == 'ـ' || first_char == '\'' {
                    return true;
                }
                for ch in g.chars() {
                    if is_arabic_diacritic(ch) {
                        return true;
                    }
                }
            }
            false
        }

        while let Some(ref grapheme) = self.current_grapheme {
            if is_identifier_grapheme(grapheme) {
                ident.push_str(grapheme);
                self.advance();
            } else {
                break;
            }
        }

        // البحث في الكاش أولاً
        let token_type = self.keywords_cache
            .get(ident.as_str())
            .cloned()
            .unwrap_or_else(|| TokenType::Identifier(ident));
        
        Token::new(token_type, start_line, start_col)
    }

    fn read_comment(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;
        let mut comment = String::new();

        self.advance();

        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            comment.push(ch);
            self.advance();
        }

        Token::new(TokenType::Comment(comment), start_line, start_col)
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        let start_line = self.line;
        let start_col = self.column;

        match self.current_char {
            None => Ok(Token::new(TokenType::EOF, start_line, start_col)),

            Some('#') => Ok(self.read_comment()),

            Some('/') if self.peek_char() == Some('/') => Ok(self.read_slash_comment()),

            Some('/') if self.peek_char() == Some('*') => {
                self.skip_block_comment()?;
                self.next_token()
            }

            Some('`') => match self.read_multiline_string() {
                Ok(s) => Ok(Token::new(TokenType::String(s), start_line, start_col)),
                Err(e) => Err(e),
            },

            Some(ch) if ch.is_ascii_digit() || ('\u{0660}'..='\u{0669}').contains(&ch) => {
                Ok(self.read_number())
            }

            Some(ch) if ch.is_alphabetic() || ch == '_' => Ok(self.read_identifier()),

            Some(ch) if matches!(ch, '\u{064B}'..='\u{065F}' | '\u{0670}') => {
                Ok(self.read_identifier())
            }

            Some('"') | Some('\'') => match self.read_string() {
                Ok(s) => Ok(Token::new(TokenType::String(s), start_line, start_col)),
                Err(e) => Err(e),
            },

            Some('+') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::PlusAssign, start_line, start_col))
                } else if self.current_char == Some('+') {
                    self.advance();
                    Ok(Token::new(TokenType::Increment, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Plus, start_line, start_col))
                }
            }

            Some('-') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::MinusAssign, start_line, start_col))
                } else if self.current_char == Some('>') {
                    self.advance();
                    Ok(Token::new(TokenType::Arrow, start_line, start_col))
                } else if self.current_char == Some('-') {
                    self.advance();
                    Ok(Token::new(TokenType::Decrement, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Minus, start_line, start_col))
                }
            }

            Some('*') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::MultAssign, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Multiply, start_line, start_col))
                }
            }

            Some('/') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::DivAssign, start_line, start_col))
                } else if self.current_char == Some('/') {
                    self.advance();
                    Ok(Token::new(TokenType::FloorDiv, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Divide, start_line, start_col))
                }
            }

            Some('%') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::ModAssign, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Modulo, start_line, start_col))
                }
            }

            Some('^') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::PowAssign, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Power, start_line, start_col))
                }
            }

            Some('=') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::Equal, start_line, start_col))
                } else if self.current_char == Some('>') {
                    self.advance();
                    Ok(Token::new(TokenType::FatArrow, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Assign, start_line, start_col))
                }
            }

            Some('!') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::NotEqual, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Not, start_line, start_col))
                }
            }

            Some('<') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::LessEqual, start_line, start_col))
                } else if self.current_char == Some('<') {
                    self.advance();
                    Ok(Token::new(TokenType::ShiftLeft, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Less, start_line, start_col))
                }
            }

            Some('>') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Ok(Token::new(TokenType::GreaterEqual, start_line, start_col))
                } else if self.current_char == Some('>') {
                    self.advance();
                    Ok(Token::new(TokenType::ShiftRight, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Greater, start_line, start_col))
                }
            }

            Some('&') => {
                self.advance();
                if self.current_char == Some('&') {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Ok(Token::new(TokenType::AndAssign, start_line, start_col))
                    } else {
                        Ok(Token::new(TokenType::And, start_line, start_col))
                    }
                } else {
                    Ok(Token::new(TokenType::BitAnd, start_line, start_col))
                }
            }

            Some('|') => {
                self.advance();
                if self.current_char == Some('|') {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Ok(Token::new(TokenType::OrAssign, start_line, start_col))
                    } else {
                        Ok(Token::new(TokenType::Or, start_line, start_col))
                    }
                } else if self.current_char == Some('>') {
                    self.advance();
                    Ok(Token::new(TokenType::Pipe, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::BitOr, start_line, start_col))
                }
            }

            Some('~') => {
                self.advance();
                if self.current_char == Some('~') {
                    self.advance();
                    Ok(Token::new(TokenType::BitXor, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::BitNot, start_line, start_col))
                }
            }

            Some('؛') | Some(';') => {
                self.advance();
                Ok(Token::new(TokenType::Semicolon, start_line, start_col))
            }

            Some('،') | Some(',') => {
                self.advance();
                Ok(Token::new(TokenType::Comma, start_line, start_col))
            }

            Some('.') => {
                self.advance();
                if self.current_char == Some('.') {
                    self.advance();
                    if self.current_char == Some('.') {
                        self.advance();
                        Ok(Token::new(TokenType::DotDotDot, start_line, start_col))
                    } else {
                        Ok(Token::new(TokenType::DotDot, start_line, start_col))
                    }
                } else {
                    Ok(Token::new(TokenType::Dot, start_line, start_col))
                }
            }

            Some(':') => {
                self.advance();
                Ok(Token::new(TokenType::Colon, start_line, start_col))
            }

            Some('?') | Some('؟') => {
                self.advance();
                if self.current_char == Some('.') {
                    self.advance();
                    Ok(Token::new(TokenType::QuestionDot, start_line, start_col))
                } else if self.current_char == Some('?') || self.current_char == Some('؟') {
                    self.advance();
                    Ok(Token::new(TokenType::QuestionQuestion, start_line, start_col))
                } else {
                    Ok(Token::new(TokenType::Question, start_line, start_col))
                }
            }

            Some('@') => {
                self.advance();
                Ok(Token::new(TokenType::At, start_line, start_col))
            }

            Some('(') => {
                self.advance();
                Ok(Token::new(TokenType::LParen, start_line, start_col))
            }

            Some(')') => {
                self.advance();
                Ok(Token::new(TokenType::RParen, start_line, start_col))
            }

            Some('{') => {
                self.advance();
                Ok(Token::new(TokenType::LBrace, start_line, start_col))
            }

            Some('}') => {
                self.advance();
                Ok(Token::new(TokenType::RBrace, start_line, start_col))
            }

            Some('[') => {
                self.advance();
                Ok(Token::new(TokenType::LBracket, start_line, start_col))
            }

            Some(']') => {
                self.advance();
                Ok(Token::new(TokenType::RBracket, start_line, start_col))
            }

            Some(ch) => Err(format!(
                "خطأ في السطر {}: رمز غير معروف '{}'",
                self.line, ch
            )),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::with_capacity(256);

        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.token_type, TokenType::EOF);
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("١٢٣ ٤٥.٦٧");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0].token_type, TokenType::Number(123.0)));
        assert!(matches!(tokens[1].token_type, TokenType::Number(45.67)));
    }

    #[test]
    fn test_arabic_keywords() {
        let mut lexer = Lexer::new("متغير س = ١٠؛");
        let tokens = lexer.tokenize().unwrap();
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[1].token_type, TokenType::Identifier(_)));
        assert!(matches!(tokens[2].token_type, TokenType::Assign));
    }

    #[test]
    fn test_hex_numbers() {
        let mut lexer = Lexer::new("0xFF");
        let tokens = lexer.tokenize().unwrap();
        assert!(matches!(tokens[0].token_type, TokenType::Number(255.0)));
    }

    #[test]
    fn test_short_keywords() {
        let mut lexer = Lexer::new("م س = ١٠؛");
        let tokens = lexer.tokenize().unwrap();
        assert!(matches!(tokens[0].token_type, TokenType::Let));
    }

    #[test]
    fn test_performance() {
        // اختبار أداء Lexer
        let large_input: String = (0..1000)
            .map(|i| format!("متغير س{} = {}؛\n", i, i))
            .collect();
        
        let start = std::time::Instant::now();
        let mut lexer = Lexer::new(&large_input);
        let tokens = lexer.tokenize().unwrap();
        let duration = start.elapsed();
        
        println!("Lexer tokenized {} tokens in {:?}", tokens.len(), duration);
        assert!(tokens.len() > 5000);
    }
}
