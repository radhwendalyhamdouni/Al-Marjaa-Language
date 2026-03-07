// src/stdlib/database/sqlite.rs
// SQLite Driver

use super::{ConnectionConfig, DatabaseConnection, DatabaseType, QueryResult, Row, Value};
use std::collections::HashMap;

/// اتصال SQLite
pub struct SqliteConnection {
    connection: DatabaseConnection,
    database_path: String,
    tables: HashMap<String, Vec<(String, String)>>, // جدول -> [(عمود, نوع)]
    data: HashMap<String, Vec<Row>>, // جدول -> صفوف
}

impl SqliteConnection {
    /// إنشاء اتصال جديد
    pub fn new(path: &str) -> Self {
        let config = ConnectionConfig {
            database: path.to_string(),
            ..Default::default()
        };
        
        Self {
            connection: DatabaseConnection::new(DatabaseType::Sqlite, config),
            database_path: path.to_string(),
            tables: HashMap::new(),
            data: HashMap::new(),
        }
    }
    
    /// الاتصال
    pub fn connect(&mut self) -> Result<(), String> {
        self.connection.connect()
    }
    
    /// إنشاء جدول
    pub fn create_table(&mut self, name: &str, columns: &[(&str, &str)]) -> Result<(), String> {
        let cols: Vec<(String, String)> = columns
            .iter()
            .map(|(name, typ)| (name.to_string(), typ.to_string()))
            .collect();
        
        self.tables.insert(name.to_string(), cols);
        self.data.insert(name.to_string(), Vec::new());
        
        Ok(())
    }
    
    /// تنفيذ استعلام
    pub fn execute(&self, query: &str) -> Result<QueryResult, String> {
        self.connection.execute(query)
    }
    
    /// إدراج صف
    pub fn insert(&mut self, table: &str, row: Row) -> Result<(), String> {
        if let Some(rows) = self.data.get_mut(table) {
            rows.push(row);
            Ok(())
        } else {
            Err(format!("الجدول {} غير موجود", table))
        }
    }
    
    /// البحث
    pub fn select(&self, table: &str) -> Result<Vec<&Row>, String> {
        if let Some(rows) = self.data.get(table) {
            Ok(rows.iter().collect())
        } else {
            Err(format!("الجدول {} غير موجود", table))
        }
    }
}

// ===== دوال عربية =====

/// اتصال SQLite جديد
pub fn اتصال_sqlite(path: &str) -> SqliteConnection {
    SqliteConnection::new(path)
}
