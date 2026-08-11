// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::types::{
    challenge::*,
    error::*,
    verify::*,
    guard::*
};

use ng_rs_common::prelude::*;

use ng_rs_common::{
    traits::Router,
    types::ApiCtx
};

use reqwest::{Response, StatusCode, Url, header};

//  API resolver

impl<'p> Router for Api<'p> {
    type  RouterRoot = ApiCtx;
    const ROUTER_PATH:&'static str = "/_guard/api/";
    fn get_route(&self, endpoint: &str) -> Result<Url,url::ParseError> {
        Url::parse(self.ctx.get_origin())
            ?.join(Self::ROUTER_PATH)
            ?.join(self.version.into())
            ?.join(endpoint)
    }
}

// API hooking

impl<'p> Api<'p> {
    pub fn guard_response_maybe(res:&Response)->bool {
        res.status() == StatusCode::FORBIDDEN
            && res.headers().get("content-encoding").is_some_and(|enc| enc == "gzip")
            && res.content_length().is_some_and(|len| len == 344)
    }
}

// API definitions

impl<'p> Api<'p> {
    async fn get_challenge_raw(&self) -> Result<ChallengeRaw,ApiError> {
        Ok(self.ctx.session
            .get(self.get_route("/challenge")?)
            .header(header::ACCEPT, "aplication/json")
            .send().await?
            .error_for_status()?
            .json::<ChallengeRaw>().await?)
    }
    /// Fetches new challenge to solve
    pub async fn get_challenge(&self) -> Result<Challenge,ApiError> {
        Challenge::try_from(self.get_challenge_raw().await?)
    }
    /// Verifies solution nonce
    pub async fn verify_nonce(&self, challenge: Challenge, solution: ChallengeSolution) -> Result<VerifyResult,ApiError> {
        Ok(self.ctx.session
            .post(self.get_route("/verify")?)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&VerifyRequest {
                challenge,
                nonce: solution.nonce.to_string(),
                solve_time_ms: solution.solve_time.as_millis(),
                demo: false
            })
            .send().await?
            .error_for_status()?
            .json::<VerifyResult>().await?)
    }
}
