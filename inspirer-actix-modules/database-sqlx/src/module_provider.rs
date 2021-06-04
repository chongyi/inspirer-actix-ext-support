pub mod mysql {
    use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
    use sqlx::{Result, MySqlPool};

    use inspirer_actix_ext_core::preludes::*;

    use crate::config::DatabaseConfig;
    use inspirer_actix_ext_core::config::Config;

    pub async fn register(ctx: &ModuleProvider) -> Result<MySqlPool> {
        debug!("Register MySQL database (sqlx) module.");

        debug!("Get database config from module provider.");
        let config = ctx.get_ref::<DatabaseConfig>()
            .cloned()
            .or_else(|| {
                debug!("Module provider is not contain <DatabaseConfig>, load config from <Config> module.");
                ctx.get_ref::<Config>()
                    .and_then(|config|
                        config.get::<DatabaseConfig>("database").ok())
            })
            .expect("No database connection configuration!")
            .clone();

        debug!("Convert database config into connect options.");
        let options: MySqlConnectOptions = config.into();

        debug!("Connect options is {:?}", options);

        MySqlPoolOptions::new()
            .after_connect(|_| Box::pin(async move {
                info!("Database (mysql) connection pool's connection is created.");
                Ok(())
            }))
            .connect_with(options)
            .await
    }
}