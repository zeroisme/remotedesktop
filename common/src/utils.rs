use bytes::{BytesMut, BufMut, Bytes};

pub fn image_exclusive(img1: &Bytes, img2: &Bytes, dst: &mut BytesMut) {
    if img1.len() != img2.len() { 
        println!("image1 and image2 size not the same!");
    }
    let n = img1.len();
    dst.reserve(n);
    for i in 0..n {
        dst.put_u8(img1[i] ^ img2[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{capture};

    #[test]
    fn test_image_exclusive() {
        let mut cap = capture::Cap::new();
        let mut img1 = BytesMut::new();
        cap.capture(&mut img1);
        let img1 = img1.freeze();

        let mut img2 = BytesMut::new();
        cap.capture(&mut img2);
        let img2 = img2.freeze();
        // img3 = img1 ^ img2
        let mut img3 = BytesMut::new();
        image_exclusive(&img1, &img2, &mut img3);
        // then img2 = img1 ^ img3
        let mut img4 = BytesMut::new();
        image_exclusive(&img1, &img3.freeze(), &mut img4);

        assert_eq!(img2, img4);
    }
}