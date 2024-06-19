use std::sync::LazyLock;
use std::time::Duration;

use log::warn;
use serde::{Deserialize, Serialize};
use sled::Db;
use tokio::time;
use typed_sled::Tree;

pub mod users;

pub static DATABASE: LazyLock<Db> = LazyLock::new(|| {
    let db = sled::open("database.db").unwrap();
    tokio::spawn(flushing(db));
    db
});

pub static USERS_TREE: LazyLock<Tree<UserId, UserData>> =
    LazyLock::new(|| Tree::open(&DATABASE, "users"));

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UserId(pub u64);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserData {
    pub exp: f64,
    pub level: u32,
}

async fn flushing(db: Db) {
    let mut interval = time::interval(Duration::from_mins(20));
    loop {
        interval.tick().await;
        db.flush_async().await.expect("Unable to flush database");
        warn!("Flushing database.db")
    }
}
