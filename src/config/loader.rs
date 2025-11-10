use std::fs;
use std::io;

use super::types::Config;

pub fn load_config(path: &str) -> io::Result<Config> {
    let content = fs::read_to_string(path)?;
    toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
