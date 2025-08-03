use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    max_connections: u32,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
struct FeaturesConfig {
    logging: bool,
    caching: bool,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    address: String,
    port: u16,
    timeout_seconds: u8,
}

#[derive(Debug, Deserialize)]
struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
    features: FeaturesConfig,
}

fn read_config() -> anyhow::Result<Config> {
    let config_content = fs::read_to_string("config/config.toml")?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let config = read_config()?;
    println!("config: {:#?}", config);
    Ok(())
}
