use serenity::all::{Context, Message};
use serenity::async_trait;

use crate::commands::Command;
use crate::error::BoxResult;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    async fn run(&self, ctx: &Context, message: &Message, _: Option<&str>) -> BoxResult<()> {
        message.reply_ping(&ctx.http, "Pong!").await?;
        Ok(())
    }
}
