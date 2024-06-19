#![feature(async_closure, const_trait_impl, effects)]
#![feature(duration_constructors)]
#![feature(async_drop)]
#![feature(let_chains)]
#![feature(inherent_associated_types)]

extern crate core;

use std::env;

use dotenv::dotenv;
use log::LevelFilter;
use serenity::all::GatewayIntents;
use serenity::Client;

use crate::utils::error::BoxResult;
use crate::utils::events::Events;

mod commands;
mod database;
mod levels;
mod utils;

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
    run().await
}
