use teloxide::{utils::command::BotCommands};


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Start Bot. We now its alive!")]
    Start,
    #[command(description = "display this text.")]
    Help
}