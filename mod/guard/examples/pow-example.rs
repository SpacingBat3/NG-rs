// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;

use ng_rs_guard::{
    api::*,
    types::*
};

async fn pow_example() {
    println!("\n==> Calculate Proof-of-Work...");
    let api = GuardApi::default();
    println!("--- Fetching challenge...");
    let challenge = api
        .get_challenge()
        .await
        .unwrap();
    println!("--- {:?}", challenge);
    let solution = challenge
        .solve(ChallengeSolverStopCond::Timeout(Duration::from_secs(10)))
        .unwrap();
    println!("--- Solution (nonce): {:?}", solution);

    println!("\n==> Verify results...");
    let verify = api
        .verify_nonce(challenge,solution)
        .await.unwrap();

    println!("--- {:?}", verify);

    println!("\n==> Success for PoW API!");
    println!("--- Note: tokens still need to need validated. (not yet supported)\n")
}

cfg_select! {
    feature = "tokio-rt" => {
        #[tokio::main]
        async fn main() {
            pow_example().await
        }
    }
    feature = "smol-rt" => {
        fn main() {
            smol::block_on(async_compat::Compat::new(
                pow_example()
            ))
        }
    }
    _ => {
        compile_error!(concat!(
            "No runtime for async scoped execution! ",
            "Please enable 'tokio-rt' or 'smol-rt' features."
        ));
        fn main(){ panic!("Bad compilation: no async runtime!") }
    }
}
