use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] common::restful::RequestError),
    #[error(transparent)]
    Stream(#[from] common::websocket::ConnectivityError),
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("failed to send {0}: {1}")]
    SendError(String, String),
    #[error("failed to parse response: {0}")]
    ParseResponse(String),
    #[error("`{0}` value is invalid")]
    InvalidValue(String),
    #[error("service unavailable")]
    ServiceUnavailable,
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Msg(String),
}
