pub mod help;
pub mod start;

use std::error::Error;

use teloxide::macros::BotCommands;

pub type CommandHandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "show this message.")]
    Help,
}
