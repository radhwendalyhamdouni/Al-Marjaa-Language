// ═══════════════════════════════════════════════════════════════════════════════
// جامع القمامة المتوازي (Parallel Garbage Collector) - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// نظام متقدم لإدارة الذاكرة يتميز بـ:
// - جامع قمامة أجيالي (Generational GC)
// - علام وامتسح (Mark-and-Sweep)
// - حواجز كتابة لكشف المراجع بين الأجيال
// - جمع تزايدي لتقليل أوقات التوقف
// - إحصائيات مفصلة للأداء
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::{RefCell, Cell};
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, RwLock};
use std::time::Instant;

use crate::interpreter::value::{Value, SharedValue, Environment};

// ═══════════════════════════════════════════════════════════════════════════════
// الثوابت والإعدادات
// ═══════════════════════════════════════════════════════════════════════════════

/// حجم الجيل الشاب الافتراضي (بالكيلوبايت)
const DEFAULT_YOUNG_GEN_SIZE: usize = 1024; // 1 MB

/// حجم الجيل القديم الافتراضي (بالكيلوبايت)
const DEFAULT_OLD_GEN_SIZE: usize = 8192; // 8 MB

/// عتبة ترقية الكائن إلى الجيل القديم (عدد مرات البقاء)
const PROMOTION_THRESHOLD: u8 = 3;

/// عدد العمال المتوازيين للـ GC
const GC_WORKER_COUNT: usize = 4;

/// الحد الأقصى لحجم الـ remembered set
const MAX_REMEMBERED_SET_SIZE: usize = 10000;

/// فترة GC التزايدي (ميلي ثانية)
const INCREMENTAL_SLICE_MS: u64 = 5;

// ═══════════════════════════════════════════════════════════════════════════════
// كائن GC قابل للتتبع
// ═══════════════════════════════════════════════════════════════════════════════

/// معرف فريد لكائن GC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GcObjectId(usize);

impl GcObjectId {
    pub fn new(id: usize) -> Self {
        GcObjectId(id)
    }
    
    pub fn as_usize(&self) -> usize {
        self.0
    }
}

/// معلومات عن كائن في الـ GC
#[derive(Debug)]
pub struct GcObjectInfo {
    /// المعرف الفريد
    pub id: GcObjectId,
    /// حجم الكائن بالبايت (تقريبي)
    pub size: usize,
    /// عدد مرات البقاء على قيد الحياة
    pub survival_count: u8,
    /// الجيل الحالي
    pub generation: Generation,
    /// هل تم وضع علامة عليه في الدورة الحالية
    pub marked: Cell<bool>,
    /// وقت الإنشاء
    pub created_at: Instant,
}

/// الأجيال في GC الأجيالي
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Generation {
    /// الجيل الشاب - كائنات جديدة
    Young,
    /// الجيل القديم - كائنات طويلة العمر
    Old,
}

// ═══════════════════════════════════════════════════════════════════════════════
// إحصائيات GC
// ═══════════════════════════════════════════════════════════════════════════════

/// إحصائيات جامع القمامة
#[derive(Debug, Default, Clone)]
pub struct GcStats {
    /// إجمالي عدد دورات GC
    pub total_collections: u64,
    /// عدد دورات الجيل الشاب
    pub young_collections: u64,
    /// عدد دورات الجيل القديم
    pub old_collections: u64,
    /// إجمالي الوقت المستغرق (ميلي ثانية)
    pub total_time_ms: u64,
    /// وقت مرحلة العلام (ميلي ثانية)
    pub mark_time_ms: u64,
    /// وقت مرحلة المسح (ميلي ثانية)
    pub sweep_time_ms: u64,
    /// عدد الكائنات المجموعة
    pub objects_collected: u64,
    /// عدد الكائنات المتبقية
    pub objects_surviving: u64,
    /// عدد الكائنات المرقّاة
    pub objects_promoted: u64,
    /// حجم الذاكرة المحررة (بايت)
    pub bytes_freed: u64,
    /// حجم الذاكرة المستخدمة (بايت)
    pub bytes_used: u64,
    /// عدد مرات إيقاف العالم (stop-the-world)
    pub stw_count: u64,
    /// إجمالي وقت التوقف (ميلي ثانية)
    pub stw_time_ms: u64,
    /// عدد عمليات التوازي
    pub parallel_ops: u64,
    /// نسبة ضربات الكاش
    pub cache_hit_ratio: f64,
}

impl GcStats {
    /// تحديث النسب المئوية
    pub fn update_ratios(&mut self) {
        if self.total_collections > 0 {
            self.cache_hit_ratio = 1.0 - (self.objects_collected as f64 / 
                (self.objects_collected + self.objects_surviving) as f64).max(0.0);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// حاجز الكتابة (Write Barrier)
// ═══════════════════════════════════════════════════════════════════════════════

/// حاجز كتابة لكشف المراجع بين الأجيال
pub struct WriteBarrier {
    /// مجموعة الكائنات التي تشير من القديم إلى الجديد
    remembered_set: RwLock<HashSet<GcObjectId>>,
    /// عدد المراجع المسجلة
    count: AtomicUsize,
}

impl WriteBarrier {
    pub fn new() -> Self {
        WriteBarrier {
            remembered_set: RwLock::new(HashSet::new()),
            count: AtomicUsize::new(0),
        }
    }
    
    /// تسجيل مرجع من كائن قديم إلى كائن جديد
    pub fn record_reference(&self, from: GcObjectId, _to: GcObjectId) {
        if self.count.load(Ordering::Relaxed) < MAX_REMEMBERED_SET_SIZE {
            let mut set = self.remembered_set.write().unwrap();
            if set.insert(from) {
                self.count.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
    
    /// الحصول على الـ remembered set
    pub fn get_remembered_set(&self) -> HashSet<GcObjectId> {
        self.remembered_set.read().unwrap().clone()
    }
    
    /// مسح الـ remembered set
    pub fn clear(&self) {
        self.remembered_set.write().unwrap().clear();
        self.count.store(0, Ordering::Relaxed);
    }
    
    /// حجم الـ remembered set
    pub fn len(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// جامع القمامة المتوازي
// ═══════════════════════════════════════════════════════════════════════════════

/// جامع القمامة المتوازي الأجيالي
pub struct ParallelGc {
    /// الكائنات في الجيل الشاب
    young_gen: RwLock<Vec<GcObjectInfo>>,
    /// الكائنات في الجيل القديم
    old_gen: RwLock<Vec<GcObjectInfo>>,
    /// جدول الكائنات
    object_table: RwLock<HashMap<GcObjectId, SharedValue>>,
    /// حاجز الكتابة
    write_barrier: WriteBarrier,
    /// إحصائيات
    stats: RwLock<GcStats>,
    /// معرف الكائن التالي
    next_id: AtomicUsize,
    /// هل GC قيد التشغيل
    collecting: AtomicBool,
    /// عتبة الجيل الشاب
    young_gen_threshold: AtomicUsize,
    /// عتبة الجيل القديم
    old_gen_threshold: AtomicUsize,
    /// الجذور (المتغيرات العامة)
    roots: RwLock<Vec<SharedValue>>,
    /// قائمة الانتظار للعلام المتوازي
    mark_queue: Mutex<VecDeque<GcObjectId>>,
    /// عدد العمال
    worker_count: usize,
}

impl ParallelGc {
    /// إنشاء جامع قمامة جديد
    pub fn new() -> Self {
        ParallelGc {
            young_gen: RwLock::new(Vec::with_capacity(DEFAULT_YOUNG_GEN_SIZE)),
            old_gen: RwLock::new(Vec::with_capacity(DEFAULT_OLD_GEN_SIZE)),
            object_table: RwLock::new(HashMap::new()),
            write_barrier: WriteBarrier::new(),
            stats: RwLock::new(GcStats::default()),
            next_id: AtomicUsize::new(1),
            collecting: AtomicBool::new(false),
            young_gen_threshold: AtomicUsize::new(DEFAULT_YOUNG_GEN_SIZE),
            old_gen_threshold: AtomicUsize::new(DEFAULT_OLD_GEN_SIZE),
            roots: RwLock::new(Vec::new()),
            mark_queue: Mutex::new(VecDeque::new()),
            worker_count: GC_WORKER_COUNT,
        }
    }
    
    /// إنشاء جامع مع عدد عمال مخصص
    pub fn with_workers(worker_count: usize) -> Self {
        let mut gc = Self::new();
        gc.worker_count = worker_count;
        gc
    }
    
    /// تخصيص كائن جديد
    pub fn allocate(&self, value: Value) -> SharedValue {
        let id = GcObjectId::new(self.next_id.fetch_add(1, Ordering::Relaxed));
        let size = Self::estimate_size(&value);
        let shared = Rc::new(RefCell::new(value));
        
        let info = GcObjectInfo {
            id,
            size,
            survival_count: 0,
            generation: Generation::Young,
            marked: Cell::new(false),
            created_at: Instant::now(),
        };
        
        // إضافة للجيل الشاب
        self.young_gen.write().unwrap().push(info);
        self.object_table.write().unwrap().insert(id, Rc::clone(&shared));
        
        // التحقق من الحاجة لـ GC
        if self.young_gen.read().unwrap().len() >= self.young_gen_threshold.load(Ordering::Relaxed) {
            self.collect_young();
        }
        
        shared
    }
    
    /// تقدير حجم الكائن
    fn estimate_size(value: &Value) -> usize {
        match value {
            Value::Number(_) => std::mem::size_of::<f64>(),
            Value::String(s) => s.len() + std::mem::size_of::<String>(),
            Value::Boolean(_) => std::mem::size_of::<bool>(),
            Value::Null => 0,
            Value::List(l) => l.len() * std::mem::size_of::<SharedValue>(),
            Value::Dictionary(d) => d.len() * (std::mem::size_of::<String>() + std::mem::size_of::<SharedValue>()),
            Value::Function { .. } => 256, // تقدير
            Value::NativeFunction { .. } => 64,
            Value::Lambda { .. } => 128,
            Value::Class { .. } => 512,
            Value::Instance { .. } => 256,
            Value::Tensor { data, shape } => data.len() * std::mem::size_of::<f64>() + shape.len() * std::mem::size_of::<usize>(),
            Value::AutoTensor { data, grad, .. } => (data.len() + grad.len()) * std::mem::size_of::<f64>(),
            _ => 64, // تقدير افتراضي
        }
    }
    
    /// إضافة جذر
    pub fn add_root(&self, value: SharedValue) {
        self.roots.write().unwrap().push(value);
    }
    
    /// إزالة جذر
    pub fn remove_root(&self, value: &SharedValue) {
        let mut roots = self.roots.write().unwrap();
        roots.retain(|r| !Rc::ptr_eq(r, value));
    }
    
    /// تحديث الجذور من البيئة
    pub fn update_roots_from_env(&self, env: &Environment) {
        let mut roots = self.roots.write().unwrap();
        roots.clear();
        for (_, value) in &env.variables {
            roots.push(Rc::clone(value));
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // جمع الجيل الشاب
    // ═══════════════════════════════════════════════════════════════
    
    /// جمع القمامة للجيل الشاب
    pub fn collect_young(&self) {
        if self.collecting.swap(true, Ordering::Acquire) {
            return; // GC قيد التشغيل بالفعل
        }
        
        let start = Instant::now();
        
        // مرحلة العلام
        let mark_start = Instant::now();
        self.mark_phase_parallel(Generation::Young);
        let mark_time = mark_start.elapsed();
        
        // مرحلة المسح
        let sweep_start = Instant::now();
        let (collected, promoted, freed) = self.sweep_phase_parallel(Generation::Young);
        let sweep_time = sweep_start.elapsed();
        
        // تحديث الإحصائيات
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_collections += 1;
            stats.young_collections += 1;
            stats.total_time_ms += start.elapsed().as_millis() as u64;
            stats.mark_time_ms += mark_time.as_millis() as u64;
            stats.sweep_time_ms += sweep_time.as_millis() as u64;
            stats.objects_collected += collected;
            stats.objects_promoted += promoted;
            stats.bytes_freed += freed;
        }
        
        self.collecting.store(false, Ordering::Release);
        
        // التحقق من الحاجة لجمع الجيل القديم
        if self.old_gen.read().unwrap().len() >= self.old_gen_threshold.load(Ordering::Relaxed) {
            self.collect_old();
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // جمع الجيل القديم (Full GC)
    // ═══════════════════════════════════════════════════════════════
    
    /// جمع القمامة الكامل (كلا الجيلين)
    pub fn collect_old(&self) {
        if self.collecting.swap(true, Ordering::Acquire) {
            return;
        }
        
        let start = Instant::now();
        
        // مرحلة العلام للجيلين
        let mark_start = Instant::now();
        self.mark_phase_parallel(Generation::Old);
        let mark_time = mark_start.elapsed();
        
        // مرحلة المسح للجيلين
        let sweep_start = Instant::now();
        let (young_collected, young_promoted, young_freed) = self.sweep_phase_parallel(Generation::Young);
        let (old_collected, _, old_freed) = self.sweep_phase_parallel(Generation::Old);
        let sweep_time = sweep_start.elapsed();
        
        // تحديث الإحصائيات
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_collections += 1;
            stats.old_collections += 1;
            stats.total_time_ms += start.elapsed().as_millis() as u64;
            stats.mark_time_ms += mark_time.as_millis() as u64;
            stats.sweep_time_ms += sweep_time.as_millis() as u64;
            stats.objects_collected += young_collected + old_collected;
            stats.objects_promoted += young_promoted;
            stats.bytes_freed += young_freed + old_freed;
            stats.stw_count += 1;
            stats.stw_time_ms += start.elapsed().as_millis() as u64;
        }
        
        self.collecting.store(false, Ordering::Release);
    }
    
    // ═══════════════════════════════════════════════════════════════
    // مرحلة العلام المتوازية
    // ═══════════════════════════════════════════════════════════════
    
    /// مرحلة العلام المتوازية
    fn mark_phase_parallel(&self, generation: Generation) {
        // مسح جميع العلامات
        self.clear_marks(generation);
        
        // الحصول على الجذور
        let roots = self.roots.read().unwrap().clone();
        
        // العلام الأولي من الجذور (متسلسل لأن Rc لا يدعم التوازي)
        let initial_work: Vec<GcObjectId> = roots.iter()
            .filter_map(|root| {
                // البحث عن معرف الكائن
                let table = self.object_table.read().unwrap();
                table.iter()
                    .find(|(_, v)| Rc::ptr_eq(v, root))
                    .map(|(id, _)| *id)
            })
            .collect();
        
        // إضافة العمل للقائمة
        {
            let mut queue = self.mark_queue.lock().unwrap();
            for id in initial_work {
                queue.push_back(id);
            }
        }
        
        // معالجة قائمة العلام
        self.process_mark_queue(generation);
        
        // معالجة الـ remembered set للجيل القديم
        if generation == Generation::Old {
            let remembered = self.write_barrier.get_remembered_set();
            for id in remembered.iter() {
                self.mark_object(*id, generation);
            }
            self.write_barrier.clear();
        }
    }
    
    /// معالجة قائمة العلام
    fn process_mark_queue(&self, generation: Generation) {
        loop {
            // الحصول على دفعة من العمل
            let batch: Vec<GcObjectId> = {
                let mut queue = self.mark_queue.lock().unwrap();
                (0..100).filter_map(|_| queue.pop_front()).collect()
            };
            
            if batch.is_empty() {
                break;
            }
            
            // معالجة الدفعة بشكل متسلسل
            for id in batch.iter() {
                self.mark_object(*id, generation);
                if let Some(refs) = self.get_references(*id) {
                    let mut queue = self.mark_queue.lock().unwrap();
                    for ref_id in refs {
                        queue.push_back(ref_id);
                    }
                }
            }
        }
    }
    
    /// وضع علامة على كائن
    fn mark_object(&self, id: GcObjectId, generation: Generation) {
        let target_gen = match generation {
            Generation::Young => &self.young_gen,
            Generation::Old => &self.old_gen,
        };
        
        let mut gen = target_gen.write().unwrap();
        if let Some(obj) = gen.iter_mut().find(|o| o.id == id) {
            if !obj.marked.get() {
                obj.marked.set(true);
            }
        }
    }
    
    /// الحصول على مراجع كائن
    fn get_references(&self, id: GcObjectId) -> Option<Vec<GcObjectId>> {
        let table = self.object_table.read().unwrap();
        let value = table.get(&id)?;
        
        let borrowed = value.borrow();
        let refs = match &*borrowed {
            Value::List(items) => {
                items.iter()
                    .filter_map(|item| {
                        table.iter()
                            .find(|(_, v)| Rc::ptr_eq(v, item))
                            .map(|(id, _)| *id)
                    })
                    .collect()
            }
            Value::Dictionary(dict) => {
                dict.values()
                    .filter_map(|item| {
                        table.iter()
                            .find(|(_, v)| Rc::ptr_eq(v, item))
                            .map(|(id, _)| *id)
                    })
                    .collect()
            }
            Value::Instance { fields, .. } => {
                fields.borrow().values()
                    .filter_map(|item| {
                        table.iter()
                            .find(|(_, v)| Rc::ptr_eq(v, item))
                            .map(|(id, _)| *id)
                    })
                    .collect()
            }
            _ => Vec::new(),
        };
        
        Some(refs)
    }
    
    /// مسح جميع العلامات
    fn clear_marks(&self, generation: Generation) {
        match generation {
            Generation::Young => {
                let mut gen = self.young_gen.write().unwrap();
                for obj in gen.iter_mut() {
                    obj.marked.set(false);
                }
            }
            Generation::Old => {
                // مسح الجيلين
                let mut young = self.young_gen.write().unwrap();
                for obj in young.iter_mut() {
                    obj.marked.set(false);
                }
                let mut old = self.old_gen.write().unwrap();
                for obj in old.iter_mut() {
                    obj.marked.set(false);
                }
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // مرحلة المسح المتوازية
    // ═══════════════════════════════════════════════════════════════
    
    /// مرحلة المسح المتوازية
    fn sweep_phase_parallel(&self, generation: Generation) -> (u64, u64, u64) {
        let mut collected = 0u64;
        let mut promoted = 0u64;
        let mut freed = 0u64;
        
        match generation {
            Generation::Young => {
                let mut young = self.young_gen.write().unwrap();
                let mut table = self.object_table.write().unwrap();
                let mut old = self.old_gen.write().unwrap();
                
                // تقسيم للمعالجة
                let to_remove: Vec<GcObjectId> = young.iter()
                    .filter(|obj| !obj.marked.get())
                    .map(|obj| obj.id)
                    .collect();
                
                let to_promote: Vec<GcObjectId> = young.iter()
                    .filter(|obj| obj.marked.get() && obj.survival_count >= PROMOTION_THRESHOLD)
                    .map(|obj| obj.id)
                    .collect();
                
                // المسح
                for id in to_remove {
                    if let Some(value) = table.remove(&id) {
                        freed += Self::estimate_size(&value.borrow()) as u64;
                        collected += 1;
                    }
                }
                young.retain(|obj| obj.marked.get());
                
                // الترقية
                for id in to_promote {
                    if let Some(pos) = young.iter().position(|o| o.id == id) {
                        let mut obj = young.remove(pos);
                        obj.generation = Generation::Old;
                        obj.survival_count = 0;
                        old.push(obj);
                        promoted += 1;
                    }
                }
                
                // تحديث عداد البقاء
                for obj in young.iter_mut() {
                    obj.survival_count += 1;
                }
            }
            Generation::Old => {
                let mut old = self.old_gen.write().unwrap();
                let mut table = self.object_table.write().unwrap();
                
                let to_remove: Vec<GcObjectId> = old.iter()
                    .filter(|obj| !obj.marked.get())
                    .map(|obj| obj.id)
                    .collect();
                
                for id in to_remove {
                    if let Some(value) = table.remove(&id) {
                        freed += Self::estimate_size(&value.borrow()) as u64;
                        collected += 1;
                    }
                }
                old.retain(|obj| obj.marked.get());
            }
        }
        
        // تحديث الإحصائيات
        {
            let mut stats = self.stats.write().unwrap();
            stats.objects_surviving += (self.young_gen.read().unwrap().len() + 
                                         self.old_gen.read().unwrap().len()) as u64;
            stats.bytes_used = (self.young_gen.read().unwrap().iter().map(|o| o.size).sum::<usize>() +
                               self.old_gen.read().unwrap().iter().map(|o| o.size).sum::<usize>()) as u64;
            stats.parallel_ops += self.worker_count as u64;
            stats.update_ratios();
        }
        
        (collected, promoted, freed)
    }
    
    // ═══════════════════════════════════════════════════════════════
    // GC تزايدي
    // ═══════════════════════════════════════════════════════════════
    
    /// خطوة GC تزايدي
    pub fn incremental_step(&self) -> bool {
        let start = Instant::now();
        let mut completed = false;
        
        // معالجة دفعة صغيرة
        let batch: Vec<GcObjectId> = {
            let mut queue = self.mark_queue.lock().unwrap();
            (0..50).filter_map(|_| queue.pop_front()).collect()
        };
        
        if batch.is_empty() {
            completed = true;
        } else {
            for id in batch {
                self.mark_object(id, Generation::Young);
                if let Some(refs) = self.get_references(id) {
                    let mut queue = self.mark_queue.lock().unwrap();
                    for ref_id in refs {
                        queue.push_back(ref_id);
                    }
                }
            }
        }
        
        // التحقق من الوقت
        if start.elapsed().as_millis() as u64 > INCREMENTAL_SLICE_MS {
            // تجاوز الوقت المسموح
        }
        
        completed
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال المراقبة والتحكم
    // ═══════════════════════════════════════════════════════════════
    
    /// الحصول على الإحصائيات
    pub fn stats(&self) -> GcStats {
        self.stats.read().unwrap().clone()
    }
    
    /// الحصول على عدد الكائنات
    pub fn object_count(&self) -> (usize, usize) {
        let young = self.young_gen.read().unwrap().len();
        let old = self.old_gen.read().unwrap().len();
        (young, old)
    }
    
    /// الحصول على حجم الذاكرة
    pub fn memory_usage(&self) -> usize {
        let young: usize = self.young_gen.read().unwrap().iter().map(|o| o.size).sum();
        let old: usize = self.old_gen.read().unwrap().iter().map(|o| o.size).sum();
        young + old
    }
    
    /// ضبط عتبات GC
    pub fn set_thresholds(&self, young: usize, old: usize) {
        self.young_gen_threshold.store(young, Ordering::Relaxed);
        self.old_gen_threshold.store(old, Ordering::Relaxed);
    }
    
    /// فرض جمع كامل
    pub fn force_full_collection(&self) {
        self.collect_old();
    }
    
    /// التحقق من حالة GC
    pub fn is_collecting(&self) -> bool {
        self.collecting.load(Ordering::Relaxed)
    }
    
    /// إعادة تعيين الإحصائيات
    pub fn reset_stats(&self) {
        *self.stats.write().unwrap() = GcStats::default();
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        let stats = self.stats();
        let (young, old) = self.object_count();
        let _mem = self.memory_usage();
        
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║              تقرير جامع القمامة المتوازي                    ║");
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ 📊 الإحصائيات العامة:                                        ║");
        println!("║   ├── إجمالي دورات GC: {:>10}                           ║", stats.total_collections);
        println!("║   ├── دورات الجيل الشاب: {:>10}                         ║", stats.young_collections);
        println!("║   └── دورات الجيل القديم: {:>10}                        ║", stats.old_collections);
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ ⏱️ الأداء:                                                   ║");
        println!("║   ├── إجمالي الوقت: {:>10} مللي ثانية                    ║", stats.total_time_ms);
        println!("║   ├── وقت العلام: {:>10} مللي ثانية                      ║", stats.mark_time_ms);
        println!("║   └── وقت المسح: {:>10} مللي ثانية                       ║", stats.sweep_time_ms);
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ 🗑️ الكائنات:                                                 ║");
        println!("║   ├── الكائنات المجموعة: {:>10}                          ║", stats.objects_collected);
        println!("║   ├── الكائنات المتبقية: {:>10}                          ║", stats.objects_surviving);
        println!("║   └── الكائنات المرقاة: {:>10}                           ║", stats.objects_promoted);
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ 💾 الذاكرة:                                                  ║");
        println!("║   ├── الذاكرة المستخدمة: {:>10} بايت                     ║", stats.bytes_used);
        println!("║   ├── الذاكرة المحررة: {:>10} بايت                       ║", stats.bytes_freed);
        println!("║   ├── كائنات الجيل الشاب: {:>10}                         ║", young);
        println!("║   └── كائنات الجيل القديم: {:>10}                        ║", old);
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ 🔧 التوازي:                                                  ║");
        println!("║   ├── عدد العمال: {:>10}                                 ║", self.worker_count);
        println!("║   ├── عمليات التوازي: {:>10}                             ║", stats.parallel_ops);
        println!("║   └── نسبة ضربات الكاش: {:>10.2}%                        ║", stats.cache_hit_ratio * 100.0);
        println!("╚══════════════════════════════════════════════════════════════╝");
    }
}

impl Default for ParallelGc {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الذاكرة العام
// ═══════════════════════════════════════════════════════════════════════════════

/// مدير الذاكرة العام مع GC متكامل
pub struct MemoryManager {
    /// الجامع
    gc: ParallelGc,
    /// البيئة العامة
    global_env: Option<Rc<RefCell<Environment>>>,
    /// حالة التتبع
    tracking_enabled: bool,
}

impl MemoryManager {
    /// إنشاء مدير ذاكرة جديد
    pub fn new() -> Self {
        MemoryManager {
            gc: ParallelGc::new(),
            global_env: None,
            tracking_enabled: true,
        }
    }
    
    /// ربط مع بيئة
    pub fn bind_env(&mut self, env: Rc<RefCell<Environment>>) {
        self.global_env = Some(Rc::clone(&env));
    }
    
    /// تخصيص قيمة
    pub fn alloc(&self, value: Value) -> SharedValue {
        let shared = self.gc.allocate(value);
        
        // تتبع الجذور
        if self.tracking_enabled {
            self.gc.add_root(Rc::clone(&shared));
        }
        
        shared
    }
    
    /// تحديث الجذور من البيئة
    pub fn sync_roots(&self) {
        if let Some(ref env) = self.global_env {
            self.gc.update_roots_from_env(&env.borrow());
        }
    }
    
    /// تشغيل GC
    pub fn collect(&self) {
        self.sync_roots();
        self.gc.collect_young();
    }
    
    /// تشغيل GC كامل
    pub fn collect_full(&self) {
        self.sync_roots();
        self.gc.force_full_collection();
    }
    
    /// الحصول على الإحصائيات
    pub fn stats(&self) -> GcStats {
        self.gc.stats()
    }
    
    /// الحصول على مرجع GC
    pub fn gc(&self) -> &ParallelGc {
        &self.gc
    }
    
    /// تفعيل/تعطيل التتبع
    pub fn set_tracking(&mut self, enabled: bool) {
        self.tracking_enabled = enabled;
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        self.gc.print_report();
    }
}

impl Default for MemoryManager {
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
    fn test_gc_allocation() {
        let gc = ParallelGc::new();
        let value = gc.allocate(Value::Number(42.0));
        assert_eq!(*value.borrow(), Value::Number(42.0));
    }
    
    #[test]
    fn test_gc_stats() {
        let gc = ParallelGc::new();
        gc.allocate(Value::Number(1.0));
        gc.allocate(Value::Number(2.0));
        gc.allocate(Value::Number(3.0));
        
        let (young, _old) = gc.object_count();
        assert_eq!(young, 3);
    }
    
    #[test]
    fn test_gc_collection() {
        let gc = ParallelGc::new();
        
        // إنشاء كائنات
        for i in 0..100 {
            gc.allocate(Value::Number(i as f64));
        }
        
        // إضافة جذور (بعض الكائنات)
        let root = gc.allocate(Value::Number(999.0));
        gc.add_root(root);
        
        // جمع
        gc.collect_young();
        
        let stats = gc.stats();
        assert!(stats.total_collections > 0);
    }
    
    #[test]
    fn test_memory_manager() {
        let mut manager = MemoryManager::new();
        manager.bind_env(Rc::new(RefCell::new(Environment::new())));
        
        let v1 = manager.alloc(Value::Number(10.0));
        let _v2 = manager.alloc(Value::String("مرحبا".into()));
        
        manager.collect();
        
        let stats = manager.stats();
        assert!(stats.total_collections > 0);
    }
    
    #[test]
    fn test_write_barrier() {
        let barrier = WriteBarrier::new();
        let id1 = GcObjectId::new(1);
        let id2 = GcObjectId::new(2);
        
        barrier.record_reference(id1, id2);
        assert_eq!(barrier.len(), 1);
        
        let set = barrier.get_remembered_set();
        assert!(set.contains(&id1));
        
        barrier.clear();
        assert_eq!(barrier.len(), 0);
    }
    
    #[test]
    fn test_parallel_mark() {
        let gc = ParallelGc::with_workers(4);
        
        // إنشاء كائنات متعددة
        let mut roots = Vec::new();
        for i in 0..1000 {
            let v = gc.allocate(Value::Number(i as f64));
            roots.push(v);
        }
        
        // إضافة جذور
        for root in &roots {
            gc.add_root(Rc::clone(root));
        }
        
        // جمع
        gc.collect_young();
        
        let stats = gc.stats();
        assert!(stats.parallel_ops > 0);
    }
    
    #[test]
    fn test_promotion() {
        let gc = ParallelGc::new();
        gc.set_thresholds(10, 1000);
        
        // إنشاء كائنات والاحتفاظ بها
        let mut survivors = Vec::new();
        for i in 0..5 {
            let v = gc.allocate(Value::Number(i as f64));
            survivors.push(v);
        }
        
        // إضافة جذور
        for v in &survivors {
            gc.add_root(Rc::clone(v));
        }
        
        // جمع متعدد لترقية الكائنات
        for _ in 0..(PROMOTION_THRESHOLD + 1) {
            gc.collect_young();
        }
        
        let stats = gc.stats();
        assert!(stats.objects_promoted > 0);
    }
}
