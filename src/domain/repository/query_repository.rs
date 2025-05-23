use super::RepositoryError;
use crate::domain::model::{ConnectionId, Query, QueryId};
use async_trait::async_trait;

#[async_trait]
pub trait QueryRepository: Send + Sync {
    async fn save(&self, query: &Query) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &QueryId) -> Result<Option<Query>, RepositoryError>;
    async fn find_by_connection(
        &self,
        connection_id: &ConnectionId,
    ) -> Result<Vec<Query>, RepositoryError>;
    async fn find_recent(&self, limit: usize) -> Result<Vec<Query>, RepositoryError>;
    async fn delete(&self, id: &QueryId) -> Result<(), RepositoryError>;
    async fn update(&self, query: &Query) -> Result<(), RepositoryError>;
}
