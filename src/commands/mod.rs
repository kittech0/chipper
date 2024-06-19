use std::sync::Arc;

use dashmap::{DashMap, ReadOnlyView};
use serenity::all::{Context, Message};
use serenity::async_trait;

use crate::utils::error::BoxResult;

pub mod handler;
mod level;
mod ping;

pub type AStr = Arc<str>;

pub type CommandMap = ReadOnlyView<AStr, Arc<dyn Command + Send + Sync>>;
pub type CommandMapBuilder = DashMap<AStr, Arc<dyn Command + Send + Sync>>;
const PREFIX: &str = ">";

#[async_trait]
pub trait Command {
    fn get_name(&self) -> AStr;
    fn get_description(&self) -> AStr;
    async fn run(&self, ctx: &Context, message: &Message, text: Option<&str>) -> BoxResult;
}

pub struct CommandHandler {
    commands: CommandMap,
}
