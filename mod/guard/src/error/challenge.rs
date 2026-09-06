use std::string::FromUtf8Error;
use ng_rs_common::map_err;

pub type ArgonHashError = cfg_select! {
    feature = "rustcrypto" => argon2::Error,
    feature = "openssl" => openssl::error::ErrorStack
};

pub type Base64Error = cfg_select! {
    feature = "rustcrypto" => base64ct::Error,
    feature = "openssl" => openssl::error::ErrorStack
};

#[derive(Debug)]
pub struct NotFoundError;
impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Solution not found within constrains")
    }
}
impl std::error::Error for NotFoundError {}

#[derive(Debug)]
pub enum SerializeError {
    SerializeError,
    UnimplementedHash(Box<str>),
    Base64DecodeError(Base64Error),
    Utf8DecodeError(FromUtf8Error),
}

impl std::fmt::Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::SerializeError::*;
        match self {
            Base64DecodeError(err)  => err.fmt(f),
            Utf8DecodeError(err)    => err.fmt(f),
            SerializeError          => write!(f,"Invalid challenge structure"),
            UnimplementedHash(hash) => write!(f,"Unimplemented hash \"{hash}\"")
        }
    }
}

map_err! {
    Base64Error => SerializeError::Base64DecodeError,
    FromUtf8Error => SerializeError::Utf8DecodeError
}
