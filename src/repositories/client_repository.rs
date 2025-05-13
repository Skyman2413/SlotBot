use crate::repositories::repo::{Repository, RoleRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct ClientRepository {
    pool: PgPool,
}

impl Repository for ClientRepository {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl RoleRepository for ClientRepository {
    async fn user_has_permission(&self, user_id: u64) -> bool {
        true
    }
}
