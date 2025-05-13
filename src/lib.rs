mod command_handlers;
mod commands;
mod repositories;

use crate::{
    command_handlers::{
        admin_commands_handlers::answer_admin_command,
        base_commands_handlers::answer_base_command,
        client_commands_handlers::answer_client_command,
        worker_commands_handlers::answer_worker_command,
    },
    commands::*,
    repositories::{
        repo::{Repository, RoleRepository},
        admin_repository::*, client_repository::*, user_repository::*,
        worker_repository::*,
    },
};
use commands::*;
use sqlx::PgPool;
use std::sync::Arc;
use teloxide::dispatching::DefaultKey;
use teloxide::prelude::*;
use teloxide::types::User;
use teloxide::utils::command::BotCommands;
use teloxide::{Bot, RequestError};
use tracing::{info, warn, error};

pub struct BotState {
    pub user_repo: UserRepository,
    pub worker_repo: WorkerRepository,
    pub admin_repo: AdminRepository,
    pub client_repo: ClientRepository,
}

pub async fn run() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();
    info!("Starting command bot...");

    let bot = Bot::from_env();
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Error connecting to database");
    info!("Successfully connected to database");
    let admin_repo = AdminRepository::new(pool.clone());
    let worker_repo = WorkerRepository::new(pool.clone());
    let client_repo = ClientRepository::new(pool.clone());

    let bot_state = BotState {
        user_repo: UserRepository::new(pool.clone()),
        worker_repo: worker_repo.clone(),
        admin_repo: admin_repo.clone(),
        client_repo: client_repo.clone(),
    };
    let mut dispatcher = create_dispatcher(bot, admin_repo, worker_repo, client_repo, bot_state);
    dispatcher.dispatch().await;
}

fn create_dispatcher(
    bot: Bot,
    admin_repo: AdminRepository,
    worker_repo: WorkerRepository,
    client_repo: ClientRepository,
    bot_state: BotState
) -> Dispatcher<Bot, RequestError, DefaultKey> {
    let admin_filter = move |msg: Message| {
        // clone the variable from the environment to capture it
        let admin_repo = admin_repo.clone();
        async move {
            match msg.from {
                None => false,
                Some(user) => admin_repo.user_has_permission(user.id.0).await,
            }
        }
    };
    let worker_filter = move |msg: Message| {
        // clone the variable from the environment to capture it
        let worker_repo = worker_repo.clone();
        async move {
            match msg.from {
                None => false,
                Some(user) => worker_repo.user_has_permission(user.id.0).await,
            }
        }
    };
    let client_filter = move |msg: Message| {
        // clone the variable from the environment to capture it
        let client_repo = client_repo.clone();
        async move {
            match msg.from {
                None => false,
                Some(user) => client_repo.user_has_permission(user.id.0).await,
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

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![bot_state])
        .default_handler(|upd| async move {
            warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text("An error"))
        .enable_ctrlc_handler()
        .build()
}
