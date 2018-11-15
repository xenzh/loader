use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::convert::From;

use std::io::Error as IoError;

use config::ConfigError;


#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    CfgError(ConfigError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::IoError(ref io) => io.fmt(f),
            Error::CfgError(ref cfg) => cfg.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref io) => io.description(),
            Error::CfgError(ref cfg) => cfg.description(),
        }
    }
}

impl From<IoError> for Error {
    fn from(io: IoError) -> Self {
        Error::IoError(io)
    }
}

impl From<ConfigError> for Error {
    fn from(cfg: ConfigError) -> Self {
        Error::CfgError(cfg)
    }
}

pub type Result<T> = StdResult<T, Error>;
