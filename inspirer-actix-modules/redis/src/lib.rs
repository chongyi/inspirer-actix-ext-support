#[macro_use]
extern crate log;

pub mod module_provider;
pub mod config;

pub mod prelude {
    pub use redis;
    pub use crate::config;
    pub use crate::module_provider;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
