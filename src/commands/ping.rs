use serenity::all::{Context, Message};
use serenity::async_trait;

use crate::commands::{AStr, Command};
use crate::utils::error::BoxResult;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn get_name(&self) -> AStr {
        "ping".into()
    }

    fn get_description(&self) -> AStr {
        "ping".into()
    }

    async fn run(&self, ctx: &Context, message: &Message, _: Option<&str>) -> BoxResult {
        message.reply_ping(&ctx.http, "Pong!").await?;
        Ok(())
    }
}
