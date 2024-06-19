use log::info;
use serenity::all::{Context, EventHandler, Message, Ready};
use serenity::async_trait;

use crate::commands::CommandHandler;
use crate::levels::LevelHandler;
use crate::utils::error::error_print;

pub struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(err) = CommandHandler::get().on_message(&ctx, &msg).await {
            error_print(err)
        } else if let Err(err) = LevelHandler.on_message(&ctx, &msg).await {
            error_print(err)
        }
    }
    async fn ready(&self, _: Context, data: Ready) {
        info!("Ready: {}", data.user.name)
    }
}
