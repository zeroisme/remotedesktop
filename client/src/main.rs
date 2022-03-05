use bytes::BytesMut;
use common::{ProstClientStream, Image, image::Type, image_exclusive};
use tokio::net::TcpStream;
use fltk::{app, prelude::*, window::Window, frame::Frame, image as fltk_image};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::default().with_size(1920, 1080);
    let mut frame = Frame::default().size_of(&wind);
    let (tx, rx) = app::channel::<Image>();
    // 异步获取图片流，通过channel发送给fltk
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
            let mut prev_image = None;
            loop {
                let mut pb_image = stream.recv().await.unwrap();
                let mut img = BytesMut::new();
                if pb_image.typ == Type::Exclusive.into() {
                    // 还原图片
                    if let Some(ref prev) = prev_image {
                        image_exclusive(prev, &pb_image.data, &mut img);
                        let data = img.freeze();
                        prev_image = Some(data.clone());
                        pb_image.data = data;
                    } else {
                        println!("无法还原图片，前置图片缺失");
                    }

                }
                tx.send(pb_image);
            }
        })
    });

    wind.make_resizable(true);
    wind.end();
    wind.show();
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
