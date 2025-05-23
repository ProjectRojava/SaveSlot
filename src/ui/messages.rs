use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::model::{ConnectionId, DatabaseConnection, QueryId};

#[derive(Debug, Clone)]
pub enum Message {
    // Application lifecycle
    Initialize,
    
    // Tab navigation
    TabChanged(usize),
    
    // Sidebar navigation
    SidebarToggled,
    NavigateToConnections,
    NavigateToQueryEditor,
    NavigateToResults,
    NavigateToSchema,
    NavigateToHistory,
    
    // Connection management
    NewConnectionRequested,
    ConnectionFormUpdated(ConnectionFormData),
    ConnectionRequested(ConnectionId),
    ConnectionSelected(ConnectionId),
    ConnectionEditRequested(ConnectionId),
    ConnectionDeleteRequested(ConnectionId),
    ConnectionEstablished(ConnectionId),
    ConnectionFailed(ConnectionId, String),
    DisconnectRequested(ConnectionId),
    
    // Query operations
    QueryTextChanged(String),
    QueryExecuteRequested,
    QueryExecuted(String), // Result message
    QuerySaveRequested,
    QueryLoadRequested,
    QueryCleared,
    QueryFormatRequested,
    QueryExplainRequested,
    QueryHistoryRequested,
    QueryFromHistorySelected(QueryId),
    
    // Results
    ResultsCleared,
    ResultsExported,
    
    // Schema exploration
    SchemaRefreshRequested,
    SchemaTableSelected(String),
    SchemaColumnSelected(String, String),
    
    // Error handling
    ErrorOccurred(String),
    ErrorDismissed,
}

#[derive(Debug, Clone, Default)]
pub struct ConnectionFormData {
    pub name: String,
    pub host: String,
    pub port: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub connection_type: String,
}

#[derive(Debug, Clone, Default)]
pub struct QueryEditorState {
    pub current_query: String,
    pub is_executing: bool,
    pub last_execution_time: Option<std::time::Duration>,
}

#[derive(Debug, Clone)]
pub struct QueryHistoryItem {
    pub id: QueryId,
    pub query: String,
    pub executed_at: DateTime<Utc>,
    pub success: bool,
    pub execution_time: Option<std::time::Duration>,
}

#[derive(Debug, Clone)]
pub struct ResultSet {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub row_count: usize,
    pub execution_time: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct SchemaInfo {
    pub tables: Vec<TableInfo>,
    pub views: Vec<ViewInfo>,
}

#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
    pub row_count: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ViewInfo {
    pub name: String,
    pub definition: String,
}

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub foreign_key: Option<String>,
}