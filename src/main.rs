use clap::Parser;
use std::io;

mod config;
mod sender;
mod server;

use config::{load_config, Mode};

#[derive(Parser, Debug)]
#[command(name = "hayabusa")]
#[command(about = "Lightweight UDP packet sender", long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let config = load_config(&args.config)?;

    match config.server.mode {
        Mode::Echo => server::run_echo(&config.server.listen_address).await,
        Mode::Flood => {
            let flood_config = config
                .flood
                .expect("Flood mode requires [flood] configuration");
            server::run_flood(&config.server.listen_address, flood_config).await
        }
    }
}
