// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{ops::BitOr, time::Duration};

use super::error::ApiError;

cfg_select! {
    feature = "rustcrypto" => {
        use base64ct::{
            Encoding,
            Base64Unpadded
        };
    }
    feature = "openssl" => {
        use openssl::base64;
    }
}


#[derive(Debug,Clone,Copy,serde::Serialize,serde::Deserialize)]
pub struct Argon2IdParams {
    #[serde(rename = "hashLength")]
    pub(crate) hash_len: u8,
    #[serde(rename = "iterations")]
    pub(crate) iters: u8,
    #[serde(rename = "memorySize")]
    pub(crate) mem_size: u32,
    #[serde(rename = "parallelism")]
    pub(crate) parallel: u8,
}

#[derive(Debug,Clone,Copy,serde::Serialize,serde::Deserialize)]
#[serde(untagged)]
pub enum ChallengeAlgo {
    Argon2Id(Argon2IdParams)
}

#[derive(serde::Serialize,serde::Deserialize,Debug)]
pub(crate) struct ChallengeRaw {
    bits: u32,
    #[serde(rename = "payload")]
    payload_base64url: String,
    sig: String,
    algo: String,
    #[serde(default)]
    params: Option<ChallengeAlgo>,
}

#[derive(Debug,Clone)]
pub struct ChallengeTask {
    pub(crate) bits: u32,
    pub(crate) payload: String,
    pub(crate) sig: String,
}

#[derive(Clone,Copy)]
pub enum ChallengeSolverStopCond {
    Timeout(Duration),
    Iterations(usize),
    Both(Duration,usize)
}

/// Union
impl BitOr for ChallengeSolverStopCond {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let union = [self,rhs]
            .map(|it| match it {
                Self::Both(t, i) => (Some(t),Some(i)),
                Self::Timeout(t) => (Some(t),None),
                Self::Iterations(i) => (None,Some(i))
            })
            .into_iter()
            .reduce(|x,y| (x.0.or(y.0),x.1.or(y.1)))
            .unwrap()
            .to_owned();
        match union {
            (Some(t), Some(i)) => Self::Both(t, i),
            (Some(t), None)    => Self::Timeout(t),
            (None, Some(i))    => Self::Iterations(i),
            _                  => unreachable!()
        }
    }
}

#[derive(Debug,Clone,serde::Serialize)]
#[serde(into = "ChallengeRaw")]
pub struct Challenge {
    pub(crate) task: ChallengeTask,
    pub(crate) algo: ChallengeAlgo,
}

#[derive(Debug)]
pub struct ChallengeSolution {
    pub(crate) nonce: usize,
    pub(crate) solve_time: Duration,
}

// Convertions

impl<'a> Into<&'a str> for &ChallengeAlgo {
    fn into(self) -> &'a str {
        match self {
            ChallengeAlgo::Argon2Id(_) => "argon2id"
        }
    }
}

impl<'a> Into<&'a str> for ChallengeAlgo {
    fn into(self) -> &'a str {
        match self {
            Self::Argon2Id(_) => "argon2id"
        }
    }
}

impl TryFrom<ChallengeRaw> for Challenge {
    type Error = ApiError;
    fn try_from(value: ChallengeRaw) -> Result<Self,Self::Error> {
        let payload = { cfg_select! {
            feature = "rustcrypto" => String::from_utf8(
                Base64Unpadded::decode_vec(
                    &value.payload_base64url
                        .as_str()
                )?
            )?,
            feature = "openssl" => {
                // Ensure padding
                let pad_len = value.payload_base64url.len() & 0b11;
                let payload_base64 = {
                    value.payload_base64url
                        + "=".repeat(pad_len).as_str()
                };
                String::from_utf8(base64
                    ::decode_block(
                        &payload_base64
                            .as_str()
                    )?
                )?
            }
        }};
        let algo = match value.algo.as_str() {
            "argon2id" => if let Some(ChallengeAlgo::Argon2Id(params)) = value.params {
                Ok(ChallengeAlgo::Argon2Id(params))
            } else {
                Err(ApiError::RequestTypeMismatch)
            }
            _ => Err(ApiError::UnsupportedAlgo)
        }?;
        return Ok(Challenge {
            task: ChallengeTask {
                bits: value.bits,
                payload,
                sig: value.sig
            },
            algo
        });
    }
}

impl From<Challenge> for ChallengeRaw {
    fn from(value: Challenge) -> Self {
        let algo:&str = value.algo.into();
        Self {
            payload_base64url: cfg_select! {
                feature = "rustcrypto" => Base64Unpadded::encode_string(
                    value.task.payload.as_bytes()
                ),
                feature = "openssl" => String::from(
                    base64::encode_block(
                        value.task.payload.as_bytes()
                    ).trim_end_matches("=")
                )
            },
            sig: value.task.sig,
            bits: value.task.bits,
            algo: String::from(algo),
            params: Some(value.algo),
        }
    }
}
