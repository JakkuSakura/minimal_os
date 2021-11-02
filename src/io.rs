use core::pin::Pin;
use core::task::Poll;

pub trait AsyncWrite {
    type Error;
    fn poll_write(self: Pin<&mut Self>, buf: &[u8]) -> Poll<Result<usize, Self::Error>>;
    fn poll_flush(self: Pin<&mut Self>) -> Poll<Result<(), Self::Error>>;
    fn poll_close(self: Pin<&mut Self>) -> Poll<Result<(), Self::Error>>;
}

pub trait AsyncRead {
    type Error;
    fn poll_read(self: Pin<&mut Self>, buf: &mut [u8]) -> Poll<Result<usize, Self::Error>>;
}
