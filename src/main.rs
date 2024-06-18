#![feature(async_closure, const_trait_impl, effects)]

use std::env;
use std::sync::LazyLock;

use dotenv::dotenv;
use log::LevelFilter;
use serenity::all::GatewayIntents;
use serenity::Client;
use sled::Db;

use crate::error::{BoxResult, error_print};
use crate::events::Events;

mod commands;
mod error;
mod events;
mod levels;

static DB: LazyLock<Db> = LazyLock::new(|| sled::open("database.sled").unwrap());

async fn run() -> BoxResult<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    dotenv().ok();
    let token = env::var("TOKEN")?;
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Events)
        .await?;

    client.start().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        error_print(error);
    }
}
