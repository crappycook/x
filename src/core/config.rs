use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub directory: String,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/dev"))
            .build()?;

        config.try_deserialize()
    }
}
