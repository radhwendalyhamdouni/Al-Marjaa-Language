// src/stdlib/database/query.rs
// منشئ الاستعلامات
// Query Builder

use super::Value;
use std::collections::HashMap;

/// منشئ الاستعلامات
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    table: String,
    select_columns: Vec<String>,
    where_clauses: Vec<String>,
    where_params: Vec<Value>,
    join_clauses: Vec<String>,
    order_by: Vec<String>,
    group_by: Vec<String>,
    having_clauses: Vec<String>,
    limit_value: Option<usize>,
    offset_value: Option<usize>,
    insert_values: HashMap<String, Value>,
    update_values: HashMap<String, Value>,
    query_type: QueryType,
}

#[derive(Debug, Clone, PartialEq)]
enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}

impl QueryBuilder {
    /// إنشاء منشئ جديد
    pub fn new() -> Self {
        Self {
            table: String::new(),
            select_columns: Vec::new(),
            where_clauses: Vec::new(),
            where_params: Vec::new(),
            join_clauses: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            having_clauses: Vec::new(),
            limit_value: None,
            offset_value: None,
            insert_values: HashMap::new(),
            update_values: HashMap::new(),
            query_type: QueryType::Select,
        }
    }
    
    /// تحديد الجدول
    pub fn table(mut self, table: &str) -> Self {
        self.table = table.to_string();
        self
    }
    
    /// استعلام SELECT
    pub fn select(mut self, columns: &[&str]) -> Self {
        self.query_type = QueryType::Select;
        self.select_columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }
    
    /// استعلام SELECT *
    pub fn select_all(mut self) -> Self {
        self.query_type = QueryType::Select;
        self.select_columns = vec!["*".to_string()];
        self
    }
    
    /// إضافة شرط WHERE
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_clauses.push(condition.to_string());
        self
    }
    
    /// WHERE مع معامل
    pub fn where_eq(mut self, column: &str, value: Value) -> Self {
        self.where_clauses.push(format!("{} = ?", column));
        self.where_params.push(value);
        self
    }
    
    /// WHERE مع LIKE
    pub fn where_like(mut self, column: &str, pattern: &str) -> Self {
        self.where_clauses.push(format!("{} LIKE ?", column));
        self.where_params.push(Value::Text(pattern.to_string()));
        self
    }
    
    /// WHERE مع IN
    pub fn where_in(mut self, column: &str, values: Vec<Value>) -> Self {
        let placeholders: Vec<String> = values.iter().map(|_| "?".to_string()).collect();
        self.where_clauses.push(format!("{} IN ({})", column, placeholders.join(", ")));
        self.where_params.extend(values);
        self
    }
    
    /// WHERE مع BETWEEN
    pub fn where_between(mut self, column: &str, start: Value, end: Value) -> Self {
        self.where_clauses.push(format!("{} BETWEEN ? AND ?", column));
        self.where_params.push(start);
        self.where_params.push(end);
        self
    }
    
    /// WHERE مع NULL
    pub fn where_null(mut self, column: &str) -> Self {
        self.where_clauses.push(format!("{} IS NULL", column));
        self
    }
    
    /// WHERE مع NOT NULL
    pub fn where_not_null(mut self, column: &str) -> Self {
        self.where_clauses.push(format!("{} IS NOT NULL", column));
        self
    }
    
    /// إضافة JOIN
    pub fn join(mut self, table: &str, on: &str) -> Self {
        self.join_clauses.push(format!("JOIN {} ON {}", table, on));
        self
    }
    
    /// إضافة LEFT JOIN
    pub fn left_join(mut self, table: &str, on: &str) -> Self {
        self.join_clauses.push(format!("LEFT JOIN {} ON {}", table, on));
        self
    }
    
    /// ترتيب تصاعدي
    pub fn order_by_asc(mut self, column: &str) -> Self {
        self.order_by.push(format!("{} ASC", column));
        self
    }
    
    /// ترتيب تنازلي
    pub fn order_by_desc(mut self, column: &str) -> Self {
        self.order_by.push(format!("{} DESC", column));
        self
    }
    
    /// تجميع
    pub fn group_by(mut self, column: &str) -> Self {
        self.group_by.push(column.to_string());
        self
    }
    
    /// HAVING
    pub fn having(mut self, condition: &str) -> Self {
        self.having_clauses.push(condition.to_string());
        self
    }
    
    /// LIMIT
    pub fn limit(mut self, count: usize) -> Self {
        self.limit_value = Some(count);
        self
    }
    
    /// OFFSET
    pub fn offset(mut self, count: usize) -> Self {
        self.offset_value = Some(count);
        self
    }
    
    /// استعلام INSERT
    pub fn insert(mut self, values: HashMap<String, Value>) -> Self {
        self.query_type = QueryType::Insert;
        self.insert_values = values;
        self
    }
    
    /// استعلام UPDATE
    pub fn update(mut self, values: HashMap<String, Value>) -> Self {
        self.query_type = QueryType::Update;
        self.update_values = values;
        self
    }
    
    /// استعلام DELETE
    pub fn delete(mut self) -> Self {
        self.query_type = QueryType::Delete;
        self
    }
    
    /// بناء الاستعلام
    pub fn build(&self) -> (String, Vec<Value>) {
        let query = match self.query_type {
            QueryType::Select => self.build_select(),
            QueryType::Insert => self.build_insert(),
            QueryType::Update => self.build_update(),
            QueryType::Delete => self.build_delete(),
        };
        
        (query, self.where_params.clone())
    }
    
    fn build_select(&self) -> String {
        let mut sql = String::new();
        
        // SELECT
        sql.push_str("SELECT ");
        sql.push_str(&self.select_columns.join(", "));
        
        // FROM
        sql.push_str(" FROM ");
        sql.push_str(&self.table);
        
        // JOIN
        for join in &self.join_clauses {
            sql.push(' ');
            sql.push_str(join);
        }
        
        // WHERE
        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }
        
        // GROUP BY
        if !self.group_by.is_empty() {
            sql.push_str(" GROUP BY ");
            sql.push_str(&self.group_by.join(", "));
        }
        
        // HAVING
        if !self.having_clauses.is_empty() {
            sql.push_str(" HAVING ");
            sql.push_str(&self.having_clauses.join(" AND "));
        }
        
        // ORDER BY
        if !self.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            sql.push_str(&self.order_by.join(", "));
        }
        
        // LIMIT
        if let Some(limit) = self.limit_value {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        
        // OFFSET
        if let Some(offset) = self.offset_value {
            sql.push_str(&format!(" OFFSET {}", offset));
        }
        
        sql
    }
    
    fn build_insert(&self) -> String {
        let columns: Vec<&String> = self.insert_values.keys().collect();
        let placeholders: Vec<&str> = columns.iter().map(|_| "?").collect();
        
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table,
            columns.join(", "),
            placeholders.join(", ")
        )
    }
    
    fn build_update(&self) -> String {
        let set_clauses: Vec<String> = self.update_values
            .keys()
            .map(|k| format!("{} = ?", k))
            .collect();
        
        let mut sql = format!("UPDATE {} SET {}", self.table, set_clauses.join(", "));
        
        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }
        
        sql
    }
    
    fn build_delete(&self) -> String {
        let mut sql = format!("DELETE FROM {}", self.table);
        
        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }
        
        sql
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ===== دوال عربية =====

/// إنشاء منشئ استعلامات جديد
pub fn استعلام() -> QueryBuilder {
    QueryBuilder::new()
}

/// SELECT
pub fn اختر(columns: &[&str]) -> QueryBuilder {
    QueryBuilder::new().select(columns)
}

/// INSERT
pub fn ادخل(table: &str, values: HashMap<String, Value>) -> QueryBuilder {
    QueryBuilder::new().table(table).insert(values)
}

/// UPDATE
pub fn عدل(table: &str, values: HashMap<String, Value>) -> QueryBuilder {
    QueryBuilder::new().table(table).update(values)
}

/// DELETE
pub fn احذف(table: &str) -> QueryBuilder {
    QueryBuilder::new().table(table).delete()
}
