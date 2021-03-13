use std::sync::{Arc, Mutex};

use warp::Filter;

use crate::file_index::FileIndex;

pub fn all_routes(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    version().or(files(file_index.clone()))
}

pub fn version() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| "rusty_indexer v0.1.0\n")
}

pub fn files(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let files = move || {
        let file_index = file_index.lock().unwrap();
        serde_json::to_string(
            &file_index
                .as_ref()
                .iter()
                .map(|fe| &fe.path)
                .collect::<Vec<&std::path::PathBuf>>(),
        )
        .unwrap()
    };

    warp::path!("files")
        .map(files)
        .map(|reply| warp::reply::with_header(reply, "content-type", "application/json"))
}
