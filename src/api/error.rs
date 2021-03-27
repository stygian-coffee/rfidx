use serde::ser::{Serialize, SerializeStruct, Serializer};
use warp::http::StatusCode;

#[derive(Debug)]
pub struct Error {
    status_code: StatusCode,
    message: String,
}

impl Error {
    pub fn new(status_code: warp::http::StatusCode, message: &str) -> Self {
        Self {
            status_code,
            message: message.into(),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("status_code", &self.status_code.as_u16())?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

impl warp::reject::Reject for Error {}

pub trait WithStatus: std::error::Error {
    fn with_status(&self, status_code: StatusCode) -> Error {
        Error {
            status_code,
            message: self.to_string(),
        }
    }
}

impl<T: std::error::Error> WithStatus for T {}
