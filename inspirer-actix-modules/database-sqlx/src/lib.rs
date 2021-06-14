#[macro_use]
extern crate log;
#[macro_use]
extern crate async_trait;

pub mod config;
pub mod module_provider;
pub mod dao;

pub mod prelude {
    pub use sqlx;

    pub use crate::config;
    pub use crate::dao::*;
    pub use crate::module_provider::mysql;
}