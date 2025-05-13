use crate::repositories::repo::{Repository, RoleRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AdminRepository {
    pool: PgPool,
}

impl Repository for AdminRepository {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl RoleRepository for AdminRepository {
    async fn user_has_permission(&self, user_id: u64) -> bool {
        true
    }
}
