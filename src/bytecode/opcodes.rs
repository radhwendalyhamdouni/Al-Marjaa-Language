// ═══════════════════════════════════════════════════════════════════════════════
// تعليمات الـ Bytecode - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// هذه التعليمات تشكل لغة الآلة الافتراضية للغة المرجع
// مصممة لتكون سريعة ومدمجة في الذاكرة
// ═══════════════════════════════════════════════════════════════════════════════

use std::fmt;

/// تعليمة bytecode واحدة (1 byte opcode + operands)
#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    // ═══════════════════════════════════════════════════════════════
    // تعليمات المكدس (Stack Operations)
    // ═══════════════════════════════════════════════════════════════
    
    /// دفع رقم إلى المكدس
    /// PUSH_NUMBER <f64 as 8 bytes>
    PushNumber(f64),
    
    /// دفع نص إلى المكدس
    /// PUSH_STRING <string_index as u32>
    PushString(u32),
    
    /// دفع قيمة منطقية
    PushBool(bool),
    
    /// دفع null
    PushNull,
    
    /// إزالة قيمة من المكدس
    Pop,
    
    /// نسخ القيمة العليا
    Dup,
    
    /// تبديل القيمتين العلويتين
    Swap,
    
    // ═══════════════════════════════════════════════════════════════
    // العمليات الحسابية (Arithmetic Operations)
    // ═══════════════════════════════════════════════════════════════
    
    /// جمع: a + b
    Add,
    
    /// طرح: a - b
    Sub,
    
    /// ضرب: a * b
    Mul,
    
    /// قسمة: a / b
    Div,
    
    /// باقي القسمة: a % b
    Mod,
    
    /// أس: a ^ b
    Pow,
    
    /// سالب: -a
    Neg,
    
    /// XOR: a ^ b (bitwise)
    Xor,
    
    /// Shift Left: a << b
    Shl,
    
    /// Shift Right: a >> b
    Shr,
    
    // ═══════════════════════════════════════════════════════════════
    // العمليات المقارنة (Comparison Operations)
    // ═══════════════════════════════════════════════════════════════
    
    /// مساوٍ: a == b
    Equal,
    
    /// غير مساوٍ: a != b
    NotEqual,
    
    /// أصغر: a < b
    Less,
    
    /// أكبر: a > b
    Greater,
    
    /// أصغر أو يساوي: a <= b
    LessEqual,
    
    /// أكبر أو يساوي: a >= b
    GreaterEqual,
    
    // ═══════════════════════════════════════════════════════════════
    // العمليات المنطقية (Logical Operations)
    // ═══════════════════════════════════════════════════════════════
    
    /// و منطقي: a && b
    And,
    
    /// أو منطقي: a || b
    Or,
    
    /// نفي منطقي: !a
    Not,
    
    // ═══════════════════════════════════════════════════════════════
    // المتغيرات (Variables)
    // ═══════════════════════════════════════════════════════════════
    
    /// تحميل متغير عام
    /// LOAD_GLOBAL <name_index as u32>
    LoadGlobal(u32),
    
    /// تخزين متغير عام
    /// STORE_GLOBAL <name_index as u32>
    StoreGlobal(u32),
    
    /// تحميل متغير محلي
    /// LOAD_LOCAL <slot as u16>
    LoadLocal(u16),
    
    /// تخزين متغير محلي
    /// STORE_LOCAL <slot as u16>
    StoreLocal(u16),
    
    /// تعريف متغير ثابت
    /// DEFINE_CONST <name_index as u32>
    DefineConst(u32),
    
    // ═══════════════════════════════════════════════════════════════
    // التحكم في التدفق (Control Flow)
    // ═══════════════════════════════════════════════════════════════
    
    /// قفز غير مشروط
    /// JUMP <offset as i32>
    Jump(i32),
    
    /// قفز إذا خطأ
    /// JUMP_IF_FALSE <offset as i32>
    JumpIfFalse(i32),
    
    /// قفز إذا صحيح
    /// JUMP_IF_TRUE <offset as i32>
    JumpIfTrue(i32),
    
    /// قفز للخلف (للحلقات)
    /// JUMP_BACK <offset as i32>
    JumpBack(i32),
    
    // ═══════════════════════════════════════════════════════════════
    // الدوال (Functions)
    // ═══════════════════════════════════════════════════════════════
    
    /// استدعاء دالة
    /// CALL <arg_count as u8>
    Call(u8),
    
    /// استدعاء دالة أصلية (native)
    /// CALL_NATIVE <func_index as u32> <arg_count as u8>
    CallNative { func_index: u32, arg_count: u8 },
    
    /// إرجاع من دالة
    Return,
    
    /// إرجاع قيمة من دالة
    ReturnValue,
    
    // ═══════════════════════════════════════════════════════════════
    // القوائم والقواميس (Collections)
    // ═══════════════════════════════════════════════════════════════
    
    /// إنشاء قائمة
    /// BUILD_LIST <element_count as u16>
    BuildList(u16),
    
    /// إنشاء قاموس
    /// BUILD_DICT <entry_count as u16>
    BuildDict(u16),
    
    /// فهرسة: obj[index]
    Index,
    
    /// تعيين فهرس: obj[index] = value
    IndexSet,
    
    /// الوصول لخاصية: obj.property
    GetProperty(u32),
    
    /// تعيين خاصية: obj.property = value
    SetProperty(u32),
    
    // ═══════════════════════════════════════════════════════════════
    // الحلقات (Loops)
    // ═══════════════════════════════════════════════════════════════
    
    /// بدء حلقة for
    /// LOOP_START <iterator_slot as u16>
    LoopStart(u16),
    
    /// التكرار التالي في الحلقة
    /// LOOP_NEXT <jump_back_offset as i32>
    LoopNext(i32),
    
    /// إنهاء الحلقة
    LoopEnd,
    
    // ═══════════════════════════════════════════════════════════════
    // التعامل مع الأخطاء (Error Handling)
    // ═══════════════════════════════════════════════════════════════
    
    /// بدء try
    /// TRY_START <catch_offset as i32>
    TryStart(i32),
    
    /// نهاية try
    TryEnd,
    
    /// إلقاء خطأ
    Throw,
    
    // ═══════════════════════════════════════════════════════════════
    // عمليات متقدمة (Advanced Operations)
    // ═══════════════════════════════════════════════════════════════
    
    /// نطاق: range(start, end)
    Range,
    
    /// نطاق مع خطوة: range(start, end, step)
    RangeStep,
    
    /// طباعة
    Print,
    
    /// نوع القيمة
    TypeOf,
    
    /// طول القائمة/النص
    Length,
    
    /// توقف (break)
    Break,
    
    /// متابعة (continue)
    Continue,
    
    // ═══════════════════════════════════════════════════════════════
    // تعليمات خاصة (Special Instructions)
    // ═══════════════════════════════════════════════════════════════
    
    /// نقطة تفتح للتتبع
    Nop,
    
    /// نهاية البرنامج
    Halt,
    
    // ═══════════════════════════════════════════════════════════════
    // Async/Await Operations
    // ═══════════════════════════════════════════════════════════════
    
    /// انتظار async
    Await,
    
    /// إنشاء async function
    AsyncStart { func_id: u32 },
    
    /// إرجاع من async function
    AsyncReturn,
    
    /// إلغاء async
    AsyncCancel { task_id: u64 },
}

/// Chunk من التعليمات (وحدة تجميع)
#[derive(Clone, Debug, Default)]
pub struct Chunk {
    /// التعليمات
    pub instructions: Vec<OpCode>,
    /// أسماء المتغيرات (string pool)
    pub strings: Vec<String>,
    /// أرقام الأسطر للتصحيح
    pub line_numbers: Vec<(usize, usize)>, // (instruction_index, line_number)
}

impl Chunk {
    /// إنشاء chunk جديد
    pub fn new() -> Self {
        Chunk {
            instructions: Vec::new(),
            strings: Vec::new(),
            line_numbers: Vec::new(),
        }
    }
    
    /// إضافة تعليمة
    pub fn emit(&mut self, opcode: OpCode) {
        self.instructions.push(opcode);
    }
    
    /// إضافة تعليمة مع رقم السطر
    pub fn emit_with_line(&mut self, opcode: OpCode, line: usize) {
        let index = self.instructions.len();
        self.instructions.push(opcode);
        self.line_numbers.push((index, line));
    }
    
    /// إضافة اسم إلى string pool وإرجاع الفهرس
    pub fn add_string(&mut self, s: &str) -> u32 {
        // البحث أولاً
        for (i, existing) in self.strings.iter().enumerate() {
            if existing == s {
                return i as u32;
            }
        }
        // إضافة جديد
        let index = self.strings.len() as u32;
        self.strings.push(s.to_string());
        index
    }
    
    /// الحصول على اسم من string pool
    pub fn get_string(&self, index: u32) -> Option<&str> {
        self.strings.get(index as usize).map(|s| s.as_str())
    }
    
    /// عدد التعليمات
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
    
    /// هل فارغ
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
    
    /// الحصول على تعليمة بالفهرس
    pub fn get(&self, index: usize) -> Option<&OpCode> {
        self.instructions.get(index)
    }
    
    /// تعديل تعليمة قفز
    pub fn patch_jump(&mut self, index: usize, offset: i32) {
        match &mut self.instructions[index] {
            OpCode::Jump(o) => *o = offset,
            OpCode::JumpIfFalse(o) => *o = offset,
            OpCode::JumpIfTrue(o) => *o = offset,
            OpCode::JumpBack(o) => *o = offset,
            _ => {}
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::PushNumber(n) => write!(f, "PUSH_NUMBER {}", n),
            OpCode::PushString(i) => write!(f, "PUSH_STRING @{}", i),
            OpCode::PushBool(b) => write!(f, "PUSH_BOOL {}", b),
            OpCode::PushNull => write!(f, "PUSH_NULL"),
            OpCode::Pop => write!(f, "POP"),
            OpCode::Dup => write!(f, "DUP"),
            OpCode::Swap => write!(f, "SWAP"),
            
            OpCode::Add => write!(f, "ADD"),
            OpCode::Sub => write!(f, "SUB"),
            OpCode::Mul => write!(f, "MUL"),
            OpCode::Div => write!(f, "DIV"),
            OpCode::Mod => write!(f, "MOD"),
            OpCode::Pow => write!(f, "POW"),
            OpCode::Neg => write!(f, "NEG"),
            OpCode::Xor => write!(f, "XOR"),
            OpCode::Shl => write!(f, "SHL"),
            OpCode::Shr => write!(f, "SHR"),
            
            OpCode::Equal => write!(f, "EQ"),
            OpCode::NotEqual => write!(f, "NEQ"),
            OpCode::Less => write!(f, "LT"),
            OpCode::Greater => write!(f, "GT"),
            OpCode::LessEqual => write!(f, "LE"),
            OpCode::GreaterEqual => write!(f, "GE"),
            
            OpCode::And => write!(f, "AND"),
            OpCode::Or => write!(f, "OR"),
            OpCode::Not => write!(f, "NOT"),
            
            OpCode::LoadGlobal(i) => write!(f, "LOAD_GLOBAL @{}", i),
            OpCode::StoreGlobal(i) => write!(f, "STORE_GLOBAL @{}", i),
            OpCode::LoadLocal(i) => write!(f, "LOAD_LOCAL {}", i),
            OpCode::StoreLocal(i) => write!(f, "STORE_LOCAL {}", i),
            OpCode::DefineConst(i) => write!(f, "DEFINE_CONST @{}", i),
            
            OpCode::Jump(o) => write!(f, "JUMP {}", o),
            OpCode::JumpIfFalse(o) => write!(f, "JUMP_IF_FALSE {}", o),
            OpCode::JumpIfTrue(o) => write!(f, "JUMP_IF_TRUE {}", o),
            OpCode::JumpBack(o) => write!(f, "JUMP_BACK {}", o),
            
            OpCode::Call(n) => write!(f, "CALL {}", n),
            OpCode::CallNative { func_index, arg_count } => {
                write!(f, "CALL_NATIVE @{} args={}", func_index, arg_count)
            }
            OpCode::Return => write!(f, "RETURN"),
            OpCode::ReturnValue => write!(f, "RETURN_VALUE"),
            
            OpCode::BuildList(n) => write!(f, "BUILD_LIST {}", n),
            OpCode::BuildDict(n) => write!(f, "BUILD_DICT {}", n),
            OpCode::Index => write!(f, "INDEX"),
            OpCode::IndexSet => write!(f, "INDEX_SET"),
            OpCode::GetProperty(i) => write!(f, "GET_PROP @{}", i),
            OpCode::SetProperty(i) => write!(f, "SET_PROP @{}", i),
            
            OpCode::LoopStart(slot) => write!(f, "LOOP_START slot={}", slot),
            OpCode::LoopNext(offset) => write!(f, "LOOP_NEXT {}", offset),
            OpCode::LoopEnd => write!(f, "LOOP_END"),
            
            OpCode::TryStart(offset) => write!(f, "TRY_START {}", offset),
            OpCode::TryEnd => write!(f, "TRY_END"),
            OpCode::Throw => write!(f, "THROW"),
            
            OpCode::Range => write!(f, "RANGE"),
            OpCode::RangeStep => write!(f, "RANGE_STEP"),
            OpCode::Print => write!(f, "PRINT"),
            OpCode::TypeOf => write!(f, "TYPEOF"),
            OpCode::Length => write!(f, "LENGTH"),
            OpCode::Break => write!(f, "BREAK"),
            OpCode::Continue => write!(f, "CONTINUE"),
            
            OpCode::Nop => write!(f, "NOP"),
            OpCode::Halt => write!(f, "HALT"),
            
            OpCode::Await => write!(f, "AWAIT"),
            OpCode::AsyncStart { func_id } => write!(f, "ASYNC_START @{}", func_id),
            OpCode::AsyncReturn => write!(f, "ASYNC_RETURN"),
            OpCode::AsyncCancel { task_id } => write!(f, "ASYNC_CANCEL #{}", task_id),
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "═══════════════════════════════════")?;
        writeln!(f, "       Bytecode Chunk")?;
        writeln!(f, "═══════════════════════════════════")?;
        
        for (i, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", i, instr)?;
        }
        
        if !self.strings.is_empty() {
            writeln!(f, "───────────────────────────────────")?;
            writeln!(f, "String Pool:")?;
            for (i, s) in self.strings.iter().enumerate() {
                writeln!(f, "  @{} = \"{}\"", i, s)?;
            }
        }
        
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunk_creation() {
        let mut chunk = Chunk::new();
        chunk.emit(OpCode::PushNumber(42.0));
        chunk.emit(OpCode::PushNumber(8.0));
        chunk.emit(OpCode::Add);
        
        assert_eq!(chunk.len(), 3);
    }
    
    #[test]
    fn test_string_pool() {
        let mut chunk = Chunk::new();
        let idx1 = chunk.add_string("مرحبا");
        let idx2 = chunk.add_string("عالم");
        let idx3 = chunk.add_string("مرحبا"); // يجب أن يعيد نفس الفهرس
        
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(idx3, 0); // نفس الأولى
        assert_eq!(chunk.strings.len(), 2);
    }
    
    #[test]
    fn test_display() {
        let chunk = Chunk::new();
        let output = format!("{}", chunk);
        assert!(output.contains("Bytecode Chunk"));
    }
}
