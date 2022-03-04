use std::{sync::Arc, time::Duration};

use bytes::BytesMut;
use common::{ProstClientStream, Image};
use tokio::net::TcpStream;
mod ui;
use fltk::{app, prelude::*, window::Window, frame::Frame, image as fltk_image, enums::Event};

fn main() {
    let mut buf = vec![0;2880*1800*4];
    let buf = Arc::new(std::sync::RwLock::new(buf));
    let mut buf1 = buf.clone();
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::default().with_size(1920, 1080);
    let mut frame = Frame::default().size_of(&wind);
    let (tx, rx) = app::channel::<Image>();

    std::thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("fetch")
            .worker_threads(1)
            .build().unwrap();
        runtime.block_on(async move {
            let addr = "127.0.0.1:9527";
            let socket = TcpStream::connect(addr).await.unwrap();
            let mut stream = ProstClientStream::new(socket);
            loop {
                let pb_image = stream.recv().await.unwrap();
                tx.send(pb_image);
            }
        })
    });

    wind.make_resizable(true);
    wind.end();
    wind.show();
    // app.run().unwrap();
    while app.wait() {
        if let Some(image) = rx.recv() {
            if let Ok(mut img) = unsafe { fltk_image::RgbImage::from_data(&image.data, 
                image.width as i32, image.height as i32, fltk::enums::ColorDepth::Rgb8) } {
                img.scale(frame.width(), frame.height(), false, true);

                frame.set_image(Some(img));
                frame.redraw();
            }
        }
    }
}
