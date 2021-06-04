use inspirer_actix_ext_core::preludes::*;
use crate::config::RedisConfig;
use redis::{Client, RedisResult};
use redis::aio::MultiplexedConnection;
use inspirer_actix_ext_core::config::Config;

pub async fn register_redis_client(ctx: &ModuleProvider) -> RedisResult<Client> {
    debug!("Register Redis module.");

    let config = ctx.get_ref::<RedisConfig>()
        .cloned()
        .or_else(|| {
            debug!("Module provider is not contain <RedisConfig>, load config from <Config> module.");
            ctx.get_ref::<Config>()
                .and_then(|config|
                    config.get::<RedisConfig>("redis").ok())
        })
        .expect("No redis connection configuration!")
        .clone();

    Client::open(config.connection)
}

pub async fn register_redis_multiplexed_connection(ctx: &ModuleProvider) -> RedisResult<MultiplexedConnection> {
    debug!("Register Redis (Multiplexed connection) module.");

    match ctx.get_ref::<Client>() {
        Some(client) => {
            debug!("Exist redis client, use client create connection.");
            client.get_multiplexed_async_connection().await
        }
        None => {
            debug!("Not exist redis client.");
            let client = register_redis_client(ctx).await?;

            client.get_multiplexed_async_connection().await
        }
    }
}