use validator::ValidationErrors;
use actix_web::error::{JsonPayloadError, QueryPayloadError, UrlencodedError, PathError};
use actix_web::ResponseError;
use actix_web::http::StatusCode;
use std::fmt;
use std::fmt::{Formatter, Result};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Error (ValidationErrors);

impl From<ValidationErrors> for Error {
    fn from(err: ValidationErrors) -> Self {
        Error(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{}", self.0)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl Deref for Error {
    type Target = ValidationErrors;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}