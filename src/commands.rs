use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Эти команды используются всеми работниками"
)]
pub enum BaseCommand {
    #[command(description = "display this text.")]
    Help,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Эти команды используются работником:"
)]
pub enum WorkerCommand {
    #[command(description = "Изменить запись")]
    EditSlot,
    #[command(description = "Изменить рабочие часы")]
    EditWorkHours,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Эти команды используются пользователем:"
)]
pub enum ClientCommand {
    #[command(description = "Посмотреть свои записи")]
    CheckSlots,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Эти команды используются админом:"
)]
pub enum AdminCommand {
    #[command(description = "Установить роль пользователя")]
    SetRole,
}
