use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use warp::Filter;

use crate::file_index::FileIndex;

pub fn all_routes(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    version()
        .or(files(file_index.clone()))
        .or(files_glob(file_index.clone()))
}

pub fn version() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| "rfidx v0.1.0\n")
}

/// GET /files
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

/// GET /files/glob
pub fn files_glob(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let files = move |pattern: String| {
        let file_index = file_index.lock().unwrap();
        let pattern_compiled = glob::Pattern::new(&pattern).unwrap();
        serde_json::to_string(
            &file_index
                .as_ref()
                .iter()
                .map(|fe| &fe.path)
                .filter(|s| pattern_compiled.matches(&s.as_path().to_str().unwrap()))
                .collect::<Vec<&std::path::PathBuf>>(),
        )
        .unwrap()
    };

    warp::path!("files" / "glob")
        .and(warp::query::<HashMap<String, String>>())
        .map(
            move |mut params: HashMap<String, String>| match params.remove("q") {
                Some(pat) => files(pat),
                None => "error no pattern".to_string(),
            },
        )
        .map(|reply| warp::reply::with_header(reply, "content-type", "application/json"))
}
