use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use sled::Db;
use typed_sled::Tree;

pub mod users;

pub static DATABASE: LazyLock<Db> = LazyLock::new(|| sled::open("database.db").unwrap());

pub static USERS_TREE: LazyLock<Tree<UserId, UserData>> =
    LazyLock::new(|| Tree::open(&DATABASE, "users"));

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UserId(pub u64);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserData {
    pub exp: f64,
}
