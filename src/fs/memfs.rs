use crate::fs::{File, FileSystem, OpenOptions};
use crate::io::{AsyncRead, AsyncWrite};
use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::future::Ready;
use core::pin::Pin;
use core::task::Poll;
use spin::RwLockWriteGuard;

pub(crate) struct MemFileInner {
    content: spin::RwLock<Vec<u8>>,
}

pub(crate) struct MemFolderInner {
    children: spin::RwLock<BTreeMap<String, Arc<MemNode>>>,
}
#[allow(dead_code)]
impl MemFolderInner {
    pub fn find(&self, path: &str) -> Option<Arc<MemNode>> {
        self.children.read().get(path).cloned()
    }
    pub fn create_file_overwriting(&self, path: &str) -> Arc<MemNode> {
        let write = self.children.write();
        Self::create_file_overwriting_inner(write, path)
    }
    fn create_file_overwriting_inner(
        mut children: RwLockWriteGuard<BTreeMap<String, Arc<MemNode>>>,
        path: &str,
    ) -> Arc<MemNode> {
        let file = Arc::new(MemNode::File(MemFileInner {
            content: Default::default(),
        }));
        children.insert(path.to_owned(), Arc::clone(&file));
        file
    }
    pub fn open_or_create(&self, path: &str) -> (Arc<MemNode>, bool) {
        let read = self.children.upgradeable_read();
        if read.contains_key(path) {
            (read.get(path).unwrap().clone(), true)
        } else {
            let write = read.upgrade();
            (Self::create_file_overwriting_inner(write, path), false)
        }
    }
    pub fn open(&self, path: &str) -> Option<Arc<MemNode>> {
        self.children.read().get(path).cloned()
    }
}
pub(crate) enum MemNode {
    File(MemFileInner),
    Folder(MemFolderInner),
}
#[allow(dead_code)]
impl MemNode {
    pub fn as_folder(&self) -> Option<&MemFolderInner> {
        match self {
            MemNode::File(_) => None,
            MemNode::Folder(f) => Some(f),
        }
    }
    pub fn as_file(&self) -> Option<&MemFileInner> {
        match self {
            MemNode::File(f) => Some(f),
            MemNode::Folder(_) => None,
        }
    }
}
pub struct MemFs {
    root: Arc<MemNode>,
}

impl MemFs {
    fn find(&self, path: &str) -> Option<Arc<MemNode>> {
        assert!(path.starts_with("/"));
        let mut spt = path.split("/");
        let mut node = Arc::clone(&self.root);
        while let Some(segment) = spt.next() {
            match &*node {
                MemNode::Folder(folder) => match folder.find(segment) {
                    Some(finding) => {
                        node = finding;
                    }
                    None => {
                        return None;
                    }
                },
                MemNode::File(_) => {
                    return None;
                }
            }
        }
        return Some(node);
    }
}

pub struct MemFile {
    file: Arc<MemNode>,
    write_cursor: usize,
    read_cursor: usize,
}

impl AsyncWrite for MemFile {
    type Error = ();

    fn poll_write(mut self: Pin<&mut Self>, buf: &[u8]) -> Poll<Result<usize, Self::Error>> {
        let this = &mut *self;
        let mut file = this.file.as_file().unwrap().content.write();
        if this.write_cursor + buf.len() < file.len() {
            file.resize(this.write_cursor + buf.len(), 0);
        }
        file[this.write_cursor..this.write_cursor + buf.len()].copy_from_slice(buf);
        this.write_cursor += 1;
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl AsyncRead for MemFile {
    type Error = ();

    fn poll_read(mut self: Pin<&mut Self>, buf: &mut [u8]) -> Poll<Result<usize, Self::Error>> {
        let this = &mut *self;
        let file = this.file.as_file().unwrap().content.read();
        if this.read_cursor > file.len() {
            return Poll::Pending;
        } else {
            let len = core::cmp::min(buf.len(), file.len() - this.read_cursor);
            buf.copy_from_slice(&file.as_slice()[this.read_cursor..this.read_cursor + len]);
            this.read_cursor += 1;
            return Poll::Ready(Ok(len));
        }
    }
}

impl File for MemFile {}
impl MemFs {
    pub fn new() -> Self {
        Self {
            root: Arc::new(MemNode::Folder(MemFolderInner {
                children: Default::default(),
            })),
        }
    }
    pub fn open(&self, path: &str, open_options: OpenOptions) -> Result<MemFile, &'static str> {
        let spt = path.rfind("/").unwrap();
        let folder = if spt > 0 {
            self.find(&path[..spt]).unwrap()
        } else {
            Arc::clone(&self.root)
        };
        match &*folder {
            MemNode::File(_) => {
                return Err("Could not proceed with folder, got file");
            }
            MemNode::Folder(x) if open_options.create => {
                let (file, exists) = x.open_or_create(&path[spt + 1..]);
                let len = file.as_file().unwrap().content.read().len();
                return Ok(MemFile {
                    file,
                    write_cursor: if exists { len } else { 0 },
                    read_cursor: 0,
                });
            }
            MemNode::Folder(x) => {
                let file = x.open(&path[spt + 1..]);

                match file {
                    Some(file) => {
                        let len = file.as_file().unwrap().content.read().len();
                        return Ok(MemFile {
                            file,
                            write_cursor: len,
                            read_cursor: 0,
                        });
                    }
                    None => return Err("Could not find file"),
                }
            }
        }
    }
}

impl FileSystem for MemFs {
    type Path = str;
    type File = MemFile;
    type Error = &'static str;
    type OpenFile = Ready<Result<Self::File, Self::Error>>;

    fn open(&self, path: &Self::Path, open_options: OpenOptions) -> Self::OpenFile {
        core::future::ready(Self::open(self, path, open_options))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_memfs_create_file() {
        let memfs = MemFs::new();
        let file = memfs.open("/test", OpenOptions { create: true }).unwrap();
        assert_eq!(file.file.as_file().unwrap().content.read().len(), 0);
        let file = memfs.open("/test", OpenOptions { create: false }).unwrap();
    }
}
