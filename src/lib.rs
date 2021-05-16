#[macro_use]
extern crate log;

pub mod module;
pub mod config;

pub mod preludes {
    pub use crate::module::{ModuleFactoryFn, ModuleProvider, ModuleContainer};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
