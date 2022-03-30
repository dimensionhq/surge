use serde::{Deserialize, Serialize};

/// This is the config of the toml file present in the config directory
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host_port: u16,
    pub name: String,
    pub storage_port: u16,
}
