use aes::Aes128;
use cfb8::{Decryptor, Encryptor};
use cipher::{Block, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

type AesCfb8Encryptor = Encryptor<Aes128>;
type AesCfb8Decryptor = Decryptor<Aes128>;

pub struct EncryptedStream<S> {
    inner: S,
    decryptor: AesCfb8Decryptor,
    encryptor: AesCfb8Encryptor,
}

impl<S> EncryptedStream<S> {
    pub fn new(inner: S, secret: &[u8; 16]) -> Self {
        Self {
            inner,
            decryptor: AesCfb8Decryptor::new(secret.into(), secret.into()),
            encryptor: AesCfb8Encryptor::new(secret.into(), secret.into()),
        }
    }
}

impl<S: AsyncRead + Unpin> AsyncRead for EncryptedStream<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let before = buf.filled().len();
        let result = Pin::new(&mut self.inner).poll_read(cx, buf);
        if let Poll::Ready(Ok(())) = &result {
            for byte in &mut buf.filled_mut()[before..] {
                let mut block = Block::<AesCfb8Decryptor>::default();
                block[0] = *byte;
                self.decryptor.decrypt_block_mut(&mut block);
                *byte = block[0];
            }
        }
        result
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for EncryptedStream<S> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        input: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let mut encrypted = input.to_vec();
        for byte in &mut encrypted {
            let mut block = Block::<AesCfb8Encryptor>::default();
            block[0] = *byte;
            self.encryptor.encrypt_block_mut(&mut block);
            *byte = block[0];
        }
        Pin::new(&mut self.inner).poll_write(cx, &encrypted)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}
