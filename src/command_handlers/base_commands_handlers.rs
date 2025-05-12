use crate::commands::BaseCommand;
use teloxide::Bot;
use teloxide::prelude::{Message, Requester, ResponseResult};

pub async fn answer_base_command(bot: Bot, msg: Message, cmd: BaseCommand) -> ResponseResult<()> {
    match cmd {
        BaseCommand::Help => {
            bot.send_message(msg.chat.id, "Help text").await?;
        }
    };

    Ok(())
}
