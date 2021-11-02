pub mod memfs;

use crate::io::{AsyncRead, AsyncWrite};
use core::future::Future;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileId(u64);
// TODO: async version?
pub trait File: AsyncWrite + AsyncRead {}
pub struct OpenOptions {
    create: bool,
}
pub trait FileSystem {
    type Path: ?Sized;
    type File: File;
    type Error;
    type OpenFile: Future<Output = Result<Self::File, Self::Error>>;
    fn open(&self, path: &Self::Path, open_options: OpenOptions) -> Self::OpenFile;
}
