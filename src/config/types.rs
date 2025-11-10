use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    #[serde(default)]
    pub flood: Option<FloodConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub listen_address: String,
    pub mode: Mode,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Echo,
    Flood,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FloodConfig {
    pub target_address: String,
    #[serde(default = "default_packet_size")]
    pub packet_size: usize,
    #[serde(default = "default_fill_byte")]
    pub fill_byte: u8,
}

fn default_packet_size() -> usize {
    5120
}

fn default_fill_byte() -> u8 {
    0xFF
}

impl FloodConfig {
    pub fn target_addr(&self) -> Result<SocketAddr, std::io::Error> {
        self.target_address
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
    }
}
