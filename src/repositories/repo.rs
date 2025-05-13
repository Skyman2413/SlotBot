use sqlx::PgPool;

pub trait Repository {
    fn new(pool: PgPool) -> Self;
}

pub trait RoleRepository: Repository {
    async fn user_has_permission(&self, user_id: u64) -> bool;
}
