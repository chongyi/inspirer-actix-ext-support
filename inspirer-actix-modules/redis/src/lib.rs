#[macro_use]
extern crate log;

pub mod module_provider;
pub mod config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
