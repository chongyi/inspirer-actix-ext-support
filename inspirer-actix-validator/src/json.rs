use actix_web::{FromRequest, HttpRequest, Error};
use crate::{Validated, Validate};
use actix_web::web::Json;
use std::future::Future;
use actix_web::dev::Payload;
use serde::de::DeserializeOwned;
use futures::future::{LocalBoxFuture, FutureExt};
use futures::TryFutureExt;

impl<T> FromRequest for Validated<Json<T>>
    where T: DeserializeOwned + Validate + 'static
{
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Error>>;
    type Config = <Json<T> as FromRequest>::Config;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        Json::<T>::from_request(req, payload);

        todo!()
    }
}