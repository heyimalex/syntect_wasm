use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug)]
pub struct Handle {}

impl Eq for Handle {}

impl PartialEq for Handle {
    fn eq(&self, _other: &Handle) -> bool {
        false
    }
}

impl Handle {
    pub fn from_path<P: AsRef<Path>>(_p: P) -> io::Result<Handle> {
        Err(Error::new(ErrorKind::Other, "unsupported under wasm"))
    }

    pub fn from_file(_file: File) -> io::Result<Handle> {
        Err(Error::new(ErrorKind::Other, "unsupported under wasm"))
    }

    pub fn stdin() -> io::Result<Handle> {
        Err(Error::new(ErrorKind::Other, "unsupported under wasm"))
    }

    pub fn stdout() -> io::Result<Handle> {
        Err(Error::new(ErrorKind::Other, "unsupported under wasm"))
    }

    pub fn stderr() -> io::Result<Handle> {
        Err(Error::new(ErrorKind::Other, "unsupported under wasm"))
    }

    pub fn as_file(&self) -> &File {
        panic!("unimplemented!");
    }

    pub fn as_file_mut(&mut self) -> &mut File {
        panic!("unimplemented!");
    }
}
