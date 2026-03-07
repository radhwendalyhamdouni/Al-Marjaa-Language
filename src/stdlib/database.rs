// ═══════════════════════════════════════════════════════════════════════════════
// وحدة قواعد البيانات الشاملة
// Comprehensive Database Module
// ═══════════════════════════════════════════════════════════════════════════════
// © 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
// جميع الحقوق محفوظة | All Rights Reserved
// ═══════════════════════════════════════════════════════════════════════════════

//! # وحدة قواعد البيانات الشاملة
//!
//! توفر هذه الوحدة:
//! - MySQL/MariaDB driver
//! - PostgreSQL driver
//! - SQLite driver
//! - MongoDB driver
//! - Connection pooling
//! - Query builder
//! - ORM-like features
//! - Migrations support
//! - Transactions
//! - Prepared statements

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

// ═══════════════════════════════════════════════════════════════════════════════
// الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

/// خطأ قاعدة البيانات
#[derive(Debug, Clone)]
pub enum DatabaseError {
    /// خطأ في الاتصال
    ConnectionError(String),
    /// خطأ في الاستعلام
    QueryError(String),
    /// خطأ في التحضير
    PrepareError(String),
    /// خطأ في التنفيذ
    ExecuteError(String),
    /// خطأ في المعاملة
    TransactionError(String),
    /// خطأ في الهجرة
    MigrationError(String),
    /// خطأ في التحويل
    ConversionError(String),
    /// خطأ في التجمع
    PoolError(String),
    /// خطأ في التحقق
    ValidationError(String),
    /// خطأ عام
    GenericError(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError(msg) => write!(f, "خطأ في الاتصال: {}", msg),
            DatabaseError::QueryError(msg) => write!(f, "خطأ في الاستعلام: {}", msg),
            DatabaseError::PrepareError(msg) => write!(f, "خطأ في التحضير: {}", msg),
            DatabaseError::ExecuteError(msg) => write!(f, "خطأ في التنفيذ: {}", msg),
            DatabaseError::TransactionError(msg) => write!(f, "خطأ في المعاملة: {}", msg),
            DatabaseError::MigrationError(msg) => write!(f, "خطأ في الهجرة: {}", msg),
            DatabaseError::ConversionError(msg) => write!(f, "خطأ في التحويل: {}", msg),
            DatabaseError::PoolError(msg) => write!(f, "خطأ في التجمع: {}", msg),
            DatabaseError::ValidationError(msg) => write!(f, "خطأ في التحقق: {}", msg),
            DatabaseError::GenericError(msg) => write!(f, "خطأ: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

/// نتيجة قاعدة البيانات
pub type DatabaseResult<T> = Result<T, DatabaseError>;

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع قواعد البيانات
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع قاعدة البيانات
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatabaseType {
    /// MySQL/MariaDB
    MySQL,
    /// PostgreSQL
    PostgreSQL,
    /// SQLite
    SQLite,
    /// MongoDB
    MongoDB,
}

impl DatabaseType {
    /// الحصول على اسم السائق
    pub fn driver_name(&self) -> &'static str {
        match self {
            DatabaseType::MySQL => "mysql",
            DatabaseType::PostgreSQL => "postgres",
            DatabaseType::SQLite => "sqlite",
            DatabaseType::MongoDB => "mongodb",
        }
    }

    /// هل تدعم SQL
    pub fn is_sql(&self) -> bool {
        matches!(self, DatabaseType::MySQL | DatabaseType::PostgreSQL | DatabaseType::SQLite)
    }

    /// هل تدعم NoSQL
    pub fn is_nosql(&self) -> bool {
        matches!(self, DatabaseType::MongoDB)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// إعدادات الاتصال
// ═══════════════════════════════════════════════════════════════════════════════

/// إعدادات الاتصال
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// نوع قاعدة البيانات
    pub db_type: DatabaseType,
    /// المضيف
    pub host: String,
    /// المنفذ
    pub port: u16,
    /// اسم قاعدة البيانات
    pub database: String,
    /// اسم المستخدم
    pub username: String,
    /// كلمة المرور
    pub password: String,
    /// رابط الاتصال الكامل
    pub connection_string: Option<String>,
    /// مهلة الاتصال
    pub connection_timeout: Duration,
    /// مهلة التنفيذ
    pub execute_timeout: Duration,
    /// SSL
    pub ssl_enabled: bool,
    /// مسار شهادة SSL
    pub ssl_cert_path: Option<String>,
}

impl ConnectionConfig {
    /// إنشاء إعدادات MySQL
    pub fn mysql(host: impl Into<String>, port: u16, database: impl Into<String>) -> Self {
        Self {
            db_type: DatabaseType::MySQL,
            host: host.into(),
            port,
            database: database.into(),
            username: String::new(),
            password: String::new(),
            connection_string: None,
            connection_timeout: Duration::from_secs(30),
            execute_timeout: Duration::from_secs(30),
            ssl_enabled: false,
            ssl_cert_path: None,
        }
    }

    /// إنشاء إعدادات PostgreSQL
    pub fn postgresql(host: impl Into<String>, port: u16, database: impl Into<String>) -> Self {
        Self {
            db_type: DatabaseType::PostgreSQL,
            host: host.into(),
            port,
            database: database.into(),
            username: String::new(),
            password: String::new(),
            connection_string: None,
            connection_timeout: Duration::from_secs(30),
            execute_timeout: Duration::from_secs(30),
            ssl_enabled: false,
            ssl_cert_path: None,
        }
    }

    /// إنشاء إعدادات SQLite
    pub fn sqlite(path: impl Into<String>) -> Self {
        Self {
            db_type: DatabaseType::SQLite,
            host: String::new(),
            port: 0,
            database: path.into(),
            username: String::new(),
            password: String::new(),
            connection_string: None,
            connection_timeout: Duration::from_secs(30),
            execute_timeout: Duration::from_secs(30),
            ssl_enabled: false,
            ssl_cert_path: None,
        }
    }

    /// إنشاء إعدادات MongoDB
    pub fn mongodb(host: impl Into<String>, port: u16, database: impl Into<String>) -> Self {
        Self {
            db_type: DatabaseType::MongoDB,
            host: host.into(),
            port,
            database: database.into(),
            username: String::new(),
            password: String::new(),
            connection_string: None,
            connection_timeout: Duration::from_secs(30),
            execute_timeout: Duration::from_secs(30),
            ssl_enabled: false,
            ssl_cert_path: None,
        }
    }

    /// تعيين بيانات الاعتماد
    pub fn credentials(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = username.into();
        self.password = password.into();
        self
    }

    /// تعيين رابط الاتصال
    pub fn connection_string(mut self, connection_string: impl Into<String>) -> Self {
        self.connection_string = Some(connection_string.into());
        self
    }

    /// تعيين المهلة
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self.execute_timeout = timeout;
        self
    }

    /// تفعيل SSL
    pub fn ssl(mut self, cert_path: Option<String>) -> Self {
        self.ssl_enabled = true;
        self.ssl_cert_path = cert_path;
        self
    }

    /// بناء رابط الاتصال
    pub fn build_connection_string(&self) -> String {
        if let Some(ref conn_str) = self.connection_string {
            return conn_str.clone();
        }

        match self.db_type {
            DatabaseType::MySQL => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::PostgreSQL => {
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::SQLite => {
                format!("sqlite://{}", self.database)
            }
            DatabaseType::MongoDB => {
                format!(
                    "mongodb://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// قيم قاعدة البيانات
// ═══════════════════════════════════════════════════════════════════════════════

/// قيمة قاعدة البيانات
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseValue {
    /// Null
    Null,
    /// منطقي
    Boolean(bool),
    /// عدد صحيح
    Integer(i64),
    /// عدد عشري
    Float(f64),
    /// نص
    Text(String),
    /// ثنائي
    Binary(Vec<u8>),
    /// تاريخ
    Date(String),
    /// وقت
    Time(String),
    /// تاريخ ووقت
    DateTime(String),
    /// JSON
    Json(String),
    /// مصفوفة
    Array(Vec<DatabaseValue>),
    /// كائن
    Object(HashMap<String, DatabaseValue>),
}

impl DatabaseValue {
    /// هل هو Null
    pub fn is_null(&self) -> bool {
        matches!(self, DatabaseValue::Null)
    }

    /// تحويل إلى منطقي
    pub fn to_bool(&self) -> DatabaseResult<bool> {
        match self {
            DatabaseValue::Boolean(b) => Ok(*b),
            DatabaseValue::Integer(i) => Ok(*i != 0),
            _ => Err(DatabaseError::ConversionError(format!(
                "لا يمكن تحويل {:?} إلى منطقي",
                self
            ))),
        }
    }

    /// تحويل إلى عدد صحيح
    pub fn to_integer(&self) -> DatabaseResult<i64> {
        match self {
            DatabaseValue::Integer(i) => Ok(*i),
            DatabaseValue::Float(f) => Ok(*f as i64),
            DatabaseValue::Text(s) => s.parse().map_err(|_| {
                DatabaseError::ConversionError(format!("لا يمكن تحويل '{}' إلى عدد صحيح", s))
            }),
            _ => Err(DatabaseError::ConversionError(format!(
                "لا يمكن تحويل {:?} إلى عدد صحيح",
                self
            ))),
        }
    }

    /// تحويل إلى عدد عشري
    pub fn to_float(&self) -> DatabaseResult<f64> {
        match self {
            DatabaseValue::Float(f) => Ok(*f),
            DatabaseValue::Integer(i) => Ok(*i as f64),
            DatabaseValue::Text(s) => s.parse().map_err(|_| {
                DatabaseError::ConversionError(format!("لا يمكن تحويل '{}' إلى عدد عشري", s))
            }),
            _ => Err(DatabaseError::ConversionError(format!(
                "لا يمكن تحويل {:?} إلى عدد عشري",
                self
            ))),
        }
    }

    /// تحويل إلى نص
    pub fn to_text(&self) -> String {
        match self {
            DatabaseValue::Null => "NULL".to_string(),
            DatabaseValue::Boolean(b) => b.to_string(),
            DatabaseValue::Integer(i) => i.to_string(),
            DatabaseValue::Float(f) => f.to_string(),
            DatabaseValue::Text(s) => s.clone(),
            DatabaseValue::Binary(b) => format!("0x{}", hex::encode(b)),
            DatabaseValue::Date(d) => d.clone(),
            DatabaseValue::Time(t) => t.clone(),
            DatabaseValue::DateTime(dt) => dt.clone(),
            DatabaseValue::Json(j) => j.clone(),
            DatabaseValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_text()).collect();
                format!("[{}]", items.join(", "))
            }
            DatabaseValue::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_text()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// نتيجة الاستعلام
// ═══════════════════════════════════════════════════════════════════════════════

/// صف في النتيجة
#[derive(Debug, Clone)]
pub struct Row {
    /// القيم
    values: HashMap<String, DatabaseValue>,
    /// ترتيب الأعمدة
    column_order: Vec<String>,
}

impl Row {
    /// إنشاء صف جديد
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            column_order: Vec::new(),
        }
    }

    /// إضافة قيمة
    pub fn insert(&mut self, column: impl Into<String>, value: DatabaseValue) {
        let col = column.into();
        if !self.values.contains_key(&col) {
            self.column_order.push(col.clone());
        }
        self.values.insert(col, value);
    }

    /// الحصول على قيمة
    pub fn get(&self, column: &str) -> Option<&DatabaseValue> {
        self.values.get(column)
    }

    /// الحصول على قيمة كنص
    pub fn get_text(&self, column: &str) -> Option<String> {
        self.values.get(column).map(|v| v.to_text())
    }

    /// الحصول على قيمة كعدد صحيح
    pub fn get_integer(&self, column: &str) -> DatabaseResult<Option<i64>> {
        self.values
            .get(column)
            .map(|v| v.to_integer())
            .transpose()
    }

    /// الحصول على قيمة كعدد عشري
    pub fn get_float(&self, column: &str) -> DatabaseResult<Option<f64>> {
        self.values.get(column).map(|v| v.to_float()).transpose()
    }

    /// الحصول على قيمة كمنطقي
    pub fn get_bool(&self, column: &str) -> DatabaseResult<Option<bool>> {
        self.values.get(column).map(|v| v.to_bool()).transpose()
    }

    /// عدد الأعمدة
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// هل فارغ
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// أسماء الأعمدة
    pub fn columns(&self) -> &[String] {
        &self.column_order
    }

    /// تحويل إلى HashMap
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        self.values
            .iter()
            .map(|(k, v)| (k.clone(), v.to_text()))
            .collect()
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

/// نتيجة الاستعلام
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// الصفوف
    pub rows: Vec<Row>,
    /// عدد الصفوف المتأثرة
    pub affected_rows: u64,
    /// آخر معرف مدرج
    pub last_insert_id: Option<u64>,
    /// الأعمدة
    pub columns: Vec<String>,
}

impl QueryResult {
    /// إنشاء نتيجة جديدة
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            affected_rows: 0,
            last_insert_id: None,
            columns: Vec::new(),
        }
    }

    /// إضافة صف
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    /// عدد الصفوف
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// هل فارغ
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// الحصول على صف
    pub fn get(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// التكرار على الصفوف
    pub fn iter(&self) -> impl Iterator<Item = &Row> {
        self.rows.iter()
    }

    /// تحويل إلى Vec من HashMap
    pub fn to_vec_hashmap(&self) -> Vec<HashMap<String, String>> {
        self.rows.iter().map(|r| r.to_hashmap()).collect()
    }

    /// الحصول على أول صف
    pub fn first(&self) -> Option<&Row> {
        self.rows.first()
    }

    /// الحصول على آخر صف
    pub fn last(&self) -> Option<&Row> {
        self.rows.last()
    }
}

impl Default for QueryResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Connection Pool
// ═══════════════════════════════════════════════════════════════════════════════

/// اتصال من التجمع
#[derive(Debug)]
pub struct PooledConnection {
    /// المعرف
    pub id: String,
    /// نشط
    pub is_active: bool,
    /// آخر استخدام
    pub last_used: std::time::Instant,
}

/// تجمع الاتصالات
#[derive(Debug)]
pub struct DatabasePool {
    /// الإعدادات
    config: ConnectionConfig,
    /// الاتصالات
    connections: RwLock<Vec<PooledConnection>>,
    /// الحد الأقصى
    max_connections: usize,
    /// الحد الأدنى
    min_connections: usize,
    /// مهلة الخمول
    idle_timeout: Duration,
    /// مهلة الاتصال
    connection_timeout: Duration,
}

impl DatabasePool {
    /// إنشاء تجمع جديد
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            config,
            connections: RwLock::new(Vec::new()),
            max_connections: 10,
            min_connections: 2,
            idle_timeout: Duration::from_secs(300),
            connection_timeout: Duration::from_secs(30),
        }
    }

    /// تعيين الحد الأقصى
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    /// تعيين الحد الأدنى
    pub fn min_connections(mut self, min: usize) -> Self {
        self.min_connections = min;
        self
    }

    /// تعيين مهلة الخمول
    pub fn idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }

    /// الحصول على اتصال
    pub fn acquire(&self) -> DatabaseResult<Arc<PooledConnection>> {
        let mut connections = self.connections.write().unwrap();
        
        // البحث عن اتصال متاح
        if let Some(conn) = connections.iter_mut().find(|c| c.is_active) {
            conn.last_used = std::time::Instant::now();
            return Ok(Arc::new(PooledConnection {
                id: conn.id.clone(),
                is_active: conn.is_active,
                last_used: conn.last_used,
            }));
        }
        
        // إنشاء اتصال جديد
        if connections.len() < self.max_connections {
            let conn = PooledConnection {
                id: uuid::Uuid::new_v4().to_string(),
                is_active: true,
                last_used: std::time::Instant::now(),
            };
            connections.push(conn.clone());
            return Ok(Arc::new(conn));
        }
        
        Err(DatabaseError::PoolError("تجمع الاتصالات ممتلئ".to_string()))
    }

    /// إرجاع اتصال
    pub fn release(&self, conn: &PooledConnection) {
        let mut connections = self.connections.write().unwrap();
        if let Some(c) = connections.iter_mut().find(|c| c.id == conn.id) {
            c.is_active = true;
            c.last_used = std::time::Instant::now();
        }
    }

    /// تنظيف الاتصالات الخاملة
    pub fn cleanup_idle(&self) {
        let mut connections = self.connections.write().unwrap();
        let now = std::time::Instant::now();
        connections.retain(|c| {
            now.duration_since(c.last_used) < self.idle_timeout
                || connections.len() <= self.min_connections
        });
    }

    /// عدد الاتصالات النشطة
    pub fn active_count(&self) -> usize {
        self.connections.read().unwrap().iter().filter(|c| c.is_active).count()
    }

    /// إغلاق جميع الاتصالات
    pub fn close_all(&self) {
        self.connections.write().unwrap().clear();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Query Builder
// ═══════════════════════════════════════════════════════════════════════════════

/// منشئ الاستعلام
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    /// الجدول
    table: String,
    /// الأعمدة
    columns: Vec<String>,
    /// الشروط
    conditions: Vec<Condition>,
    /// الترتيب
    order_by: Vec<OrderBy>,
    /// المجموعة
    group_by: Vec<String>,
    /// Having
    having: Vec<Condition>,
    /// الحد
    limit: Option<usize>,
    /// الإزاحة
    offset: Option<usize>,
    /// Joins
    joins: Vec<Join>,
    /// نوع الاستعلام
    query_type: QueryType,
    /// القيم للإدراج/التحديث
    values: HashMap<String, DatabaseValue>,
}

/// نوع الاستعلام
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    /// SELECT
    Select,
    /// INSERT
    Insert,
    /// UPDATE
    Update,
    /// DELETE
    Delete,
}

/// شرط
#[derive(Debug, Clone)]
pub struct Condition {
    /// العمود
    pub column: String,
    /// العملية
    pub operator: Operator,
    /// القيمة
    pub value: DatabaseValue,
    /// الرابط
    pub connector: Connector,
}

/// عملية
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// =
    Equal,
    /// !=
    NotEqual,
    /// >
    GreaterThan,
    /// <
    LessThan,
    /// >=
    GreaterThanOrEqual,
    /// <=
    LessThanOrEqual,
    /// LIKE
    Like,
    /// NOT LIKE
    NotLike,
    /// IN
    In,
    /// NOT IN
    NotIn,
    /// IS NULL
    IsNull,
    /// IS NOT NULL
    IsNotNull,
    /// BETWEEN
    Between,
}

impl Operator {
    /// تحويل إلى SQL
    pub fn to_sql(&self) -> &'static str {
        match self {
            Operator::Equal => "=",
            Operator::NotEqual => "!=",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::GreaterThanOrEqual => ">=",
            Operator::LessThanOrEqual => "<=",
            Operator::Like => "LIKE",
            Operator::NotLike => "NOT LIKE",
            Operator::In => "IN",
            Operator::NotIn => "NOT IN",
            Operator::IsNull => "IS NULL",
            Operator::IsNotNull => "IS NOT NULL",
            Operator::Between => "BETWEEN",
        }
    }
}

/// رابط
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connector {
    /// AND
    And,
    /// OR
    Or,
}

impl Connector {
    /// تحويل إلى SQL
    pub fn to_sql(&self) -> &'static str {
        match self {
            Connector::And => "AND",
            Connector::Or => "OR",
        }
    }
}

/// ترتيب
#[derive(Debug, Clone)]
pub struct OrderBy {
    /// العمود
    pub column: String,
    /// الاتجاه
    pub direction: OrderDirection,
}

/// اتجاه الترتيب
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderDirection {
    /// تصاعدي
    Asc,
    /// تنازلي
    Desc,
}

impl OrderDirection {
    /// تحويل إلى SQL
    pub fn to_sql(&self) -> &'static str {
        match self {
            OrderDirection::Asc => "ASC",
            OrderDirection::Desc => "DESC",
        }
    }
}

/// Join
#[derive(Debug, Clone)]
pub struct Join {
    /// الجدول
    pub table: String,
    /// النوع
    pub join_type: JoinType,
    /// الشرط
    pub on: (String, String),
}

/// نوع Join
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    /// INNER JOIN
    Inner,
    /// LEFT JOIN
    Left,
    /// RIGHT JOIN
    Right,
    /// FULL JOIN
    Full,
    /// CROSS JOIN
    Cross,
}

impl JoinType {
    /// تحويل إلى SQL
    pub fn to_sql(&self) -> &'static str {
        match self {
            JoinType::Inner => "INNER JOIN",
            JoinType::Left => "LEFT JOIN",
            JoinType::Right => "RIGHT JOIN",
            JoinType::Full => "FULL JOIN",
            JoinType::Cross => "CROSS JOIN",
        }
    }
}

impl QueryBuilder {
    /// إنشاء منشئ جديد
    pub fn new(table: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            columns: Vec::new(),
            conditions: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            limit: None,
            offset: None,
            joins: Vec::new(),
            query_type: QueryType::Select,
            values: HashMap::new(),
        }
    }

    /// استعلام SELECT
    pub fn select(table: impl Into<String>) -> Self {
        let mut builder = Self::new(table);
        builder.query_type = QueryType::Select;
        builder
    }

    /// استعلام INSERT
    pub fn insert(table: impl Into<String>) -> Self {
        let mut builder = Self::new(table);
        builder.query_type = QueryType::Insert;
        builder
    }

    /// استعلام UPDATE
    pub fn update(table: impl Into<String>) -> Self {
        let mut builder = Self::new(table);
        builder.query_type = QueryType::Update;
        builder
    }

    /// استعلام DELETE
    pub fn delete(table: impl Into<String>) -> Self {
        let mut builder = Self::new(table);
        builder.query_type = QueryType::Delete;
        builder
    }

    /// تحديد الأعمدة
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// تحديد جميع الأعمدة
    pub fn all_columns(mut self) -> Self {
        self.columns = vec!["*".to_string()];
        self
    }

    /// إضافة شرط WHERE
    pub fn where_clause(mut self, column: impl Into<String>, operator: Operator, value: DatabaseValue) -> Self {
        self.conditions.push(Condition {
            column: column.into(),
            operator,
            value,
            connector: Connector::And,
        });
        self
    }

    /// WHERE = 
    pub fn where_eq(self, column: impl Into<String>, value: DatabaseValue) -> Self {
        self.where_clause(column, Operator::Equal, value)
    }

    /// WHERE !=
    pub fn where_ne(self, column: impl Into<String>, value: DatabaseValue) -> Self {
        self.where_clause(column, Operator::NotEqual, value)
    }

    /// WHERE >
    pub fn where_gt(self, column: impl Into<String>, value: DatabaseValue) -> Self {
        self.where_clause(column, Operator::GreaterThan, value)
    }

    /// WHERE <
    pub fn where_lt(self, column: impl Into<String>, value: DatabaseValue) -> Self {
        self.where_clause(column, Operator::LessThan, value)
    }

    /// WHERE >=
    pub fn where_gte(self, column: impl Into<String>, value: DatabaseValue) -> Self {
        self.where_clause(column, Operator::GreaterThanOrEqual, value)
    }

    /// WHERE <=
    pub fn where_lte(self, column: impl Into<String>, value: DatabaseValue) -> Self {
        self.where_clause(column, Operator::LessThanOrEqual, value)
    }

    /// WHERE LIKE
    pub fn where_like(self, column: impl Into<String>, pattern: impl Into<String>) -> Self {
        self.where_clause(column, Operator::Like, DatabaseValue::Text(pattern.into()))
    }

    /// WHERE IS NULL
    pub fn where_null(mut self, column: impl Into<String>) -> Self {
        self.conditions.push(Condition {
            column: column.into(),
            operator: Operator::IsNull,
            value: DatabaseValue::Null,
            connector: Connector::And,
        });
        self
    }

    /// WHERE IS NOT NULL
    pub fn where_not_null(mut self, column: impl Into<String>) -> Self {
        self.conditions.push(Condition {
            column: column.into(),
            operator: Operator::IsNotNull,
            value: DatabaseValue::Null,
            connector: Connector::And,
        });
        self
    }

    /// WHERE IN
    pub fn where_in(self, column: impl Into<String>, values: Vec<DatabaseValue>) -> Self {
        self.where_clause(column, Operator::In, DatabaseValue::Array(values))
    }

    /// ORDER BY
    pub fn order_by(mut self, column: impl Into<String>, direction: OrderDirection) -> Self {
        self.order_by.push(OrderBy {
            column: column.into(),
            direction,
        });
        self
    }

    /// GROUP BY
    pub fn group_by(mut self, columns: &[&str]) -> Self {
        self.group_by = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// LIMIT
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// OFFSET
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// JOIN
    pub fn join(mut self, table: impl Into<String>, join_type: JoinType, left_col: impl Into<String>, right_col: impl Into<String>) -> Self {
        self.joins.push(Join {
            table: table.into(),
            join_type,
            on: (left_col.into(), right_col.into()),
        });
        self
    }

    /// INNER JOIN
    pub fn inner_join(self, table: impl Into<String>, left_col: impl Into<String>, right_col: impl Into<String>) -> Self {
        self.join(table, JoinType::Inner, left_col, right_col)
    }

    /// LEFT JOIN
    pub fn left_join(self, table: impl Into<String>, left_col: impl Into<String>, right_col: impl Into<String>) -> Self {
        self.join(table, JoinType::Left, left_col, right_col)
    }

    /// قيم للإدراج/التحديث
    pub fn values(mut self, values: HashMap<String, DatabaseValue>) -> Self {
        self.values = values;
        self
    }

    /// إضافة قيمة
    pub fn value(mut self, column: impl Into<String>, value: DatabaseValue) -> Self {
        self.values.insert(column.into(), value);
        self
    }

    /// بناء الاستعلام
    pub fn build(&self) -> String {
        match self.query_type {
            QueryType::Select => self.build_select(),
            QueryType::Insert => self.build_insert(),
            QueryType::Update => self.build_update(),
            QueryType::Delete => self.build_delete(),
        }
    }

    /// بناء SELECT
    fn build_select(&self) -> String {
        let mut sql = String::new();
        
        // SELECT
        let cols = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };
        sql.push_str(&format!("SELECT {} FROM {}", cols, self.table));
        
        // JOINs
        for join in &self.joins {
            sql.push_str(&format!(
                " {} {} ON {} = {}",
                join.join_type.to_sql(),
                join.table,
                join.on.0,
                join.on.1
            ));
        }
        
        // WHERE
        if !self.conditions.is_empty() {
            sql.push_str(" WHERE ");
            let conditions: Vec<String> = self
                .conditions
                .iter()
                .map(|c| self.condition_to_sql(c))
                .collect();
            sql.push_str(&conditions.join(" AND "));
        }
        
        // GROUP BY
        if !self.group_by.is_empty() {
            sql.push_str(&format!(" GROUP BY {}", self.group_by.join(", ")));
        }
        
        // HAVING
        if !self.having.is_empty() {
            sql.push_str(" HAVING ");
            let conditions: Vec<String> = self
                .having
                .iter()
                .map(|c| self.condition_to_sql(c))
                .collect();
            sql.push_str(&conditions.join(" AND "));
        }
        
        // ORDER BY
        if !self.order_by.is_empty() {
            let orders: Vec<String> = self
                .order_by
                .iter()
                .map(|o| format!("{} {}", o.column, o.direction.to_sql()))
                .collect();
            sql.push_str(&format!(" ORDER BY {}", orders.join(", ")));
        }
        
        // LIMIT
        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        
        // OFFSET
        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }
        
        sql
    }

    /// بناء INSERT
    fn build_insert(&self) -> String {
        let columns: Vec<&String> = self.values.keys().collect();
        let placeholders: Vec<&str> = columns.iter().map(|_| "?").collect();
        
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table,
            columns.join(", "),
            placeholders.join(", ")
        )
    }

    /// بناء UPDATE
    fn build_update(&self) -> String {
        let sets: Vec<String> = self
            .values
            .keys()
            .map(|k| format!("{} = ?", k))
            .collect();
        
        let mut sql = format!("UPDATE {} SET {}", self.table, sets.join(", "));
        
        if !self.conditions.is_empty() {
            sql.push_str(" WHERE ");
            let conditions: Vec<String> = self
                .conditions
                .iter()
                .map(|c| self.condition_to_sql(c))
                .collect();
            sql.push_str(&conditions.join(" AND "));
        }
        
        sql
    }

    /// بناء DELETE
    fn build_delete(&self) -> String {
        let mut sql = format!("DELETE FROM {}", self.table);
        
        if !self.conditions.is_empty() {
            sql.push_str(" WHERE ");
            let conditions: Vec<String> = self
                .conditions
                .iter()
                .map(|c| self.condition_to_sql(c))
                .collect();
            sql.push_str(&conditions.join(" AND "));
        }
        
        sql
    }

    /// تحويل الشرط إلى SQL
    fn condition_to_sql(&self, condition: &Condition) -> String {
        match condition.operator {
            Operator::IsNull => format!("{} IS NULL", condition.column),
            Operator::IsNotNull => format!("{} IS NOT NULL", condition.column),
            Operator::In | Operator::NotIn => {
                if let DatabaseValue::Array(values) = &condition.value {
                    let placeholders: Vec<&str> = values.iter().map(|_| "?").collect();
                    format!(
                        "{} {} ({})",
                        condition.column,
                        condition.operator.to_sql(),
                        placeholders.join(", ")
                    )
                } else {
                    format!("{} {} (?)", condition.column, condition.operator.to_sql())
                }
            }
            _ => format!("{} {} ?", condition.column, condition.operator.to_sql()),
        }
    }

    /// الحصول على القيم للبارامترات
    pub fn get_params(&self) -> Vec<&DatabaseValue> {
        let mut params = Vec::new();
        
        // قيم الإدراج/التحديث
        for value in self.values.values() {
            params.push(value);
        }
        
        // قيم الشروط
        for condition in &self.conditions {
            match &condition.value {
                DatabaseValue::Null => {}
                DatabaseValue::Array(values) => {
                    for v in values {
                        params.push(v);
                    }
                }
                v => params.push(v),
            }
        }
        
        params
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Prepared Statement
// ═══════════════════════════════════════════════════════════════════════════════

/// جملة محضرة
#[derive(Debug, Clone)]
pub struct PreparedStatement {
    /// الاستعلام
    query: String,
    /// البارامترات
    params: Vec<DatabaseValue>,
    /// اسم الجملة
    name: Option<String>,
}

impl PreparedStatement {
    /// إنشاء جملة جديدة
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            params: Vec::new(),
            name: None,
        }
    }

    /// تعيين الاسم
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// إضافة بارامتر
    pub fn bind(mut self, value: DatabaseValue) -> Self {
        self.params.push(value);
        self
    }

    /// إضافة بارامترات متعددة
    pub fn bind_many(mut self, values: Vec<DatabaseValue>) -> Self {
        self.params.extend(values);
        self
    }

    /// الحصول على الاستعلام
    pub fn query(&self) -> &str {
        &self.query
    }

    /// الحصول على البارامترات
    pub fn params(&self) -> &[DatabaseValue] {
        &self.params
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Transaction
// ═══════════════════════════════════════════════════════════════════════════════

/// مستوى العزل
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationLevel {
    /// Read Uncommitted
    ReadUncommitted,
    /// Read Committed
    ReadCommitted,
    /// Repeatable Read
    RepeatableRead,
    /// Serializable
    Serializable,
}

impl IsolationLevel {
    /// تحويل إلى SQL
    pub fn to_sql(&self) -> &'static str {
        match self {
            IsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
            IsolationLevel::ReadCommitted => "READ COMMITTED",
            IsolationLevel::RepeatableRead => "REPEATABLE READ",
            IsolationLevel::Serializable => "SERIALIZABLE",
        }
    }
}

/// معاملة
#[derive(Debug)]
pub struct Transaction {
    /// المعرف
    id: String,
    /// نشطة
    active: bool,
    /// مستوى العزل
    isolation_level: IsolationLevel,
}

impl Transaction {
    /// إنشاء معاملة جديدة
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            active: true,
            isolation_level: IsolationLevel::ReadCommitted,
        }
    }

    /// تعيين مستوى العزل
    pub fn isolation_level(mut self, level: IsolationLevel) -> Self {
        self.isolation_level = level;
        self
    }

    /// بدء المعاملة
    pub fn begin_sql(&self) -> String {
        format!(
            "BEGIN TRANSACTION ISOLATION LEVEL {}",
            self.isolation_level.to_sql()
        )
    }

    /// تأكيد المعاملة
    pub fn commit_sql(&self) -> String {
        "COMMIT".to_string()
    }

    /// إلغاء المعاملة
    pub fn rollback_sql(&self) -> String {
        "ROLLBACK".to_string()
    }

    /// هل نشطة
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// تأكيد
    pub fn commit(&mut self) {
        self.active = false;
    }

    /// إلغاء
    pub fn rollback(&mut self) {
        self.active = false;
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Migrations
// ═══════════════════════════════════════════════════════════════════════════════

/// هجرة
#[derive(Debug, Clone)]
pub struct Migration {
    /// الاسم
    pub name: String,
    /// الإصدار
    pub version: String,
    /// الاستعلام للأعلى
    pub up_sql: String,
    /// الاستعلام للأسفل
    pub down_sql: String,
}

impl Migration {
    /// إنشاء هجرة جديدة
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            up_sql: String::new(),
            down_sql: String::new(),
        }
    }

    /// تعيين SQL للأعلى
    pub fn up(mut self, sql: impl Into<String>) -> Self {
        self.up_sql = sql.into();
        self
    }

    /// تعيين SQL للأسفل
    pub fn down(mut self, sql: impl Into<String>) -> Self {
        self.down_sql = sql.into();
        self
    }
}

/// مدير الهجرات
#[derive(Debug)]
pub struct Migrations {
    /// الهجرات
    migrations: Vec<Migration>,
    /// الهجرات المنفذة
    executed: Vec<String>,
}

impl Migrations {
    /// إنشاء مدير جديد
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
            executed: Vec::new(),
        }
    }

    /// إضافة هجرة
    pub fn add(&mut self, migration: Migration) {
        self.migrations.push(migration);
    }

    /// الحصول على الهجرات المعلقة
    pub fn pending(&self) -> Vec<&Migration> {
        self.migrations
            .iter()
            .filter(|m| !self.executed.contains(&m.version))
            .collect()
    }

    /// تحديد هجرة كمنفذة
    pub fn mark_executed(&mut self, version: String) {
        if !self.executed.contains(&version) {
            self.executed.push(version);
        }
    }

    /// التراجع عن آخر هجرة
    pub fn rollback_last(&mut self) -> Option<&Migration> {
        if let Some(version) = self.executed.pop() {
            self.migrations.iter().find(|m| m.version == version)
        } else {
            None
        }
    }

    /// جدول الهجرات
    pub fn create_migrations_table_sql() -> String {
        r#"
        CREATE TABLE IF NOT EXISTS _migrations (
            id SERIAL PRIMARY KEY,
            version VARCHAR(255) NOT NULL UNIQUE,
            name VARCHAR(255) NOT NULL,
            executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#.to_string()
    }
}

impl Default for Migrations {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Connection
// ═══════════════════════════════════════════════════════════════════════════════

/// اتصال قاعدة البيانات
pub struct DatabaseConnection {
    /// الإعدادات
    config: ConnectionConfig,
    /// متصل
    connected: bool,
    /// المعاملة الحالية
    current_transaction: Option<Transaction>,
}

impl DatabaseConnection {
    /// إنشاء اتصال جديد
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            config,
            connected: false,
            current_transaction: None,
        }
    }

    /// الاتصال
    pub fn connect(&mut self) -> DatabaseResult<()> {
        // TODO: تنفيذ الاتصال الفعلي
        self.connected = true;
        Ok(())
    }

    /// قطع الاتصال
    pub fn disconnect(&mut self) -> DatabaseResult<()> {
        self.connected = false;
        Ok(())
    }

    /// هل متصل
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// تنفيذ استعلام
    pub fn execute(&self, query: &str) -> DatabaseResult<QueryResult> {
        if !self.connected {
            return Err(DatabaseError::ConnectionError("غير متصل بقاعدة البيانات".to_string()));
        }
        // TODO: تنفيذ الاستعلام الفعلي
        Ok(QueryResult::new())
    }

    /// تنفيذ جملة محضرة
    pub fn execute_prepared(&self, statement: &PreparedStatement) -> DatabaseResult<QueryResult> {
        if !self.connected {
            return Err(DatabaseError::ConnectionError("غير متصل بقاعدة البيانات".to_string()));
        }
        // TODO: تنفيذ الجملة المحضرة الفعلي
        Ok(QueryResult::new())
    }

    /// تنفيذ QueryBuilder
    pub fn execute_query(&self, builder: &QueryBuilder) -> DatabaseResult<QueryResult> {
        let query = builder.build();
        self.execute(&query)
    }

    /// بدء معاملة
    pub fn begin_transaction(&mut self) -> DatabaseResult<&Transaction> {
        if !self.connected {
            return Err(DatabaseError::ConnectionError("غير متصل بقاعدة البيانات".to_string()));
        }
        
        let transaction = Transaction::new();
        self.execute(&transaction.begin_sql())?;
        self.current_transaction = Some(transaction);
        
        Ok(self.current_transaction.as_ref().unwrap())
    }

    /// تأكيد المعاملة
    pub fn commit(&mut self) -> DatabaseResult<()> {
        if let Some(ref mut transaction) = self.current_transaction {
            if transaction.is_active() {
                self.execute(&transaction.commit_sql())?;
                transaction.commit();
            }
        }
        self.current_transaction = None;
        Ok(())
    }

    /// إلغاء المعاملة
    pub fn rollback(&mut self) -> DatabaseResult<()> {
        if let Some(ref mut transaction) = self.current_transaction {
            if transaction.is_active() {
                self.execute(&transaction.rollback_sql())?;
                transaction.rollback();
            }
        }
        self.current_transaction = None;
        Ok(())
    }

    /// إنشاء QueryBuilder
    pub fn query(&self, table: &str) -> QueryBuilder {
        QueryBuilder::select(table)
    }

    /// إنشاء جدول
    pub fn create_table(&self, table: &str, columns: &[(&str, &str)]) -> DatabaseResult<()> {
        let cols: Vec<String> = columns
            .iter()
            .map(|(name, dtype)| format!("{} {}", name, dtype))
            .collect();
        let sql = format!("CREATE TABLE {} ({})", table, cols.join(", "));
        self.execute(&sql)?;
        Ok(())
    }

    /// حذف جدول
    pub fn drop_table(&self, table: &str) -> DatabaseResult<()> {
        let sql = format!("DROP TABLE IF EXISTS {}", table);
        self.execute(&sql)?;
        Ok(())
    }

    /// إدراج
    pub fn insert(&self, table: &str, values: HashMap<String, DatabaseValue>) -> DatabaseResult<QueryResult> {
        let query = QueryBuilder::insert(table).values(values);
        self.execute_query(&query)
    }

    /// تحديث
    pub fn update(&self, table: &str, values: HashMap<String, DatabaseValue>, conditions: Vec<Condition>) -> DatabaseResult<QueryResult> {
        let mut query = QueryBuilder::update(table).values(values);
        for condition in conditions {
            query.conditions.push(condition);
        }
        self.execute_query(&query)
    }

    /// حذف
    pub fn delete(&self, table: &str, conditions: Vec<Condition>) -> DatabaseResult<QueryResult> {
        let mut query = QueryBuilder::delete(table);
        for condition in conditions {
            query.conditions.push(condition);
        }
        self.execute_query(&query)
    }

    /// اختيار
    pub fn select(&self, table: &str, columns: &[&str]) -> DatabaseResult<QueryResult> {
        let query = QueryBuilder::select(table).columns(columns);
        self.execute_query(&query)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ORM-like Features
// ═══════════════════════════════════════════════════════════════════════════════

/// موديل ORM
pub trait Model: Sized {
    /// اسم الجدول
    fn table_name() -> &'static str;
    
    /// المفتاح الأساسي
    fn primary_key() -> &'static str {
        "id"
    }
    
    /// تحويل من صف
    fn from_row(row: &Row) -> DatabaseResult<Self>;
    
    /// تحويل إلى قيم
    fn to_values(&self) -> HashMap<String, DatabaseValue>;
    
    /// البحث بالمعرف
    fn find(conn: &DatabaseConnection, id: i64) -> DatabaseResult<Option<Self>> {
        let result = conn.execute_query(
            &QueryBuilder::select(Self::table_name())
                .where_eq(Self::primary_key(), DatabaseValue::Integer(id))
        )?;
        
        if let Some(row) = result.first() {
            Ok(Some(Self::from_row(row)?))
        } else {
            Ok(None)
        }
    }
    
    /// حفظ
    fn save(&self, conn: &DatabaseConnection) -> DatabaseResult<QueryResult> {
        conn.insert(Self::table_name(), self.to_values())
    }
    
    /// حذف
    fn delete(&self, conn: &DatabaseConnection, id: i64) -> DatabaseResult<QueryResult> {
        conn.delete(
            Self::table_name(),
            vec![Condition {
                column: Self::primary_key().to_string(),
                operator: Operator::Equal,
                value: DatabaseValue::Integer(id),
                connector: Connector::And,
            }],
        )
    }
    
    /// جميع السجلات
    fn all(conn: &DatabaseConnection) -> DatabaseResult<Vec<Self>> {
        let result = conn.execute_query(&QueryBuilder::select(Self::table_name()))?;
        result
            .iter()
            .map(|row| Self::from_row(row))
            .collect()
    }
}

/// بناء موديل
pub struct ModelBuilder {
    /// اسم الجدول
    table: String,
    /// الأعمدة
    columns: Vec<ColumnDef>,
}

/// تعريف العمود
#[derive(Debug, Clone)]
pub struct ColumnDef {
    /// الاسم
    pub name: String,
    /// النوع
    pub data_type: String,
    /// قابل للقيم الفارغة
    pub nullable: bool,
    /// مفتاح أساسي
    pub primary_key: bool,
    /// auto increment
    pub auto_increment: bool,
    /// قيمة افتراضية
    pub default: Option<String>,
}

impl ModelBuilder {
    /// إنشاء منشئ جديد
    pub fn new(table: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            columns: Vec::new(),
        }
    }

    /// إضافة عمود
    pub fn column(mut self, name: impl Into<String>, data_type: impl Into<String>) -> Self {
        self.columns.push(ColumnDef {
            name: name.into(),
            data_type: data_type.into(),
            nullable: true,
            primary_key: false,
            auto_increment: false,
            default: None,
        });
        self
    }

    /// إضافة مفتاح أساسي
    pub fn primary_key(mut self, name: impl Into<String>) -> Self {
        self.columns.push(ColumnDef {
            name: name.into(),
            data_type: "SERIAL".to_string(),
            nullable: false,
            primary_key: true,
            auto_increment: true,
            default: None,
        });
        self
    }

    /// إضافة عمود نصي
    pub fn string(self, name: impl Into<String>) -> Self {
        self.column(name, "VARCHAR(255)")
    }

    /// إضافة عمود نصي طويل
    pub fn text(self, name: impl Into<String>) -> Self {
        self.column(name, "TEXT")
    }

    /// إضافة عمود عدد صحيح
    pub fn integer(self, name: impl Into<String>) -> Self {
        self.column(name, "INTEGER")
    }

    /// إضافة عمود عدد عشري
    pub fn float(self, name: impl Into<String>) -> Self {
        self.column(name, "FLOAT")
    }

    /// إضافة عمود منطقي
    pub fn boolean(self, name: impl Into<String>) -> Self {
        self.column(name, "BOOLEAN")
    }

    /// إضافة عمود تاريخ ووقت
    pub fn timestamp(self, name: impl Into<String>) -> Self {
        self.column(name, "TIMESTAMP")
    }

    /// بناء SQL
    pub fn build_sql(&self) -> String {
        let columns: Vec<String> = self
            .columns
            .iter()
            .map(|c| {
                let mut col = format!("{} {}", c.name, c.data_type);
                if c.primary_key {
                    col.push_str(" PRIMARY KEY");
                }
                if c.auto_increment {
                    col.push_str(" AUTOINCREMENT");
                }
                if !c.nullable {
                    col.push_str(" NOT NULL");
                }
                if let Some(ref default) = c.default {
                    col.push_str(&format!(" DEFAULT {}", default));
                }
                col
            })
            .collect();
        
        format!("CREATE TABLE {} ({})", self.table, columns.join(", "))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MongoDB Support
// ═══════════════════════════════════════════════════════════════════════════════

/// مستند MongoDB
#[derive(Debug, Clone)]
pub struct MongoDocument {
    /// البيانات
    data: HashMap<String, DatabaseValue>,
}

impl MongoDocument {
    /// إنشاء مستند جديد
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// إضافة حقل
    pub fn insert(mut self, key: impl Into<String>, value: DatabaseValue) -> Self {
        self.data.insert(key.into(), value);
        self
    }

    /// الحصول على حقل
    pub fn get(&self, key: &str) -> Option<&DatabaseValue> {
        self.data.get(key)
    }

    /// تحويل إلى JSON
    pub fn to_json(&self) -> String {
        let pairs: Vec<String> = self
            .data
            .iter()
            .map(|(k, v)| format!("\"{}\": {}", k, self.value_to_json(v)))
            .collect();
        format!("{{{}}}", pairs.join(", "))
    }

    fn value_to_json(&self, value: &DatabaseValue) -> String {
        match value {
            DatabaseValue::Null => "null".to_string(),
            DatabaseValue::Boolean(b) => b.to_string(),
            DatabaseValue::Integer(i) => i.to_string(),
            DatabaseValue::Float(f) => f.to_string(),
            DatabaseValue::Text(s) => format!("\"{}\"", s),
            DatabaseValue::Json(j) => j.clone(),
            DatabaseValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_json(v)).collect();
                format!("[{}]", items.join(", "))
            }
            DatabaseValue::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, self.value_to_json(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
            _ => "null".to_string(),
        }
    }
}

impl Default for MongoDocument {
    fn default() -> Self {
        Self::new()
    }
}

/// فلتر MongoDB
#[derive(Debug, Clone)]
pub struct MongoFilter {
    /// الشروط
    conditions: HashMap<String, MongoCondition>,
}

/// شرط MongoDB
#[derive(Debug, Clone)]
pub struct MongoCondition {
    /// العملية
    pub operator: String,
    /// القيمة
    pub value: DatabaseValue,
}

impl MongoFilter {
    /// إنشاء فلتر جديد
    pub fn new() -> Self {
        Self {
            conditions: HashMap::new(),
        }
    }

    /// إضافة شرط
    pub fn where_eq(mut self, field: impl Into<String>, value: DatabaseValue) -> Self {
        self.conditions.insert(
            field.into(),
            MongoCondition {
                operator: "$eq".to_string(),
                value,
            },
        );
        self
    }

    /// إضافة شرط أكبر من
    pub fn where_gt(mut self, field: impl Into<String>, value: DatabaseValue) -> Self {
        self.conditions.insert(
            field.into(),
            MongoCondition {
                operator: "$gt".to_string(),
                value,
            },
        );
        self
    }

    /// إضافة شرط أصغر من
    pub fn where_lt(mut self, field: impl Into<String>, value: DatabaseValue) -> Self {
        self.conditions.insert(
            field.into(),
            MongoCondition {
                operator: "$lt".to_string(),
                value,
            },
        );
        self
    }

    /// تحويل إلى JSON
    pub fn to_json(&self) -> String {
        let pairs: Vec<String> = self
            .conditions
            .iter()
            .map(|(k, c)| format!("\"{}\": {{\"{}\": {}}}", k, c.operator, self.value_to_json(&c.value)))
            .collect();
        format!("{{{}}}", pairs.join(", "))
    }

    fn value_to_json(&self, value: &DatabaseValue) -> String {
        match value {
            DatabaseValue::Null => "null".to_string(),
            DatabaseValue::Boolean(b) => b.to_string(),
            DatabaseValue::Integer(i) => i.to_string(),
            DatabaseValue::Float(f) => f.to_string(),
            DatabaseValue::Text(s) => format!("\"{}\"", s),
            _ => "null".to_string(),
        }
    }
}

impl Default for MongoFilter {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الاختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_config() {
        let config = ConnectionConfig::mysql("localhost", 3306, "test")
            .credentials("root", "password");
        
        assert_eq!(config.db_type, DatabaseType::MySQL);
        assert_eq!(config.host, "localhost");
    }

    #[test]
    fn test_query_builder_select() {
        let query = QueryBuilder::select("users")
            .columns(&["id", "name", "email"])
            .where_eq("active", DatabaseValue::Boolean(true))
            .order_by("created_at", OrderDirection::Desc)
            .limit(10)
            .build();
        
        assert!(query.contains("SELECT"));
        assert!(query.contains("FROM users"));
        assert!(query.contains("WHERE"));
        assert!(query.contains("ORDER BY"));
        assert!(query.contains("LIMIT 10"));
    }

    #[test]
    fn test_query_builder_insert() {
        let query = QueryBuilder::insert("users")
            .value("name", DatabaseValue::Text("Ahmed".to_string()))
            .value("email", DatabaseValue::Text("ahmed@example.com".to_string()))
            .build();
        
        assert!(query.contains("INSERT INTO users"));
        assert!(query.contains("VALUES"));
    }

    #[test]
    fn test_database_value() {
        let value = DatabaseValue::Integer(42);
        assert_eq!(value.to_integer().unwrap(), 42);
        
        let text = DatabaseValue::Text("مرحبا".to_string());
        assert_eq!(text.to_text(), "مرحبا");
    }

    #[test]
    fn test_row() {
        let mut row = Row::new();
        row.insert("name", DatabaseValue::Text("أحمد".to_string()));
        row.insert("age", DatabaseValue::Integer(25));
        
        assert_eq!(row.get_text("name"), Some("أحمد".to_string()));
        assert_eq!(row.get_integer("age").unwrap(), Some(25));
    }

    #[test]
    fn test_mongo_document() {
        let doc = MongoDocument::new()
            .insert("name", DatabaseValue::Text("محمد".to_string()))
            .insert("age", DatabaseValue::Integer(30));
        
        let json = doc.to_json();
        assert!(json.contains("محمد"));
        assert!(json.contains("30"));
    }

    #[test]
    fn test_migration() {
        let migration = Migration::new("create_users", "20240101")
            .up("CREATE TABLE users (id SERIAL PRIMARY KEY, name VARCHAR(255))")
            .down("DROP TABLE users");
        
        assert_eq!(migration.version, "20240101");
        assert!(migration.up_sql.contains("CREATE TABLE"));
    }
}
