use std::io::Error as IOError;

use crate::semantics::resolve::ImportStack;
use crate::syntax::{Import, ParseError};
use crate::NormalizedExpr;

mod builder;
pub(crate) use builder::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    IO(IOError),
    Parse(ParseError),
    Decode(DecodeError),
    Encode(EncodeError),
    Resolve(ImportError),
    Typecheck(TypeError),
}

#[derive(Debug)]
pub enum ImportError {
    Recursive(Import<NormalizedExpr>, Box<Error>),
    UnexpectedImport(Import<NormalizedExpr>),
    ImportCycle(ImportStack, Import<NormalizedExpr>),
}

#[derive(Debug)]
pub enum DecodeError {
    CBORError(serde_cbor::error::Error),
    WrongFormatError(String),
}

#[derive(Debug)]
pub enum EncodeError {
    CBORError(serde_cbor::error::Error),
}

/// A structured type error
#[derive(Debug)]
pub struct TypeError {
    message: TypeMessage,
}

/// The specific type error
#[derive(Debug)]
pub(crate) enum TypeMessage {
    Sort,
    Custom(String),
}

impl TypeError {
    pub(crate) fn new(message: TypeMessage) -> Self {
        TypeError { message }
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use TypeMessage::*;
        let msg = match &self.message {
            Sort => format!("Type error: Unhandled error: {:?}", self.message),
            Custom(s) => format!("Type error: {}", s),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for TypeError {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IO(err) => write!(f, "{}", err),
            Error::Parse(err) => write!(f, "{}", err),
            Error::Decode(err) => write!(f, "{:?}", err),
            Error::Encode(err) => write!(f, "{:?}", err),
            Error::Resolve(err) => write!(f, "{:?}", err),
            Error::Typecheck(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}
impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IO(err)
    }
}
impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::Parse(err)
    }
}
impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Error {
        Error::Decode(err)
    }
}
impl From<EncodeError> for Error {
    fn from(err: EncodeError) -> Error {
        Error::Encode(err)
    }
}
impl From<ImportError> for Error {
    fn from(err: ImportError) -> Error {
        Error::Resolve(err)
    }
}
impl From<TypeError> for Error {
    fn from(err: TypeError) -> Error {
        Error::Typecheck(err)
    }
}
