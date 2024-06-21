use std::path::Path;

use clap::{Args, Parser, ValueEnum};
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::Level;

use crate::{cli::Cli, error::BlogError, utils::URLEncode};

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let args = Cli::parse();
    let conf = args.config;
    let path = args.conf_path;

    match Config::figment(&path, conf) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    }
});

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct Config {
    #[clap(flatten)]
    pub web: Web,
    #[clap(flatten)]
    pub db: Db,
    #[clap(flatten)]
    pub log: LogLevel,
}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct Web {
    #[clap(long = "web-host")]
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_host: Option<String>,
    #[clap(long = "web-port")]
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_port: Option<u32>,
    #[clap(long = "jwt-secret")]
    #[serde(rename = "jwtsecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_secret: Option<String>,
    #[clap(long = "jwt-exp")]
    #[serde(rename = "jwtexp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_exp: Option<u64>,
}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct Db {
    #[clap(long = "db-host")]
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_host: Option<String>,
    #[clap(long = "db-port")]
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_port: Option<u32>,
    #[clap(long = "db-ssl")]
    #[serde(rename = "ssl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_ssl: Option<bool>,
    #[clap(long = "db-name")]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    #[clap(long = "db-user")]
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[clap(long = "db-passwd")]
    #[serde(rename = "passwd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_passwd: Option<String>,
    #[clap(long = "db-max-size")]
    #[serde(rename = "maxsize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u32>,
}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct LogLevel {
    #[clap(long = "log-level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Log>,
}

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
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
    pub fn figment(path: &Path, conf: Config) -> Result<Self, BlogError> {
        dotenvy::dotenv()?;

        let config = Figment::new()
            .merge(Toml::file(path))
            .merge(Env::prefixed("TSUIIO_BLOG_").split('_'))
            .merge(Serialized::defaults(conf))
            .extract()?;

        Ok(config)
    }

    pub fn listener_host(&self) -> String {
        format!(
            "{}:{}",
            self.web.web_host.as_ref().unwrap(),
            self.web.web_port.unwrap()
        )
    }

    pub fn db_url(&self) -> String {
        let ssl_param = if self.db.db_ssl.unwrap() {
            "?sslmode=require"
        } else {
            ""
        };
        format!(
            "postgres://{}:{}@{}:{}/{}{}",
            self.db.db_user.as_ref().unwrap().encode(),
            self.db.db_passwd.as_ref().unwrap().encode(),
            self.db.db_host.as_ref().unwrap(),
            self.db.db_port.as_ref().unwrap(),
            self.db.db_name.as_ref().unwrap(),
            ssl_param
        )
    }

    pub fn log_level(&self) -> Level {
        self.log.level.as_ref().unwrap().into()
    }
}
