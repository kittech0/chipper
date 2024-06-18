use std::time::Duration;

use log::warn;
use serenity::all::{Context, Message};
use sled::IVec;
use tokio::time;

use crate::error::BoxResult;
use crate::levels::{LEVEL_HANDLER, LevelHandler};

impl LevelHandler {
    pub fn new() -> Self {
        tokio::spawn(async {
            let mut interval = time::interval(Duration::from_mins(20));
            loop {
                interval.tick().await;
                LEVEL_HANDLER
                    .db
                    .flush_async()
                    .await
                    .expect("Unable to flush database");
                warn!("Flushing level.db")
            }
        });
        Self {
            db: sled::open("level.db").unwrap(),
        }
    }
    pub async fn on_message(&self, _: &Context, msg: &Message) -> BoxResult {
        if msg.author.bot {
            return Ok(());
        }
        let user_id = msg.author.id.get();
        self.db.fetch_and_update(user_id.to_le_bytes(), |data| {
            let num: u64 = match data {
                Some(data) => u64::from_le_bytes(data.try_into().unwrap()) + 1,
                None => 0,
            };
            Some(IVec::from(&num.to_le_bytes()))
        })?;

        Ok(())
    }
}
