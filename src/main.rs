/*
use std::net::TcpStream;
//use std::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::io;
use std::io::Write;
use std::thread;mod router;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn handle_client(mut stream : TcpStream) -> io::Result<()> {
    write!(stream, "Hello, world!")?;
    Ok(())
}

#[tokio::main]
pub async fn main(address: "127.0.0.1") -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(address).await?;

    loop {
        let ( mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
*/

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7070").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to write to socket; err = {:?}", e);
                        return;
                    }
                };
                let response = "Hello, World!";
                return;
            }
        });
    }
}
