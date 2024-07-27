use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Nonce(String);