use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlConnectOptions;

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub username: String,
    pub password: Option<String>,
    pub database: Option<String>,
    #[serde(default)]
    pub port: u16
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            host: "127.0.0.1".into(),
            username: "root".into(),
            password: None,
            database: None,
            port: 3306
        }
    }
}

impl Into<MySqlConnectOptions> for DatabaseConfig {
    fn into(self) -> MySqlConnectOptions {
        let options = MySqlConnectOptions::default()
            .host(self.host.as_str())
            .username(self.username.as_str())
            .port(self.port);

        let options = match self.password {
            Some(password) => options.password(password.as_str()),
            None => options,
        };

        let options = match self.database {
            Some(database) => options.database(database.as_str()),
            None => options,
        };

        options
    }
}