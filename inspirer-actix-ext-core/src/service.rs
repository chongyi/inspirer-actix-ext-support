//! 应用服务层组件
//!
//! 绝大部分 web 应用需要抽象服务层，对此进行了相关封装，便于构建服务层。
//!
//! 例如：
//!
//! ```
//! use sqlx::MySqlPool;
//! use inspirer_actix_ext_core::service::{IntoService, Service};
//!
//! pub struct DemoService (MySqlPool);
//!
//! impl IntoService<(MySqlPool, )> for DemoService {
//!     fn init(deps: (MySqlPool, )) -> Self {
//!         DemoService (deps.0)
//!     }
//! }
//!
//! #[get("/")]
//! async fn handler(srv: Service) {
//!     let demo_service = src.get::<DemoService>();
//!
//!     // 使用 demo_service 的方法
//! }
//! ```


use std::any::type_name;

use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use futures::future::{ok, Ready};

use crate::error::Error;

/// 应用 Service 层提供者
pub struct Service (HttpRequest);

impl Service {
    pub fn get<D, S: DependencyFactory<D>>(&self) -> Result<S, Error> {
        S::make(&self.0)
    }
}

impl FromRequest for Service
{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, actix_web::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ok(Service(req.clone()))
    }
}

pub trait IntoService<T> {
    /// 服务模块初始化方法
    fn init(deps: T) -> Self;
}

/// 依赖工厂
pub trait DependencyFactory<D> {
    fn make(req: &HttpRequest) -> Result<Self, Error> where Self: Sized;
}

macro_rules! factory_tuple {
    ($($T:ident),+) => {
        impl<S, $($T,)+> DependencyFactory<($($T,)+)> for S
        where S: IntoService<($($T,)+)>,
            $($T: Clone + 'static,)+
        {
            fn make(req: &HttpRequest) -> Result<Self, Error> {
                let deps = (
                    $(
                        req.app_data::<Data<$T>>()
                            .map(|c| c.get_ref().clone())
                            .ok_or(Error::DependencyNotFound(type_name::<$T>()))?,
                    )+
                );

                Ok(S::init(deps))
            }
        }
    };
}

impl<S> DependencyFactory<()> for S
    where S: IntoService<()>
{
    fn make(_req: &HttpRequest) -> Result<Self, Error> {
        Ok(S::init(()))
    }
}

factory_tuple!(A);
factory_tuple!(A, B);
factory_tuple!(A, B, C);
factory_tuple!(A, B, C, D);
factory_tuple!(A, B, C, D, E);
factory_tuple!(A, B, C, D, E, F);
factory_tuple!(A, B, C, D, E, F, G);
factory_tuple!(A, B, C, D, E, F, G, H);
factory_tuple!(A, B, C, D, E, F, G, H, I);