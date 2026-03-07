// src/stdlib/database/pool.rs
// مجمع الاتصالات
// Connection Pool

use super::{DatabaseConnection, DatabaseType, ConnectionConfig, QueryResult};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// مجمع الاتصالات
pub struct ConnectionPool {
    connections: Arc<Mutex<VecDeque<DatabaseConnection>>>,
    config: ConnectionConfig,
    db_type: DatabaseType,
    max_size: usize,
}

impl ConnectionPool {
    /// إنشاء مجمع جديد
    pub fn new(db_type: DatabaseType, config: ConnectionConfig, max_size: usize) -> Self {
        Self {
            connections: Arc::new(Mutex::new(VecDeque::new())),
            config,
            db_type,
            max_size,
        }
    }
    
    /// تهيئة المجمع
    pub fn initialize(&self) -> Result<(), String> {
        let mut connections = self.connections.lock().unwrap();
        
        for _ in 0..self.max_size {
            let mut conn = DatabaseConnection::new(self.db_type.clone(), self.config.clone());
            conn.connect()?;
            connections.push_back(conn);
        }
        
        Ok(())
    }
    
    /// الحصول على اتصال
    pub fn get(&self) -> Result<PooledConnection, String> {
        let mut connections = self.connections.lock().unwrap();
        
        if let Some(mut conn) = connections.pop_front() {
            if !conn.is_connected() {
                conn.connect()?;
            }
            return Ok(PooledConnection {
                connection: Some(conn),
                pool: self.connections.clone(),
            });
        }
        
        Err("لا توجد اتصالات متاحة".to_string())
    }
    
    /// عدد الاتصالات المتاحة
    pub fn available(&self) -> usize {
        self.connections.lock().unwrap().len()
    }
    
    /// إغلاق جميع الاتصالات
    pub fn close_all(&self) {
        let mut connections = self.connections.lock().unwrap();
        while let Some(mut conn) = connections.pop_front() {
            conn.disconnect();
        }
    }
}

/// اتصال من المجمع
pub struct PooledConnection {
    connection: Option<DatabaseConnection>,
    pool: Arc<Mutex<VecDeque<DatabaseConnection>>>,
}

impl PooledConnection {
    /// تنفيذ استعلام
    pub fn execute(&self, query: &str) -> Result<QueryResult, String> {
        self.connection.as_ref().unwrap().execute(query)
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(conn) = self.connection.take() {
            self.pool.lock().unwrap().push_back(conn);
        }
    }
}
