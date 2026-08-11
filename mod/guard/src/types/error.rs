// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug)]
pub enum ApiError {
    // Fetch phase
    Url(url::ParseError),
    Request(reqwest::Error),
    // Data deserialization phase
    Deserialize(serde_json::error::Error),
    Encoding(std::string::FromUtf8Error),
    // Crypto phase
    #[cfg(feature = "openssl")]
    OpenSSL(openssl::error::ErrorStack),
    #[cfg(feature = "rustcrypto")]
    Argon2(argon2::Error),
    #[cfg(feature = "rustcrypto")]
    Base64(base64ct::Error),
    // etc.
    SessionError,
    RequestTypeMismatch,
    UnsupportedAlgo,
    NotFound,
    IO(std::io::Error)
}

/* BEGIN ApiError: convertions */

impl From<url::ParseError> for ApiError {
    fn from(value: url::ParseError) -> Self {
        ApiError::Url(value)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        ApiError::Request(value)
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(value: serde_json::error::Error) -> Self {
        ApiError::Deserialize(value)
    }
}

#[cfg(feature = "openssl")]
impl From<openssl::error::ErrorStack> for ApiError {
    fn from(value: openssl::error::ErrorStack) -> Self {
        ApiError::OpenSSL(value)
    }
}

#[cfg(feature = "rustcrypto")]
impl From<argon2::Error> for ApiError {
    fn from(value: argon2::Error) -> Self {
        ApiError::Argon2(value)
    }
}

#[cfg(feature = "rustcrypto")]
impl From<base64ct::Error> for ApiError {
    fn from(value: base64ct::Error) -> Self {
        ApiError::Base64(value)
    }
}

impl From<std::string::FromUtf8Error> for ApiError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        ApiError::Encoding(value)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        ApiError::IO(value)
    }
}


impl std::fmt::Display for ApiError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
