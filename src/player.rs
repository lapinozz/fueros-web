use std::num::NonZeroU8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    Real(NonZeroU8),
    System
}

pub enum Player {
    Bot { username: String },
    User { username: String },
}
