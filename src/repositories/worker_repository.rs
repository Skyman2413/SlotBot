use crate::repositories::repo::{Repository, RoleRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct WorkerRepository {
    pool: PgPool,
}

impl Repository for WorkerRepository {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl RoleRepository for WorkerRepository {
    async fn user_has_permission(&self, user_id: u64) -> bool {
        true
    }
}
