#[macro_use]
extern crate serde;
#[macro_use]
extern crate validator;

pub use validator::*;
use std::sync::Arc;
use crate::error::Error;
use actix_web::HttpRequest;

mod json;
pub mod error;

pub struct Validated<T>(pub T);

#[derive(Clone, Default)]
pub struct ValidateConfig {
    error_handler: Option<Arc<dyn Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync>>,
}

impl ValidateConfig {
    pub fn error_handler<F>(mut self, f: F) -> Self
        where F: Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync + 'static,
    {
        self.error_handler = Some(Arc::new(f));
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
