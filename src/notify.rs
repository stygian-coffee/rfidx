use std::fs;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

use crate::file_index::{FileEntry, FileIndex};

pub async fn listen_and_update(file_index: Arc<Mutex<FileIndex>>) {
    let (tx, rx) = mpsc::channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher.watch(".", RecursiveMode::Recursive).unwrap();

    loop {
        let file_index = file_index.clone();
        match rx.recv() {
            Ok(event) => update_from_event(file_index, event),
            Err(e) => log::error!("{:?}", e),
        }
    }
}

pub fn update_from_event(file_index: Arc<Mutex<FileIndex>>, event: DebouncedEvent) {
    match event {
        DebouncedEvent::Create(path) => {
            log::info!("Create event: {:?}", &path);

            let metadata = match fs::metadata(&path) {
                Ok(m) => m,
                Err(e) => {
                    log::error!("Error reading file metadata: {}", e);
                    return;
                }
            };
            if metadata.is_file() {
                let mut file_index = file_index.lock().unwrap();
                file_index.as_mut().push(FileEntry { path });
            }
        }
        DebouncedEvent::Remove(path) => {
            log::info!("Remove event: {:?}", &path);

            let mut file_index = file_index.lock().unwrap();
            file_index.as_mut().retain(|f| f.path != path);
        }
        _ => {}
    }
}
