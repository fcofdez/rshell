use nix::Error;
use rustyline::error::ReadlineError;
use std::convert::From;

#[derive(Debug, Fail)]
pub enum Failures {
    #[fail(display = "SysError")]
    SysError,
    #[fail(display = "EOF")]
    EOF,
}

impl From<Error> for Failures {
    fn from(err: Error) -> Self {
        match err {
            Error::Sys(_) => Failures::SysError,
            Error::InvalidPath => Failures::SysError,
            Error::InvalidUtf8 => Failures::SysError,
            Error::UnsupportedOperation => Failures::SysError,
        }
    }
}

impl From<ReadlineError> for Failures {
    fn from(err: ReadlineError) -> Self {
        match err {
            ReadlineError::Io(_) => Failures::SysError,
            ReadlineError::Eof => Failures::EOF,
            ReadlineError::Interrupted => Failures::SysError,
            ReadlineError::Char(_) => Failures::SysError,
            ReadlineError::Errno(_) => Failures::SysError,
        }
    }
}
