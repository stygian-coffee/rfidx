use walkdir::DirEntry;
use warp::Filter;

use crate::app::FileIndex;

pub fn all_routes(
    file_index: FileIndex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    version().or(files(file_index.clone()))
}

pub fn version() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| "rusty_indexer v0.1.0\n")
}

pub fn files(
    file_index: FileIndex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let files = move || {
        let file_index = file_index.lock().unwrap();
        let filtered: Vec<&DirEntry> = file_index
            .iter()
            .filter(|d| d.file_name().to_str().unwrap().ends_with(".rs"))
            .collect();
        format!("{:?}", filtered)
    };

    warp::path!("files").map(files)
}
