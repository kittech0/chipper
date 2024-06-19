use crate::database::UserId;

impl From<serenity::all::UserId> for UserId {
    fn from(value: serenity::all::UserId) -> Self {
        Self(value.get())
    }
}

impl From<u64> for UserId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
