use std::fmt;
/// Main error type
#[derive(Debug)]
pub enum Error {
    /// Returned when we fail to parse a value
    ParseError,
    /// Returned if unsupported compositor detected
    CompositorUnsupported,
    /// Returned if no XDG_RUNTIME and/or additional compositor specific env vars can be found
    EnvVarNotFound,
    /// Returned when we either fail to connect to compositor's socket or read and write to it
    SocketErr,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError => write!(f, "Unexpected data format returned by compositor"),
            Error::CompositorUnsupported => {
                write!(f, "Compositor you are trying to use is not supported")
            }
            Error::EnvVarNotFound => write!(f, "Essential environment variables are not set"),
            Error::SocketErr => write!(f, "Failed to connect to compositor's IPC"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
