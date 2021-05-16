use inspirer_actix_ext_core::preludes::*;
use crate::config::RedisConfig;
use redis::{Client, RedisResult};
use redis::aio::MultiplexedConnection;

pub async fn register_redis_client(ctx: &ModuleProvider) -> RedisResult<Client> {
    debug!("Register Redis module.");

    let config = ctx.get_ref::<RedisConfig>()
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
        },
        None => {
            debug!("Not exist redis client.");
            let client = register_redis_client(ctx).await?;

            client.get_multiplexed_async_connection().await
        }
    }
}