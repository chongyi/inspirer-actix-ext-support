#[macro_use]
extern crate log;
#[macro_use]
extern crate async_trait;

use sqlx::{Database, Executor};

pub mod config;
pub mod module_provider;
pub mod dao;

pub mod prelude {
    pub use sqlx;

    pub use crate::config;
    pub use crate::dao::DAO;
    pub use crate::module_provider::mysql;
}