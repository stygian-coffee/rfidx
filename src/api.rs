mod file;

use std::sync::{Arc, Mutex};

use warp::Filter;

use crate::file_index::FileIndex;

pub fn all_filters(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    version()
        .or(file::files(file_index.clone()))
        .or(file::files_glob(file_index.clone()))
}

/// GET /
pub fn version() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| "rfidx v0.1.0\n")
}
