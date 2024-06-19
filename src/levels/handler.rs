use std::f64;

use rand::Rng;
use serenity::all::{Context, Message};

use crate::database::{UserData, USERS_TREE};
use crate::levels::LevelHandler;
use crate::utils::error::BoxResult;

impl LevelHandler {
    pub async fn on_message(&self, _: &Context, msg: &Message) -> BoxResult {
        if msg.author.bot {
            return Ok(());
        }
        let user_id = msg.author.id.get();

        handle_levels(user_id)?;
        Ok(())
    }
}

fn handle_levels(user_id: u64) -> BoxResult {
    let data = USERS_TREE
        .update_and_fetch(user_id.into(), update_levels)?
        .unwrap();
    Ok(())
}

fn update_levels(data: Option<UserData>) -> Option<UserData> {
    let random_value = rand::thread_rng().gen_range(1..=15) as f64 / 10.;
    let mut data = data.clone();
    if let Some(data) = data {
        let new = f64_le_from(data) + 1. * random_value;
        Some(new.into())
    } else {
        Some(0_f64.into())
    }
}

fn get_level_exp(level: u32) -> f64 {
    let level = level as f64;
    let a = 15.;
    let b = 2.;
    a * level - (b * level.cos())
}

pub fn f64_le_from<T: AsRef<[u8]>>(data: T) -> f64 {
    let data = data.as_ref();
    f64::from_le_bytes(data.try_into().unwrap())
}
