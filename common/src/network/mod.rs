mod frame;
use bytes::BytesMut;
pub use frame::{read_frame, FrameCoder};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::{Image, AppError};

pub struct ProstServerStream<S> {
    inner: S,
}

impl<S> ProstServerStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(stream: S) -> Self {
        Self { inner: stream }
    }

    pub async fn send(&mut self, image: Image) -> Result<(), AppError> {
        let mut buf = BytesMut::new();
        image.encode_frame(&mut buf)?;
        let encoded = buf.freeze();
        self.inner.write_all(&encoded).await?;
        Ok(())
    }
}

pub struct ProstClientStream<S> {
    inner: S,
}

impl<S> ProstClientStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(stream: S) -> Self {
        Self { inner: stream }
    }

    pub async fn recv(&mut self) -> Result<Image, AppError> {
        let mut buf = BytesMut::new();
        let stream = &mut self.inner;
        read_frame(stream, &mut buf).await?;
        Image::decode_frame(&mut buf)
    }
}