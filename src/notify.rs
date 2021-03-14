use std::fs;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use anyhow::Result;
use notify::{watcher, DebouncedEvent, INotifyWatcher, RecursiveMode, Watcher};
use tokio::sync::oneshot;

use crate::file_index::FileIndex;

pub async fn listen_and_update(notify_tx: oneshot::Sender<()>, file_index: Arc<Mutex<FileIndex>>) {
    let (watcher_tx, watcher_rx) = mpsc::channel();
    // the watcher can't go out of scope
    let _watcher = match init_watcher(watcher_tx) {
        Ok(w) => w,
        Err(e) => {
            log::error!("{}", e);
            notify_tx.send(()).unwrap(); // What can we do if this fails?
            return;
        }
    };

    loop {
        let file_index = file_index.clone();
        match watcher_rx.recv().map(|event| update_from_event(file_index, event)) {
            Err(_) => { // RecvError
                log::error!("mpsc RecvError");
                notify_tx.send(()).unwrap(); // What can we do if this fails?
                break;
            }
            Ok(Err(e)) => log::error!("{}", e),
            Ok(_) => {}
        }
    }
}

fn init_watcher(tx: mpsc::Sender<DebouncedEvent>) -> Result<INotifyWatcher> {
    let mut watcher = watcher(tx, Duration::from_secs(1))?;

    watcher.watch(".", RecursiveMode::Recursive)?;

    Ok(watcher)
}

fn update_from_event(file_index: Arc<Mutex<FileIndex>>, event: DebouncedEvent) -> Result<()> {
    match event {
        DebouncedEvent::Create(path) => {
            log::info!("Create event: {}", &path.display());

            let metadata = fs::metadata(&path)?;
            if metadata.is_file() {
                let mut file_index = file_index.lock().unwrap();
                file_index.insert(path);
            }
        }
        DebouncedEvent::Remove(path) => {
            log::info!("Remove event: {}", &path.display());

            let mut file_index = file_index.lock().unwrap();
            file_index.remove(path);
        }
        _ => {}
    }

    Ok(())
}
