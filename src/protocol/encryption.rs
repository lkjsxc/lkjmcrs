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
        let mut preview = self.encryptor.clone();
        encrypt_bytes(&mut preview, &mut encrypted);
        match Pin::new(&mut self.inner).poll_write(cx, &encrypted) {
            Poll::Ready(Ok(written)) => {
                advance_encryptor(&mut self.encryptor, &input[..written]);
                Poll::Ready(Ok(written))
            }
            other => other,
        }
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

fn encrypt_bytes(encryptor: &mut AesCfb8Encryptor, bytes: &mut [u8]) {
    for byte in bytes {
        let mut block = Block::<AesCfb8Encryptor>::default();
        block[0] = *byte;
        encryptor.encrypt_block_mut(&mut block);
        *byte = block[0];
    }
}

fn advance_encryptor(encryptor: &mut AesCfb8Encryptor, plain: &[u8]) {
    let mut ignored = plain.to_vec();
    encrypt_bytes(encryptor, &mut ignored);
}

#[cfg(test)]
mod tests {
    use super::EncryptedStream;
    use std::io;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tokio::io::{AsyncWrite, AsyncWriteExt};

    struct PartialSink {
        max_write: usize,
        bytes: Vec<u8>,
    }

    impl AsyncWrite for PartialSink {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            input: &[u8],
        ) -> Poll<io::Result<usize>> {
            let written = self.max_write.min(input.len());
            self.bytes.extend_from_slice(&input[..written]);
            Poll::Ready(Ok(written))
        }

        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn partial_writes_advance_cipher_only_for_written_bytes() {
        let secret = [7_u8; 16];
        let sink = PartialSink {
            max_write: 2,
            bytes: Vec::new(),
        };
        let mut stream = EncryptedStream::new(sink, &secret);
        assert_eq!(stream.write(b"abcd").await.unwrap(), 2);
        assert_eq!(stream.write(b"cd").await.unwrap(), 2);

        let mut expected = EncryptedStream::new(
            PartialSink {
                max_write: 4,
                bytes: Vec::new(),
            },
            &secret,
        );
        assert_eq!(expected.write(b"abcd").await.unwrap(), 4);
        assert_eq!(stream.inner.bytes, expected.inner.bytes);
    }
}
