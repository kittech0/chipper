use std::f64;

use rand::Rng;
use serenity::all::{Context, Message};

use crate::database::{UserData, USERS_TREE};
use crate::levels::LevelHandler;
use crate::utils::error::BoxResult;

impl LevelHandler {
    pub async fn on_message(&self, ctx: &Context, msg: &Message) -> BoxResult {
        if !msg.author.bot {
            let user_id = msg.author.id.get();
            handle_levels(user_id)
        } else {
            Ok(())
        }
    }
}

fn handle_levels(user_id: u64) -> BoxResult {
    USERS_TREE.update_and_fetch(&user_id.into(), update_levels)?;
    Ok(())
}

fn update_levels(data: Option<UserData>) -> Option<UserData> {
    Some(if let Some(mut data) = data {
        update_exp(&mut data);
        data
    } else {
        UserData::default()
    })
}

fn update_exp(data: &mut UserData) {
    let random_value = rand::thread_rng().gen_range(1..=15) as f64 / 10.;
    data.exp += 1. * random_value;
}

pub fn level_to_exp(level: u32) -> f64 {
    let level = level as f64;
    10. * level.powi(2)
}

pub fn exp_to_level(exp: f64) -> u32 {
    let constant = 1. / 10_f64.sqrt();
    (constant * exp.sqrt()) as u32
}
