use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub directory: String,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/dev"))
            .build()?;

        let mut app_config: AppConfig = config.try_deserialize()?;

        // Expand $HOME in the database URL
        if app_config.database.url.contains("$HOME") {
            let home = env::var("HOME").expect("HOME environment variable not set");
            app_config.database.url = app_config.database.url.replace("$HOME", &home);
        }

        Ok(app_config)
    }
}
