use std::sync::LazyLock;

mod handler;

pub static LEVEL_HANDLER: LazyLock<LevelHandler> = LazyLock::new(LevelHandler::new);

pub struct LevelHandler {
    pub db: sled::Db,
}
