// ═══════════════════════════════════════════════════════════════════════════════
// أنواع بيانات ONNX
// ═══════════════════════════════════════════════════════════════════════════════

use std::fmt;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
// ONNX Tensor
// ═══════════════════════════════════════════════════════════════════════════════

/// تمثيل موتر ONNX (Tensor)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ONNXTensor {
    /// البيانات كقائمة مسطحة
    pub data: Vec<f64>,
    /// أبعاد الموتر
    pub shape: Vec<usize>,
    /// نوع البيانات
    pub data_type: ONNXDataType,
    /// اسم الموتر (اختياري)
    pub name: Option<String>,
}

impl ONNXTensor {
    /// إنشاء موتر جديد
    pub fn new(data: Vec<f64>, shape: Vec<usize>) -> Self {
        Self {
            data,
            shape,
            data_type: ONNXDataType::Double,
            name: None,
        }
    }

    /// إنشاء موتر مع اسم
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// إنشاء موتر مع نوع بيانات محدد
    pub fn with_type(mut self, data_type: ONNXDataType) -> Self {
        self.data_type = data_type;
        self
    }

    /// إنشاء موتر من قيمة واحدة (Scalar)
    pub fn scalar(value: f64) -> Self {
        Self {
            data: vec![value],
            shape: vec![],
            data_type: ONNXDataType::Double,
            name: None,
        }
    }

    /// إنشاء موتر من متجه (1D)
    pub fn vector(data: Vec<f64>) -> Self {
        let len = data.len();
        Self {
            data,
            shape: vec![len],
            data_type: ONNXDataType::Double,
            name: None,
        }
    }

    /// إنشاء موتر من مصفوفة (2D)
    pub fn matrix(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        Self {
            data,
            shape: vec![rows, cols],
            data_type: ONNXDataType::Double,
            name: None,
        }
    }

    /// إنشاء موتر صفري
    pub fn zeros(shape: Vec<usize>) -> Self {
        let total: usize = shape.iter().product();
        Self {
            data: vec![0.0; total],
            shape,
            data_type: ONNXDataType::Double,
            name: None,
        }
    }

    /// إنشاء موتر من واحد
    pub fn ones(shape: Vec<usize>) -> Self {
        let total: usize = shape.iter().product();
        Self {
            data: vec![1.0; total],
            shape,
            data_type: ONNXDataType::Double,
            name: None,
        }
    }

    /// إنشاء موتر عشوائي
    pub fn random(shape: Vec<usize>) -> Self {
        let total: usize = shape.iter().product();
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self {
            data: (0..total).map(|_| rng.gen::<f64>()).collect(),
            shape,
            data_type: ONNXDataType::Double,
            name: None,
        }
    }

    /// الحصول على عدد العناصر
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// هل الموتر فارغ؟
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// الحصول على عدد الأبعاد
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// الحصول على الفهرس في البيانات المسطحة
    pub fn flat_index(&self, indices: &[usize]) -> Option<usize> {
        if indices.len() != self.shape.len() {
            return None;
        }

        let mut index = 0;
        let mut multiplier = 1;

        for i in (0..self.shape.len()).rev() {
            if indices[i] >= self.shape[i] {
                return None;
            }
            index += indices[i] * multiplier;
            multiplier *= self.shape[i];
        }

        Some(index)
    }

    /// الحصول على قيمة عند فهرس معين
    pub fn get(&self, indices: &[usize]) -> Option<f64> {
        self.flat_index(indices).map(|i| self.data[i])
    }

    /// تعيين قيمة عند فهرس معين
    pub fn set(&mut self, indices: &[usize], value: f64) -> bool {
        if let Some(i) = self.flat_index(indices) {
            self.data[i] = value;
            true
        } else {
            false
        }
    }

    /// إعادة تشكيل الموتر
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Self, String> {
        let new_size: usize = new_shape.iter().product();
        if new_size != self.data.len() {
            return Err(format!(
                "لا يمكن إعادة التشكيل: {} عنصر ≠ {} عنصر",
                self.data.len(),
                new_size
            ));
        }
        Ok(Self {
            data: self.data.clone(),
            shape: new_shape,
            data_type: self.data_type.clone(),
            name: self.name.clone(),
        })
    }

    /// تحويل إلى قائمة
    pub fn to_vec(&self) -> Vec<f64> {
        self.data.clone()
    }

    /// تحويل إلى مصفوفة 2D
    pub fn to_matrix(&self) -> Option<Vec<Vec<f64>>> {
        if self.shape.len() != 2 {
            return None;
        }
        let rows = self.shape[0];
        let cols = self.shape[1];
        let mut result = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for j in 0..cols {
                row.push(self.data[i * cols + j]);
            }
            result.push(row);
        }
        Some(result)
    }

    /// نسخ الموتر
    pub fn clone_tensor(&self) -> Self {
        self.clone()
    }
}

impl fmt::Display for ONNXTensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ONNXموتر({}) [{} عنصر]",
            self.shape.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("×"),
            self.data.len()
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ONNX Shape
// ═══════════════════════════════════════════════════════════════════════════════

/// شكل موتر ONNX
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ONNXShape {
    /// الأبعاد
    pub dims: Vec<i64>,
    /// هل الشكل ديناميكي؟
    pub is_dynamic: bool,
}

impl ONNXShape {
    /// إنشاء شكل جديد
    pub fn new(dims: Vec<i64>) -> Self {
        let is_dynamic = dims.iter().any(|&d| d < 0);
        Self { dims, is_dynamic }
    }

    /// شكل ثابت
    pub fn static_shape(dims: Vec<usize>) -> Self {
        Self {
            dims: dims.iter().map(|&d| d as i64).collect(),
            is_dynamic: false,
        }
    }

    /// شكل ديناميكي (مع أبعاد غير معروفة)
    pub fn dynamic_shape(dims: Vec<Option<usize>>) -> Self {
        Self {
            dims: dims.iter().map(|d| d.map(|v| v as i64).unwrap_or(-1)).collect(),
            is_dynamic: true,
        }
    }

    /// الحجم الإجمالي (إذا كان ثابتاً)
    pub fn total_size(&self) -> Option<usize> {
        if self.is_dynamic {
            return None;
        }
        Some(self.dims.iter().map(|&d| d as usize).product())
    }

    /// عدد الأبعاد
    pub fn rank(&self) -> usize {
        self.dims.len()
    }
}

impl fmt::Display for ONNXShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.dims
                .iter()
                .map(|&d| if d < 0 { "?".to_string() } else { d.to_string() })
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ONNX Data Type
// ═══════════════════════════════════════════════════════════════════════════════

/// أنواع بيانات ONNX
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ONNXDataType {
    /// Float32 (الأكثر شيوعاً)
    Float,
    /// Double/Float64
    Double,
    /// Int32
    Int32,
    /// Int64
    Int64,
    /// UInt8
    UInt8,
    /// Int8
    Int8,
    /// UInt16
    UInt16,
    /// Int16
    Int16,
    /// Boolean
    Bool,
    /// String
    String,
    /// Float16
    Float16,
    /// BFloat16
    BFloat16,
    /// Complex64
    Complex64,
    /// Complex128
    Complex128,
}

impl ONNXDataType {
    /// الحجم بالبايت
    pub fn size_in_bytes(&self) -> usize {
        match self {
            ONNXDataType::Float => 4,
            ONNXDataType::Double => 8,
            ONNXDataType::Int32 => 4,
            ONNXDataType::Int64 => 8,
            ONNXDataType::UInt8 => 1,
            ONNXDataType::Int8 => 1,
            ONNXDataType::UInt16 => 2,
            ONNXDataType::Int16 => 2,
            ONNXDataType::Bool => 1,
            ONNXDataType::String => 0, // متغير
            ONNXDataType::Float16 => 2,
            ONNXDataType::BFloat16 => 2,
            ONNXDataType::Complex64 => 8,
            ONNXDataType::Complex128 => 16,
        }
    }

    /// الاسم في ONNX
    pub fn onnx_name(&self) -> &'static str {
        match self {
            ONNXDataType::Float => "tensor(float)",
            ONNXDataType::Double => "tensor(double)",
            ONNXDataType::Int32 => "tensor(int32)",
            ONNXDataType::Int64 => "tensor(int64)",
            ONNXDataType::UInt8 => "tensor(uint8)",
            ONNXDataType::Int8 => "tensor(int8)",
            ONNXDataType::UInt16 => "tensor(uint16)",
            ONNXDataType::Int16 => "tensor(int16)",
            ONNXDataType::Bool => "tensor(bool)",
            ONNXDataType::String => "tensor(string)",
            ONNXDataType::Float16 => "tensor(float16)",
            ONNXDataType::BFloat16 => "tensor(bfloat16)",
            ONNXDataType::Complex64 => "tensor(complex64)",
            ONNXDataType::Complex128 => "tensor(complex128)",
        }
    }

    /// الاسم بالعربية
    pub fn arabic_name(&self) -> &'static str {
        match self {
            ONNXDataType::Float => "عائم_32",
            ONNXDataType::Double => "عائم_64",
            ONNXDataType::Int32 => "صحيح_32",
            ONNXDataType::Int64 => "صحيح_64",
            ONNXDataType::UInt8 => "طبيعي_8",
            ONNXDataType::Int8 => "صحيح_8",
            ONNXDataType::UInt16 => "طبيعي_16",
            ONNXDataType::Int16 => "صحيح_16",
            ONNXDataType::Bool => "منطقي",
            ONNXDataType::String => "نص",
            ONNXDataType::Float16 => "عائم_16",
            ONNXDataType::BFloat16 => "عائم_مختصر_16",
            ONNXDataType::Complex64 => "مركب_64",
            ONNXDataType::Complex128 => "مركب_128",
        }
    }
}

impl fmt::Display for ONNXDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.onnx_name())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ONNX Model Info
// ═══════════════════════════════════════════════════════════════════════════════

/// معلومات نموذج ONNX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ONNXModelInfo {
    /// اسم النموذج
    pub name: String,
    /// إصدار النموذج
    pub version: i64,
    /// اسم المنتج
    pub producer: String,
    /// وقت الإنتاج
    pub producer_time: Option<String>,
    /// وصف النموذج
    pub description: Option<String>,
    /// رقم إصدار ONNX
    pub onnx_version: String,
    /// عدد العقد (الطبقات)
    pub node_count: usize,
    /// المدخلات
    pub inputs: Vec<TensorInfo>,
    /// المخرجات
    pub outputs: Vec<TensorInfo>,
}

/// معلومات موتر
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorInfo {
    /// اسم الموتر
    pub name: String,
    /// نوع البيانات
    pub data_type: ONNXDataType,
    /// الشكل
    pub shape: ONNXShape,
}

impl fmt::Display for ONNXModelInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "═══════════════════════════════════════")?;
        writeln!(f, "  معلومات نموذج ONNX")?;
        writeln!(f, "═══════════════════════════════════════")?;
        writeln!(f, "  الاسم: {}", self.name)?;
        writeln!(f, "  الإصدار: {}", self.version)?;
        writeln!(f, "  المنتج: {}", self.producer)?;
        writeln!(f, "  ONNX: {}", self.onnx_version)?;
        writeln!(f, "  العقد: {}", self.node_count)?;
        writeln!(f, "  المدخلات:")?;
        for input in &self.inputs {
            writeln!(f, "    - {} {} {}", input.name, input.data_type, input.shape)?;
        }
        writeln!(f, "  المخرجات:")?;
        for output in &self.outputs {
            writeln!(f, "    - {} {} {}", output.name, output.data_type, output.shape)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let tensor = ONNXTensor::vector(vec![1.0, 2.0, 3.0]);
        assert_eq!(tensor.ndim(), 1);
        assert_eq!(tensor.len(), 3);
    }

    #[test]
    fn test_tensor_matrix() {
        let tensor = ONNXTensor::matrix(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        assert_eq!(tensor.ndim(), 2);
        assert_eq!(tensor.get(&[0, 0]), Some(1.0));
        assert_eq!(tensor.get(&[1, 1]), Some(4.0));
    }

    #[test]
    fn test_tensor_reshape() {
        let tensor = ONNXTensor::vector(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let reshaped = tensor.reshape(vec![2, 3]).unwrap();
        assert_eq!(reshaped.shape, vec![2, 3]);
    }

    #[test]
    fn test_shape() {
        let shape = ONNXShape::static_shape(vec![3, 224, 224]);
        assert_eq!(shape.total_size(), Some(3 * 224 * 224));
        assert!(!shape.is_dynamic);
    }

    #[test]
    fn test_dynamic_shape() {
        let shape = ONNXShape::dynamic_shape(vec![None, Some(3), Some(224), Some(224)]);
        assert!(shape.is_dynamic);
        assert!(shape.total_size().is_none());
    }

    #[test]
    fn test_data_types() {
        assert_eq!(ONNXDataType::Float.size_in_bytes(), 4);
        assert_eq!(ONNXDataType::Double.size_in_bytes(), 8);
        assert_eq!(ONNXDataType::Float.onnx_name(), "tensor(float)");
    }
}
