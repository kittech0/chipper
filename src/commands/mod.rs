use std::sync::{Arc, LazyLock};

use dashmap::ReadOnlyView;
use serenity::all::{Context, Message};
use serenity::async_trait;

use crate::error::BoxResult;

pub mod handler;
mod ping;

pub type CommandMap = ReadOnlyView<&'static str, Arc<dyn Command + Send + Sync>>;

const PREFIX: &str = ">";
pub static COMMAND_HANDLER: LazyLock<CommandHandler> = LazyLock::new(CommandHandler::new);

#[async_trait]
pub trait Command {
    async fn run(&self, ctx: &Context, message: &Message, text: Option<&str>) -> BoxResult;
}

pub struct CommandHandler {
    commands: CommandMap,
}
