pub use inspirer_actix_ext_core::preludes::{error, config, service, ModuleProvider, ModuleContainer, ModuleFactoryFn};

#[cfg(feature = "database")]
pub mod database {
    pub use inspirer_actix_module_database_sqlx::prelude::*;
}

#[cfg(feature = "redis")]
pub mod redis {
    pub use inspirer_actix_module_redis::prelude::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
