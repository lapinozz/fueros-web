use std::num::NonZeroU8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(NonZeroU8);
pub const SYSTEM_PLAYER_ID: PlayerId = PlayerId(unsafe { NonZeroU8::new_unchecked(u8::MAX) });

pub struct Player {
    username: String,
}

impl Player {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}
