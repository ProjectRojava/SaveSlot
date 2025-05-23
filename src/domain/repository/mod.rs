pub mod connection_repository;
pub mod query_repository;

pub use connection_repository::*;
pub use query_repository::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),
}
