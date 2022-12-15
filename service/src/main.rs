use std::{env, fs, str::FromStr};

use anyhow::Result;
use app::App;
use config::Config;

mod app;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let mut cfgpath = String::from("config.toml");
    if env::args().len() > 1 {
        let args: Vec<String> = env::args().collect();
        cfgpath = String::from(&args[1]);
    }

    // Parse config file
    let toml = fs::read_to_string(cfgpath)?;
    let cfg = Config::from_str(&toml)?;

    let app = App::new(cfg).await?;
    app.run().await?;

    Ok(())
}
