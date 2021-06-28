#[macro_use]
extern crate serde;
#[macro_use]
extern crate validator;

use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::{Form, Json, Path, Query};
use futures::future::{err, Future, FutureExt, LocalBoxFuture, ok, TryFutureExt};
use serde::de::DeserializeOwned;
use serde_qs::actix::QsQuery;
pub use validator::*;

pub use crate::error::Error;

pub mod error;

pub struct Validated<T>(pub T);

impl<T> fmt::Debug for Validated<T>
    where
        T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T> fmt::Display for Validated<T>
    where
        T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Default)]
pub struct ValidateConfig {
    error_handler: Option<Arc<dyn Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync>>,
}

const DEFAULT_CONFIG: ValidateConfig = ValidateConfig {
    error_handler: None
};

impl ValidateConfig {
    pub fn error_handler<F>(mut self, f: F) -> Self
        where F: Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync + 'static,
    {
        self.error_handler = Some(Arc::new(f));
        self
    }

    pub fn from_req(req: &HttpRequest) -> &Self {
        req.app_data::<Self>()
            .or_else(|| req.app_data::<actix_web::web::Data<Self>>().map(|d| d.as_ref()))
            .unwrap_or(&DEFAULT_CONFIG)
    }
}

macro_rules! validator {
    ($source:ident) => {
        impl<T> Deref for Validated<$source<T>> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0.deref()
            }
        }

        impl<T> DerefMut for Validated<$source<T>> {
            fn deref_mut(&mut self) -> &mut T {
                (&mut self.0).deref_mut()
            }
        }

        impl<T> FromRequest for Validated<$source<T>>
            where
                T: DeserializeOwned + Validate + 'static,
        {
            type Error = actix_web::Error;
            type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
            type Config = ValidateConfig;

            fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
                let req2 = req.clone();
                $source::<T>::from_request(req, payload)
                    .and_then(move |res| {
                        let result = res.validate().map_err(|err| {
                            let config = ValidateConfig::from_req(&req2);
                            let wrapped_err = Error::from(err);
                            match &config.error_handler {
                                Some(error_handler) => (*error_handler)(wrapped_err, &req2),
                                None => actix_web::Error::from(wrapped_err),
                            }
                        });

                        match result {
                            Ok(_) => ok(Validated(res)),
                            Err(e) => err(e),
                        }
                    })
                    .boxed_local()
            }
        }
    };
}

validator!(Json);
validator!(Query);
validator!(Path);
validator!(Form);
validator!(QsQuery);