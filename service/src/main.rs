use std::{env, fs, str::FromStr};

use anyhow::Result;
use config::Config;

mod config;
mod database;

fn main() -> Result<()> {
    let mut cfgpath = String::from("config.toml");
    if env::args().len() > 1 {
        let args: Vec<String> = env::args().collect();
        cfgpath = String::from(&args[1]);
    }

    // Parse config file
    let toml = fs::read_to_string(cfgpath)?;
    let cfg = Config::from_str(&toml)?;

    Ok(())
}
