use serenity::all::{Context, Message};
use serenity::async_trait;
use sled::IVec;

use crate::commands::Command;
use crate::db::LEVEL_TREE;
use crate::error::BoxResult;

pub struct LevelCommand;

#[async_trait]
impl Command for LevelCommand {
    async fn run(&self, ctx: &Context, message: &Message, _: Option<&str>) -> BoxResult {
        let user = if let Some(user) = message.mentions.first() {
            user
        } else {
            &message.author
        };
        let value = if let Some(data) = LEVEL_TREE.get(user.id.get().to_le_bytes())? {
            u64::from_le_bytes(data.as_ref().try_into()?)
        } else {
            LEVEL_TREE.insert(
                user.id.get().to_le_bytes(),
                IVec::from(&0_u64.to_le_bytes()),
            )?;
            0
        };
        message
            .reply_ping(&ctx.http, format!("Exp: {value}"))
            .await?;
        Ok(())
    }
}
