#[macro_use]
extern crate inspirer_actix_ext_derive;

pub use inspirer_actix_ext_core::preludes::{config, service, ModuleProvider, ModuleContainer, ModuleFactoryFn};
pub use inspirer_actix_ext_core::error;
pub use inspirer_actix_ext_derive::*;

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
