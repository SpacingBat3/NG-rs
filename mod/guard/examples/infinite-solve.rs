// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::thread::sleep;
use std::time::Duration;

use ng_rs_guard::{
    types::*,
    demo::*
};

fn main() {
    println!("\n==> Calculate Proof-of-Work...");
    let generator = ChallengeGenerator {};
    println!("--- Generating challenges infinitely...");
    let mut avg: Option<(f64,f64)> = None;
    for challenge in generator {
        sleep(Duration::from_secs_f64(0.5));
        let solution = challenge
            .solve(ChallengeSolverStopCond::Timeout(Duration::from_secs(10)))
            .unwrap();
        let perf = get_challenge_performance(&solution);
        println!("--- {:?}", solution);
        println!("--- Performance: {:.0} nonce/s", perf);
        if let Some(mut average) = avg {
            average.0 += perf;
            average.1 += 1.;
            println!("    (avg. {:.0} nonce/s)", average.0/average.1);
            avg = Some(average);
        } else {
            avg = Some((perf,1.));
        }
    }
}
