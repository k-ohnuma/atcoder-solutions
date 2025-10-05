use async_trait::async_trait;

#[async_trait]
pub trait HealthCheckRepository {
    async fn check_db(&self) -> bool;
}
