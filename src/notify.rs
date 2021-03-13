use std::fs;
use std::sync::mpsc;
use std::time::Duration;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

use crate::app::{FileEntry, FileIndex};

pub async fn listen_and_update(file_index: FileIndex) {
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

pub fn update_from_event(file_index: FileIndex, event: DebouncedEvent) {
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
                file_index.push(FileEntry { path });
            }
        }
        DebouncedEvent::Remove(path) => {
            log::info!("Remove event: {:?}", &path);

            let mut file_index = file_index.lock().unwrap();
            file_index.retain(|f| f.path != path);
        }
        _ => {}
    }
}
