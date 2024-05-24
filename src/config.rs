use std::{
    env, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use once_cell::sync::Lazy;
use serde::Deserialize;
use tracing::{error, Level};

use crate::error::BlogError;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let path = env::var("BLOG_CONFIG").unwrap_or_else(|_| String::from("blog.toml"));
    let path = match PathBuf::from_str(&path) {
        Ok(path) => path,
        Err(e) => {
            error!("Failed to get path: {}", e);
            std::process::exit(1);
        }
    };

    match Config::from_file(&path) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    }
});

#[derive(Deserialize)]
pub struct Config {
    pub web: Web,
    pub db: Db,
    pub log: LogConfig,
}

#[derive(Deserialize)]
pub struct Web {
    pub host: String,
    pub port: u32,
    pub path: String,
}

#[derive(Deserialize)]
pub struct Db {
    pub host: String,
    pub port: u32,
    pub ssl: bool,
    pub name: String,
    pub user: String,
    pub passwd: String,
}

#[derive(Deserialize)]
pub struct LogConfig {
    pub level: Log,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Log {
    Info,
    Warn,
    Debug,
    Error,
    Trace,
}

impl From<&Log> for Level {
    fn from(log: &Log) -> Self {
        match log {
            Log::Info => Level::INFO,
            Log::Warn => Level::WARN,
            Log::Debug => Level::DEBUG,
            Log::Error => Level::ERROR,
            Log::Trace => Level::TRACE,
        }
    }
}

impl Config {
    pub fn from_file(file_path: &Path) -> Result<Self, BlogError> {
        let config_content = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }

    pub fn listener_host(&self) -> String {
        format!("{}:{}", self.web.host, self.web.port)
    }

    pub fn db_url(&self) -> String {
        let protocol = if self.db.ssl { "postgres" } else { "postgres" };
        format!(
            "{}://{}:{}@{}:{}/{}",
            protocol, self.db.user, self.db.passwd, self.db.host, self.db.port, self.db.name
        )
    }

    pub fn log_level(&self) -> Level {
        (&self.log.level).into()
    }
}
