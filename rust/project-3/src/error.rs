use bson;
use failure::Fail;
use std::io;
use std::string::FromUtf8Error;

/// Error type for kvs
#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "IO Error: {}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "JSON Deserialization Error: {}", _0)]
    Serde(#[cause] serde_json::Error),

    #[fail(display = "BSON Encode Error: {}", _0)]
    Encode(#[cause] bson::EncoderError),

    #[fail(display = "BSON Decode Error: {}", _0)]
    Decode(#[cause] bson::DecoderError),

    #[fail(display = "Invalid UTF-8 Error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),

    #[fail(display = "Sled Error: {}", _0)]
    Sled(#[cause] sled::Error),

    #[fail(display = "Key not found")]
    KeyNotFound,

    #[fail(display = "Invalid Command")]
    InvalidCommand,

    #[fail(display = "{}", _0)]
    StringError(String),
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

impl From<bson::EncoderError> for KvsError {
    fn from(err: bson::EncoderError) -> KvsError {
        KvsError::Encode(err)
    }
}

impl From<bson::DecoderError> for KvsError {
    fn from(err: bson::DecoderError) -> KvsError {
        KvsError::Decode(err)
    }
}

impl From<FromUtf8Error> for KvsError {
    fn from(err: FromUtf8Error) -> KvsError {
        KvsError::Utf8(err)
    }
}

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> KvsError {
        KvsError::Sled(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;
