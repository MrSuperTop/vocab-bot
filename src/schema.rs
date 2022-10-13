use std::error::Error;

use teloxide::{dispatching::UpdateHandler, prelude::*};

use crate::commands::{help::help_handler, start::start_handler, Command};

pub fn schema() -> UpdateHandler<Box<dyn Error + Sync + Send + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help_handler))
        .branch(case![Command::Start].endpoint(start_handler));

    let message_handler = Update::filter_message().branch(command_handler);

    return dptree::entry().branch(message_handler);
}
