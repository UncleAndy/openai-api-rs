#[cfg(not(all(target_arch = "wasm32", target_os = "wasi")))]
use reqwest::{self};

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum APIError {
    ReqwestError(anyhow::Error),
    CustomError { message: String },
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::ReqwestError(err) => write!(f, "ReqwestError: {err}"),
            APIError::CustomError { message } => write!(f, "APIError: {message}"),
        }
    }
}

impl Error for APIError {}

impl From<anyhow::Error> for APIError {
    fn from(err: anyhow::Error) -> APIError {
        APIError::ReqwestError(err)
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
impl From<golem_wasi_http::Error> for APIError {
    fn from(err: golem_wasi_http::Error) -> APIError {
        APIError::ReqwestError(err.into())
    }
}
