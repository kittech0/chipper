#![feature(async_closure, const_trait_impl, effects)]
#![feature(duration_constructors)]
#![feature(async_drop)]

extern crate core;

use std::env;

use dotenv::dotenv;
use log::LevelFilter;
use serenity::all::GatewayIntents;
use serenity::Client;

use crate::error::{BoxResult, error_print};
use crate::events::Events;

mod commands;
mod db;
mod error;
mod events;
mod levels;

async fn run() -> BoxResult {
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
async fn main() -> BoxResult {
    if let Err(error) = run().await {
        error_print(&error);
        return Err(error);
    };
    Ok(())
}
