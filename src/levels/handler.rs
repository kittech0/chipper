use serenity::all::{Context, Message};
use sled::IVec;

use crate::db::LEVEL_TREE;
use crate::error::BoxResult;
use crate::levels::LevelHandler;

impl LevelHandler {
    pub async fn on_message(&self, _: &Context, msg: &Message) -> BoxResult {
        if msg.author.bot {
            return Ok(());
        }
        let user_id = msg.author.id.get();
        LEVEL_TREE.fetch_and_update(user_id.to_le_bytes(), |data| -> Option<IVec> {
            if let Some(data) = data {
                let new = u64::from_le_bytes(data.try_into().unwrap()) + 1;
                Some((&new.to_le_bytes()).into())
            } else {
                Some((&0_u64.to_le_bytes()).into())
            }
        })?;

        Ok(())
    }
}
