//! 模块组件扩展支持
//!
//!
//! 通过模块扩展注册器进行模块注册我们用于 Actix web 使用的组件。
//!
//! ```
//! use inspirer_actix_ext_core::module::ModuleProvider;
//!
//! let mut module_provider = ModuleProvider::new();
//! module_provider.insert(1);
//! module_provider.insert(Vec::new());
//! ```
//!
//! 当然我们也可以使用工厂方法，这样更为灵活。同时工厂方法会接收模块注册器
//! 的引用，便于我们在某些时候获得相关依赖，例如获取建立数据库连接时必要的配置信息。
//!
//! ```
//! use inspirer_actix_ext_core::module::ModuleProvider;
//! use std::io::Result;
//!
//! async fn database_conn_factory(ctx: &ModuleProvider) -> Result<&'static str> {
//!     // 假设是获取配置文本
//!     let _config = ctx.get_ref::<String>();
//!
//!     // 假设这里返回数据库链接
//!     Ok("database conn")
//! }
//!
//! let mut module_provider = ModuleProvider::new();
//! module_provider.register(database_conn_factory);
//! ```

use std::any::{Any, type_name, TypeId};
use std::future::Future;
use std::sync::Arc;

use actix_web::web::ServiceConfig;
use ahash::AHashMap;

/// 应用模块注册器 trait
pub trait ModuleRegister: Sync + Send + Any {
    /// 注册应用模块
    ///
    /// Actix web 框架是通过 ServiceConfig 注册应用内的模块，
    /// 可以通过该方法实现进行向其注册的逻辑
    fn register(&self, service: &mut ServiceConfig);

    /// 获取模块
    fn get_module(&self) -> Box<dyn Any>;
}

/// 标准应用模块注册器
///
/// 可通过这个简单快速实现模块注册，其实现的 `register` 方法仅提供了纯粹的写入逻辑，
/// 不会有更多的前置后置逻辑在内。对于大多数情况基本适用。
pub struct StandardModuleRegister<T>(pub T);

impl<T> StandardModuleRegister<T>
    where T: Send + Sync + Clone + 'static
{
    pub fn boxed(obj: T) -> Box<dyn ModuleRegister> {
        Box::new(StandardModuleRegister(obj))
    }
}

impl<T> ModuleRegister for StandardModuleRegister<T>
    where T: Send + Sync + Clone + 'static
{
    fn register(&self, service: &mut ServiceConfig) {
        service.data(self.0.clone());
    }

    fn get_module(&self) -> Box<dyn Any> {
        Box::new(self.0.clone())
    }
}

/// Actix Web 应用模块容器
///
/// 应用模块管理器是用于传递应用模块的一个容器。
#[derive(Clone)]
pub struct ModuleContainer(Arc<Vec<Box<dyn ModuleRegister>>>);

impl ModuleContainer {
    pub fn new(inner: Vec<Box<dyn ModuleRegister>>) -> Self {
        ModuleContainer(Arc::new(inner))
    }

    /// 获取模块提供者
    ///
    /// 这个方法可作为 actix web 中 App 的 `configure` 方法的参数提供。
    pub fn module_provider(&self) -> Box<dyn FnOnce(&mut ServiceConfig)> {
        let registers = self.0.clone();
        Box::new(move |srv: &mut ServiceConfig| {
            info!("Configure application service, [{}] modules provide.", registers.len());

            for module_register in registers.iter() {
                module_register.register(srv);
            }
        })
    }
}

#[derive(Clone)]
struct Module<T: Send + Sync + Clone>(pub T);

pub struct ModuleProvider(AHashMap<TypeId, Box<dyn Any>>, Vec<Box<dyn ModuleRegister>>);

impl ModuleProvider {
    pub fn new() -> Self {
        ModuleProvider(AHashMap::new(), vec![])
    }

    pub fn initialize<T>(init_obj: T) -> Self
        where T: Send + Sync + Clone + 'static
    {
        let mut module_provider = ModuleProvider::new();
        module_provider.insert(init_obj);
        module_provider
    }

    pub fn insert<T>(&mut self, obj: T)
        where T: Send + Sync + Clone + 'static
    {
        info!("Register module [{}]", type_name::<T>());
        self.0.insert(TypeId::of::<T>(), Box::new(Module(obj.clone())));
        self.1.push(StandardModuleRegister::boxed(obj));
    }

    pub fn get<T>(&self) -> Option<T>
        where T: Send + Sync + Clone + 'static
    {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<Module<T>>())
            .map(|obj| obj.0.clone())
    }

    pub fn get_ref<T>(&self) -> Option<&T>
        where T: Send + Sync + Clone + 'static
    {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<Module<T>>())
            .map(|obj| &obj.0)
    }

    pub fn contains<T>(&self) -> bool
        where T: Send + Sync + Clone + 'static
    {
        self.0
            .contains_key(&TypeId::of::<T>())
    }

    pub fn clear(&mut self) {
        self.0.clear();
        self.1.clear();
    }

    pub async fn register<T, F, E>(&mut self, factory: F) -> anyhow::Result<()>
        where
                for<'a> F: ModuleFactoryFn<'a, T, E>,
                T: Send + Sync + Clone + 'static,
                E: std::error::Error + Send + Sync + 'static,
    {
        let result = factory.call(self).await?;
        self.insert(result);
        Ok(())
    }

    pub fn into_module_container(self) -> ModuleContainer {
        ModuleContainer::new(self.1)
    }
}

pub trait ModuleFactoryFn<'a, T, E>
    where T: Send + Sync + Clone + 'static,
          E: std::error::Error + Send + Sync
{
    type Res: Future<Output=Result<T, E>>;
    fn call(self, s: &'a ModuleProvider) -> Self::Res;
}

impl<'a, T, F, E, R> ModuleFactoryFn<'a, T, E> for F
    where F: Fn(&'a ModuleProvider) -> R,
          R: Future<Output=Result<T, E>> + 'a,
          T: Send + Sync + Clone + 'static,
          E: std::error::Error + Send + Sync
{
    type Res = R;

    fn call(self, s: &'a ModuleProvider) -> Self::Res {
        self(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut module_provider = ModuleProvider::new();
        module_provider.insert(1u8);
        module_provider.insert(2u16);
        module_provider.insert(4u32);
        module_provider.insert(8u64);

        assert!(module_provider.contains::<u8>());
        assert!(module_provider.contains::<u16>());
        assert!(module_provider.contains::<u32>());
        assert!(module_provider.contains::<u64>());

        assert_eq!(1, module_provider.get::<u8>().unwrap());
        assert_eq!(2, module_provider.get::<u16>().unwrap());
        assert_eq!(4, module_provider.get::<u32>().unwrap());
        assert_eq!(8, module_provider.get::<u64>().unwrap());
    }

    #[test]
    fn test_clear() {
        let mut module_provider = ModuleProvider::new();
        module_provider.insert(1u8);
        module_provider.insert(2u16);
        module_provider.insert(4u32);

        assert!(module_provider.contains::<u8>());
        assert!(module_provider.contains::<u16>());
        assert!(module_provider.contains::<u32>());

        module_provider.clear();

        assert!(!module_provider.contains::<u8>());
        assert!(!module_provider.contains::<u16>());
        assert!(!module_provider.contains::<u32>());
    }

    #[tokio::test]
    async fn test_factory() {
        let mut module_provider = ModuleProvider::new();

        async fn register_u8(ctx: &ModuleProvider) -> std::io::Result<u8>{
            Ok(1)
        }

        async fn register_u16(ctx: &ModuleProvider) -> std::io::Result<u16>{
            Ok(2)
        }

        module_provider.register(register_u8).await.unwrap();
        module_provider.register(register_u16).await.unwrap();

        assert!(module_provider.contains::<u8>());
        assert!(module_provider.contains::<u16>());

        assert_eq!(1, module_provider.get::<u8>().unwrap());
        assert_eq!(2, module_provider.get::<u16>().unwrap());
    }
}