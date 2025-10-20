use std::sync::LazyLock;

use chrono_tz::Tz;
use client_config::ClientSettings;
use config::{Config, ConfigError};
use dotenv::dotenv;
use serde::Deserialize;
use sniper_confg::SniperSettings;

mod client_config;
mod sniper_confg;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub client: ClientSettings,
    pub sniper: SniperSettings,
    pub timezone: Tz,
}

impl Settings {
    fn load() -> Result<Self, ConfigError> {
        dotenv().ok();
        let builder = Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .add_source(
                config::File::with_name("config")
                    .format(config::FileFormat::Toml)
                    .required(true),
            );
        let settings: Settings = builder.build()?.try_deserialize()?;
        Ok(settings)
    }
}

pub static SETTINGS: LazyLock<Settings> =
    LazyLock::new(|| Settings::load().expect("Invalid config"));
