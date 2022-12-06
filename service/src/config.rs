use std::str::FromStr;

use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Redis {
    pub host: String,
    pub channel: String,
}

#[derive(Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u64,
    pub user: String,
    pub pass: String,
}

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u64,
}

#[derive(Deserialize)]
pub struct Config {
    redis: Redis,
    database: Database,
    server: Server,
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded: Config = toml::from_str(s)?;
        Ok(decoded)
    }
}

impl Config {
    pub fn redis(&self) -> &Redis {
        &self.redis
    }

    pub fn db(&self) -> &Database {
        &self.database
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}
