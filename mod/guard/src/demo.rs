// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! "Demo" API is temporary **unstable** API to test
//! library performance, mainly how multi-threading
//! will perform.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::challenge::*;
use openssl::rand::rand_bytes;

pub struct ChallengeGenerator {}

impl Iterator for ChallengeGenerator {
    type Item = Challenge;
    fn next(&mut self) -> Option<Self::Item> {
        let payload = {
            let epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let hex = {
                let mut rng:[u8;4] = Default::default();
                rand_bytes(&mut rng).unwrap();
                let mut hex = String::with_capacity(rng.len()*2);
                const ALPHABET: &[u8; 16] = b"0123456789abcdef";
                for byte in rng {
                    hex.push(ALPHABET[((byte>>4) & 0xf) as usize] as char);
                    hex.push(ALPHABET[(byte & 0xf) as usize] as char);
                }
                hex
            };
            epoch.to_string() + "|" + hex.as_str()
        };
        Some(Challenge {
            algo: ChallengeAlgo::Argon2Id({
                Argon2IdParams {
                    hash_len: 32,
                    iters: 1,
                    mem_size: 4096,
                    parallel: 1
                }
            }),
            task: ChallengeTask {
                bits: 8,
                payload,
                sig: "DEMO".to_string(),
            }
        })
    }
}

pub fn get_challenge_performance(sol:&ChallengeSolution)->f64 {
    (1.+sol.nonce as f64)/sol.solve_time.as_secs_f64()
}
