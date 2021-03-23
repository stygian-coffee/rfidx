use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection};

use crate::file_index::FileIndex;

/// GET /files
pub fn files(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("files")
        .and(warp::get())
        .and(with_file_index(file_index))
        .and_then(files_handler)
}

#[derive(Debug)]
struct PatternError;
impl warp::reject::Reject for PatternError {}
impl From<glob::PatternError> for PatternError {
    fn from(_err: glob::PatternError) -> Self {
        Self {}
    }
}

/// GET /files/glob?q=*.rs
pub fn files_glob(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("files" / "glob")
        .and(warp::get())
        .and(with_file_index(file_index))
        .and(warp::query::<GlobQuery>())
        .and_then(files_glob_handler)
}

fn with_file_index(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = (Arc<Mutex<FileIndex>>,), Error = Infallible> + Clone {
    warp::any().map(move || file_index.clone())
}

async fn files_handler(file_index: Arc<Mutex<FileIndex>>) -> Result<impl warp::Reply, Infallible> {
    let file_index = file_index.lock().unwrap();
    let files = file_index
        .as_ref()
        .iter()
        .map(|fe| &fe.path)
        .collect::<Vec<&std::path::PathBuf>>();
    Ok(warp::reply::json(&files))
}

#[derive(Serialize, Deserialize)]
struct GlobQuery {
    q: String,
}

async fn files_glob_handler(
    file_index: Arc<Mutex<FileIndex>>,
    query: GlobQuery,
) -> Result<impl warp::Reply, Rejection> {
    let file_index = file_index.lock().unwrap();
    let pattern_compiled = match glob::Pattern::new(&query.q) {
        Ok(p) => p,
        Err(e) => return Err(warp::reject::custom(PatternError::from(e))),
    };
    let files = file_index
        .as_ref()
        .iter()
        .map(|fe| &fe.path)
        .filter(|s| pattern_compiled.matches(&s.as_path().to_str().unwrap()))
        .collect::<Vec<&std::path::PathBuf>>();
    Ok(warp::reply::json(&files))
}
