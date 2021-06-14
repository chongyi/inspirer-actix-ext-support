use thiserror::Error;
use actix_web::ResponseError;
use actix_web::web::BytesMut;
use actix_web::body::Body;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot found dependency: {0}")]
    DependencyNotFound(&'static str),
}

impl ResponseError for Error {}