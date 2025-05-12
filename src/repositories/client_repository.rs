use sqlx::PgPool;

#[derive(Clone)]
pub struct ClientRepository {
    pool: PgPool,
}
impl ClientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn is_user_client(&self, user_id: u64) -> bool {
        true
    }
}
