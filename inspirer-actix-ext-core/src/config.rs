pub use config::*;
use crate::module::ModuleProvider;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ConfigProvider {
    Path(PathBuf),
    String(String),
    Env(String),
}

impl fmt::Display for ConfigProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigProvider::Path(path_buf) => writeln!(f, "{:?}", path_buf),
            ConfigProvider::String(string) => writeln!(f, "{}", string),
            ConfigProvider::Env(prefix) => writeln!(f, "Environment prefix = {}", prefix),
        }
    }
}

pub fn config_provider(config_providers: Vec<ConfigProvider>) -> impl Fn(&ModuleProvider) -> Pin<Box<dyn Future<Output=Result<Config, ConfigError>>>>
{
    move |_| {
        debug!("Register configuration manager module.");
        let config_files = config_providers.clone();
        Box::pin(async move {
            let mut config = Config::new();

            for config_file in config_files.iter() {
                debug!("Load config file: {}", config_file);
                match config_file {
                    ConfigProvider::Path(path) => config.merge(File::from(path.as_path())),
                    ConfigProvider::String(name) => config.merge(File::with_name(name.as_str())),
                    ConfigProvider::Env(prefix) => config.merge(Environment::with_prefix(prefix.as_str())),
                }.map_err(|err| {
                    error!("Load config file error: {}", err);
                    err
                })?;
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