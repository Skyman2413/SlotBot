use teloxide::{utils::command::BotCommands};


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum BaseCommand {
    #[command(description = "Показывает стартовое сообщение")]
    Start,
    #[command(description = "Список доступных команд")]
    Help
}