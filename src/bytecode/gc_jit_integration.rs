// ═══════════════════════════════════════════════════════════════════════════════
// GC-JIT Integration - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// تكامل جامع القمامة مع الـ JIT:
// - safepoints للـ GC
// - تتبع المؤشرات في الـ JIT code
// - حواجز الكتابة في الكود المُحسّن
// - تخصيص ذكي للذاكرة
// - تنسيق الـ GC مع الـ JIT compilation
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::{HashMap, HashSet, BTreeMap};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

use super::gc::{ParallelGc, GcObjectId, Generation};
use super::opcodes::OpCode;
use crate::interpreter::value::{Value, SharedValue};

// ═══════════════════════════════════════════════════════════════════════════════
// Safepoints
// ═══════════════════════════════════════════════════════════════════════════════

/// نقطة أمان للـ GC
#[derive(Debug, Clone)]
pub struct Safepoint {
    /// موقع التعليمة
    pub instruction_offset: usize,
    /// موقع في الكود المترجم
    pub native_offset: usize,
    /// خريطة المكدس (أي المواقع تحتوي على مؤشرات GC)
    pub stack_map: Vec<StackSlot>,
    /// خريطة السجلات
    pub register_map: Vec<RegisterLocation>,
    /// هل هذه نقطة استدعاء
    pub is_call_site: bool,
    /// عدد المعاملات الحية
    pub live_values: u32,
}

/// موقع في المكدس
#[derive(Debug, Clone)]
pub struct StackSlot {
    /// الإزاحة من قاعدة المكدس
    pub offset: i32,
    /// هل يحتوي على مؤشر GC
    pub is_gc_pointer: bool,
    /// نوع القيمة
    pub value_type: GcValueType,
}

/// موقع في السجل
#[derive(Debug, Clone)]
pub struct RegisterLocation {
    /// رقم السجل
    pub register: u8,
    /// هل يحتوي على مؤشر GC
    pub is_gc_pointer: bool,
    /// نوع القيمة
    pub value_type: GcValueType,
}

/// نوع قيمة GC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcValueType {
    /// ليس مؤشر
    NonPointer,
    /// مؤشر مباشر
    DirectPointer,
    /// مؤشر مدمج (tagged pointer)
    TaggedPointer,
    /// مؤشر في هيكل
    EmbeddedPointer,
}

/// مدير نقاط الأمان
pub struct SafepointManager {
    /// نقاط الأمان
    safepoints: BTreeMap<usize, Safepoint>,
    /// التردد الافتراضي
    default_frequency: usize,
    /// عداد التعليمات
    instruction_counter: usize,
    /// هل الـ GC معلق
    gc_suspended: bool,
    /// طلب GC معلق
    pending_gc_request: bool,
}

impl SafepointManager {
    /// إنشاء مدير جديد
    pub fn new() -> Self {
        SafepointManager {
            safepoints: BTreeMap::new(),
            default_frequency: 100, // كل 100 تعليمة
            instruction_counter: 0,
            gc_suspended: false,
            pending_gc_request: false,
        }
    }
    
    /// إضافة نقطة أمان
    pub fn add_safepoint(&mut self, safepoint: Safepoint) {
        self.safepoints.insert(safepoint.instruction_offset, safepoint);
    }
    
    /// التحقق من الحاجة لنقطة أمان
    pub fn needs_safepoint(&self, ip: usize) -> bool {
        // نقاط الأمان عند:
        // - الاستدعاءات
        // - الحلقات
        // - التخصيصات
        // - بانتظام
        
        ip % self.default_frequency == 0
    }
    
    /// تسجيل تنفيذ
    pub fn record_execution(&mut self, ip: usize) -> Option<&Safepoint> {
        self.instruction_counter += 1;
        
        if self.needs_safepoint(ip) || self.pending_gc_request {
            self.safepoints.get(&ip)
        } else {
            None
        }
    }
    
    /// طلب GC
    pub fn request_gc(&mut self) {
        if !self.gc_suspended {
            self.pending_gc_request = true;
        }
    }
    
    /// تعليق GC
    pub fn suspend_gc(&mut self) {
        self.gc_suspended = true;
    }
    
    /// استئناف GC
    pub fn resume_gc(&mut self) {
        self.gc_suspended = false;
        self.pending_gc_request = false;
    }
    
    /// الحصول على أقرب نقطة أمان
    pub fn find_nearest_safepoint(&self, ip: usize) -> Option<&Safepoint> {
        // البحث عن أقرب نقطة أمان مستقبلية
        for (&offset, safepoint) in self.safepoints.iter() {
            if offset >= ip {
                return Some(safepoint);
            }
        }
        None
    }
    
    /// الحصول على جميع نقاط الأمان
    pub fn get_all_safepoints(&self) -> &BTreeMap<usize, Safepoint> {
        &self.safepoints
    }
}

impl Default for SafepointManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Write Barriers للـ JIT
// ═══════════════════════════════════════════════════════════════════════════════

/// حاجز كتابة للـ JIT
#[derive(Debug, Clone)]
pub struct WriteBarrierInfo {
    /// نوع الحاجز
    pub barrier_type: WriteBarrierType,
    /// موقع الكتابة
    pub location: WriteLocation,
    /// هل الحاجز نشط
    pub is_active: bool,
    /// عدد مرات التنفيذ
    pub execution_count: u64,
    /// عدد مرات التسجيل
    pub recorded_count: u64,
}

/// نوع حاجز الكتابة
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteBarrierType {
    /// لا حاجة لحاجز
    None,
    /// حاجز بسيط (تسجيل المرجع)
    Simple,
    /// حاجز مشروط (فقط إذا كان الهدف في الجيل الشاب)
    Conditional,
    /// حاجز كامل (للتعيينات المعقدة)
    Full,
}

/// موقع الكتابة
#[derive(Debug, Clone)]
pub enum WriteLocation {
    /// حقل في كائن
    ObjectField {
        object_reg: u8,
        field_offset: usize,
    },
    /// عنصر في مصفوفة
    ArrayElement {
        array_reg: u8,
        index_reg: u8,
    },
    /// متغير عام
    Global {
        name: String,
    },
    /// متغير محلي
    Local {
        slot: u32,
    },
}

/// مولد حواجز الكتابة
pub struct WriteBarrierGenerator {
    /// الحواجز المولدة
    barriers: Vec<WriteBarrierInfo>,
    /// إحصائيات
    stats: WriteBarrierStats,
    /// التحسينات
    optimizations: WriteBarrierOptimizations,
}

/// إحصائيات حواجز الكتابة
#[derive(Debug, Clone, Default)]
pub struct WriteBarrierStats {
    pub total_barriers: u64,
    pub eliminated_barriers: u64,
    pub conditional_barriers: u64,
    pub time_saved_ns: u64,
}

/// تحسينات حواجز الكتابة
#[derive(Debug, Clone)]
pub struct WriteBarrierOptimizations {
    /// إزالة الحواجز غير الضرورية
    pub eliminate_redundant: bool,
    /// دمج الحواجز المتتالية
    pub coalesce_consecutive: bool,
    /// استخدام حواجز مشروطة
    pub use_conditional: bool,
}

impl WriteBarrierGenerator {
    /// إنشاء مولد جديد
    pub fn new() -> Self {
        WriteBarrierGenerator {
            barriers: Vec::new(),
            stats: WriteBarrierStats::default(),
            optimizations: WriteBarrierOptimizations {
                eliminate_redundant: true,
                coalesce_consecutive: true,
                use_conditional: true,
            },
        }
    }
    
    /// توليد حاجز للكتابة
    pub fn generate_barrier(
        &mut self,
        location: WriteLocation,
        source_generation: Generation,
        target_generation: Generation,
    ) -> WriteBarrierType {
        let barrier_type = match (source_generation, target_generation) {
            // الكتابة من قديم إلى جديد تتطلب حاجز
            (Generation::Old, Generation::Young) => {
                if self.optimizations.use_conditional {
                    WriteBarrierType::Conditional
                } else {
                    WriteBarrierType::Simple
                }
            }
            // باقي الحالات لا تتطلب حاجز
            _ => WriteBarrierType::None,
        };
        
        if barrier_type != WriteBarrierType::None {
            self.barriers.push(WriteBarrierInfo {
                barrier_type,
                location,
                is_active: true,
                execution_count: 0,
                recorded_count: 0,
            });
            self.stats.total_barriers += 1;
        }
        
        barrier_type
    }
    
    /// تحسين الحواجز
    pub fn optimize_barriers(&mut self) {
        if self.optimizations.eliminate_redundant {
            self.eliminate_redundant_barriers();
        }
        
        if self.optimizations.coalesce_consecutive {
            self.coalesce_consecutive_barriers();
        }
    }
    
    /// إزالة الحواجز المكررة
    fn eliminate_redundant_barriers(&mut self) {
        let mut seen_locations: HashSet<String> = HashSet::new();
        let mut to_remove: Vec<usize> = Vec::new();
        
        for (i, barrier) in self.barriers.iter().enumerate() {
            let key = format!("{:?}", barrier.location);
            if seen_locations.contains(&key) {
                to_remove.push(i);
                self.stats.eliminated_barriers += 1;
            } else {
                seen_locations.insert(key);
            }
        }
        
        // إزالة من النهاية للبداية
        for i in to_remove.into_iter().rev() {
            self.barriers.remove(i);
        }
    }
    
    /// دمج الحواجز المتتالية
    fn coalesce_consecutive_barriers(&mut self) {
        let mut i = 0;
        while i + 1 < self.barriers.len() {
            let current = &self.barriers[i];
            let next = &self.barriers[i + 1];
            
            // إذا كانت الحواجز متشابهة ومتتالية، دمجها
            if current.barrier_type == next.barrier_type {
                // يمكن دمج الحواجز هنا
                // التنفيذ الفعلي يعتمد على السياق
            }
            i += 1;
        }
    }
    
    /// الحصول على الإحصائيات
    pub fn stats(&self) -> &WriteBarrierStats {
        &self.stats
    }
}

impl Default for WriteBarrierGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// تخصيص الذاكرة في JIT
// ═══════════════════════════════════════════════════════════════════════════════

/// معلومات التخصيص
#[derive(Debug, Clone)]
pub struct AllocationInfo {
    /// حجم التخصيص
    pub size: usize,
    /// الجيل
    pub generation: Generation,
    /// نوع الكائن
    pub object_type: GcObjectType,
    /// موقع التخصيص (للتتبع)
    pub allocation_site: usize,
    /// عدد مرات التخصيص
    pub allocation_count: u64,
    /// معدل البقاء
    pub survival_rate: f64,
}

/// نوع كائن GC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcObjectType {
    /// كائن صغير (<= 32 bytes)
    Small,
    /// كائن متوسط (<= 256 bytes)
    Medium,
    /// كائن كبير (> 256 bytes)
    Large,
    /// مصفوفة
    Array,
    /// نص
    String,
}

/// مدير التخصيص
pub struct AllocationManager {
    /// معلومات مواقع التخصيص
    allocation_sites: HashMap<usize, AllocationInfo>,
    /// إحصائيات
    stats: AllocationStats,
    /// استراتيجية التخصيص
    strategy: AllocationStrategy,
}

/// إحصائيات التخصيص
#[derive(Debug, Clone, Default)]
pub struct AllocationStats {
    pub total_allocations: u64,
    pub total_bytes: u64,
    pub young_allocations: u64,
    pub old_allocations: u64,
    pub promoted_bytes: u64,
    pub avg_allocation_size: f64,
}

/// استراتيجية التخصيص
#[derive(Debug, Clone)]
pub enum AllocationStrategy {
    /// تخصيص في الجيل الشاب دائماً
    AlwaysYoung,
    /// تخصيص بناءً على الحجم
    SizeBased { young_threshold: usize },
    /// تخصيص بناءً على العمر المتوقع
    LifetimeBased { hot_threshold: u32 },
}

impl AllocationManager {
    /// إنشاء مدير جديد
    pub fn new() -> Self {
        AllocationManager {
            allocation_sites: HashMap::new(),
            stats: AllocationStats::default(),
            strategy: AllocationStrategy::SizeBased { young_threshold: 256 },
        }
    }
    
    /// تسجيل تخصيص
    pub fn record_allocation(&mut self, site: usize, size: usize) -> Generation {
        // تصنيف الكائن أولاً
        let object_type = if size <= 32 {
            GcObjectType::Small
        } else if size <= 256 {
            GcObjectType::Medium
        } else {
            GcObjectType::Large
        };
        
        // الحصول على أو إنشاء معلومات التخصيص
        let info = self.allocation_sites.entry(site).or_insert(AllocationInfo {
            size,
            generation: Generation::Young,
            object_type,
            allocation_site: site,
            allocation_count: 0,
            survival_rate: 0.0,
        });
        
        info.allocation_count += 1;
        let allocation_count = info.allocation_count;
        
        // تحديث الإحصائيات
        self.stats.total_allocations += 1;
        self.stats.total_bytes += size as u64;
        self.stats.avg_allocation_size = self.stats.total_bytes as f64 / 
            self.stats.total_allocations as f64;
        
        // تحديد الجيل
        let generation = self.determine_generation(size, allocation_count);
        
        // تحديث إحصائيات الأجيال
        match generation {
            Generation::Young => self.stats.young_allocations += 1,
            Generation::Old => self.stats.old_allocations += 1,
        }
        
        generation
    }
    
    /// تصنيف الكائن
    fn classify_object(&self, size: usize) -> GcObjectType {
        if size <= 32 {
            GcObjectType::Small
        } else if size <= 256 {
            GcObjectType::Medium
        } else {
            GcObjectType::Large
        }
    }
    
    /// تحديد الجيل
    fn determine_generation(&self, size: usize, allocation_count: u64) -> Generation {
        match &self.strategy {
            AllocationStrategy::AlwaysYoung => Generation::Young,
            AllocationStrategy::SizeBased { young_threshold } => {
                if size > *young_threshold {
                    Generation::Old
                } else {
                    Generation::Young
                }
            }
            AllocationStrategy::LifetimeBased { hot_threshold } => {
                // الكائنات المتكررة قد تعيش طويلاً
                if allocation_count > *hot_threshold as u64 {
                    Generation::Old
                } else {
                    Generation::Young
                }
            }
        }
    }
    
    /// تحديث معدل البقاء
    pub fn update_survival_rate(&mut self, site: usize, survived: bool) {
        if let Some(info) = self.allocation_sites.get_mut(&site) {
            // تحديث المعدل بشكل تزايدي
            let alpha = 0.1; // معامل النعومة
            info.survival_rate = if survived {
                info.survival_rate * (1.0 - alpha) + alpha
            } else {
                info.survival_rate * (1.0 - alpha)
            };
            
            // تحديث الجيل بناءً على معدل البقاء
            if info.survival_rate > 0.8 && info.generation == Generation::Young {
                info.generation = Generation::Old;
            }
        }
    }
    
    /// الحصول على الإحصائيات
    pub fn stats(&self) -> &AllocationStats {
        &self.stats
    }
}

impl Default for AllocationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// GC-JIT Coordinator
// ═══════════════════════════════════════════════════════════════════════════════

/// منسق GC-JIT
pub struct GcJitCoordinator {
    /// مدير نقاط الأمان
    safepoint_manager: SafepointManager,
    /// مولد حواجز الكتابة
    write_barrier_generator: WriteBarrierGenerator,
    /// مدير التخصيص
    allocation_manager: AllocationManager,
    /// الـ GC
    gc: Option<Rc<RefCell<ParallelGc>>>,
    /// هل التكامل نشط
    is_active: bool,
    /// إحصائيات
    stats: CoordinatorStats,
}

/// إحصائيات المنسق
#[derive(Debug, Clone, Default)]
pub struct CoordinatorStats {
    pub safepoint_hits: u64,
    pub gc_collections_triggered: u64,
    pub barriers_executed: u64,
    pub allocations_in_jit_code: u64,
    pub time_in_gc_ns: u64,
    pub time_in_jit_ns: u64,
}

impl GcJitCoordinator {
    /// إنشاء منسق جديد
    pub fn new() -> Self {
        GcJitCoordinator {
            safepoint_manager: SafepointManager::new(),
            write_barrier_generator: WriteBarrierGenerator::new(),
            allocation_manager: AllocationManager::new(),
            gc: None,
            is_active: true,
            stats: CoordinatorStats::default(),
        }
    }
    
    /// ربط مع GC
    pub fn bind_gc(&mut self, gc: Rc<RefCell<ParallelGc>>) {
        self.gc = Some(gc);
    }
    
    /// معالجة نقطة أمان
    pub fn handle_safepoint(&mut self, ip: usize, stack: &[SharedValue]) {
        if !self.is_active {
            return;
        }
        
        // نسخ المعلومات المطلوبة لتجنب مشاكل الاستعارة
        let pending_gc = self.safepoint_manager.pending_gc_request;
        let safepoint_opt = self.safepoint_manager.record_execution(ip);
        
        if let Some(safepoint) = safepoint_opt {
            self.stats.safepoint_hits += 1;
            
            // التحقق من طلب GC
            if pending_gc {
                // نسخ نقطة الأمان لتجنب مشاكل الاستعارة
                let safepoint_clone = safepoint.clone();
                self.trigger_gc(&safepoint_clone, stack);
            }
        }
    }
    
    /// تشغيل GC
    fn trigger_gc(&mut self, safepoint: &Safepoint, stack: &[SharedValue]) {
        let start = std::time::Instant::now();
        
        // تحديث جذور الـ GC من المكدس
        if let Some(ref gc) = self.gc {
            let mut gc_mut = gc.borrow_mut();
            
            // إضافة العناصر من المكدس كجذور
            for slot in &safepoint.stack_map {
                if slot.is_gc_pointer {
                    let idx = (slot.offset as usize).min(stack.len().saturating_sub(1));
                    if idx < stack.len() {
                        gc_mut.add_root(Rc::clone(&stack[idx]));
                    }
                }
            }
            
            // تشغيل جمع الجيل الشاب
            gc_mut.collect_young();
        }
        
        self.stats.gc_collections_triggered += 1;
        self.stats.time_in_gc_ns += start.elapsed().as_nanos() as u64;
        self.safepoint_manager.pending_gc_request = false;
    }
    
    /// تنفيذ حاجز كتابة
    pub fn execute_write_barrier(
        &mut self,
        location: WriteLocation,
        source_gen: Generation,
        target_gen: Generation,
    ) {
        if !self.is_active {
            return;
        }
        
        let barrier_type = self.write_barrier_generator.generate_barrier(
            location,
            source_gen,
            target_gen,
        );
        
        if barrier_type != WriteBarrierType::None {
            self.stats.barriers_executed += 1;
            
            // تسجيل المرجع في الـ remembered set
            if let Some(ref gc) = self.gc {
                // استخدام write barrier من GC
                // gc.borrow().write_barrier.record_reference(from, to);
            }
        }
    }
    
    /// تخصيص في كود JIT
    pub fn allocate_in_jit(&mut self, site: usize, size: usize) -> Option<SharedValue> {
        if !self.is_active {
            return None;
        }
        
        let generation = self.allocation_manager.record_allocation(site, size);
        self.stats.allocations_in_jit_code += 1;
        
        // التخصيص الفعلي
        if let Some(ref gc) = self.gc {
            let value = gc.borrow_mut().allocate(Value::Null);
            Some(value)
        } else {
            None
        }
    }
    
    /// تحسين التكامل
    pub fn optimize(&mut self) {
        // تحسين الحواجز
        self.write_barrier_generator.optimize_barriers();
        
        // تحسين نقاط الأمان
        self.optimize_safepoints();
    }
    
    /// تحسين نقاط الأمان
    fn optimize_safepoints(&mut self) {
        // إزالة نقاط الأمان غير الضرورية
        // وتقليل التردد في الكود البارد
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        println!("╔══════════════════════════════════════════════════════════════════════════╗");
        println!("║              🔗 تقرير GC-JIT Integration - لغة المرجع                    ║");
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        
        println!("║ 📊 الإحصائيات العامة:                                                    ║");
        println!("║    نقاط الأمان المستخدمة: {:15}                              ║", self.stats.safepoint_hits);
        println!("║    مجموعات GC المطلوبة: {:15}                                ║", self.stats.gc_collections_triggered);
        println!("║    حواجز الكتابة المنفذة: {:15}                             ║", self.stats.barriers_executed);
        println!("║    التخصيصات في JIT: {:15}                                   ║", self.stats.allocations_in_jit_code);
        
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        println!("║ ⏱️ الأداء:                                                                ║");
        println!("║    وقت GC: {} ns                                                      ║", self.stats.time_in_gc_ns);
        println!("║    وقت JIT: {} ns                                                     ║", self.stats.time_in_jit_ns);
        
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        println!("║ 📍 نقاط الأمان:                                                          ║");
        println!("║    عدد النقاط: {:15}                                        ║", 
            self.safepoint_manager.get_all_safepoints().len());
        
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        println!("║ 🧱 التخصيصات:                                                            ║");
        let alloc_stats = self.allocation_manager.stats();
        println!("║    إجمالي التخصيصات: {:15}                                 ║", alloc_stats.total_allocations);
        println!("║    إجمالي البايتات: {:15}                                     ║", alloc_stats.total_bytes);
        println!("║    متوسط الحجم: {:.2} bytes                                           ║", alloc_stats.avg_allocation_size);
        
        println!("╚══════════════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for GcJitCoordinator {
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
    fn test_safepoint_manager() {
        let mut manager = SafepointManager::new();
        
        manager.add_safepoint(Safepoint {
            instruction_offset: 100,
            native_offset: 200,
            stack_map: vec![],
            register_map: vec![],
            is_call_site: false,
            live_values: 0,
        });
        
        assert!(manager.get_all_safepoints().contains_key(&100));
    }
    
    #[test]
    fn test_write_barrier_generator() {
        let mut generator = WriteBarrierGenerator::new();
        
        let barrier_type = generator.generate_barrier(
            WriteLocation::Global { name: "test".into() },
            Generation::Old,
            Generation::Young,
        );
        
        // الكتابة من قديم إلى جديد تتطلب حاجز
        assert_ne!(barrier_type, WriteBarrierType::None);
    }
    
    #[test]
    fn test_allocation_manager() {
        let mut manager = AllocationManager::new();
        
        let gen = manager.record_allocation(100, 64);
        assert_eq!(gen, Generation::Young);
        
        let stats = manager.stats();
        assert_eq!(stats.total_allocations, 1);
    }
    
    #[test]
    fn test_gc_jit_coordinator() {
        let mut coordinator = GcJitCoordinator::new();
        
        coordinator.handle_safepoint(0, &[]);
        
        assert_eq!(coordinator.stats.safepoint_hits, 0);
    }
}
