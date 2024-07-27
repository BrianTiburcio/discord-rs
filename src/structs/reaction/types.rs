use serde::{Deserialize, Serialize};

use crate::structs::emoji::Emoji;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reaction {
    pub count: u64,
    pub me: bool,
    pub emoji: Emoji
}