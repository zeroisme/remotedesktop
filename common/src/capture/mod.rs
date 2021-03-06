use bytes::BytesMut;
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;
unsafe impl Send for Cap {}

pub struct Cap {
    w: usize,
    h: usize,
    capturer: Capturer,
}

impl Cap {
    pub fn new() -> Self {
        let display = Display::primary().unwrap();
        let capturer = Capturer::new(display).unwrap();
        let w = capturer.width();
        let h = capturer.height();
        println!("w: {}, h: {}", w, h);
        Self { w: w, h: h, capturer: capturer }
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn capture(&mut self, buf: &mut BytesMut) {
        let one_second = Duration::new(1, 0);
        // TODO: 让帧率可以配置
        let one_frame = one_second / 20;
        loop {
            let frame = self.capturer.frame();
            let buffer = match frame {
                Ok(buffer) => buffer,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(one_frame);
                        continue;
                    }
                    else {
                        panic!("Error: {}", error);
                    }
                }
            };
            // BGRA to RGB
            let mut n = 0;
            while n < buffer.len() {
                buf.extend_from_slice(&[
                    buffer[n + 2] & 0b1111_1000,
                    buffer[n + 1] & 0b1111_1000,
                    buffer[n] & 0b1111_1000,
                ]);
                n += 4;
            }
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image;

    #[test]
    fn test_capture_image() {
        let mut cap = Cap::new();
        let w = cap.width();
        let h = cap.height();
        let mut buf = BytesMut::new();
        cap.capture(&mut buf);


        let mut imgbuf = image::ImageBuffer::new(w as u32, h as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let stride = (y as u32 * w as u32 + x) * 3;
            let r: u8 = buf[(stride) as usize];
            let g: u8 = buf[(stride + 1) as usize];
            let b: u8 = buf[(stride + 2) as usize];
            *pixel = image::Rgb([r, g, b]);
        }
        imgbuf.save("test.png").unwrap();
    }
    
}
