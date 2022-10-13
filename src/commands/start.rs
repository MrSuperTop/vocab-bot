use teloxide::{prelude::*, types::Me};

use super::CommandHandlerResult;

pub async fn start_handler(bot: Bot, msg: Message, me: Me) -> CommandHandlerResult {
    let message = format!(
        "Hello and welcome, my name is {}. Start your journey by typing /help.",
        me.username()
    );

    bot.send_message(msg.chat.id, &message).await?;

    Ok(())
}
