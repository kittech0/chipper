use std::ops::Deref;
use std::sync::{Arc, LazyLock};

use dashmap::DashMap;
use serenity::all::{Context, Message};

use crate::commands::{Command, CommandHandler, CommandMap, PREFIX};
use crate::commands::level::LevelCommand;
use crate::commands::ping::PingCommand;
use crate::error::BoxResult;

impl CommandHandler {
    fn new() -> Self {
        Self {
            commands: Self::commands(),
        }
    }

    pub fn get() -> &'static Self {
        static INIT: LazyLock<CommandHandler> = LazyLock::new(CommandHandler::new);
        INIT.deref()
    }

    fn commands() -> CommandMap {
        let map = DashMap::<&'static str, Arc<dyn Command + Send + Sync>>::new();
        map.insert("ping", Arc::new(PingCommand));
        map.insert("level", Arc::new(LevelCommand));
        map.into_read_only()
    }

    pub async fn on_message(&self, ctx: &Context, message: &Message) -> BoxResult {
        if message.author.bot {
            return Ok(());
        }
        if let Some(command_str) = message.content.strip_prefix(PREFIX) {
            let (command_name, text) = command_str
                .split_once(' ')
                .map(|(a, b)| (a, Some(b)))
                .unwrap_or((command_str, None));

            if let Some(cmd) = self.commands.get(command_name) {
                cmd.run(ctx, message, text).await?;
            } else {
                message.reply_ping(&ctx.http, "Unknown command!").await?;
            }
        }
        Ok(())
    }
}
