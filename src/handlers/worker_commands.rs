use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды работника:")]
pub enum WorkerCommand {
    #[command(description = "Установить рабочее время")]
    SetSchedule,
    #[command(description = "Записи на день")]
    TodaySchedule,
    #[command(description = "Редактировать запись")]
    ChangeAppointment,
}