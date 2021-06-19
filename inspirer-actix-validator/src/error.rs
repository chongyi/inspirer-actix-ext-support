use validator::ValidationError;
use actix_web::error::{JsonPayloadError, QueryPayloadError, UrlencodedError, PathError};
use actix_web::ResponseError;
use actix_web::http::StatusCode;
use std::fmt;
use std::fmt::{Formatter, Result};

#[derive(Debug)]
pub enum Error {
    ValidatorError(ValidationError),
    JsonPayloadError(JsonPayloadError),
    QueryPayloadError(QueryPayloadError),
    UrlencodedError(UrlencodedError),
    PathError(PathError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::ValidatorError(err) => writeln!(f, "{}", err),
            Error::JsonPayloadError(err) => writeln!(f, "{}", err),
            Error::QueryPayloadError(err) => writeln!(f, "{}", err),
            Error::UrlencodedError(err) => writeln!(f, "{}", err),
            Error::PathError(err) => writeln!(f, "{}", err),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}