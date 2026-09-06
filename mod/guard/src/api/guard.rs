// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;

use crate::types::{
    challenge::*,
    error::*,
    verify::*,
    guard::*
};

use ng_rs_common::prelude::*;

use ng_rs_common::{
    traits::Router,
    types::Context
};

use reqwest::{RequestBuilder, Response, StatusCode, header};

//  API resolver

impl<'p> Router for Api<'p> {
    #[inline] fn router_parent(&self)->Option<&impl Router> {
        Some(self.ctx.as_ref())
    }
    #[inline] fn router_path(&self)->impl AsRef<str> {
        let v:&str = self.version.into();
        format!("/_guard/api/{v}")
    }
}

// API hooking

impl<'p> Api<'p> {
    /// Captures request and handles `send` with NG Guard detection.
    pub async fn req_send_guard(ctx: &'p Context,req:RequestBuilder)->reqwest::Result<Response> {
        let res = req.try_clone().unwrap().send().await?;
        if res.status() == StatusCode::FORBIDDEN
                && res.headers().get("content-encoding")
                    .is_some_and(|enc| enc == "gzip")
                && res.content_length().is_some_and(|len| len == 344) {
            
            std::hint::cold_path();
            let guard = Self::from(ctx);
            let challenge = guard.get_challenge().await.unwrap();
            let solution = challenge.solve(ChallengeSolverStopCond::Timeout(
                Duration::from_secs(10)
            )).unwrap();
            let _ = guard.verify_nonce(challenge, solution).await.ok();
            req.send().await
        } else { Ok(res) }
    }
}

// API definitions

impl<'p> Api<'p> {
    async fn get_challenge_raw(&self) -> Result<ChallengeRaw,ApiError> {
        Ok(self.ctx.session
            .get(self.route("/challenge"))
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
            .post(self.route("/verify"))
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
