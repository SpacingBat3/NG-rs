// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;
use super::challenge::Challenge;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyRequest {
    #[serde(flatten)]
    pub challenge: Challenge,
    pub nonce: String,
    pub solve_time_ms: u128,
    pub demo: bool,
}

#[derive(serde::Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyResultRaw {
    expires_in: u16,
    ok: bool,
    token: String,
}

#[derive(serde::Deserialize,Debug)]
#[serde(from = "VerifyResultRaw")]
pub struct VerifyResult {
    /// FIXME: unit of expiresIn
    pub expires_in: Duration,
    pub ok: bool,
    pub token: String
}

impl From<VerifyResultRaw> for VerifyResult {
    fn from(value: VerifyResultRaw) -> Self {
        Self {
            expires_in: Duration::from_millis(value.expires_in as u64),
            ok: value.ok,
            token: value.token
        }
    }
}
