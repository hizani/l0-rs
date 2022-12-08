use std::str::FromStr;

use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    redis: Redis,
    database: Database,
    server: Server,
}

impl Config {
    pub fn redis(&self) -> &Redis {
        &self.redis
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}

#[derive(Deserialize)]
pub struct Redis {
    host: String,
    channel: String,
}

impl Redis {
    pub fn channel(&self) -> &str {
        self.channel.as_ref()
    }

    pub fn host(&self) -> &str {
        self.host.as_ref()
    }
}

#[derive(Deserialize)]
pub struct Database {
    host: String,
    port: u64,
    user: String,
    pass: String,
    db: String,
}

impl Database {
    pub fn host(&self) -> &str {
        self.host.as_ref()
    }

    pub fn port(&self) -> u64 {
        self.port
    }

    pub fn user(&self) -> &str {
        self.user.as_ref()
    }

    pub fn pass(&self) -> &str {
        self.pass.as_ref()
    }

    pub fn db(&self) -> &str {
        self.db.as_ref()
    }
}

#[derive(Deserialize)]
pub struct Server {
    host: String,
    port: u64,
}

impl Server {
    pub fn host(&self) -> &str {
        self.host.as_ref()
    }

    pub fn port(&self) -> u64 {
        self.port
    }
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded: Config = toml::from_str(s)?;
        Ok(decoded)
    }
}
