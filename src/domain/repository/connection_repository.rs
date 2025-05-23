use super::RepositoryError;
use crate::domain::model::{ConnectionId, DatabaseConnection};
use async_trait::async_trait;

#[async_trait]
pub trait ConnectionRepository: Send + Sync {
    async fn save(&self, connection: &DatabaseConnection) -> Result<(), RepositoryError>;
    async fn find_by_id(
        &self,
        id: &ConnectionId,
    ) -> Result<Option<DatabaseConnection>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<DatabaseConnection>, RepositoryError>;
    async fn delete(&self, id: &ConnectionId) -> Result<(), RepositoryError>;
    async fn update(&self, connection: &DatabaseConnection) -> Result<(), RepositoryError>;
}
