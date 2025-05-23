use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use super::ConnectionId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub struct QueryId(pub Uuid);

impl QueryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for QueryId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for QueryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryStatus {
    Draft,
    Executing,
    Completed {
        execution_time_ms: u64,
        rows_affected: usize,
    },
    Failed {
        error: String,
        execution_time_ms: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    id: QueryId,
    connection_id: ConnectionId,
    sql: String,
    status: QueryStatus,
    created_at: chrono::DateTime<chrono::Utc>,
    executed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Query {
    pub fn new(connection_id: ConnectionId, sql: String) -> Result<Self, String> {
        if sql.trim().is_empty() {
            return Err("SQL query cannot be empty".to_string());
        }

        // Basic SQL validation
        let sql_trimmed = sql.trim().to_lowercase();
        if !sql_trimmed.starts_with("select")
            && !sql_trimmed.starts_with("insert")
            && !sql_trimmed.starts_with("update")
            && !sql_trimmed.starts_with("delete")
            && !sql_trimmed.starts_with("create")
            && !sql_trimmed.starts_with("alter")
            && !sql_trimmed.starts_with("drop")
        {
            return Err("Invalid SQL statement".to_string());
        }

        Ok(Self {
            id: QueryId::new(),
            connection_id,
            sql,
            status: QueryStatus::Draft,
            created_at: chrono::Utc::now(),
            executed_at: None,
        })
    }

    // Getters
    pub fn id(&self) -> &QueryId {
        &self.id
    }

    pub fn connection_id(&self) -> &ConnectionId {
        &self.connection_id
    }

    pub fn sql(&self) -> &str {
        &self.sql
    }

    pub fn status(&self) -> &QueryStatus {
        &self.status
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    pub fn executed_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.executed_at
    }

    // State mutations
    pub fn mark_as_executing(&mut self) {
        self.status = QueryStatus::Executing;
        self.executed_at = Some(chrono::Utc::now());
    }

    pub fn mark_as_completed(&mut self, execution_time_ms: u64, rows_affected: usize) {
        self.status = QueryStatus::Completed {
            execution_time_ms,
            rows_affected,
        };
    }

    pub fn mark_as_failed(&mut self, error: String, execution_time_ms: u64) {
        self.status = QueryStatus::Failed {
            error,
            execution_time_ms,
        };
    }

    pub fn update_sql(&mut self, sql: String) -> Result<(), String> {
        if sql.trim().is_empty() {
            return Err("SQL query cannot be empty".to_string());
        }

        self.sql = sql;
        self.status = QueryStatus::Draft;
        Ok(())
    }
}
