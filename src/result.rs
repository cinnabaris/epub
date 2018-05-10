use std::{error, fmt, io, result, string};

use serde_xml_rs;
use zip;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    StringFromUtf8(string::FromUtf8Error),
    Zip(zip::result::ZipError),
    SerdeXml(serde_xml_rs::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::StringFromUtf8(ref err) => err.fmt(f),
            Error::Zip(ref err) => err.fmt(f),
            Error::SerdeXml(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::StringFromUtf8(ref err) => err.description(),
            Error::Zip(ref err) => err.description(),
            Error::SerdeXml(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::StringFromUtf8(ref err) => Some(err),
            Error::Zip(ref err) => Some(err),
            Error::SerdeXml(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringFromUtf8(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Error {
        Error::Zip(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Error {
        Error::SerdeXml(err)
    }
}
