use core::pin::Pin;
use core::task::{Context, Poll};

pub trait AsyncWrite {
    type Error;
    fn poll_write(
        self: Pin<&mut Self>,
        cx: Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::Error>>;
    fn poll_flush(self: Pin<&mut Self>, cx: Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn poll_close(self: Pin<&mut Self>, cx: Context<'_>) -> Poll<Result<(), Self::Error>>;
}

pub trait AsyncRead {
    type Error;
    fn poll_read(
        self: Pin<&mut Self>,
        cx: Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>>;
}
