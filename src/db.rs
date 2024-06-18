use std::sync::LazyLock;
use std::time::Duration;

use log::warn;
use sled::{Db, Tree};
use tokio::time;

async fn flushing() {
    let mut interval = time::interval(Duration::from_mins(20));
    loop {
        interval.tick().await;
        DATABASE
            .flush_async()
            .await
            .expect("Unable to flush database");
        warn!("Flushing database.db")
    }
}
pub static DATABASE: LazyLock<Db> = LazyLock::new(|| {
    let db = sled::open("database.db").unwrap();
    tokio::spawn(flushing());
    db
});
pub static LEVEL_TREE: LazyLock<Tree> = LazyLock::new(|| DATABASE.open_tree("level").unwrap());
