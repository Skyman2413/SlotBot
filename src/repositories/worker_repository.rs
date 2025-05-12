use sqlx::PgPool;

#[derive(Clone)]
pub struct WorkerRepository {
    pool: PgPool,
}

impl WorkerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn is_user_worker(&self, user_id: u64) -> bool {
        true
    }
}
