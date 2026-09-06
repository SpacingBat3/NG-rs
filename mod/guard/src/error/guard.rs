// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use ng_rs_common::map_err;

use super::challenge::SerializeError;

#[derive(Debug)]
pub enum GetChallengeError {
    SerializeError(SerializeError),
    Request(reqwest::Error)
}



impl std::fmt::Display for GetChallengeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(err)        => err.fmt(f),
            Self::SerializeError(err) => err.fmt(f)
        }
    }
}

impl std::error::Error for GetChallengeError {}

map_err! {
    SerializeError => GetChallengeError::SerializeError,
    reqwest::Error => GetChallengeError::Request
}