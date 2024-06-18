use log::info;
use serenity::all::{Context, EventHandler, Message, Ready};
use serenity::async_trait;

use crate::commands::COMMAND_HANDLER;
use crate::error::error_print;

pub struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(err) = COMMAND_HANDLER.command_run(&ctx, &msg).await {
            error_print(err)
        }
    }
    async fn ready(&self, _: Context, data: Ready) {
        info!("Ready: {}", data.user.name)
    }
}
