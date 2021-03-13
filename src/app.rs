use std::sync::{Arc, Mutex};

use crate::api;
use crate::file_index::FileIndex;
use crate::notify;

pub struct App {
    file_index: Arc<Mutex<FileIndex>>,
}

impl App {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            file_index: Arc::new(Mutex::new(FileIndex::from_path(".")?)),
        })
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        tokio::spawn(notify::listen_and_update(self.file_index.clone()));

        warp::serve(api::all_routes(self.file_index.clone()))
            .run(([127, 0, 0, 1], 8000))
            .await;

        Ok(())
    }
}
