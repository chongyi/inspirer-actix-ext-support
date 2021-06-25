use thiserror::Error;
use actix_web::ResponseError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot found dependency: {0}")]
    DependencyNotFound(&'static str),
}

impl ResponseError for Error {}