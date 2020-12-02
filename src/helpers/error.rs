use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    FromUtf8(FromUtf8Error),
    Io(io::Error),
    Msg(String),
    Utf8(Utf8Error),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            Error::FromUtf8(ref error) => error.fmt(formatter),
            Error::Io(ref error) => error.fmt(formatter),
            Error::Utf8(ref error) => error.fmt(formatter),
            Error::Msg(ref msg) => write!(formatter, "{}", msg),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::FromUtf8(ref error) => error.description(),
            Error::Io(ref error) => error.description(),
            Error::Utf8(ref error) => error.description(),
            Error::Msg(ref msg) => msg,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        let cause: &error::Error = match *self {
            Error::FromUtf8(ref error) => error,
            Error::Io(ref error) => error,
            Error::Utf8(ref error) => error,
            Error::Msg(_) => return None,
        };
        Some(cause)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(message: &'a str) -> Self {
        Error::Msg(message.to_string())
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::Utf8(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::FromUtf8(error)
    }
}
