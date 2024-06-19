use crate::database::UserId;

impl From<serenity::all::UserId> for UserId {
    fn from(value: serenity::all::UserId) -> Self {
        Self(value.get())
    }
}

impl Into<u64> for UserId {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for UserId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
