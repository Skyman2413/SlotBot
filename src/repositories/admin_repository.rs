use sqlx::PgPool;

#[derive(Clone)]
pub struct AdminRepository {
    pool: PgPool,
}

impl AdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn is_user_admin(&self, user_id: u64) -> bool {
        true
    }
}
