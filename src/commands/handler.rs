use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::{Arc, LazyLock};

use dashmap::DashMap;
use serenity::all::{Context, Message};

use crate::commands::{Command, CommandHandler, CommandMap, CommandMapBuilder, PREFIX};
use crate::commands::level::LevelCommand;
use crate::commands::ping::PingCommand;
use crate::utils::error::BoxResult;

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
        let registry: CommandMapBuilder = DashMap::new();
        Self::register(&registry, Arc::from(PingCommand));
        Self::register(&registry, Arc::from(LevelCommand));
        registry.into_read_only()
    }

    fn register<C: Command + Send + Sync + 'static>(registry: &CommandMapBuilder, command: Arc<C>) {
        registry.insert(command.get_name(), command);
    }

    pub async fn on_message(&self, ctx: &Context, message: &Message) -> BoxResult {
        if !message.author.bot
            && let Some(command_str) = message.content.strip_prefix(PREFIX)
        {
            self.run_command(command_str, ctx, message).await?;
        }
        Ok(())
    }

    async fn run_command(&self, command_str: &str, ctx: &Context, message: &Message) -> BoxResult {
        let (command_name, text) = command_str
            .split_once(' ')
            .map(|(a, b)| (a, Some(b)))
            .unwrap_or((command_str, None));

        if let Some(cmd) = self.commands.get(command_name) {
            cmd.run(ctx, message, text).await?;
        } else {
            message.reply_ping(&ctx.http, "Unknown command!").await?;
        }
        Ok(())
    }
}
