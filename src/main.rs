use async_std::task::sleep;
use futures::{
    future::select,
    FutureExt, StreamExt,
};
use std::{
    net::SocketAddr,
    str::FromStr,
    thread,
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    runtime::Builder,
};

pub async fn server_loop() {
    let mut listener = TcpListener::bind(SocketAddr::from_str("127.0.0.1:9000").unwrap()).await.unwrap();
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                continue;
            },
        };
        let peer_addr = stream.peer_addr().unwrap();
        let (read, mut write) = stream.into_split();
        let read_loop = async move {
            let mut read = BufReader::new(read);
            let mut buffer = String::with_capacity(8096);
            loop {
                match read.read_line(&mut buffer).await {
                    Ok(read) => if read > 0 && buffer.len() > 0 {
                        println!("{}", buffer);
                        buffer.clear();
                    } else if read == 0 {
                        println!("{} disconnected", peer_addr);
                        break;
                    },
                    Err(e) => {
                        println!("{} disconnected, error {}", peer_addr, e);
                        break;
                    }
                }
            }
        };
        let write_loop = async move {
            loop {
                match write.write_all(b"hello\n").await {
                    Ok(_) => (),
                    Err(_) => break,
                };
                sleep(Duration::from_secs(5)).await;
            }
        };
        // selecting over the read and write parts processing loops in order to
        // drop both parts and close connection in case of errors
        tokio::spawn(select(Box::pin(read_loop), Box::pin(write_loop)));
    }
}

pub async fn client_loop() {
    let str = b"Hello\n ";
    for _ in 0..2 {
        let mut stream = TcpStream::connect(SocketAddr::from_str("127.0.0.1:9000").unwrap()).await.unwrap();
        stream.write_all(str).await.unwrap();
        // uncomment the following to avoid panic
        /*
        let mut stream = BufReader::new(stream);
        let mut buffer = String::with_capacity(1024);
        stream.read_line(&mut buffer).await.unwrap();
        println!("{}", buffer);
        */
        sleep(Duration::from_millis(500)).await;
    }
}

fn main() {
    // let mut runtime = tokio::runtime::Runtime::new().unwrap();
    let mut runtime = Builder::new().basic_scheduler().enable_all().build().unwrap();
    runtime.block_on(select(Box::pin(server_loop()), Box::pin(client_loop())));
}
