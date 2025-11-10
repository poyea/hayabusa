use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

use crate::sender::spawn_sender;

pub async fn run(listen_addr: &str) -> io::Result<()> {
    let socket = UdpSocket::bind(listen_addr.parse::<SocketAddr>().unwrap()).await?;
    let r = Arc::new(socket);
    let tx = spawn_sender(r.clone()).await;

    println!("Echo mode: Listening at {}", listen_addr);

    loop {
        let mut buf = [0; 65536];
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        // Echo
        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}
