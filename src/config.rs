use std::sync::LazyLock;

use anyhow::Context;
use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub credentials: Credentials,
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username: Option<String>,
    pub password: Option<String>,
}

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to initailize config"));

impl Credentials {
    pub fn username(&self) -> &str {
        self.username.as_deref().unwrap_or("202300555")
    }

    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("12345678")
    }
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let cfg = Config::builder()
            .add_source(
                config::File::with_name("config.toml")
                    .format(config::FileFormat::Toml)
                    .required(true),
            )
            .build()
            .with_context(|| anyhow::anyhow!("Failed to load config"))?;

        cfg.try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to deserialize config"))
    }

    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
