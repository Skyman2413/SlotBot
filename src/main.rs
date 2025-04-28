mod handler;

use dotenv::dotenv;
use teloxide::{prelude::*, types::User};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    dotenv().ok();
    let bot = Bot::from_env();
    let schema = Update::filter_message()
        .filter_map(|update: Update| update.from().cloned())
        .branch(Message::filter_text().endpoint(process_text_message));
    log::info!("Bot version: {} is configured", env!("CARGO_PKG_VERSION"));
    Dispatcher::builder(bot, schema).build().dispatch().await;
    Ok(())
}

async fn process_text_message(bot: Bot, user: User, message_text: String) -> Result<(), Error> {
    log::info!("Got a message: {:?}", message_text);
    bot.send_message(user.id, format!("Hi! U sent {message_text}")).await;
    Ok(())
}