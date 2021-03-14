use std::sync::{Arc, Mutex};

use anyhow::Result;
use tokio::sync::oneshot;

use crate::api;
use crate::file_index::FileIndex;
use crate::notify;

pub struct App {
    file_index: Arc<Mutex<FileIndex>>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            file_index: Arc::new(Mutex::new(FileIndex::from_path(".")?)),
        })
    }

    pub async fn run(&mut self) {
        tokio::spawn(
            warp::serve(api::all_routes(self.file_index.clone())).run(([127, 0, 0, 1], 8000)),
        );

        //TODO put this in a loop and check if it's repeatedly failing,
        // breaking out of the loop if it fails too often
        let (notify_tx, notify_rx) = oneshot::channel();
        tokio::spawn(notify::listen_and_update(
            notify_tx,
            self.file_index.clone(),
        ));
        notify_rx.await.unwrap(); // What can we do if this fails?
        log::error!("Filesystem listener crashed!");
    }
}
