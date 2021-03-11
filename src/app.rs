use std::io::Result;
use std::sync::{Arc, Mutex};

use walkdir::{DirEntry, WalkDir};
use warp::Filter;

type FileIndex = Arc<Mutex<Vec<DirEntry>>>;

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

        let file_index = self.file_index.clone();

        let print_filtered = move || {
            let file_index = file_index.lock().unwrap();
            let filtered: Vec<&DirEntry> = file_index
                .iter()
                .filter(|d| d.file_name().to_str().unwrap().ends_with(".rs"))
                .collect();
            format!("{:?}", filtered)
        };

        let root = warp::path::end().map(print_filtered);

        warp::serve(root).run(([127, 0, 0, 1], 8000)).await;

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
