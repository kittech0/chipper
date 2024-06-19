use std::hash::{Hash, Hasher};

use serenity::all::{Context, Message};
use serenity::async_trait;

use crate::commands::{AStr, Command};
use crate::database::{UserData, USERS_TREE};
use crate::utils::error::BoxResult;

pub struct LevelCommand;

impl Hash for LevelCommand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_name().hash(state)
    }
}

#[async_trait]
impl Command for LevelCommand {
    fn get_name(&self) -> AStr {
        "level".into()
    }

    fn get_description(&self) -> AStr {
        "Shows your level and exp".into()
    }

    async fn run(&self, ctx: &Context, message: &Message, _: Option<&str>) -> BoxResult {
        let user = if let Some(user) = message.mentions.first()
            && !user.bot
        {
            user
        } else {
            &message.author
        };
        let (exp, level) = if let Some(data) = USERS_TREE.get(user.id.into())? {
            (data.exp, data.level)
        } else {
            USERS_TREE.insert(user.id.into(), &UserData::default())?;
            (0., 0)
        };
        message
            .reply_ping(&ctx.http, format!("Exp: {exp:.1}"))
            .await?;
        Ok(())
    }
}
