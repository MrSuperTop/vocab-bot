use crate::config::Config;
use teloxide::prelude::*;

mod commands;
mod config;
mod schema;

use crate::schema::schema;

#[tokio::main]
async fn main() {
    let config = Config::load("./src/config/dev.json");
    let bot = Bot::new(config.token);
    let handler = schema();

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
