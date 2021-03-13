use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};

#[derive(Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
}

pub enum FromDirEntryError {
    NotAFile,
}

impl TryFrom<DirEntry> for FileEntry {
    type Error = FromDirEntryError;

    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        if !entry.file_type().is_file() {
            Err(FromDirEntryError::NotAFile)
        } else {
            Ok(Self {
                path: entry.into_path(),
            })
        }
    }
}

pub struct FileIndex {
    root: PathBuf,
    entries: Vec<FileEntry>,
}

impl FileIndex {
    pub fn from_path<T: AsRef<Path>>(root: T) -> std::io::Result<Self> {
        log::info!("Indexing files...");

        let mut entries = vec![];
        let walkdir = WalkDir::new(&root);
        for f in walkdir.into_iter() {
            if let Ok(file_entry) = FileEntry::try_from(f?) {
                entries.push(file_entry);
            }
        }

        log::info!("Indexed {} files.", entries.len());

        Ok(Self { root: PathBuf::from(root.as_ref()), entries })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }
}

impl AsRef<Vec<FileEntry>> for FileIndex {
    fn as_ref(&self) -> &Vec<FileEntry> {
        &self.entries
    }
}

impl AsMut<Vec<FileEntry>> for FileIndex {
    fn as_mut(&mut self) -> &mut Vec<FileEntry> {
        &mut self.entries
    }
}
