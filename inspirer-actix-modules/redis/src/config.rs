use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    pub connection: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig {
            connection: "redis://127.0.0.1:6379".into(),
        }
    }
}