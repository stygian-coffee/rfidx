use std::convert::TryFrom;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};
use walkdir::{DirEntry, WalkDir};

use crate::api;

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
                path: entry.into_path()
            })
        }
    }
}

pub type FileIndex = Arc<Mutex<Vec<FileEntry>>>;

pub struct App {
    file_index: FileIndex,
}

impl App {
    pub fn new() -> Self {
        Self {
            file_index: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        self.generate_index()?;

        warp::serve(api::all_routes(self.file_index.clone()))
            .run(([127, 0, 0, 1], 8000))
            .await;

        Ok(())
    }

    fn generate_index(&mut self) -> std::io::Result<()> {
        let mut file_index = self.file_index.lock().unwrap();
        let walkdir = WalkDir::new(".");
        for f in walkdir.into_iter() {
            if let Ok(file_entry) = FileEntry::try_from(f?) {
                file_index.push(file_entry);
            }
        }

        Ok(())
    }
}
