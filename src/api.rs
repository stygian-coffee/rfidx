mod error;
mod file;

use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use log;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use crate::api::error::Error;
use crate::file_index::FileIndex;

pub fn all_filters(
    file_index: Arc<Mutex<FileIndex>>,
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    version()
        .or(file::files(file_index.clone()))
        .or(file::files_glob(file_index.clone()))
        .recover(handle_rejection)
}

/// GET /
pub fn version() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| "rfidx v0.1.0\n")
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            warp::reply::json(&Error::new(
                StatusCode::NOT_FOUND,
                "Not found",
            )),
            StatusCode::NOT_FOUND
        ))
    } else if let Some(api_err) = err.find::<Error>() {
        Ok(warp::reply::with_status(
            warp::reply::json(api_err),
            api_err.status_code(),
        ))
    } else {
        log::error!("Unhandled rejection: {:?}", err);
        Ok(warp::reply::with_status(
            warp::reply::json(&Error::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
