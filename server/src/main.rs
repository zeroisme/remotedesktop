use std::io::Result;
use tokio::net::{TcpListener, TcpStream};
use common::{Image, ProstServerStream, Cap, image_exclusive, image::{Type, Mode}};

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
    let mut prev_img = None;
    loop {
        let mut capture_img = BytesMut::new();
        cap.capture(&mut capture_img);
        let mut img = BytesMut::new();
        let mode = Mode::Rgb;
        let mut typ = Type::Nomal;
        if let Some(ref prev) = prev_img {
            let capture_img = capture_img.freeze();
            if prev == &capture_img {
                continue;
            }
            image_exclusive(prev, &capture_img, &mut img);
            prev_img = Some(capture_img);
            typ = Type::Exclusive;
        } else {
            img = capture_img;
        }

        let pb_image = Image::new(cap.width(), cap.height(), 
            typ, mode, img.freeze());
        stream.send(pb_image).await.unwrap();
    }
}