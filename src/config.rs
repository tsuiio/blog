use std::{env, fs, path::Path};

use once_cell::sync::Lazy;
use serde::Deserialize;
use tracing::Level;

use crate::{error::BlogError, utils::URLEncode};

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let path_s = env::var("BLOG_CONFIG").unwrap_or(String::from("blog.toml"));
    let path = Path::new(&path_s);

    match Config::from_file(&path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!(
                "Failed to load configuration with path({:?}): {}",
                path_s, e
            );
            std::process::exit(1);
        }
    }
});

#[derive(Deserialize)]
pub struct Config {
    pub web: Web,
    pub db: Db,
    pub log: LogLevel,
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
    pub max_size: u32,
}

#[derive(Deserialize)]
pub struct LogLevel {
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
            protocol,
            self.db.user.encode(),
            self.db.passwd.encode(),
            self.db.host,
            self.db.port,
            self.db.name
        )
    }

    pub fn log_level(&self) -> Level {
        (&self.log.level).into()
    }
}
