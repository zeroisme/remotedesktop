use std::io::Result;
use tokio::net::{TcpListener, TcpStream};
use common::{Image, ProstServerStream, Cap};

use bytes::BytesMut;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await.unwrap();
    
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}

async fn handle_client(socket: TcpStream) {
    let mut cap = Cap::new();
    let mut stream = ProstServerStream::new(socket);
    loop {
        let mut img = BytesMut::new();
        cap.capture(&mut img);

        let pb_image = Image::new(cap.width(), cap.height(), img.freeze());
        stream.send(pb_image).await.unwrap();
    }
}