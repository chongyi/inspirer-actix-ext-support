pub use config::*;
use crate::module::ModuleProvider;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;

pub fn config_provider(config_files: Vec<String>) -> impl Fn(&ModuleProvider) -> Pin<Box<dyn Future<Output=Result<Config, ConfigError>>>>
{
    move |module_provider| {
        let config_files = config_files.clone();
        Box::pin(async move {
            let mut config = Config::new();

            for config_file in config_files.iter() {
                config.merge(File::from(Path::new(config_file)));
            }

            Ok(config)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::module::ModuleProvider;
    use crate::config::config_provider;
    use config::Config;

    #[tokio::test]
    async fn test_provider() {
        let mut module_provider = ModuleProvider::new();
        assert!(module_provider.register(config_provider(vec![])).await.is_ok());
        assert!(module_provider.contains::<Config>());
    }
}