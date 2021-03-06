#[macro_use]
extern crate log;

pub mod module;
pub mod service;
pub mod error;
pub mod config;


pub mod preludes {
    pub use crate::module::{ModuleFactoryFn, ModuleProvider, ModuleContainer};
    pub use crate::config;
    pub use crate::service;
    pub use crate::error::Error;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
