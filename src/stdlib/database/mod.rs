// src/stdlib/database/mod.rs
// وحدة قواعد البيانات المتقدمة
// Advanced Database Module

pub mod connection;
pub mod query;
pub mod pool;
pub mod mysql;
pub mod postgres;
pub mod sqlite;
pub mod mongodb;

pub use connection::*;
pub use query::*;
pub use pool::*;
pub use mysql::*;
pub use postgres::*;
pub use sqlite::*;
pub use mongodb::*;

use std::collections::HashMap;

/// نوع قاعدة البيانات
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseType {
    MySql,
    Postgres,
    Sqlite,
    MongoDb,
}

impl DatabaseType {
    pub fn from_arabic(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "mysql" | "ماي إس كيو إل" => Some(Self::MySql),
            "postgres" | "postgresql" | "بوستجرس" => Some(Self::Postgres),
            "sqlite" | "إس كيو لايت" => Some(Self::Sqlite),
            "mongodb" | "mongo" | "مونجو" => Some(Self::MongoDb),
            _ => None,
        }
    }
}

/// إعدادات الاتصال
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// المضيف
    pub host: String,
    /// المنفذ
    pub port: u16,
    /// اسم المستخدم
    pub username: String,
    /// كلمة المرور
    pub password: String,
    /// اسم قاعدة البيانات
    pub database: String,
    /// الاتصال الآمن (SSL)
    pub ssl: bool,
    /// حجم مجمع الاتصالات
    pub pool_size: u32,
    /// مهلة الاتصال (ثواني)
    pub timeout: u64,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3306,
            username: "root".to_string(),
            password: String::new(),
            database: "test".to_string(),
            ssl: false,
            pool_size: 10,
            timeout: 30,
        }
    }
}

impl ConnectionConfig {
    /// إنشاء إعدادات جديدة
    pub fn new() -> Self {
        Self::default()
    }
    
    /// تعيين المضيف
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }
    
    /// تعيين المنفذ
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    /// تعيين المستخدم
    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }
    
    /// تعيين كلمة المرور
    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }
    
    /// تعيين اسم قاعدة البيانات
    pub fn database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
    }
    
    /// تفعيل SSL
    pub fn ssl(mut self) -> Self {
        self.ssl = true;
        self
    }
    
    /// تعيين حجم المجمع
    pub fn pool_size(mut self, size: u32) -> Self {
        self.pool_size = size;
        self
    }
    
    /// بناء سلسلة الاتصال
    pub fn to_connection_string(&self, db_type: &DatabaseType) -> String {
        match db_type {
            DatabaseType::MySql => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::Postgres => {
                format!(
                    "postgresql://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::Sqlite => {
                format!("sqlite://{}", self.database)
            }
            DatabaseType::MongoDb => {
                format!(
                    "mongodb://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
        }
    }
}

/// صف النتيجة
#[derive(Debug, Clone, Default)]
pub struct Row {
    pub columns: HashMap<String, Value>,
}

impl Row {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }
    
    pub fn get(&self, column: &str) -> Option<&Value> {
        self.columns.get(column)
    }
    
    pub fn insert(&mut self, column: &str, value: Value) {
        self.columns.insert(column.to_string(), value);
    }
}

/// قيمة في قاعدة البيانات
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Binary(Vec<u8>),
    Json(serde_json::Value),
    Array(Vec<Value>),
}

impl Value {
    /// هل هي null؟
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
    
    /// الحصول على نص
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s),
            _ => None,
        }
    }
    
    /// الحصول على رقم صحيح
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }
    
    /// الحصول على رقم عشري
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
    
    /// الحصول على منطقي
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    /// تحويل إلى نص
    pub fn to_string(&self) -> String {
        match self {
            Value::Null => "NULL".to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Text(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Binary(b) => format!("0x{}", hex::encode(b)),
            Value::Json(j) => j.to_string(),
            Value::Array(a) => format!("{:?}", a),
        }
    }
}

/// نتيجة الاستعلام
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rows: Vec<Row>,
    pub rows_affected: u64,
    pub last_insert_id: Option<u64>,
}

impl QueryResult {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            rows_affected: 0,
            last_insert_id: None,
        }
    }
    
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    
    pub fn first(&self) -> Option<&Row> {
        self.rows.first()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &Row> {
        self.rows.iter()
    }
}

impl Default for QueryResult {
    fn default() -> Self {
        Self::new()
    }
}
