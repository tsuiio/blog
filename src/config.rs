use serde::Deserialize;
use tracing::Level;

#[derive(Deserialize)]
pub struct Config {
    pub web: Web,
    pub db: Db,
    pub log: Log,
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
    pub name: String,
    pub user: String,
    pub passwd: String,
}

#[derive(Deserialize)]
pub enum Log {
    Info,
    Warn,
    Debug,
    Error,
    Tarce,
}

impl From<Log> for Level {
    fn from(log: Log) -> Self {
        match log {
            Log::Info => Level::INFO,
            Log::Warn => Level::WARN,
            Log::Debug => Level::DEBUG,
            Log::Error => Level::ERROR,
            Log::Tarce => Level::TRACE,
        }
    }
}

impl Config {
    pub fn from_config(&self) -> &Self {
        todo!()
    }

    pub fn db_url(&self) -> String {
        todo!()
    }

    pub fn log_level(self) -> Level {
        self.log.into()
    }
}
