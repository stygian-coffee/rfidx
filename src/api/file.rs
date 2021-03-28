use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection};
use warp::http::StatusCode;

use crate::file_index::FileIndex;
use super::error::WithStatus;

/// GET /files
pub fn files(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("files")
        .and(warp::get())
        .and(with_file_index(file_index))
        .and_then(files_handler)
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

/// GET /files/startswith?q=file_nam
pub fn files_startswith(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("files" / "startswith")
        .and(warp::get())
        .and(with_file_index(file_index))
        .and(warp::query::<StartswithQuery>())
        .and_then(files_startswith_handler)
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
        Err(e) => return Err(warp::reject::custom(e.with_status(StatusCode::BAD_REQUEST))),
    };

    let files = file_index
        .as_ref()
        .par_iter()
        .map(|fe| &fe.path)
        .filter(|s| pattern_compiled.matches(&s.as_path().to_str().unwrap()))
        .collect::<Vec<&std::path::PathBuf>>();
    Ok(warp::reply::json(&files))
}

#[derive(Serialize, Deserialize)]
struct StartswithQuery {
    q: String,
}

async fn files_startswith_handler(
    file_index: Arc<Mutex<FileIndex>>,
    query: StartswithQuery,
) -> Result<impl warp::Reply, Rejection> {
    let file_index = file_index.lock().unwrap();
    let start = query.q;

    //TODO consider multiple files with the same name but in different directories
    let files = file_index
        .as_ref()
        .par_iter()
        .map(|fe| fe.path.file_name().unwrap()) //TODO check the unwrap
        .filter(|s| s.to_str().unwrap().starts_with(&start)) //TODO don't rely on valid Unicode
        .map(|s| s.to_str().unwrap()) //TODO Unicode
        .collect::<Vec<&str>>();
    Ok(warp::reply::json(&files))
}
