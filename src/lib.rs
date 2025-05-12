mod command_handlers;
mod commands;
mod repositories;

use crate::command_handlers::admin_commands_handlers::answer_admin_command;
use crate::command_handlers::base_commands_handlers::answer_base_command;
use crate::command_handlers::client_commands_handlers::answer_client_command;
use crate::command_handlers::worker_commands_handlers::answer_worker_command;
use crate::commands::*;
use crate::repositories::{
    admin_repository::*, client_repository::*, user_repository::*, worker_repository::*,
};
use commands::*;
use sqlx::PgPool;
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::User;
use teloxide::utils::command::BotCommands;

pub struct BotState {
    pub user_repo: UserRepository,
    pub worker_repo: WorkerRepository,
    pub admin_repo: AdminRepository,
    pub client_repo: ClientRepository,
}

pub async fn run() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let admin_repo = AdminRepository::new(pool.clone());
    let worker_repo = WorkerRepository::new(pool.clone());
    let client_repo = ClientRepository::new(pool.clone());
    let admin_filter = |msg: Message| {
        // clone the variable from the environment to capture it
        let admin_repo = admin_repo.clone();
        async move {
            match msg.from {
                None => false,
                Some(user) => admin_repo.is_user_admin(user.id.0).await,
            }
        }
    };
    let worker_filter = |msg: Message| {
        // clone the variable from the environment to capture it
        let worker_repo = worker_repo.clone();
        async move {
            match msg.from {
                None => false,
                Some(user) => worker_repo.is_user_worker(user.id.0).await,
            }
        }
    };
    let client_filter = |msg: Message| {
        // clone the variable from the environment to capture it
        let client_repo = client_repo.clone();
        async move {
            match msg.from {
                None => false,
                Some(user) => client_repo.is_user_client(user.id.0).await,
            }
        }
    };
    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<BaseCommand>()
                .endpoint(answer_base_command),
        )
        .branch(
            dptree::filter_async(admin_filter)
            .filter_command::<AdminCommand>()
            .endpoint(answer_admin_command),
        )
        .branch(
            dptree::filter_async(worker_filter)
                .filter_command::<WorkerCommand>()
                .endpoint(answer_worker_command),
        )
        .branch(
            dptree::filter_async(client_filter)
                .filter_command::<ClientCommand>()
                .endpoint(answer_client_command),
        );
    let bot_state = BotState {
        user_repo: UserRepository::new(pool.clone()),
        worker_repo: worker_repo.clone(),
        admin_repo: admin_repo.clone(),
        client_repo: client_repo.clone(),
    };

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![bot_state])
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text("An error"))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
