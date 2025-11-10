use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

use crate::config::FloodConfig;
use crate::sender::spawn_sender;

pub async fn run(listen_addr: &str, flood_config: FloodConfig) -> io::Result<()> {
    let socket = UdpSocket::bind(listen_addr.parse::<SocketAddr>().unwrap()).await?;
    let r = Arc::new(socket);
    let tx = spawn_sender(r.clone()).await;

    let target = flood_config.target_addr()?;
    let flood_buf = vec![flood_config.fill_byte; flood_config.packet_size];

    println!("Flood mode: Listening at {}", listen_addr);
    println!(
        "Flooding target: {} with {} byte packets",
        target, flood_config.packet_size
    );

    // Flood
    loop {
        tx.send((flood_buf.clone(), target)).await.unwrap();
    }
}
