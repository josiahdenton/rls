use std::{ffi::OsString, fmt::Display};

#[derive(Debug)]
pub struct FileNode {
    name: OsString,
    size: u64,
}

impl FileNode {
    pub fn new(name: OsString, size: u64) -> Self {
        Self { name, size }
    }
}

impl Display for FileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
