use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum UserCommand {
    #[command(description = "Мои записи")]
    MyAppointments,
    #[command(description = "Создать запись")]
    CreateAppointment,
}