use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::Result;
use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
}

impl FileEntry {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }
}

pub struct FileIndex {
    root: PathBuf,
    entries: HashSet<FileEntry>,
}

fn canonical_relative_path_priv<T, U>(root: T, path: U) -> Result<PathBuf>
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    Ok(path
        .as_ref()
        .absolutize()?
        .into_owned()
        .strip_prefix(root)?
        .into())
}

impl FileIndex {
    pub fn from_path<T: AsRef<Path>>(root: T) -> Result<Self> {
        let root = PathBuf::from(root.as_ref().absolutize()?.into_owned());

        log::info!("Indexing files...");

        let mut entries = HashSet::new();
        let walkdir = WalkDir::new(&root);
        for f in walkdir.into_iter() {
            let f = f?;
            if !f.file_type().is_file() {
                continue;
            }
            entries.insert(FileEntry {
                path: canonical_relative_path_priv(&root, f.into_path())?,
            });
        }

        log::info!("Indexed {} files.", entries.len());

        Ok(Self { root, entries })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    fn canonical_relative_path<T: AsRef<Path>>(&self, path: T) -> Result<PathBuf> {
        canonical_relative_path_priv(&self.root, path)
    }

    /// Insert a file path, putting the path in a canonical form relative to the root beforehand
    pub fn insert<T: AsRef<Path>>(&mut self, path: T) {
        self.entries
            .insert(FileEntry::new(match self.canonical_relative_path(path) {
                Ok(p) => p,
                Err(e) => {
                    log::error!("{}", self.root.display());
                    log::error!("{}", e);
                    return;
                }
            }));
    }

    /// Remove a file path, putting the path in a canonical form relative to the root beforehand
    pub fn remove<T: AsRef<Path>>(&mut self, path: T) {
        self.entries
            .remove(&FileEntry::new(match self.canonical_relative_path(path) {
                Ok(p) => p,
                Err(e) => {
                    log::error!("{}", e);
                    return;
                }
            }));
    }
}

impl AsRef<HashSet<FileEntry>> for FileIndex {
    fn as_ref(&self) -> &HashSet<FileEntry> {
        &self.entries
    }
}

impl AsMut<HashSet<FileEntry>> for FileIndex {
    fn as_mut(&mut self) -> &mut HashSet<FileEntry> {
        &mut self.entries
    }
}
