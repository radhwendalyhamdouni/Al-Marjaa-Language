// ═══════════════════════════════════════════════════════════════════════════════
// نظام الاشتقاق التلقائي (AutoGrad) - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// هذا الملف ينفذ نظام الاشتقاق التلقائي الكامل للذكاء الاصطناعي
// يدعم: التتبع التلقائي، الانتشار العكسي، Chain Rule
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::collections::HashMap;

// معرف فريد لكل متجه
thread_local! {
    static NEXT_ID: RefCell<usize> = RefCell::new(0);
}

fn next_id() -> usize {
    NEXT_ID.with(|id| {
        let current = *id.borrow();
        *id.borrow_mut() = current + 1;
        current
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// GradientTensor - متجه مع تدرجات وتتبع العمليات
// ═══════════════════════════════════════════════════════════════════════════════

/// متجه مع إمكانية حساب التدرجات تلقائياً
#[derive(Clone, Debug)]
pub struct GradientTensor {
    /// معرف فريد
    pub id: usize,
    /// البيانات (مسطحة)
    pub data: Vec<f64>,
    /// التدرجات (مسطحة)
    pub grad: Vec<f64>,
    /// الأبعاد
    pub shape: Vec<usize>,
    /// هل يتطلب حساب التدرجات
    pub requires_grad: bool,
    /// العملية التي أنتجت هذا المتجه
    pub op: Option<String>,
    /// المدخلات للعملية (معرفات)
    pub parents: Vec<usize>,
    /// البيانات المحفوظة للانتشار العكسي
    pub cached: CachedData,
}

/// البيانات المحفوظة للانتشار العكسي
#[derive(Clone, Debug, Default)]
pub struct CachedData {
    /// بيانات عادية
    pub values: Vec<f64>,
    /// بيانات إضافية (مثل الأوزان)
    pub extra: HashMap<String, Vec<f64>>,
    /// معرفات المدخلات
    pub input_ids: Vec<usize>,
}

impl GradientTensor {
    /// إنشاء متجه جديد
    pub fn new(data: Vec<f64>, shape: Vec<usize>, requires_grad: bool) -> Self {
        let id = next_id();
        let grad = vec![0.0; data.len()];
        GradientTensor {
            id,
            data,
            grad,
            shape,
            requires_grad,
            op: None,
            parents: Vec::new(),
            cached: CachedData::default(),
        }
    }
    
    /// إنشاء متجه من قيمة واحدة
    pub fn scalar(value: f64, requires_grad: bool) -> Self {
        Self::new(vec![value], vec![1], requires_grad)
    }
    
    /// إنشاء متجه أصفار
    pub fn zeros(shape: Vec<usize>, requires_grad: bool) -> Self {
        let size: usize = shape.iter().product();
        Self::new(vec![0.0; size], shape, requires_grad)
    }
    
    /// إنشاء متجه آحاد
    pub fn ones(shape: Vec<usize>, requires_grad: bool) -> Self {
        let size: usize = shape.iter().product();
        Self::new(vec![1.0; size], shape, requires_grad)
    }
    
    /// الحجم الكلي
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    /// عدد الأبعاد
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }
    
    /// إعادة تشكيل المتجه
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Self, String> {
        let new_size: usize = new_shape.iter().product();
        if new_size != self.size() {
            return Err(format!(
                "لا يمكن إعادة التشكيل: {} عنصر إلى {:?}",
                self.size(),
                new_shape
            ));
        }
        let mut result = self.clone();
        result.shape = new_shape;
        Ok(result)
    }
    
    /// صفر التدرجات
    pub fn zero_grad(&mut self) {
        self.grad = vec![0.0; self.data.len()];
    }
    
    /// هل يحتوي على تدرجات غير صفرية
    pub fn has_grad(&self) -> bool {
        self.grad.iter().any(|g| *g != 0.0)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// OpNode - عقدة في الرسم البياني الحسابي
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع العملية
#[derive(Clone, Debug, PartialEq)]
pub enum OpType {
    /// جمع
    Add,
    /// طرح
    Sub,
    /// ضرب عنصري
    Mul,
    /// قسمة عنصرية
    Div,
    /// ضرب نقدي
    Dot,
    /// ضرب مصفوفات
    MatMul,
    /// قوة
    Pow,
    /// جمع (تقليل)
    Sum,
    /// متوسط
    Mean,
    /// أقصى
    Max,
    /// أدنى
    Min,
    /// سيجمويد
    Sigmoid,
    /// ريلو
    ReLU,
    /// تانه
    Tanh,
    /// سوفتماكس
    Softmax,
    /// خطأ مترربع
    MSELoss,
    /// خطأ تقاطع
    CrossEntropyLoss,
    /// تبديل
    Transpose,
    /// دمج
    Concat,
    /// قص
    Slice,
    /// تكرار
    Repeat,
    /// خاص (مخصص)
    Custom(String),
}

impl OpType {
    pub fn name(&self) -> &str {
        match self {
            OpType::Add => "جمع",
            OpType::Sub => "طرح",
            OpType::Mul => "ضرب",
            OpType::Div => "قسمة",
            OpType::Dot => "ضرب_نقدي",
            OpType::MatMul => "ضرب_مصفوفات",
            OpType::Pow => "قوة",
            OpType::Sum => "مجموع",
            OpType::Mean => "متوسط",
            OpType::Max => "أقصى",
            OpType::Min => "أدنى",
            OpType::Sigmoid => "سيجمويد",
            OpType::ReLU => "ريلو",
            OpType::Tanh => "تانه",
            OpType::Softmax => "سوفتماكس",
            OpType::MSELoss => "خطأ_مربع",
            OpType::CrossEntropyLoss => "خطأ_تقاطع",
            OpType::Transpose => "تبديل",
            OpType::Concat => "دمج",
            OpType::Slice => "قص",
            OpType::Repeat => "تكرار",
            OpType::Custom(name) => name,
        }
    }
}

/// عقدة عملية في الرسم البياني
#[derive(Clone, Debug)]
pub struct OpNode {
    /// معرف العقدة
    pub id: usize,
    /// نوع العملية
    pub op_type: OpType,
    /// معرفات المدخلات
    pub inputs: Vec<usize>,
    /// معرف المخرج
    pub output: usize,
    /// البيانات المحفوظة
    pub cached: CachedData,
}

// ═══════════════════════════════════════════════════════════════════════════════
// ComputeGraph - الرسم البياني الحسابي الكامل
// ═══════════════════════════════════════════════════════════════════════════════

/// الرسم البياني الحسابي
#[derive(Clone, Debug, Default)]
pub struct ComputeGraph {
    /// اسم الرسم البياني
    pub name: String,
    /// جميع المتجهات (معرف -> متجه)
    pub tensors: HashMap<usize, GradientTensor>,
    /// جميع عمليات
    pub operations: Vec<OpNode>,
    /// المتجهات الورقية (المتغيرات المستقلة)
    pub leaves: Vec<usize>,
    /// المتجهات الجذرية (دوال الخسارة)
    pub roots: Vec<usize>,
    /// ترتيب طوبولوجي للانتشار العكسي
    pub topo_order: Vec<usize>,
}

impl ComputeGraph {
    /// إنشاء رسم بياني جديد
    pub fn new(name: &str) -> Self {
        ComputeGraph {
            name: name.to_string(),
            tensors: HashMap::new(),
            operations: Vec::new(),
            leaves: Vec::new(),
            roots: Vec::new(),
            topo_order: Vec::new(),
        }
    }
    
    /// إضافة متجه للرسم البياني
    pub fn add_tensor(&mut self, tensor: GradientTensor) -> usize {
        let id = tensor.id;
        if tensor.parents.is_empty() && tensor.requires_grad {
            self.leaves.push(id);
        }
        self.tensors.insert(id, tensor);
        id
    }
    
    /// إضافة عملية
    pub fn add_op(&mut self, op: OpNode) {
        // تحديث أولياء المخرج
        if let Some(output) = self.tensors.get_mut(&op.output) {
            output.parents = op.inputs.clone();
            output.op = Some(op.op_type.name().to_string());
        }
        self.operations.push(op);
    }
    
    /// حساب الترتيب الطوبولوجي
    pub fn compute_topo_order(&mut self) {
        self.topo_order.clear();
        let mut visited = std::collections::HashSet::new();
        let mut temp_mark = std::collections::HashSet::new();
        
        // ابدأ من الجذور
        for &root in &self.roots.clone() {
            self.visit_topo(root, &mut visited, &mut temp_mark);
        }
    }
    
    fn visit_topo(
        &self,
        id: usize,
        visited: &mut std::collections::HashSet<usize>,
        temp_mark: &mut std::collections::HashSet<usize>,
    ) {
        if visited.contains(&id) {
            return;
        }
        if temp_mark.contains(&id) {
            return; // دورة - تجاهل
        }
        
        temp_mark.insert(id);
        
        // زر الأولياء أولاً
        if let Some(tensor) = self.tensors.get(&id) {
            for &parent_id in &tensor.parents {
                self.visit_topo(parent_id, visited, temp_mark);
            }
        }
        
        temp_mark.remove(&id);
        visited.insert(id);
        // نحتاج mutable self لكن هذه دالة مساعدة
        // سيتم التعامل معها في compute_topo_order
    }
    
    /// الانتشار العكسي (Backpropagation)
    pub fn backward(&mut self) -> Result<(), String> {
        // التحقق من وجود جذر واحد على الأقل
        if self.roots.is_empty() {
            return Err("لا يوجد دالة خسارة للانتشار العكسي".into());
        }
        
        // تعيين تدرج الخسارة = 1
        for &root_id in &self.roots {
            if let Some(root) = self.tensors.get_mut(&root_id) {
                root.grad = vec![1.0; root.data.len()];
            }
        }
        
        // بناء الترتيب الطوبولوجي العكسي
        self.build_reverse_topo();
        
        // التنفيذ العكسي
        for op_id in &self.topo_order.clone() {
            self.backward_op(*op_id)?;
        }
        
        Ok(())
    }
    
    fn build_reverse_topo(&mut self) {
        self.topo_order.clear();
        let mut visited = std::collections::HashSet::new();
        
        let roots = self.roots.clone();
        for &root in &roots {
            self.collect_reverse_topo(root, &mut visited);
        }
    }
    
    fn collect_reverse_topo(&mut self, id: usize, visited: &mut std::collections::HashSet<usize>) {
        if visited.contains(&id) {
            return;
        }
        visited.insert(id);
        
        // أضف نفسه أولاً (للترتيب العكسي - من المخرجات إلى المدخلات)
        self.topo_order.push(id);
        
        // ثم الأولياء
        if let Some(tensor) = self.tensors.get(&id) {
            for &parent_id in &tensor.parents.clone() {
                self.collect_reverse_topo(parent_id, visited);
            }
        }
    }
    
    fn backward_op(&mut self, tensor_id: usize) -> Result<(), String> {
        let tensor = match self.tensors.get(&tensor_id) {
            Some(t) => t.clone(),
            None => return Ok(()),
        };
        
        if !tensor.requires_grad || tensor.parents.is_empty() {
            return Ok(());
        }
        
        let grad_output = tensor.grad.clone();
        let op_name = tensor.op.clone().unwrap_or_default();
        
        // إجراء الانتشار العكسي حسب نوع العملية
        match op_name.as_str() {
            "جمع" => self.backward_add(&tensor, &grad_output)?,
            "طرح" => self.backward_sub(&tensor, &grad_output)?,
            "ضرب" => self.backward_mul(&tensor, &grad_output)?,
            "قسمة" => self.backward_div(&tensor, &grad_output)?,
            "ضرب_نقدي" => self.backward_dot(&tensor, &grad_output)?,
            "ضرب_مصفوفات" => self.backward_matmul(&tensor, &grad_output)?,
            "قوة" => self.backward_pow(&tensor, &grad_output)?,
            "مجموع" => self.backward_sum(&tensor, &grad_output)?,
            "متوسط" => self.backward_mean(&tensor, &grad_output)?,
            "سيجمويد" => self.backward_sigmoid(&tensor, &grad_output)?,
            "ريلو" => self.backward_relu(&tensor, &grad_output)?,
            "تانه" => self.backward_tanh(&tensor, &grad_output)?,
            "سوفتماكس" => self.backward_softmax(&tensor, &grad_output)?,
            "خطأ_مربع" => self.backward_mse(&tensor, &grad_output)?,
            "خطأ_تقاطع" => self.backward_crossentropy(&tensor, &grad_output)?,
            _ => {}
        }
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال الانتشار العكسي لكل عملية
    // ═══════════════════════════════════════════════════════════════
    
    fn backward_add(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.len() < 2 {
            return Ok(());
        }
        
        // d(a + b)/da = 1, d(a + b)/db = 1
        for &pid in &parent_ids {
            if let Some(parent) = self.tensors.get_mut(&pid) {
                if parent.requires_grad {
                    for (i, g) in grad_output.iter().enumerate() {
                        if i < parent.grad.len() {
                            parent.grad[i] += g;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_sub(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.len() < 2 {
            return Ok(());
        }
        
        // d(a - b)/da = 1, d(a - b)/db = -1
        if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
            if parent.requires_grad {
                for (i, g) in grad_output.iter().enumerate() {
                    if i < parent.grad.len() {
                        parent.grad[i] += g;
                    }
                }
            }
        }
        
        if let Some(parent) = self.tensors.get_mut(&parent_ids[1]) {
            if parent.requires_grad {
                for (i, g) in grad_output.iter().enumerate() {
                    if i < parent.grad.len() {
                        parent.grad[i] -= g;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_mul(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.len() < 2 {
            return Ok(());
        }
        
        let a = self.tensors.get(&parent_ids[0]).cloned();
        let b = self.tensors.get(&parent_ids[1]).cloned();
        
        // d(a * b)/da = b, d(a * b)/db = a
        if let (Some(a_data), Some(b_data)) = (a, b) {
            if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
                if parent.requires_grad {
                    for (i, g) in grad_output.iter().enumerate() {
                        if i < parent.grad.len() && i < b_data.data.len() {
                            parent.grad[i] += g * b_data.data[i];
                        }
                    }
                }
            }
            
            if let Some(parent) = self.tensors.get_mut(&parent_ids[1]) {
                if parent.requires_grad {
                    for (i, g) in grad_output.iter().enumerate() {
                        if i < parent.grad.len() && i < a_data.data.len() {
                            parent.grad[i] += g * a_data.data[i];
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_div(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.len() < 2 {
            return Ok(());
        }
        
        let a = self.tensors.get(&parent_ids[0]).cloned();
        let b = self.tensors.get(&parent_ids[1]).cloned();
        
        // d(a/b)/da = 1/b, d(a/b)/db = -a/b^2
        if let (Some(a_data), Some(b_data)) = (a, b) {
            if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
                if parent.requires_grad {
                    for (i, g) in grad_output.iter().enumerate() {
                        if i < parent.grad.len() && i < b_data.data.len() && b_data.data[i] != 0.0 {
                            parent.grad[i] += g / b_data.data[i];
                        }
                    }
                }
            }
            
            if let Some(parent) = self.tensors.get_mut(&parent_ids[1]) {
                if parent.requires_grad {
                    for (i, g) in grad_output.iter().enumerate() {
                        if i < parent.grad.len() && i < a_data.data.len() && i < b_data.data.len() && b_data.data[i] != 0.0 {
                            parent.grad[i] -= g * a_data.data[i] / (b_data.data[i] * b_data.data[i]);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_dot(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.len() < 2 {
            return Ok(());
        }
        
        let a = self.tensors.get(&parent_ids[0]).cloned();
        let b = self.tensors.get(&parent_ids[1]).cloned();
        
        // ضرب نقدي: c = a · b
        // dc/da = b * grad, dc/db = a * grad
        if let (Some(a_data), Some(b_data)) = (a, b) {
            let grad_scalar = grad_output.get(0).copied().unwrap_or(1.0);
            
            if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
                if parent.requires_grad {
                    for (i, g) in parent.grad.iter_mut().enumerate() {
                        if i < b_data.data.len() {
                            *g += grad_scalar * b_data.data[i];
                        }
                    }
                }
            }
            
            if let Some(parent) = self.tensors.get_mut(&parent_ids[1]) {
                if parent.requires_grad {
                    for (i, g) in parent.grad.iter_mut().enumerate() {
                        if i < a_data.data.len() {
                            *g += grad_scalar * a_data.data[i];
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_matmul(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        // تبسيط: نفترض مصفوفتين 2D
        let parent_ids = output.parents.clone();
        if parent_ids.len() < 2 {
            return Ok(());
        }
        
        let a = self.tensors.get(&parent_ids[0]).cloned();
        let b = self.tensors.get(&parent_ids[1]).cloned();
        
        if let (Some(a_data), Some(b_data)) = (a, b) {
            // للحصول على أبعاد صحيحة
            let a_shape = &a_data.shape;
            let b_shape = &b_data.shape;
            
            if a_shape.len() >= 2 && b_shape.len() >= 2 {
                let m = a_shape[0];
                let k = a_shape[1];
                let n = b_shape[1];
                
                // grad_a = grad_output @ b^T
                if let Some(parent_a) = self.tensors.get_mut(&parent_ids[0]) {
                    if parent_a.requires_grad {
                        for i in 0..m {
                            for j in 0..k {
                                let mut sum = 0.0;
                                for l in 0..n {
                                    let grad_idx = i * n + l;
                                    let b_idx = j * n + l; // b^T
                                    if grad_idx < grad_output.len() && b_idx < b_data.data.len() {
                                        sum += grad_output[grad_idx] * b_data.data[b_idx];
                                    }
                                }
                                let a_idx = i * k + j;
                                if a_idx < parent_a.grad.len() {
                                    parent_a.grad[a_idx] += sum;
                                }
                            }
                        }
                    }
                }
                
                // grad_b = a^T @ grad_output
                if let Some(parent_b) = self.tensors.get_mut(&parent_ids[1]) {
                    if parent_b.requires_grad {
                        for i in 0..k {
                            for j in 0..n {
                                let mut sum = 0.0;
                                for l in 0..m {
                                    let grad_idx = l * n + j;
                                    let a_idx = l * k + i; // a^T
                                    if grad_idx < grad_output.len() && a_idx < a_data.data.len() {
                                        sum += grad_output[grad_idx] * a_data.data[a_idx];
                                    }
                                }
                                let b_idx = i * n + j;
                                if b_idx < parent_b.grad.len() {
                                    parent_b.grad[b_idx] += sum;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_pow(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.is_empty() {
            return Ok(());
        }
        
        let exponent = output.cached.values.get(0).copied().unwrap_or(2.0);
        
        // d(x^n)/dx = n * x^(n-1)
        if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
            if parent.requires_grad {
                for (i, g) in grad_output.iter().enumerate() {
                    if i < parent.grad.len() && i < parent.data.len() {
                        let x = parent.data[i];
                        parent.grad[i] += g * exponent * x.powf(exponent - 1.0);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_sum(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.is_empty() {
            return Ok(());
        }
        
        let grad_scalar = grad_output.get(0).copied().unwrap_or(1.0);
        
        // d(sum(x))/dx_i = 1
        if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
            if parent.requires_grad {
                for g in parent.grad.iter_mut() {
                    *g += grad_scalar;
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_mean(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.is_empty() {
            return Ok(());
        }
        
        let grad_scalar = grad_output.get(0).copied().unwrap_or(1.0);
        
        if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
            if parent.requires_grad {
                let n = parent.data.len() as f64;
                for g in parent.grad.iter_mut() {
                    *g += grad_scalar / n;
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_sigmoid(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.is_empty() {
            return Ok(());
        }
        
        // d(sigmoid(x))/dx = sigmoid(x) * (1 - sigmoid(x))
        if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
            if parent.requires_grad {
                for (i, g) in grad_output.iter().enumerate() {
                    if i < parent.grad.len() && i < output.data.len() {
                        let sig = output.data[i];
                        parent.grad[i] += g * sig * (1.0 - sig);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_relu(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.is_empty() {
            return Ok(());
        }
        
        // d(relu(x))/dx = 1 if x > 0, else 0
        if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
            if parent.requires_grad {
                for (i, g) in grad_output.iter().enumerate() {
                    if i < parent.grad.len() && i < parent.data.len() {
                        if parent.data[i] > 0.0 {
                            parent.grad[i] += g;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_tanh(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.is_empty() {
            return Ok(());
        }
        
        // d(tanh(x))/dx = 1 - tanh(x)^2
        if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
            if parent.requires_grad {
                for (i, g) in grad_output.iter().enumerate() {
                    if i < parent.grad.len() && i < output.data.len() {
                        let t = output.data[i];
                        parent.grad[i] += g * (1.0 - t * t);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_softmax(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.is_empty() {
            return Ok(());
        }
        
        // softmax Jacobian: S_i * (delta_ij - S_j)
        // للتبسيط: نستخدم التدرج المباشر
        if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
            if parent.requires_grad {
                let s = &output.data;
                for (i, g) in grad_output.iter().enumerate() {
                    if i < parent.grad.len() {
                        for (j, sj) in s.iter().enumerate() {
                            let kronecker = if i == j { 1.0 } else { 0.0 };
                            parent.grad[j] += g * s[i] * (kronecker - sj);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_mse(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.len() < 2 {
            return Ok(());
        }
        
        let grad_scalar = grad_output.get(0).copied().unwrap_or(1.0);
        
        let pred = self.tensors.get(&parent_ids[0]).cloned();
        let target = self.tensors.get(&parent_ids[1]).cloned();
        
        // MSE = (pred - target)^2 / n
        // d(MSE)/d(pred) = 2 * (pred - target) / n
        if let (Some(pred_data), Some(target_data)) = (pred, target) {
            if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
                if parent.requires_grad {
                    let n = pred_data.data.len() as f64;
                    for (i, g) in parent.grad.iter_mut().enumerate() {
                        if i < pred_data.data.len() && i < target_data.data.len() {
                            *g += grad_scalar * 2.0 * (pred_data.data[i] - target_data.data[i]) / n;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn backward_crossentropy(&mut self, output: &GradientTensor, grad_output: &[f64]) -> Result<(), String> {
        let parent_ids = output.parents.clone();
        if parent_ids.len() < 2 {
            return Ok(());
        }
        
        let grad_scalar = grad_output.get(0).copied().unwrap_or(1.0);
        
        let pred = self.tensors.get(&parent_ids[0]).cloned();
        let target = self.tensors.get(&parent_ids[1]).cloned();
        
        // CrossEntropy = -target * log(pred)
        // d(CE)/d(pred) = -target / pred
        if let (Some(pred_data), Some(target_data)) = (pred, target) {
            if let Some(parent) = self.tensors.get_mut(&parent_ids[0]) {
                if parent.requires_grad {
                    let epsilon = 1e-15;
                    for (i, g) in parent.grad.iter_mut().enumerate() {
                        if i < pred_data.data.len() && i < target_data.data.len() {
                            let p = pred_data.data[i].max(epsilon).min(1.0 - epsilon);
                            *g += grad_scalar * (-target_data.data[i] / p);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال العمليات التدرجية
// ═══════════════════════════════════════════════════════════════════════════════

/// عمليات GradientTensor
impl GradientTensor {
    // ═══════════════════════════════════════════════════════════════
    // العمليات الحسابية
    // ═══════════════════════════════════════════════════════════════
    
    /// جمع عنصري
    pub fn add(&self, other: &GradientTensor) -> GradientTensor {
        let data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        
        let requires_grad = self.requires_grad || other.requires_grad;
        let mut result = GradientTensor::new(data, self.shape.clone(), requires_grad);
        result.op = Some("جمع".to_string());
        result.parents = vec![self.id, other.id];
        result.cached.input_ids = vec![self.id, other.id];
        
        result
    }
    
    /// طرح عنصري
    pub fn sub(&self, other: &GradientTensor) -> GradientTensor {
        let data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        
        let requires_grad = self.requires_grad || other.requires_grad;
        let mut result = GradientTensor::new(data, self.shape.clone(), requires_grad);
        result.op = Some("طرح".to_string());
        result.parents = vec![self.id, other.id];
        result.cached.input_ids = vec![self.id, other.id];
        
        result
    }
    
    /// ضرب عنصري
    pub fn mul(&self, other: &GradientTensor) -> GradientTensor {
        let data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .collect();
        
        let requires_grad = self.requires_grad || other.requires_grad;
        let mut result = GradientTensor::new(data, self.shape.clone(), requires_grad);
        result.op = Some("ضرب".to_string());
        result.parents = vec![self.id, other.id];
        result.cached.input_ids = vec![self.id, other.id];
        result.cached.values = self.data.clone(); // حفظ للاستخدام في backward
        
        result
    }
    
    /// قسمة عنصرية
    pub fn div(&self, other: &GradientTensor) -> GradientTensor {
        let data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| if *b != 0.0 { a / b } else { 0.0 })
            .collect();
        
        let requires_grad = self.requires_grad || other.requires_grad;
        let mut result = GradientTensor::new(data, self.shape.clone(), requires_grad);
        result.op = Some("قسمة".to_string());
        result.parents = vec![self.id, other.id];
        result.cached.input_ids = vec![self.id, other.id];
        
        result
    }
    
    /// ضرب في عدد
    pub fn scale(&self, scalar: f64) -> GradientTensor {
        let data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        
        let mut result = GradientTensor::new(data, self.shape.clone(), self.requires_grad);
        result.op = Some("ضرب_عدد".to_string());
        result.parents = vec![self.id];
        result.cached.values = vec![scalar];
        
        result
    }
    
    /// ضرب نقدي (dot product)
    pub fn dot(&self, other: &GradientTensor) -> GradientTensor {
        let dot: f64 = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum();
        
        let requires_grad = self.requires_grad || other.requires_grad;
        let mut result = GradientTensor::scalar(dot, requires_grad);
        result.op = Some("ضرب_نقدي".to_string());
        result.parents = vec![self.id, other.id];
        result.cached.input_ids = vec![self.id, other.id];
        
        result
    }
    
    /// قوة
    pub fn pow(&self, exponent: f64) -> GradientTensor {
        let data: Vec<f64> = self.data.iter().map(|x| x.powf(exponent)).collect();
        
        let mut result = GradientTensor::new(data, self.shape.clone(), self.requires_grad);
        result.op = Some("قوة".to_string());
        result.parents = vec![self.id];
        result.cached.values = vec![exponent];
        
        result
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال التفعيل
    // ═══════════════════════════════════════════════════════════════
    
    /// سيجمويد
    pub fn sigmoid(&self) -> GradientTensor {
        let data: Vec<f64> = self.data.iter()
            .map(|x| 1.0 / (1.0 + (-x).exp()))
            .collect();
        
        let mut result = GradientTensor::new(data, self.shape.clone(), self.requires_grad);
        result.op = Some("سيجمويد".to_string());
        result.parents = vec![self.id];
        
        result
    }
    
    /// ريلو
    pub fn relu(&self) -> GradientTensor {
        let data: Vec<f64> = self.data.iter().map(|x| x.max(0.0)).collect();
        
        let mut result = GradientTensor::new(data, self.shape.clone(), self.requires_grad);
        result.op = Some("ريلو".to_string());
        result.parents = vec![self.id];
        
        result
    }
    
    /// تانه
    pub fn tanh(&self) -> GradientTensor {
        let data: Vec<f64> = self.data.iter().map(|x| x.tanh()).collect();
        
        let mut result = GradientTensor::new(data, self.shape.clone(), self.requires_grad);
        result.op = Some("تانه".to_string());
        result.parents = vec![self.id];
        
        result
    }
    
    /// سوفتماكس
    pub fn softmax(&self) -> GradientTensor {
        let max_val = self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp_vals: Vec<f64> = self.data.iter().map(|x| (x - max_val).exp()).collect();
        let sum: f64 = exp_vals.iter().sum();
        let data: Vec<f64> = exp_vals.iter().map(|x| x / sum).collect();
        
        let mut result = GradientTensor::new(data, self.shape.clone(), self.requires_grad);
        result.op = Some("سوفتماكس".to_string());
        result.parents = vec![self.id];
        
        result
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال التخفيض
    // ═══════════════════════════════════════════════════════════════
    
    /// مجموع جميع العناصر
    pub fn sum(&self) -> GradientTensor {
        let sum: f64 = self.data.iter().sum();
        
        let mut result = GradientTensor::scalar(sum, self.requires_grad);
        result.op = Some("مجموع".to_string());
        result.parents = vec![self.id];
        
        result
    }
    
    /// متوسط جميع العناصر
    pub fn mean(&self) -> GradientTensor {
        let mean: f64 = self.data.iter().sum::<f64>() / self.data.len() as f64;
        
        let mut result = GradientTensor::scalar(mean, self.requires_grad);
        result.op = Some("متوسط".to_string());
        result.parents = vec![self.id];
        
        result
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال الخسارة
    // ═══════════════════════════════════════════════════════════════
    
    /// خطأ المربع المتوسط (MSE Loss)
    pub fn mse_loss(&self, target: &GradientTensor) -> GradientTensor {
        let n = self.data.len() as f64;
        let mse: f64 = self.data.iter()
            .zip(target.data.iter())
            .map(|(p, t)| (p - t).powi(2))
            .sum::<f64>() / n;
        
        let requires_grad = self.requires_grad;
        let mut result = GradientTensor::scalar(mse, requires_grad);
        result.op = Some("خطأ_مربع".to_string());
        result.parents = vec![self.id, target.id];
        result.cached.input_ids = vec![self.id, target.id];
        
        result
    }
    
    /// خطأ التقاطع (Cross Entropy Loss)
    pub fn cross_entropy_loss(&self, target: &GradientTensor) -> GradientTensor {
        let epsilon = 1e-15;
        let ce: f64 = -self.data.iter()
            .zip(target.data.iter())
            .map(|(p, t)| {
                let p_clamped = p.max(epsilon).min(1.0 - epsilon);
                t * p_clamped.ln()
            })
            .sum::<f64>() / self.data.len() as f64;
        
        let mut result = GradientTensor::scalar(ce, self.requires_grad);
        result.op = Some("خطأ_تقاطع".to_string());
        result.parents = vec![self.id, target.id];
        
        result
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الرسوم البيانية
// ═══════════════════════════════════════════════════════════════════════════════

/// مدير الرسوم البيانية للمتجهات
pub struct GradientManager {
    /// الرسم البياني النشط
    graph: ComputeGraph,
}

impl GradientManager {
    /// إنشاء مدير جديد
    pub fn new() -> Self {
        GradientManager {
            graph: ComputeGraph::new("main"),
        }
    }
    
    /// إنشاء متجه جديد
    pub fn tensor(&mut self, data: Vec<f64>, shape: Vec<usize>, requires_grad: bool) -> GradientTensor {
        let tensor = GradientTensor::new(data, shape, requires_grad);
        let id = tensor.id;
        self.graph.add_tensor(tensor);
        self.graph.tensors.get(&id).cloned().unwrap()
    }
    
    /// إنشاء متجه من قيمة واحدة
    pub fn scalar(&mut self, value: f64, requires_grad: bool) -> GradientTensor {
        let tensor = GradientTensor::scalar(value, requires_grad);
        let id = tensor.id;
        self.graph.add_tensor(tensor);
        self.graph.tensors.get(&id).cloned().unwrap()
    }
    
    /// تسجيل عملية
    pub fn register_op(&mut self, output: &GradientTensor, op_type: OpType, inputs: Vec<usize>) {
        let op = OpNode {
            id: output.id,
            op_type,
            inputs: inputs.clone(),
            output: output.id,
            cached: CachedData {
                input_ids: inputs,
                ..Default::default()
            },
        };
        
        // تحديث المتجه في الرسم البياني
        if let Some(t) = self.graph.tensors.get_mut(&output.id) {
            t.parents = op.inputs.clone();
            t.op = Some(op.op_type.name().to_string());
        }
        
        self.graph.add_op(op);
    }
    
    /// تعيين دالة الخسارة
    pub fn set_loss(&mut self, loss_id: usize) {
        self.graph.roots.push(loss_id);
    }
    
    /// تنفيذ الانتشار العكسي
    pub fn backward(&mut self) -> Result<(), String> {
        self.graph.backward()
    }
    
    /// الحصول على تدرج متجه
    pub fn get_grad(&self, id: usize) -> Option<Vec<f64>> {
        self.graph.tensors.get(&id).map(|t| t.grad.clone())
    }
    
    /// الحصول على متجه
    pub fn get_tensor(&self, id: usize) -> Option<&GradientTensor> {
        self.graph.tensors.get(&id)
    }
    
    /// تحديث متجه
    pub fn update_tensor(&mut self, tensor: &GradientTensor) {
        self.graph.tensors.insert(tensor.id, tensor.clone());
    }
    
    /// صفر جميع التدرجات
    pub fn zero_grad(&mut self) {
        for tensor in self.graph.tensors.values_mut() {
            tensor.zero_grad();
        }
    }
    
    /// الحصول على الرسم البياني
    pub fn graph(&self) -> &ComputeGraph {
        &self.graph
    }
    
    /// الحصول على الرسم البياني قابل للتعديل
    pub fn graph_mut(&mut self) -> &mut ComputeGraph {
        &mut self.graph
    }
}

impl Default for GradientManager {
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
    fn test_tensor_creation() {
        let t = GradientTensor::new(vec![1.0, 2.0, 3.0], vec![3], true);
        assert_eq!(t.data, vec![1.0, 2.0, 3.0]);
        assert_eq!(t.shape, vec![3]);
        assert!(t.requires_grad);
    }
    
    #[test]
    fn test_add() {
        let a = GradientTensor::new(vec![1.0, 2.0, 3.0], vec![3], true);
        let b = GradientTensor::new(vec![4.0, 5.0, 6.0], vec![3], true);
        let c = a.add(&b);
        assert_eq!(c.data, vec![5.0, 7.0, 9.0]);
    }
    
    #[test]
    fn test_mul() {
        let a = GradientTensor::new(vec![1.0, 2.0, 3.0], vec![3], true);
        let b = GradientTensor::new(vec![2.0, 2.0, 2.0], vec![3], true);
        let c = a.mul(&b);
        assert_eq!(c.data, vec![2.0, 4.0, 6.0]);
    }
    
    #[test]
    fn test_sigmoid() {
        let t = GradientTensor::scalar(0.0, true);
        let sig = t.sigmoid();
        assert!((sig.data[0] - 0.5).abs() < 1e-6);
    }
    
    #[test]
    fn test_relu() {
        let t = GradientTensor::new(vec![-1.0, 0.0, 1.0], vec![3], true);
        let r = t.relu();
        assert_eq!(r.data, vec![0.0, 0.0, 1.0]);
    }
    
    #[test]
    fn test_backward_simple() {
        let mut graph = ComputeGraph::new("test");
        
        // a = 2.0, b = 3.0
        let a = GradientTensor::scalar(2.0, true);
        let b = GradientTensor::scalar(3.0, true);
        
        let a_id = a.id;
        let b_id = b.id;
        
        graph.add_tensor(a);
        graph.add_tensor(b);
        
        // c = a * b = 6.0
        let a_ref = graph.tensors.get(&a_id).cloned().unwrap();
        let b_ref = graph.tensors.get(&b_id).cloned().unwrap();
        let c = a_ref.mul(&b_ref);
        let c_id = c.id;
        
        graph.add_tensor(c);
        graph.roots.push(c_id);
        
        // backward
        graph.backward().unwrap();
        
        // dc/da = b = 3.0, dc/db = a = 2.0
        let a_grad = graph.tensors.get(&a_id).unwrap().grad[0];
        let b_grad = graph.tensors.get(&b_id).unwrap().grad[0];
        
        assert!((a_grad - 3.0).abs() < 1e-6);
        assert!((b_grad - 2.0).abs() < 1e-6);
    }
    
    #[test]
    fn test_mse_loss_backward() {
        let mut graph = ComputeGraph::new("test");
        
        // pred = 0.8, target = 1.0
        let pred = GradientTensor::scalar(0.8, true);
        let target = GradientTensor::scalar(1.0, false);
        
        let pred_id = pred.id;
        let target_id = target.id;
        
        graph.add_tensor(pred);
        graph.add_tensor(target);
        
        // loss = (pred - target)^2 = (0.8 - 1.0)^2 = 0.04
        let pred_ref = graph.tensors.get(&pred_id).cloned().unwrap();
        let target_ref = graph.tensors.get(&target_id).cloned().unwrap();
        let loss = pred_ref.mse_loss(&target_ref);
        let loss_id = loss.id;
        
        graph.add_tensor(loss);
        graph.roots.push(loss_id);
        
        // backward
        graph.backward().unwrap();
        
        // d(MSE)/d(pred) = 2 * (pred - target) / n = 2 * (0.8 - 1.0) / 1 = -0.4
        let pred_grad = graph.tensors.get(&pred_id).unwrap().grad[0];
        
        assert!((pred_grad - (-0.4)).abs() < 1e-6);
    }
}
