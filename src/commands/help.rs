use teloxide::{prelude::*, utils::command::BotCommands};

use super::{Command, CommandHandlerResult};

pub async fn help_handler(bot: Bot, msg: Message) -> CommandHandlerResult {
    let help_text = Command::descriptions().to_string();
    bot.send_message(msg.chat.id, help_text).await?;

    Ok(())
}
