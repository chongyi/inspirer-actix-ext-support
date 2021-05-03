#[macro_use]
extern crate log;

pub mod config;
mod module_provider;

pub use module_provider::mysql;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
