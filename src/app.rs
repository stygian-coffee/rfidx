use std::io::Result;
use std::sync::{Arc, Mutex};

use walkdir::{DirEntry, WalkDir};

use crate::api;

pub type FileIndex = Arc<Mutex<Vec<DirEntry>>>;

pub struct App {
    file_index: FileIndex,
}

impl App {
    pub fn new() -> Self {
        Self {
            file_index: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        self.generate_index()?;

        warp::serve(api::all_routes(self.file_index.clone()))
            .run(([127, 0, 0, 1], 8000))
            .await;

        Ok(())
    }

    fn generate_index(&mut self) -> Result<()> {
        let mut file_index = self.file_index.lock().unwrap();
        let walkdir = WalkDir::new(".");
        for f in walkdir.into_iter() {
            file_index.push(f?);
        }

        Ok(())
    }
}
