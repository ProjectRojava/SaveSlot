pub mod connection_service;
pub mod query_service;

pub use connection_service::*;
pub use query_service::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),

    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::domain::repository::RepositoryError),
}
