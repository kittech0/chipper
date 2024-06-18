use std::sync::Arc;

use dashmap::ReadOnlyView;
use serenity::all::{Context, Message};
use serenity::async_trait;

use crate::error::BoxResult;

pub mod handler;
mod level;
mod ping;

pub type CommandMap = ReadOnlyView<&'static str, Arc<dyn Command + Send + Sync>>;

const PREFIX: &str = ">";

#[async_trait]
pub trait Command {
    async fn run(&self, ctx: &Context, message: &Message, text: Option<&str>) -> BoxResult;
}

pub struct CommandHandler {
    commands: CommandMap,
}
