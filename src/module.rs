use std::any::{Any, type_name, TypeId};
use std::future::Future;
use std::sync::Arc;

use actix_web::web::ServiceConfig;
use ahash::AHashMap;
use anyhow::Result;

/// 应用模块注册器 trait
pub trait ApplicationModuleRegister: Sync + Send + Any {
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
    pub fn boxed(obj: T) -> Box<dyn ApplicationModuleRegister> {
        Box::new(StandardModuleRegister(obj))
    }
}

impl<T> ApplicationModuleRegister for StandardModuleRegister<T>
    where T: Send + Sync + Clone + 'static
{
    fn register(&self, service: &mut ServiceConfig) {
        service.data(self.0.clone());
    }

    fn get_module(&self) -> Box<dyn Any> {
        Box::new(self.0.clone())
    }
}

/// Actix Web 应用模块管理器
///
/// 应用模块管理器是用于传递应用模块的一个容器。
#[derive(Clone)]
pub struct ApplicationModuleManager(Arc<Vec<Box<dyn ApplicationModuleRegister>>>);

impl ApplicationModuleManager {
    pub fn new(inner: Vec<Box<dyn ApplicationModuleRegister>>) -> Self {
        ApplicationModuleManager(Arc::new(inner))
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

pub struct ModuleProvider(AHashMap<TypeId, Box<dyn Any>>, Vec<Box<dyn ApplicationModuleRegister>>);

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

    pub fn remove<T>(&mut self) -> Option<T>
        where T: Send + Sync + Clone + 'static
    {
        self.0
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast().ok().map(|inner| *inner))
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub async fn register<T, F>(&mut self, factory: F) -> Result<()>
        where
                for<'a> F: Wrapped<'a, T>,
                T: Send + Sync + Clone + 'static,
    {
        let result = factory.call(self).await?;
        self.insert(result);
        Ok(())
    }

    pub fn into_application_modules(self) -> ApplicationModuleManager {
        ApplicationModuleManager::new(self.1)
    }
}

pub trait Wrapped<'a, T>
    where T: Send + Sync + Clone + 'static,
{
    type Res: Future<Output=Result<T>>;
    fn call(self, s: &'a ModuleProvider) -> Self::Res;
}

impl<'a, T, F, R> Wrapped<'a, T> for F
    where F: Fn(&'a ModuleProvider) -> R,
          R: Future<Output=Result<T>> + 'a,
          T: Send + Sync + Clone + 'static,
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
}