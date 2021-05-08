use warp::{http::StatusCode, Rejection, Reply};

use std::{convert::Infallible, fmt, io};

use crate::Response;

/// Crate errors
#[derive(Debug, Clone)]
pub enum Error {
    /// Internal Server Error
    IntSrv(String),
    /// Bad request error
    BadReq(String),
    /// 404
    NotFound(String),
    /// Duplicate record
    Conflict(String),
    /// Unauthorized request
    UnAuth(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntSrv(s) => write!(f, "{}", s),
            Self::BadReq(s) => write!(f, "{}", s),
            Self::NotFound(s) => write!(f, "{}", s),
            Self::Conflict(s) => write!(f, "{}", s),
            Self::UnAuth(s) => write!(f, "{}", s),
        }
    }
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = String::from("Not Found");
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = e.to_string();
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::BadReq(e) => {
                code = StatusCode::BAD_REQUEST;
                message = e.to_string();
            }
            Error::NotFound(e) => {
                code = StatusCode::NOT_FOUND;
                message = e.to_string();
            }
            Error::Conflict(e) => {
                code = StatusCode::CONFLICT;
                message = e.to_string();
            }
            Error::UnAuth(e) => {
                code = StatusCode::UNAUTHORIZED;
                message = e.to_string();
            }
            Error::IntSrv(e) => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = e.to_string();
            }
        }
    } else if let Some(e) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = e.to_string();
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = String::from("Internal Server Error");
    }

    let json = warp::reply::json(&Response::new("", message, 1));

    Ok(warp::reply::with_status(json, code))
}

/// Type alias for Result
pub type Result<T> = std::result::Result<T, warp::Rejection>;
