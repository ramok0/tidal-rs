use std::path::Display;


#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Unauthorized,
    ParseError,
    FailedToGetDeviceCode,
    InvalidRefreshToken,
    WaitingForUserAction,
    FailedToAuthenticate
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::ParseError => write!(f, "ParseError"),
            Error::FailedToGetDeviceCode => write!(f, "FailedToGetDeviceCode"),
            Error::InvalidRefreshToken => write!(f, "InvalidRefreshToken"),
            Error::WaitingForUserAction => write!(f, "WaitingForUserAction"),
            Error::FailedToAuthenticate => write!(f, "FailedToAuthenticate")
        }
    }
}

impl std::error::Error for Error {}