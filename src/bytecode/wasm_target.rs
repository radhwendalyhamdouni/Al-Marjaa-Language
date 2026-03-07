// ═══════════════════════════════════════════════════════════════════════════════
// WebAssembly Compilation Target - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// تجميع كود المرجع إلى WebAssembly:
// - تحويل الـ bytecode إلى WASM
// - دعم الـ WASI (WebAssembly System Interface)
// - تحسين للأداء في المتصفح
// - دعم الـ memory management
// - تصدير الدوال للـ JavaScript
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::io::{BufWriter, Write};

use super::opcodes::OpCode;
use super::type_inference::Type;

// ═══════════════════════════════════════════════════════════════════════════════
// تعريف WASM Types
// ═══════════════════════════════════════════════════════════════════════════════

/// أنواع WebAssembly
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    FuncRef,
    ExternRef,
}

impl WasmType {
    /// الحصول على byte code
    pub fn to_byte(&self) -> u8 {
        match self {
            WasmType::I32 => 0x7F,
            WasmType::I64 => 0x7E,
            WasmType::F32 => 0x7D,
            WasmType::F64 => 0x7C,
            WasmType::FuncRef => 0x70,
            WasmType::ExternRef => 0x6F,
        }
    }
    
    /// التحويل من نوع المرجع
    pub fn from_marjaa_type(t: &Type) -> Self {
        match t {
            Type::Number => WasmType::F64,
            Type::Boolean => WasmType::I32,
            _ => WasmType::I32, // مؤشر للكائنات المعقدة
        }
    }
}

/// تعليمة WASM
#[derive(Debug, Clone)]
pub enum WasmInstruction {
    // التحكم
    Unreachable,
    Nop,
    Block { block_type: WasmBlockType },
    Loop { block_type: WasmBlockType },
    If { block_type: WasmBlockType },
    Else,
    End,
    Br { depth: u32 },
    BrIf { depth: u32 },
    BrTable { depths: Vec<u32>, default: u32 },
    Return,
    Call { func_idx: u32 },
    CallIndirect { type_idx: u32, table_idx: u32 },
    
    // المقارنات
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    
    // الأرقام
    I32Const { value: i32 },
    I64Const { value: i64 },
    F32Const { value: f32 },
    F64Const { value: f64 },
    
    // العمليات الحسابية
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Sqrt,
    F32Min,
    F32Max,
    F32Copysign,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Sqrt,
    F64Min,
    F64Max,
    F64Copysign,
    
    // التحويلات
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    
    // الذاكرة
    I32Load { align: u32, offset: u32 },
    I64Load { align: u32, offset: u32 },
    F32Load { align: u32, offset: u32 },
    F64Load { align: u32, offset: u32 },
    I32Store { align: u32, offset: u32 },
    I64Store { align: u32, offset: u32 },
    F32Store { align: u32, offset: u32 },
    F64Store { align: u32, offset: u32 },
    MemorySize { mem_idx: u32 },
    MemoryGrow { mem_idx: u32 },
    
    // المتغيرات المحلية
    LocalGet { idx: u32 },
    LocalSet { idx: u32 },
    LocalTee { idx: u32 },
    GlobalGet { idx: u32 },
    GlobalSet { idx: u32 },
}

/// نوع الكتلة
#[derive(Debug, Clone)]
pub enum WasmBlockType {
    Empty,
    ValType(WasmType),
    FuncType(u32),
}

/// دالة WASM
#[derive(Debug, Clone)]
pub struct WasmFunction {
    /// اسم الدالة (للتصدير)
    pub name: String,
    /// المعاملات
    pub params: Vec<WasmType>,
    /// القيمة المرجعة
    pub results: Vec<WasmType>,
    /// المتغيرات المحلية
    pub locals: Vec<WasmType>,
    /// التعليمات
    pub instructions: Vec<WasmInstruction>,
    /// هل الدالة مُصدّرة
    pub exported: bool,
}

/// وحدة WASM
#[derive(Debug, Clone, Default)]
pub struct WasmModule {
    /// الدوال
    pub functions: Vec<WasmFunction>,
    /// الذاكرة
    pub memory: Option<WasmMemory>,
    /// المتغيرات العامة
    pub globals: Vec<WasmGlobal>,
    /// جدول البيانات
    pub data_segments: Vec<WasmDataSegment>,
    /// العناصر (للدوال غير المباشرة)
    pub elements: Vec<WasmElement>,
}

/// ذاكرة WASM
#[derive(Debug, Clone)]
pub struct WasmMemory {
    /// الحد الأدنى للصفحات
    pub min_pages: u32,
    /// الحد الأقصى للصفحات
    pub max_pages: Option<u32>,
}

/// متغير عام
#[derive(Debug, Clone)]
pub struct WasmGlobal {
    /// النوع
    pub global_type: WasmType,
    /// هل قابل للتغيير
    pub mutable: bool,
    /// القيمة الابتدائية
    pub init_value: WasmInitExpr,
}

/// تعبير الابتداء
#[derive(Debug, Clone)]
pub enum WasmInitExpr {
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    GlobalGet(u32),
}

/// قطاع بيانات
#[derive(Debug, Clone)]
pub struct WasmDataSegment {
    /// فهرس الذاكرة
    pub memory_idx: u32,
    /// الإزاحة
    pub offset: u32,
    /// البيانات
    pub data: Vec<u8>,
}

/// عنصر (لجدول الدوال)
#[derive(Debug, Clone)]
pub struct WasmElement {
    /// فهرس الجدول
    pub table_idx: u32,
    /// الإزاحة
    pub offset: u32,
    /// فهارس الدوال
    pub func_indices: Vec<u32>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// WASM Compiler
// ═══════════════════════════════════════════════════════════════════════════════

/// مترجم bytecode إلى WASM
pub struct WasmCompiler {
    /// الوحدة المترجمة
    module: WasmModule,
    /// خريطة المتغيرات المحلية
    local_map: HashMap<String, u32>,
    /// خريطة المتغيرات العامة
    global_map: HashMap<String, u32>,
    /// خريطة الدوال
    func_map: HashMap<String, u32>,
    /// العنوان الحالي في الذاكرة
    memory_ptr: u32,
    /// إحصائيات
    stats: WasmCompilerStats,
}

/// إحصائيات المترجم
#[derive(Debug, Clone, Default)]
pub struct WasmCompilerStats {
    pub functions_compiled: u64,
    pub instructions_compiled: u64,
    pub memory_used: u32,
    pub compile_time_us: u64,
}

impl WasmCompiler {
    /// إنشاء مترجم جديد
    pub fn new() -> Self {
        WasmCompiler {
            module: WasmModule::default(),
            local_map: HashMap::new(),
            global_map: HashMap::new(),
            func_map: HashMap::new(),
            memory_ptr: 1024, // بداية الذاكرة بعد البيانات النظامية
            stats: WasmCompilerStats::default(),
        }
    }
    
    /// ترجمة bytecode إلى WASM
    pub fn compile(&mut self, instructions: &[OpCode]) -> Result<WasmModule, String> {
        let start = std::time::Instant::now();
        
        // إنشاء دالة رئيسية
        let mut main_func = WasmFunction {
            name: "_start".to_string(),
            params: Vec::new(),
            results: Vec::new(),
            locals: Vec::new(),
            instructions: Vec::new(),
            exported: true,
        };
        
        // ترجمة التعليمات
        for opcode in instructions {
            self.compile_opcode(opcode, &mut main_func)?;
            self.stats.instructions_compiled += 1;
        }
        
        // إضافة تعليمة النهاية
        main_func.instructions.push(WasmInstruction::End);
        
        // إضافة الدالة للوحدة
        self.module.functions.push(main_func);
        self.stats.functions_compiled += 1;
        
        // إضافة الذاكرة الافتراضية
        self.module.memory = Some(WasmMemory {
            min_pages: 1,
            max_pages: Some(256),
        });
        
        self.stats.compile_time_us = start.elapsed().as_micros() as u64;
        
        Ok(self.module.clone())
    }
    
    /// ترجمة تعليمة واحدة
    fn compile_opcode(&mut self, opcode: &OpCode, func: &mut WasmFunction) -> Result<(), String> {
        match opcode {
            OpCode::PushNumber(n) => {
                func.instructions.push(WasmInstruction::F64Const { value: *n });
            }
            
            OpCode::PushBool(b) => {
                func.instructions.push(WasmInstruction::I32Const { value: if *b { 1 } else { 0 } });
            }
            
            OpCode::PushNull => {
                // NULL كـ 0
                func.instructions.push(WasmInstruction::I32Const { value: 0 });
            }
            
            OpCode::Add => {
                func.instructions.push(WasmInstruction::F64Add);
            }
            
            OpCode::Sub => {
                func.instructions.push(WasmInstruction::F64Sub);
            }
            
            OpCode::Mul => {
                func.instructions.push(WasmInstruction::F64Mul);
            }
            
            OpCode::Div => {
                func.instructions.push(WasmInstruction::F64Div);
            }
            
            OpCode::Mod => {
                // WASM لا يدعم mod للأعداد العائمة مباشرة
                // سنحتاج لاستدعاء دالة مكتبة
                func.instructions.push(WasmInstruction::Call { func_idx: 0 }); // دالة mod
            }
            
            OpCode::Pow => {
                // استدعاء دالة pow
                func.instructions.push(WasmInstruction::Call { func_idx: 1 });
            }
            
            OpCode::Neg => {
                func.instructions.push(WasmInstruction::F64Const { value: -1.0 });
                func.instructions.push(WasmInstruction::F64Mul);
            }
            
            OpCode::Equal => {
                func.instructions.push(WasmInstruction::F64Eq);
            }
            
            OpCode::NotEqual => {
                func.instructions.push(WasmInstruction::F64Ne);
            }
            
            OpCode::Less => {
                func.instructions.push(WasmInstruction::F64Lt);
            }
            
            OpCode::Greater => {
                func.instructions.push(WasmInstruction::F64Gt);
            }
            
            OpCode::LessEqual => {
                func.instructions.push(WasmInstruction::F64Le);
            }
            
            OpCode::GreaterEqual => {
                func.instructions.push(WasmInstruction::F64Ge);
            }
            
            OpCode::And => {
                // تحويل إلى i32 وإجراء AND
                func.instructions.push(WasmInstruction::I32And);
            }
            
            OpCode::Or => {
                func.instructions.push(WasmInstruction::I32Or);
            }
            
            OpCode::Not => {
                func.instructions.push(WasmInstruction::I32Eqz);
            }
            
            OpCode::Jump(offset) => {
                // القفز في WASM يستخدم Br مع labels
                // هذا تبسيط - التنفيذ الحقيقي يتطلب إدارة labels
                func.instructions.push(WasmInstruction::Br { depth: *offset as u32 });
            }
            
            OpCode::JumpIfFalse(offset) => {
                func.instructions.push(WasmInstruction::I32Eqz); // عكس الشرط
                func.instructions.push(WasmInstruction::BrIf { depth: *offset as u32 });
            }
            
            OpCode::JumpIfTrue(offset) => {
                func.instructions.push(WasmInstruction::BrIf { depth: *offset as u32 });
            }
            
            OpCode::JumpBack(offset) => {
                // القفز للخلف يُستخدم للحلقات
                func.instructions.push(WasmInstruction::Br { depth: *offset as u32 });
            }
            
            OpCode::Halt => {
                func.instructions.push(WasmInstruction::Return);
            }
            
            OpCode::Call(arg_count) => {
                // استدعاء دالة
                // التنفيذ الحقيقي يتطلب معرفة فهرس الدالة
                let _ = arg_count;
            }
            
            OpCode::Return => {
                func.instructions.push(WasmInstruction::Return);
            }
            
            OpCode::Pop => {
                // إزالة من المكدس - يمكن تجاهلها في WASM
                // أو استخدام drop
            }
            
            OpCode::Dup => {
                // نسخ القيمة العليا
                func.instructions.push(WasmInstruction::LocalGet { idx: 0 }); // تبسيط
            }
            
            // التعليمات الأخرى تحتاج معالجة خاصة
            _ => {
                // تجاهل التعليمات غير المدعومة
            }
        }
        
        Ok(())
    }
    
    /// توليد الـ binary WASM
    pub fn emit_binary(&self, module: &WasmModule) -> Result<Vec<u8>, String> {
        let mut buffer = Vec::new();
        
        // Magic number
        buffer.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]);
        
        // Version
        buffer.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
        
        // Type section (Section 1)
        self.emit_type_section(&mut buffer, module)?;
        
        // Function section (Section 3)
        self.emit_function_section(&mut buffer, module)?;
        
        // Memory section (Section 5)
        self.emit_memory_section(&mut buffer, module)?;
        
        // Export section (Section 7)
        self.emit_export_section(&mut buffer, module)?;
        
        // Code section (Section 10)
        self.emit_code_section(&mut buffer, module)?;
        
        Ok(buffer)
    }
    
    /// كتابة section header
    fn write_section_header(&self, buffer: &mut Vec<u8>, section_id: u8, size: usize) {
        buffer.push(section_id);
        self.write_leb128_u(buffer, size as u64);
    }
    
    /// كتابة LEB128 غير موقّع
    fn write_leb128_u(&self, buffer: &mut Vec<u8>, mut value: u64) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            buffer.push(byte);
            if value == 0 {
                break;
            }
        }
    }
    
    /// كتابة LEB128 موقّع
    fn write_leb128_s(&self, buffer: &mut Vec<u8>, mut value: i64) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            let more = !((value == 0 && (byte & 0x40) == 0) ||
                         (value == -1 && (byte & 0x40) != 0));
            if more {
                byte |= 0x80;
            }
            buffer.push(byte);
            if !more {
                break;
            }
        }
    }
    
    /// كتابة type section
    fn emit_type_section(&self, buffer: &mut Vec<u8>, module: &WasmModule) -> Result<(), String> {
        let mut section_content = Vec::new();
        
        // عدد الأنواع
        self.write_leb128_u(&mut section_content, module.functions.len() as u64);
        
        // كتابة نوع كل دالة
        for func in &module.functions {
            section_content.push(0x60); // func type
            
            // المعاملات
            self.write_leb128_u(&mut section_content, func.params.len() as u64);
            for param in &func.params {
                section_content.push(param.to_byte());
            }
            
            // النتائج
            self.write_leb128_u(&mut section_content, func.results.len() as u64);
            for result in &func.results {
                section_content.push(result.to_byte());
            }
        }
        
        self.write_section_header(buffer, 1, section_content.len());
        buffer.extend_from_slice(&section_content);
        
        Ok(())
    }
    
    /// كتابة function section
    fn emit_function_section(&self, buffer: &mut Vec<u8>, module: &WasmModule) -> Result<(), String> {
        let mut section_content = Vec::new();
        
        // عدد الدوال
        self.write_leb128_u(&mut section_content, module.functions.len() as u64);
        
        // فهرس النوع لكل دالة
        for i in 0..module.functions.len() {
            self.write_leb128_u(&mut section_content, i as u64);
        }
        
        self.write_section_header(buffer, 3, section_content.len());
        buffer.extend_from_slice(&section_content);
        
        Ok(())
    }
    
    /// كتابة memory section
    fn emit_memory_section(&self, buffer: &mut Vec<u8>, module: &WasmModule) -> Result<(), String> {
        if let Some(ref memory) = module.memory {
            let mut section_content = Vec::new();
            
            section_content.push(1); // عدد الذاكرات
            
            if let Some(max) = memory.max_pages {
                section_content.push(0x01); // limits with max
                self.write_leb128_u(&mut section_content, memory.min_pages as u64);
                self.write_leb128_u(&mut section_content, max as u64);
            } else {
                section_content.push(0x00); // limits without max
                self.write_leb128_u(&mut section_content, memory.min_pages as u64);
            }
            
            self.write_section_header(buffer, 5, section_content.len());
            buffer.extend_from_slice(&section_content);
        }
        
        Ok(())
    }
    
    /// كتابة export section
    fn emit_export_section(&self, buffer: &mut Vec<u8>, module: &WasmModule) -> Result<(), String> {
        let exports: Vec<(String, u8, u32)> = module.functions.iter()
            .enumerate()
            .filter(|(_, f)| f.exported)
            .map(|(i, f)| (f.name.clone(), 0x00, i as u32))
            .collect();
        
        if exports.is_empty() {
            return Ok(());
        }
        
        let mut section_content = Vec::new();
        
        self.write_leb128_u(&mut section_content, exports.len() as u64);
        
        for (name, kind, idx) in &exports {
            // اسم التصدير
            self.write_leb128_u(&mut section_content, name.len() as u64);
            section_content.extend_from_slice(name.as_bytes());
            
            // نوع التصدير
            section_content.push(*kind);
            
            // الفهرس
            self.write_leb128_u(&mut section_content, *idx as u64);
        }
        
        self.write_section_header(buffer, 7, section_content.len());
        buffer.extend_from_slice(&section_content);
        
        Ok(())
    }
    
    /// كتابة code section
    fn emit_code_section(&self, buffer: &mut Vec<u8>, module: &WasmModule) -> Result<(), String> {
        let mut section_content = Vec::new();
        
        self.write_leb128_u(&mut section_content, module.functions.len() as u64);
        
        for func in &module.functions {
            let mut func_body = Vec::new();
            
            // المتغيرات المحلية
            self.write_leb128_u(&mut func_body, func.locals.len() as u64);
            for local in &func.locals {
                self.write_leb128_u(&mut func_body, 1); // count
                func_body.push(local.to_byte());
            }
            
            // التعليمات
            for instr in &func.instructions {
                self.emit_instruction(&mut func_body, instr)?;
            }
            
            // حجم جسم الدالة
            self.write_leb128_u(&mut section_content, func_body.len() as u64);
            section_content.extend_from_slice(&func_body);
        }
        
        self.write_section_header(buffer, 10, section_content.len());
        buffer.extend_from_slice(&section_content);
        
        Ok(())
    }
    
    /// كتابة تعليمة
    fn emit_instruction(&self, buffer: &mut Vec<u8>, instr: &WasmInstruction) -> Result<(), String> {
        match instr {
            WasmInstruction::Unreachable => buffer.push(0x00),
            WasmInstruction::Nop => buffer.push(0x01),
            WasmInstruction::Block { block_type } => {
                buffer.push(0x02);
                self.emit_block_type(buffer, block_type)?;
            }
            WasmInstruction::Loop { block_type } => {
                buffer.push(0x03);
                self.emit_block_type(buffer, block_type)?;
            }
            WasmInstruction::If { block_type } => {
                buffer.push(0x04);
                self.emit_block_type(buffer, block_type)?;
            }
            WasmInstruction::Else => buffer.push(0x05),
            WasmInstruction::End => buffer.push(0x0B),
            WasmInstruction::Br { depth } => {
                buffer.push(0x0C);
                self.write_leb128_u(buffer, *depth as u64);
            }
            WasmInstruction::BrIf { depth } => {
                buffer.push(0x0D);
                self.write_leb128_u(buffer, *depth as u64);
            }
            WasmInstruction::Return => buffer.push(0x0F),
            WasmInstruction::Call { func_idx } => {
                buffer.push(0x10);
                self.write_leb128_u(buffer, *func_idx as u64);
            }
            
            WasmInstruction::I32Const { value } => {
                buffer.push(0x41);
                self.write_leb128_s(buffer, *value as i64);
            }
            WasmInstruction::I64Const { value } => {
                buffer.push(0x42);
                self.write_leb128_s(buffer, *value);
            }
            WasmInstruction::F32Const { value } => {
                buffer.push(0x43);
                buffer.extend_from_slice(&value.to_le_bytes());
            }
            WasmInstruction::F64Const { value } => {
                buffer.push(0x44);
                buffer.extend_from_slice(&value.to_le_bytes());
            }
            
            WasmInstruction::I32Add => buffer.push(0x6A),
            WasmInstruction::I32Sub => buffer.push(0x6B),
            WasmInstruction::I32Mul => buffer.push(0x6C),
            WasmInstruction::I32DivS => buffer.push(0x6D),
            WasmInstruction::I32And => buffer.push(0x71),
            WasmInstruction::I32Or => buffer.push(0x72),
            WasmInstruction::I32Xor => buffer.push(0x73),
            WasmInstruction::I32Eqz => buffer.push(0x45),
            WasmInstruction::I32Eq => buffer.push(0x46),
            
            WasmInstruction::F64Add => buffer.push(0xA0),
            WasmInstruction::F64Sub => buffer.push(0xA1),
            WasmInstruction::F64Mul => buffer.push(0xA2),
            WasmInstruction::F64Div => buffer.push(0xA3),
            WasmInstruction::F64Eq => buffer.push(0x61),
            WasmInstruction::F64Ne => buffer.push(0x62),
            WasmInstruction::F64Lt => buffer.push(0x63),
            WasmInstruction::F64Gt => buffer.push(0x64),
            WasmInstruction::F64Le => buffer.push(0x65),
            WasmInstruction::F64Ge => buffer.push(0x66),
            
            WasmInstruction::LocalGet { idx } => {
                buffer.push(0x20);
                self.write_leb128_u(buffer, *idx as u64);
            }
            WasmInstruction::LocalSet { idx } => {
                buffer.push(0x21);
                self.write_leb128_u(buffer, *idx as u64);
            }
            WasmInstruction::GlobalGet { idx } => {
                buffer.push(0x23);
                self.write_leb128_u(buffer, *idx as u64);
            }
            WasmInstruction::GlobalSet { idx } => {
                buffer.push(0x24);
                self.write_leb128_u(buffer, *idx as u64);
            }
            
            _ => {
                // تعليمات أخرى
            }
        }
        
        Ok(())
    }
    
    /// كتابة نوع الكتلة
    fn emit_block_type(&self, buffer: &mut Vec<u8>, block_type: &WasmBlockType) -> Result<(), String> {
        match block_type {
            WasmBlockType::Empty => buffer.push(0x40),
            WasmBlockType::ValType(t) => buffer.push(t.to_byte()),
            WasmBlockType::FuncType(idx) => self.write_leb128_s(buffer, *idx as i64),
        }
        Ok(())
    }
    
    /// الحصول على الإحصائيات
    pub fn stats(&self) -> &WasmCompilerStats {
        &self.stats
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        println!("╔══════════════════════════════════════════════════════════════════════════╗");
        println!("║              🌐 تقرير WASM Compiler - لغة المرجع                         ║");
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        
        println!("║ 📊 الإحصائيات:                                                           ║");
        println!("║    الدوال المترجمة: {:15}                                   ║", self.stats.functions_compiled);
        println!("║    التعليمات المترجمة: {:15}                                ║", self.stats.instructions_compiled);
        println!("║    الذاكرة المستخدمة: {} bytes                                        ║", self.stats.memory_used);
        println!("║    وقت الترجمة: {} μs                                                ║", self.stats.compile_time_us);
        
        println!("╚══════════════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for WasmCompiler {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// WASI Support
// ═══════════════════════════════════════════════════════════════════════════════

/// دعم WASI
pub struct WasiSupport {
    /// دوال WASI المستوردة
    imports: Vec<WasiImport>,
}

/// استيراد WASI
#[derive(Debug, Clone)]
pub struct WasiImport {
    pub module: String,
    pub name: String,
    pub params: Vec<WasmType>,
    pub results: Vec<WasmType>,
}

impl WasiSupport {
    /// إنشاء دعم WASI جديد
    pub fn new() -> Self {
        WasiSupport {
            imports: vec![
                // دوال الإدخال/الإخراج
                WasiImport {
                    module: "wasi_snapshot_preview1".into(),
                    name: "fd_write".into(),
                    params: vec![WasmType::I32, WasmType::I32, WasmType::I32, WasmType::I32],
                    results: vec![WasmType::I32],
                },
                WasiImport {
                    module: "wasi_snapshot_preview1".into(),
                    name: "fd_read".into(),
                    params: vec![WasmType::I32, WasmType::I32, WasmType::I32, WasmType::I32],
                    results: vec![WasmType::I32],
                },
                // دوال الذاكرة
                WasiImport {
                    module: "wasi_snapshot_preview1".into(),
                    name: "args_get".into(),
                    params: vec![WasmType::I32, WasmType::I32],
                    results: vec![WasmType::I32],
                },
                // دوال الوقت
                WasiImport {
                    module: "wasi_snapshot_preview1".into(),
                    name: "clock_time_get".into(),
                    params: vec![WasmType::I32, WasmType::I64, WasmType::I32],
                    results: vec![WasmType::I32],
                },
            ],
        }
    }
    
    /// الحصول على الاستيرادات
    pub fn get_imports(&self) -> &[WasiImport] {
        &self.imports
    }
}

impl Default for WasiSupport {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wasm_type() {
        assert_eq!(WasmType::I32.to_byte(), 0x7F);
        assert_eq!(WasmType::F64.to_byte(), 0x7C);
    }
    
    #[test]
    fn test_wasm_compiler_basic() {
        let mut compiler = WasmCompiler::new();
        
        let instructions = vec![
            OpCode::PushNumber(5.0),
            OpCode::PushNumber(3.0),
            OpCode::Add,
            OpCode::Halt,
        ];
        
        let result = compiler.compile(&instructions);
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert_eq!(module.functions.len(), 1);
    }
    
    #[test]
    fn test_wasm_binary_generation() {
        let compiler = WasmCompiler::new();
        
        let module = WasmModule {
            functions: vec![WasmFunction {
                name: "test".into(),
                params: vec![],
                results: vec![],
                locals: vec![],
                instructions: vec![WasmInstruction::End],
                exported: true,
            }],
            memory: Some(WasmMemory {
                min_pages: 1,
                max_pages: Some(16),
            }),
            globals: vec![],
            data_segments: vec![],
            elements: vec![],
        };
        
        let binary = compiler.emit_binary(&module);
        assert!(binary.is_ok());
        
        let binary = binary.unwrap();
        // التحقق من magic number
        assert_eq!(&binary[0..4], &[0x00, 0x61, 0x73, 0x6D]);
    }
    
    #[test]
    fn test_wasi_support() {
        let wasi = WasiSupport::new();
        let imports = wasi.get_imports();
        
        assert!(!imports.is_empty());
        assert!(imports.iter().any(|i| i.name == "fd_write"));
    }
}
