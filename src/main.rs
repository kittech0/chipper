#![feature(async_closure, const_trait_impl, effects)]
#![feature(duration_constructors)]
#![feature(async_drop)]
#![feature(let_chains)]
#![feature(inherent_associated_types)]

extern crate core;

use std::env;
use std::time::Duration;

use dotenv::dotenv;
use log::{info, LevelFilter, warn};
use serenity::all::GatewayIntents;
use serenity::Client;
use sled::Db;
use tokio::time;

use utils::Events;

use crate::database::DATABASE;
use crate::utils::error::BoxResult;

mod commands;
mod database;
mod levels;
mod utils;

#[tokio::main]
async fn main() -> BoxResult {
    run().await
}

async fn run() -> BoxResult {
    env_logger::builder().filter_level(LevelFilter::Warn).init();
    dotenv().ok();
    tokio::spawn(database_flusher(&DATABASE));
    tokio::spawn(database_watcher(&DATABASE));
    let token = env::var("TOKEN")?;
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Events)
        .await?;

    client.start().await?;
    Ok(())
}

async fn database_flusher(db: &Db) {
    let mut interval = time::interval(Duration::from_mins(20));
    loop {
        interval.tick().await;
        db.flush_async().await.expect("Unable to flush database");
        warn!("Flushing database.db")
    }
}

async fn database_watcher(db: &Db) {
    let mut h = db.watch_prefix("");
    while let Some(event) = (&mut h).await {
        warn!("{event:?}")
    }
}
