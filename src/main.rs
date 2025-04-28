use sqlx::postgres::PgPoolOptions;

use dotenv::dotenv;
use sqlx::PgPool;
use teloxide::{prelude::*, types::User};
use SlotBot::configuration::get_configuration;
use SlotBot::handlers::base_commands::BaseCommand;
use SlotBot::handlers::user_commands::UserCommand;
use SlotBot::handlers::worker_commands::WorkerCommand;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
enum Command {
    Base(BaseCommand),
    Client(UserCommand),
    Worker(WorkerCommand),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    dotenv().ok();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let bot = Bot::from_env();
    let schema = Update::filter_message()
        .filter_map(|update: Update| update.from().cloned())
        .branch(handle_command);
    log::info!("Bot version: {} is configured", env!("CARGO_PKG_VERSION"));
    Dispatcher::builder(bot, schema)
        .dependencies(dptree::deps![connection_pool])
        .build()
        .dispatch()
        .await;
    Ok(())
}
async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    pool: PgPool,  // Пул подключений к БД
) -> ResponseResult<()> {

}