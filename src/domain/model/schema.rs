//! Database schema model
//!
//! This module defines the data structures for representing database schemas,
//! including tables, columns, relationships, and constraints.

use std::collections::HashMap;
use uuid::Uuid;

use super::ConnectionId;

/// Unique identifier for a database schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaId(Uuid);

impl SchemaId {
    /// Create a new random schema ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Get the underlying UUID
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for SchemaId {
    fn default() -> Self {
        Self::new()
    }
}

/// Database schema representation
#[derive(Debug, Clone)]
pub struct DatabaseSchema {
    id: SchemaId,
    connection_id: ConnectionId,
    name: String,
    tables: Vec<Table>,
    relationships: Vec<Relationship>,
}

impl DatabaseSchema {
    /// Create a new database schema
    pub fn new(connection_id: ConnectionId, name: String) -> Self {
        Self {
            id: SchemaId::new(),
            connection_id,
            name,
            tables: Vec::new(),
            relationships: Vec::new(),
        }
    }

    /// Get the schema ID
    pub fn id(&self) -> SchemaId {
        self.id
    }

    /// Get the connection ID
    pub fn connection_id(&self) -> ConnectionId {
        self.connection_id.clone()
    }

    /// Get the schema name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all tables in the schema
    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    /// Get a specific table by name
    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|t| t.name == name)
    }

    /// Add a table to the schema
    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    /// Get all relationships in the schema
    pub fn relationships(&self) -> &[Relationship] {
        &self.relationships
    }

    /// Add a relationship to the schema
    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
    }
}

/// Database table representation
#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    primary_key: Option<Vec<String>>,
    indexes: Vec<Index>,
    constraints: Vec<Constraint>,
}

impl Table {
    /// Create a new table
    pub fn new(name: String) -> Self {
        Self {
            name,
            columns: Vec::new(),
            primary_key: None,
            indexes: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Get the table name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all columns in the table
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// Get a specific column by name
    pub fn get_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.name == name)
    }

    /// Add a column to the table
    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    /// Get the primary key columns
    pub fn primary_key(&self) -> Option<&[String]> {
        self.primary_key.as_deref()
    }

    /// Set the primary key columns
    pub fn set_primary_key(&mut self, columns: Vec<String>) {
        self.primary_key = Some(columns);
    }

    /// Get all indexes on the table
    pub fn indexes(&self) -> &[Index] {
        &self.indexes
    }

    /// Add an index to the table
    pub fn add_index(&mut self, index: Index) {
        self.indexes.push(index);
    }

    /// Get all constraints on the table
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    /// Add a constraint to the table
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }
}

/// Database column representation
#[derive(Debug, Clone)]
pub struct Column {
    name: String,
    data_type: DataType,
    nullable: bool,
    default_value: Option<String>,
    description: Option<String>,
}

impl Column {
    /// Create a new column
    pub fn new(name: String, data_type: DataType, nullable: bool) -> Self {
        Self {
            name,
            data_type,
            nullable,
            default_value: None,
            description: None,
        }
    }

    /// Get the column name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the column data type
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    /// Check if the column is nullable
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    /// Get the default value
    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }

    /// Set the default value
    pub fn set_default_value(&mut self, value: Option<String>) {
        self.default_value = value;
    }

    /// Get the column description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the column description
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }
}

/// Database data types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Integer,
    BigInt,
    SmallInt,
    Float,
    Double,
    Decimal { precision: u8, scale: u8 },
    Char { length: u16 },
    Varchar { length: u16 },
    Text,
    Date,
    Time,
    Timestamp,
    Boolean,
    Binary { length: u16 },
    Blob,
    Json,
    Uuid,
    Array { element_type: Box<DataType> },
    Custom { name: String, params: Vec<String> },
}

impl DataType {}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Integer => write!(f, "INTEGER"),
            DataType::BigInt => write!(f, "BIGINT"),
            DataType::SmallInt => write!(f, "SMALLINT"),
            DataType::Float => write!(f, "FLOAT"),
            DataType::Double => write!(f, "DOUBLE"),
            DataType::Decimal { precision, scale } => {
                write!(f, "DECIMAL({}, {})", precision, scale)
            }
            DataType::Char { length } => write!(f, "CHAR({})", length),
            DataType::Varchar { length } => write!(f, "VARCHAR({})", length),
            DataType::Text => write!(f, "TEXT"),
            DataType::Date => write!(f, "DATE"),
            DataType::Time => write!(f, "TIME"),
            DataType::Timestamp => write!(f, "TIMESTAMP"),
            DataType::Boolean => write!(f, "BOOLEAN"),
            DataType::Binary { length } => write!(f, "BINARY({})", length),
            DataType::Blob => write!(f, "BLOB"),
            DataType::Json => write!(f, "JSON"),
            DataType::Uuid => write!(f, "UUID"),
            DataType::Array { element_type } => {
                write!(f, "ARRAY<{}>", element_type)
            }
            DataType::Custom { name, params } => {
                if params.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(
                        f,
                        "{}({})",
                        name,
                        params
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
        }
    }
}

/// Database index representation
#[derive(Debug, Clone)]
pub struct Index {
    name: String,
    columns: Vec<String>,
    unique: bool,
}

impl Index {
    /// Create a new index
    pub fn new(name: String, columns: Vec<String>, unique: bool) -> Self {
        Self {
            name,
            columns,
            unique,
        }
    }

    /// Get the index name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the indexed columns
    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    /// Check if the index is unique
    pub fn is_unique(&self) -> bool {
        self.unique
    }
}

/// Database constraint types
#[derive(Debug, Clone)]
pub enum Constraint {
    ForeignKey {
        name: String,
        columns: Vec<String>,
        referenced_table: String,
        referenced_columns: Vec<String>,
        on_delete: ReferentialAction,
        on_update: ReferentialAction,
    },
    Unique {
        name: String,
        columns: Vec<String>,
    },
    Check {
        name: String,
        expression: String,
    },
}

impl Constraint {
    /// Get the constraint name
    pub fn name(&self) -> &str {
        match self {
            Constraint::ForeignKey { name, .. } => name,
            Constraint::Unique { name, .. } => name,
            Constraint::Check { name, .. } => name,
        }
    }
}

/// Referential actions for foreign keys
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferentialAction {
    Restrict,
    Cascade,
    SetNull,
    SetDefault,
    NoAction,
}

impl ReferentialAction {
    /// Get a string representation of the action
    pub fn to_string(&self) -> &'static str {
        match self {
            ReferentialAction::Restrict => "RESTRICT",
            ReferentialAction::Cascade => "CASCADE",
            ReferentialAction::SetNull => "SET NULL",
            ReferentialAction::SetDefault => "SET DEFAULT",
            ReferentialAction::NoAction => "NO ACTION",
        }
    }
}

/// Relationship between tables
#[derive(Debug, Clone)]
pub struct Relationship {
    name: String,
    source_table: String,
    source_columns: Vec<String>,
    target_table: String,
    target_columns: Vec<String>,
    relationship_type: RelationshipType,
}

impl Relationship {
    /// Create a new relationship
    pub fn new(
        name: String,
        source_table: String,
        source_columns: Vec<String>,
        target_table: String,
        target_columns: Vec<String>,
        relationship_type: RelationshipType,
    ) -> Self {
        Self {
            name,
            source_table,
            source_columns,
            target_table,
            target_columns,
            relationship_type,
        }
    }

    /// Get the relationship name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the source table
    pub fn source_table(&self) -> &str {
        &self.source_table
    }

    /// Get the source columns
    pub fn source_columns(&self) -> &[String] {
        &self.source_columns
    }

    /// Get the target table
    pub fn target_table(&self) -> &str {
        &self.target_table
    }

    /// Get the target columns
    pub fn target_columns(&self) -> &[String] {
        &self.target_columns
    }

    /// Get the relationship type
    pub fn relationship_type(&self) -> RelationshipType {
        self.relationship_type
    }
}

/// Types of relationships between tables
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

impl RelationshipType {
    /// Get a string representation of the relationship type
    pub fn to_string(&self) -> &'static str {
        match self {
            RelationshipType::OneToOne => "One-to-One",
            RelationshipType::OneToMany => "One-to-Many",
            RelationshipType::ManyToOne => "Many-to-One",
            RelationshipType::ManyToMany => "Many-to-Many",
        }
    }
}

/// Schema metadata for visualization and documentation
#[derive(Debug, Clone, Default)]
pub struct SchemaMetadata {
    table_positions: HashMap<String, (f32, f32)>,
    table_colors: HashMap<String, String>,
    table_descriptions: HashMap<String, String>,
    column_descriptions: HashMap<String, HashMap<String, String>>,
}

impl SchemaMetadata {
    /// Create new schema metadata
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the position of a table in the schema visualization
    pub fn get_table_position(&self, table_name: &str) -> Option<(f32, f32)> {
        self.table_positions.get(table_name).copied()
    }

    /// Set the position of a table in the schema visualization
    pub fn set_table_position(&mut self, table_name: String, position: (f32, f32)) {
        self.table_positions.insert(table_name, position);
    }

    /// Get the color of a table in the schema visualization
    pub fn get_table_color(&self, table_name: &str) -> Option<&str> {
        self.table_colors.get(table_name).map(|s| s.as_str())
    }

    /// Set the color of a table in the schema visualization
    pub fn set_table_color(&mut self, table_name: String, color: String) {
        self.table_colors.insert(table_name, color);
    }

    /// Get the description of a table
    pub fn get_table_description(&self, table_name: &str) -> Option<&str> {
        self.table_descriptions.get(table_name).map(|s| s.as_str())
    }

    /// Set the description of a table
    pub fn set_table_description(&mut self, table_name: String, description: String) {
        self.table_descriptions.insert(table_name, description);
    }

    /// Get the description of a column
    pub fn get_column_description(&self, table_name: &str, column_name: &str) -> Option<&str> {
        self.column_descriptions
            .get(table_name)
            .and_then(|columns| columns.get(column_name))
            .map(|s| s.as_str())
    }

    /// Set the description of a column
    pub fn set_column_description(
        &mut self,
        table_name: String,
        column_name: String,
        description: String,
    ) {
        self.column_descriptions
            .entry(table_name)
            .or_default()
            .insert(column_name, description);
    }
}
