#[macro_use]
extern crate log;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate strum;
#[macro_use]
extern crate serde;

pub mod config;
pub mod module_provider;
pub mod dao;
pub mod statement;

pub mod prelude {
    pub use sqlx;

    pub use crate::config;
    pub use crate::dao::*;
    pub use crate::statement;
    pub use crate::module_provider::mysql;
}