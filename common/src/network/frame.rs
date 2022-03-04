use bytes::{BytesMut, BufMut, Buf};
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::{AppError, Operation, Image};
use zstd::{encode_all, DEFAULT_COMPRESSION_LEVEL, decode_all};

// 用多少个字节表示数据长度，这里是4个字节
// 也就是数据最大可以到 2 ** 32 = 4GB
const LEN: usize = 4;
pub trait FrameCoder
where
    Self: Message + Sized + Default,
{
    /// 将protobuf编码成 | 长度 | zstd压缩的数据 |
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), AppError> {
        let size = self.encoded_len();
        let mut buf1 = Vec::with_capacity(size);
        self.encode(&mut buf1)?;

        let encoded_buf = encode_all(&buf1[..], DEFAULT_COMPRESSION_LEVEL)?;

        buf.put_u32(encoded_buf.len() as u32);
        buf.extend(encoded_buf);
        Ok(())
    }
    fn decode_frame(buf: &mut BytesMut) -> Result<Self, AppError> {
        // 取4个字节的长度
        let length = buf.get_u32() as usize;
        // 解压缩
        let buf1 = decode_all(&buf[..length])?;
        buf.advance(length);
        Ok(Self::decode(&buf1[..])?)
    }
}

impl FrameCoder for Operation {}
impl FrameCoder for Image {}

// 从 stream 中读取一个完整的 frame
pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), AppError>
where
    S: AsyncRead + Unpin + Send,
{
    let length = stream.read_u32().await? as usize;
    // 保证至少一个frame的内存
    buf.reserve(LEN + length);
    buf.put_u32(length as u32);
    // 预留内存然后从stream里读取，读取完后它就是初始化的。所以，这里用unsafe是安全的
    unsafe { buf.advance_mut(length); }
    stream.read_exact(&mut buf[LEN..]).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use image::io::Reader as ImageReader;

    #[test]
    fn event_encode_decode_should_work() {
        let mut buf = BytesMut::new();
        let op = Operation::new_resize(2880, 1800);
        op.encode_frame(&mut buf).unwrap();

        let op1 = Operation::decode_frame(&mut buf).unwrap();
        assert_eq!(op, op1);
    }

    #[test]
    fn image_encode_decode_should_work() {
        let img = ImageReader::open("R.png").unwrap().with_guessed_format().unwrap().decode().unwrap();
        let width = img.width();
        let height = img.height();
        let mut data = BytesMut::new();
        data.extend_from_slice(img.as_bytes());
        let image_proto = Image::new(width as usize, height as usize, Bytes::from(data));

        let mut buf = BytesMut::new();
        image_proto.encode_frame(&mut buf).unwrap();


        let image_proto1 = Image::decode_frame(&mut buf).unwrap();
        assert_eq!(image_proto, image_proto1);
    }
}