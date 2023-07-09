use core::fmt;
use std::string::FromUtf8Error;

pub enum FetchError {
    JSONError(serde_json::Error),
    HTTPError(reqwest::Error),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchError::JSONError(err) => write!(f, "JSON error: {}", err),
            FetchError::HTTPError(err) => write!(f, "HTTP error: {}", err),
        }
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(error: serde_json::Error) -> Self {
        FetchError::JSONError(error)
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(error: reqwest::Error) -> Self {
        FetchError::HTTPError(error)
    }
}

#[derive(Debug)]
pub enum AuthError {
    JSONError(serde_json::Error),
    HTTPError(reqwest::Error),
    FSError(std::io::Error),
    UTF8Error(FromUtf8Error),
    AuthError(u16),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthError::JSONError(err) => write!(f, "JSON error: {}", err),
            AuthError::HTTPError(err) => write!(f, "HTTP error: {}", err),
            AuthError::FSError(err) => write!(f, "File system error: {}", err),
            AuthError::UTF8Error(err) => write!(f, "Byte parsing error: {}", err),
            AuthError::AuthError(status) => write!(f, "Authentification error: {}", status),
        }
    }
}

impl From<serde_json::Error> for AuthError {
    fn from(error: serde_json::Error) -> Self {
        AuthError::JSONError(error)
    }
}

impl From<reqwest::Error> for AuthError {
    fn from(error: reqwest::Error) -> Self {
        AuthError::HTTPError(error)
    }
}

impl From<std::io::Error> for AuthError {
    fn from(error: std::io::Error) -> Self {
        AuthError::FSError(error)
    }
}

impl From<FromUtf8Error> for AuthError {
    fn from(error: FromUtf8Error) -> Self {
        AuthError::UTF8Error(error)
    }
}
