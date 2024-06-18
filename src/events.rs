use log::info;
use serenity::all::{Context, EventHandler, Message, Ready};
use serenity::async_trait;

use crate::commands::COMMAND_HANDLER;
use crate::error::error_print;
use crate::levels::{LEVEL_HANDLER, LevelHandler};

pub struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(err) = COMMAND_HANDLER.on_message(&ctx, &msg).await {
            error_print(err)
        }
        if let Err(err) = LEVEL_HANDLER.on_message(&ctx, &msg).await {
            error_print(err)
        }
    }
    async fn ready(&self, _: Context, data: Ready) {
        info!("Ready: {}", data.user.name)
    }
}
