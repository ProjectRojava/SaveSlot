//! Query result set model
//!
//! This module defines the data structures for representing query results,
//! including columns, rows, and metadata.

use std::time::Duration;
use uuid::Uuid;

use super::QueryId;

/// Unique identifier for a result set
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResultSetId(Uuid);

impl ResultSetId {
    /// Create a new random result set ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Get the underlying UUID
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ResultSetId {
    fn default() -> Self {
        Self::new()
    }
}

/// Query result set representation
#[derive(Debug, Clone)]
pub struct ResultSet {
    id: ResultSetId,
    query_id: QueryId,
    columns: Vec<ColumnInfo>,
    rows: Vec<Row>,
    execution_time: Duration,
    affected_rows: Option<usize>,
    metadata: ResultMetadata,
}

impl ResultSet {
    /// Create a new result set
    pub fn new(
        query_id: QueryId,
        columns: Vec<ColumnInfo>,
        rows: Vec<Row>,
        execution_time: Duration,
        affected_rows: Option<usize>,
    ) -> Self {
        Self {
            id: ResultSetId::new(),
            query_id,
            columns,
            rows,
            execution_time,
            affected_rows,
            metadata: ResultMetadata::default(),
        }
    }

    /// Get the result set ID
    pub fn id(&self) -> ResultSetId {
        self.id
    }

    /// Get the query ID
    pub fn query_id(&self) -> QueryId {
        self.query_id
    }

    /// Get the column information
    pub fn columns(&self) -> &[ColumnInfo] {
        &self.columns
    }

    /// Get the result rows
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    /// Get the execution time
    pub fn execution_time(&self) -> Duration {
        self.execution_time
    }

    /// Get the number of affected rows (for non-SELECT queries)
    pub fn affected_rows(&self) -> Option<usize> {
        self.affected_rows
    }

    /// Get the result metadata
    pub fn metadata(&self) -> &ResultMetadata {
        &self.metadata
    }

    /// Get mutable access to the result metadata
    pub fn metadata_mut(&mut self) -> &mut ResultMetadata {
        &mut self.metadata
    }

    /// Get the number of rows in the result set
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get the number of columns in the result set
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Get a specific cell value
    pub fn get_value(&self, row_index: usize, column_index: usize) -> Option<&Value> {
        self.rows.get(row_index)?.values.get(column_index)
    }

    /// Get a specific cell value by column name
    pub fn get_value_by_name(&self, row_index: usize, column_name: &str) -> Option<&Value> {
        let column_index = self.columns.iter().position(|c| c.name == column_name)?;
        self.get_value(row_index, column_index)
    }
}

/// Column information
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    name: String,
    data_type: String,
    table: Option<String>,
    nullable: bool,
}

impl ColumnInfo {
    /// Create a new column info
    pub fn new(name: String, data_type: String, table: Option<String>, nullable: bool) -> Self {
        Self {
            name,
            data_type,
            table,
            nullable,
        }
    }

    /// Get the column name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the column data type
    pub fn data_type(&self) -> &str {
        &self.data_type
    }

    /// Get the table name
    pub fn table(&self) -> Option<&str> {
        self.table.as_deref()
    }

    /// Check if the column is nullable
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }
}

/// Result row
#[derive(Debug, Clone)]
pub struct Row {
    values: Vec<Value>,
}

impl Row {
    /// Create a new row
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }

    /// Get all values in the row
    pub fn values(&self) -> &[Value] {
        &self.values
    }

    /// Get a specific value by index
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }
}

/// Cell value
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Date(chrono::NaiveDate),
    Time(chrono::NaiveTime),
    Timestamp(chrono::NaiveDateTime),
    Binary(Vec<u8>),
    Array(Vec<Value>),
    Json(serde_json::Value),
}

impl Value {
    /// Check if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Date(d) => write!(f, "{}", d),
            Value::Time(t) => write!(f, "{}", t),
            Value::Timestamp(ts) => write!(f, "{}", ts),
            Value::Binary(b) => write!(f, "<BINARY: {} bytes>", b.len()),
            Value::Array(a) => {
                let items: Vec<String> = a.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Json(j) => write!(f, "{}", j),
        }
    }
}

/// Result metadata
#[derive(Debug, Clone, Default)]
pub struct ResultMetadata {
    warnings: Vec<String>,
    notices: Vec<String>,
    is_truncated: bool,
    max_rows: Option<usize>,
}

impl ResultMetadata {
    /// Create new result metadata
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the warnings
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Get the notices
    pub fn notices(&self) -> &[String] {
        &self.notices
    }

    /// Add a notice
    pub fn add_notice(&mut self, notice: String) {
        self.notices.push(notice);
    }

    /// Check if the result set is truncated
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }

    /// Set whether the result set is truncated
    pub fn set_truncated(&mut self, truncated: bool) {
        self.is_truncated = truncated;
    }

    /// Get the maximum number of rows
    pub fn max_rows(&self) -> Option<usize> {
        self.max_rows
    }

    /// Set the maximum number of rows
    pub fn set_max_rows(&mut self, max_rows: Option<usize>) {
        self.max_rows = max_rows;
    }
}

/// Result set statistics
#[derive(Debug, Clone)]
pub struct ResultStatistics {
    execution_time: Duration,
    affected_rows: Option<usize>,
    row_count: usize,
    column_count: usize,
    memory_usage: Option<usize>,
}

impl ResultStatistics {
    /// Create new result statistics
    pub fn new(
        execution_time: Duration,
        affected_rows: Option<usize>,
        row_count: usize,
        column_count: usize,
        memory_usage: Option<usize>,
    ) -> Self {
        Self {
            execution_time,
            affected_rows,
            row_count,
            column_count,
            memory_usage,
        }
    }

    /// Get the execution time
    pub fn execution_time(&self) -> Duration {
        self.execution_time
    }

    /// Get the number of affected rows
    pub fn affected_rows(&self) -> Option<usize> {
        self.affected_rows
    }

    /// Get the row count
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    /// Get the column count
    pub fn column_count(&self) -> usize {
        self.column_count
    }

    /// Get the memory usage
    pub fn memory_usage(&self) -> Option<usize> {
        self.memory_usage
    }

    /// Format the execution time as a human-readable string
    pub fn format_execution_time(&self) -> String {
        let nanos = self.execution_time.as_nanos();

        if nanos < 1_000 {
            format!("{} ns", nanos)
        } else if nanos < 1_000_000 {
            format!("{:.2} μs", nanos as f64 / 1_000.0)
        } else if nanos < 1_000_000_000 {
            format!("{:.2} ms", nanos as f64 / 1_000_000.0)
        } else {
            format!("{:.2} s", nanos as f64 / 1_000_000_000.0)
        }
    }
}
