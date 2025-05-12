use crate::commands::BaseCommand;
use teloxide::Bot;
use teloxide::prelude::{Message, ResponseResult};

pub async fn answer_admin_command(bot: Bot, msg: Message, cmd: BaseCommand) -> ResponseResult<()> {
    Ok(())
}
