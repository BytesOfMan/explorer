use std::net::SocketAddr;

use bitcoinsuite_error::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: SocketAddr,
    pub chronik_api_url: String,
}

pub fn load_config(config_string: &str) -> Result<Config> {
    let config: Config = toml::from_str(config_string).unwrap();
    Ok(config)
}
