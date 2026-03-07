# Technical Documentation - لغة المرجع (Al-Marjaa Language)
## Version 3.0.0

---

## Table of Contents

1. [Overview](#1-overview)
2. [Architecture](#2-architecture)
3. [Core Components](#3-core-components)
4. [Bytecode Virtual Machine](#4-bytecode-virtual-machine)
5. [JIT Compiler](#5-jit-compiler)
6. [Garbage Collector](#6-garbage-collector)
7. [AI Engine](#7-ai-engine)
8. [Vibe Coding System](#8-vibe-coding-system)
9. [GUI Engine](#9-gui-engine)
10. [Export System](#10-export-system)

---

## 1. Overview

**Al-Marjaa (لغة المرجع)** is the first complete Arabic programming language with native AI integration. It features:

- **Arabic-First Design**: Keywords, syntax, and standard library entirely in Arabic
- **Native AI Types**: Tensor, AutoTensor, NeuralNetwork as first-class values
- **5-Tier JIT Compiler**: From interpretation to tracing JIT with SIMD optimization
- **Parallel Generational GC**: Young/old generations with write barriers
- **Vibe Coding**: Natural language Arabic → executable code synthesis

### Key Innovations

| Feature | Description | Novelty |
|---------|-------------|---------|
| RTL-First Syntax | Arabic numerals (٠-٩), punctuation (،؛) | First Arabic-native PL |
| Native AI Types | Tensor, AutoTensor, ComputeGraph | AI as primitive types |
| 5-Tier JIT | T0→T1→T2→T3→T4 optimization levels | Most advanced JIT for Arabic PL |
| Parallel GC | Generational with work-stealing | First parallel GC for Arabic PL |
| Vibe Coding | Arabic NLP → code synthesis | First Arabic vibe coding system |

---

## 2. Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Source Code (.mrj)                          │
│                 "متغير س = ١٠؛"                                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Lexer (المحلل المعجمي)                      │
│  - Arabic keyword recognition                                   │
│  - RTL text handling                                            │
│  - Arabic numeral conversion (٠-٩ → 0-9)                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Parser (المحلل النحوي)                      │
│  - AST construction                                             │
│  - Arabic operator parsing                                      │
│  - Error recovery                                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Compiler (المجمّع)                          │
│  - AST → Bytecode                                               │
│  - Constant folding                                             │
│  - Dead code elimination                                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Bytecode VM                                 │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐   │
│  │ Interpreter│→│ Baseline   │→│ Optimizing │→│ Tracing   │   │
│  │  (T0)     │  │ JIT (T1)  │  │ JIT (T2)  │  │ JIT (T4)  │   │
│  └───────────┘  └───────────┘  └───────────┘  └───────────┘   │
│                       │                                          │
│                       ▼                                          │
│              ┌───────────────┐                                  │
│              │ SIMD JIT (T3) │                                  │
│              └───────────────┘                                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Garbage Collector                             │
│  ┌─────────────────┐    ┌─────────────────┐                    │
│  │  Young Gen      │───→│   Old Gen       │                    │
│  │  (Nursery)      │    │   (Tenured)     │                    │
│  └─────────────────┘    └─────────────────┘                    │
│  - Parallel mark-and-sweep                                      │
│  - Write barriers                                               │
│  - Incremental collection                                       │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Core Components

### 3.1 Lexer (src/lexer/)

The lexer handles Arabic text with special considerations:

```rust
// Arabic numeral conversion
'٠' => '0', '١' => '1', '٢' => '2', ...
// Arabic punctuation
'،' => ',', '؛' => ';'
// RTL-aware identifier parsing
```

**Key Features:**
- Unicode grapheme cluster handling
- Arabic diacritics (حركات) support
- Keyword recognition in RTL context

### 3.2 Parser (src/parser/)

Recursive descent parser producing an AST:

```rust
enum Stmt {
    VarDecl { name: String, value: Expr },
    Function { name: String, params: Vec<String>, body: Vec<Stmt> },
    If { condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    // ...
}

enum Expr {
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Tensor { data: Vec<f64>, shape: Vec<usize> },
    // ...
}
```

### 3.3 Value System (src/interpreter/value.rs)

The Value enum supports both traditional and AI types:

```rust
pub enum Value {
    // Traditional types
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<SharedValue>),
    
    // AI-native types
    Tensor(Tensor),
    AutoTensor(AutoTensor),  // With autodiff
    ComputeGraph(ComputeGraph),
    NeuralNetwork(NeuralNetwork),
}
```

---

## 4. Bytecode Virtual Machine

### 4.1 OpCode Set (57 opcodes)

Located in `src/bytecode/opcodes.rs`:

| Category | Opcodes |
|----------|---------|
| Stack | PushNumber, PushString, PushBool, PushNull, PushArray |
| Arithmetic | Add, Subtract, Multiply, Divide, Modulo, Power |
| Bitwise | And, Or, Xor, Not, Shl, Shr |
| Comparison | Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual |
| Control | Jump, JumpIfFalse, JumpIfTrue, Call, Return |
| Variables | GetLocal, SetLocal, GetGlobal, SetGlobal |
| Collections | GetIndex, SetIndex, GetProperty, SetProperty |
| Functions | Closure, CallClosure |
| AI | TensorCreate, TensorMatmul, NeuralForward |

### 4.2 Chunk Structure

```rust
pub struct Chunk {
    pub instructions: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub debug_info: Vec<DebugInfo>,
}
```

### 4.3 VM Execution

```rust
pub struct VM {
    stack: Vec<SharedValue>,
    globals: Rc<RefCell<Environment>>,
    frames: Vec<CallFrame>,
    jit: JitCompiler,
    gc: ParallelGc,
}
```

---

## 5. JIT Compiler

### 5.1 Tiered Compilation

The JIT implements 5 optimization levels:

| Tier | Level | Description | Threshold |
|------|-------|-------------|-----------|
| T0 | Interpreter | Direct bytecode execution | 0 |
| T1 | Baseline JIT | Quick compilation, no optimization | 100 calls |
| T2 | Optimizing JIT | Inlining, constant folding | 1000 calls |
| T3 | SIMD JIT | Vector operations | 5000 calls |
| T4 | Tracing JIT | Hot path optimization | 10000 calls |

### 5.2 Hot Spot Detection

```rust
const JIT_THRESHOLD: u32 = 100;
const OPTIMIZED_THRESHOLD: u32 = 1000;
const SIMD_THRESHOLD: u32 = 5000;
const TRACING_THRESHOLD: u32 = 10000;
```

### 5.3 Optimization Passes

```rust
pub struct OptimizedExecutor {
    inlined_functions: HashSet<String>,
    constant_folded: bool,
    dead_code_eliminated: bool,
    loop_unrolled: bool,
}
```

### 5.4 SIMD Operations

```rust
pub struct SimdProcessor {
    simd_available: bool,
    vector_width: usize,  // 256-bit AVX
    stats: SimdStats,
}

impl SimdProcessor {
    pub fn vector_add(&mut self, a: &[f64], b: &[f64], result: &mut [f64]);
    pub fn vector_mul(&mut self, a: &[f64], b: &[f64], result: &mut [f64]);
    pub fn vector_dot(&mut self, a: &[f64], b: &[f64]) -> f64;
}
```

---

## 6. Garbage Collector

### 6.1 Generational Design

```rust
pub struct ParallelGc {
    young_generation: Vec<GcObject>,
    old_generation: Vec<GcObject>,
    write_barrier: WriteBarrier,
    stats: GcStats,
}
```

### 6.2 Configuration

```rust
const DEFAULT_YOUNG_GEN_SIZE: usize = 1024 * 1024;  // 1 MB
const DEFAULT_OLD_GEN_SIZE: usize = 8 * 1024 * 1024; // 8 MB
const PROMOTION_THRESHOLD: u32 = 3;  // Survive 3 young GC cycles
const MAX_REMEMBERED_SET_SIZE: usize = 10000;
```

### 6.3 Collection Phases

1. **Young Collection**: Fast, frequent collection of nursery
2. **Old Collection**: Full mark-and-sweep when old gen fills
3. **Parallel Mark**: Work-stealing parallel marking
4. **Compaction**: Optional old gen compaction

### 6.4 Write Barriers

```rust
pub struct WriteBarrier {
    remembered_set: RwLock<HashSet<GcObjectId>>,
    count: AtomicUsize,
}
```

---

## 7. AI Engine

### 7.1 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     AI Engine                               │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   GGUF       │  │  Inference   │  │   Pipeline   │      │
│  │   Engine     │  │   Engine     │  │   Engine     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│         │                 │                 │               │
│         └─────────────────┴─────────────────┘               │
│                           │                                 │
│                           ▼                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │               Arabic NLP Pipeline                    │   │
│  │  Tokenization → Entity Extraction → Intent Parsing   │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### 7.2 Arabic NLP Pipeline

Located in `src/ai_engine/arabic_nlp.rs`:

```rust
pub struct ArabicNlp {
    keywords: HashMap<String, IntentType>,
    word_numbers: Vec<(&'static str, f64)>,
}

impl ArabicNlp {
    pub fn tokenize(&self, text: &str) -> Vec<Token>;
    pub fn extract_entities(&self, tokens: &[Token]) -> Vec<Entity>;
    pub fn classify_sentence(&self, text: &str) -> SentenceType;
    pub fn parse_arabic_number(&self, s: &str) -> Result<f64, ()>;
}
```

### 7.3 Supported Intent Types

| Intent | Arabic Examples |
|--------|-----------------|
| Variable Declaration | أنشئ، عرّف، خزن، اعمل، حط |
| Print | اطبع، اعرض، اكتب، أظهر |
| Function | أنشئ دالة، اعمل دالة، عرّف دالة |
| Condition | إذا، لو، في حال، عندما |
| Loop | كرر، طالما، لكل، مر على |
| Export | صدر، حوّل، اجعل |

---

## 8. Vibe Coding System

### 8.1 Pipeline Flow

```
Arabic Natural Language
         │
         ▼
┌─────────────────────┐
│   Tokenization      │
│   "أنشئ متغير س"    │
└─────────────────────┘
         │
         ▼
┌─────────────────────┐
│  Entity Extraction  │
│  - Action: أنشئ     │
│  - Target: متغير    │
│  - Name: س          │
└─────────────────────┘
         │
         ▼
┌─────────────────────┐
│  Intent Parsing     │
│  Intent {           │
│    action: "var",   │
│    name: "س",       │
│    value: "10"      │
│  }                  │
└─────────────────────┘
         │
         ▼
┌─────────────────────┐
│  Code Generation    │
│  متغير س = 10؛      │
└─────────────────────┘
```

### 8.2 Example Transformations

| Arabic Input | Generated Code |
|--------------|----------------|
| أنشئ متغير س يساوي 10 | `متغير س = 10؛` |
| اطبع مرحبا بالعالم | `اطبع("مرحبا بالعالم")؛` |
| إذا كان س أكبر من 5 اطبع كبير | `إذا س > 5 { اطبع("كبير")؛ }` |
| أنشئ دالة تجمع رقمين | `دالة اجمع(أ، ب) { أعطِ أ + ب؛ }` |

---

## 9. GUI Engine

### 9.1 Element Types

```rust
pub enum GUIElement {
    Window { title: String, width: u32, height: u32 },
    Button { text: String, color: GUIColor, x: i32, y: i32 },
    TextField { placeholder: String, value: String },
    Label { text: String, font_size: u32 },
    Container { width: u32, height: u32, background: GUIColor },
}
```

### 9.2 Animation System

```rust
pub enum AnimationType {
    Fade { from: f32, to: f32, duration: u32 },
    Slide { from_x: i32, from_y: i32, to_x: i32, to_y: i32 },
    Scale { from: f32, to: f32 },
    Rotate { from_degrees: f32, to_degrees: f32 },
}
```

### 9.3 Export Formats

- HTML/CSS/JavaScript
- Native desktop (Windows, Linux, macOS)
- WebAssembly (planned)

---

## 10. Export System

### 10.1 Platform Support

| Platform | Status | Output |
|----------|--------|--------|
| Windows | ✅ | .exe |
| Linux | ✅ | binary |
| macOS | ✅ | .app |
| Web | ✅ | HTML/CSS/JS |
| Android | 🔄 | APK (planned) |
| iOS | 🔄 | IPA (planned) |

### 10.2 Export Pipeline

```
.mrj Source
     │
     ▼
┌────────────────┐
│ Parse & Analyze│
└────────────────┘
     │
     ▼
┌────────────────┐
│ Generate Target│
│ - HTML/CSS/JS  │
│ - Rust binary  │
└────────────────┘
     │
     ▼
┌────────────────┐
│ Bundle & Package│
└────────────────┘
     │
     ▼
  Executable
```

---

## Performance Benchmarks

### JIT Performance

| Benchmark | T0 (ms) | T1 (ms) | T2 (ms) | T3 (ms) | T4 (ms) |
|-----------|---------|---------|---------|---------|---------|
| Fibonacci(30) | 1250 | 89 | 45 | 32 | 28 |
| Prime Sieve | 450 | 67 | 34 | 21 | 18 |
| Matrix Mult | 890 | 156 | 78 | 23 | 19 |

### GC Performance

| Metric | Value |
|--------|-------|
| Young GC pause | < 1ms |
| Full GC pause | < 10ms |
| Throughput | > 99.5% |

---

## File Structure

```
src/
├── lib.rs                    # Library entry point
├── main.rs                   # CLI entry point
├── lexer/                    # Lexical analysis
│   ├── mod.rs
│   └── tokens.rs
├── parser/                   # Parsing
│   ├── mod.rs
│   └── ast.rs
├── interpreter/              # Execution
│   ├── mod.rs
│   ├── value.rs
│   ├── autograd.rs
│   ├── gpu.rs
│   ├── jit.rs
│   ├── native_io.rs
│   └── native_stdlib.rs
├── bytecode/                 # VM & JIT
│   ├── mod.rs
│   ├── opcodes.rs
│   ├── compiler.rs
│   ├── vm.rs
│   ├── jit.rs
│   ├── advanced_jit.rs
│   ├── optimizer.rs
│   ├── gc.rs
│   └── benchmarks.rs
├── ai_engine/                # AI integration
│   ├── mod.rs
│   ├── inference.rs
│   ├── gguf_inference.rs
│   ├── arabic_nlp.rs
│   └── pipeline/
├── gui/                      # GUI engine
│   └── mod.rs
├── exporter/                 # Export system
│   └── mod.rs
├── fine_tuning/              # Model training
│   ├── mod.rs
│   └── interface.rs
├── cli/                      # CLI interface
│   ├── mod.rs
│   ├── args.rs
│   ├── commands.rs
│   └── repl.rs
├── linter/                   # Code analysis
│   └── mod.rs
├── formatter/                # Code formatting
│   └── mod.rs
├── lsp_bridge/               # LSP support
│   └── mod.rs
├── package_manager/          # Package management
│   ├── mod.rs
│   ├── registry.rs
│   ├── installer.rs
│   └── dependency.rs
├── runtime/                  # Runtime support
│   └── mod.rs
├── error/                    # Error handling
│   └── mod.rs
└── integration/              # Integration tests
    └── mod.rs
```

---

## License

© 2026 RADHWEN DALY HAMDOUNI. All Rights Reserved.

This project is the exclusive intellectual property of RADHWEN DALY HAMDOUNI (رضوان دالي حمدوني). For licensing inquiries, contact: almarjaa.project@hotmail.com

---

**Document Version**: 1.0.0  
**Last Updated**: 2025  
**Author**: رضوان دالي حمدوني (RADHWEN DALY HAMDOUNI)
