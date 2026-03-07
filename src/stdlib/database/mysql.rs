// src/stdlib/database/mysql.rs
// MySQL Driver

use super::{ConnectionConfig, DatabaseConnection, DatabaseType, QueryResult};

/// اتصال MySQL
pub struct MySqlConnection {
    connection: DatabaseConnection,
}

impl MySqlConnection {
    /// إنشاء اتصال جديد
    pub fn new(config: ConnectionConfig) -> Self {
        let mut config = config;
        config.port = config.port; // المنفذ الافتراضي 3306
        
        Self {
            connection: DatabaseConnection::new(DatabaseType::MySql, config),
        }
    }
    
    /// الاتصال
    pub fn connect(&mut self) -> Result<(), String> {
        self.connection.connect()
    }
    
    /// تنفيذ استعلام
    pub fn execute(&self, query: &str) -> Result<QueryResult, String> {
        self.connection.execute(query)
    }
}

// ===== دوال عربية =====

/// اتصال MySQL جديد
pub fn اتصال_mysql(config: ConnectionConfig) -> MySqlConnection {
    MySqlConnection::new(config)
}
