use std::{ffi::OsString, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileNode {
    name: OsString,
    size: u64,
    is_dir: bool,
}

impl FileNode {
    pub fn new(name: OsString, size: u64, is_dir: bool) -> Self {
        Self { name, size, is_dir }
    }
}

impl Display for FileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // name should always be last, everything else can have a std size
        write!(
            f,
            "{:<2} {:<10}: {}",
            if self.is_dir { "d" } else { "f" },
            self.size,
            self.name.to_str().unwrap_or("")
        )
    }
}
