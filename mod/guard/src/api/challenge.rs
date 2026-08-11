// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    sync::mpsc,
    thread
};

use crate::types::{
    challenge::*,
    error::*
};

impl Challenge {
    /// Checks if nonce fulfills the requirement
    fn check_nonce(&self,nonce:usize) -> Result<u32,ApiError> {
        let task = String::from(self.task.payload.as_str())
            + ":"
            + nonce.to_string().as_str();
        match self.algo {
            ChallengeAlgo::Argon2Id(params) => {
                let mut out = vec![0u8;params.hash_len as usize];
                cfg_select! {
                    feature = "rustcrypto" => {
                        use argon2::{
                            Argon2,
                            Algorithm,
                            ParamsBuilder,
                            Version
                        };
                        let algo_params = ParamsBuilder::new()
                            .t_cost(params.iters.into())
                            .m_cost(params.mem_size)
                            .p_cost(params.parallel.into())
                            .build()?;
                        let argon2id = Argon2::new(
                            Algorithm::Argon2id,
                            Version::default(),
                            algo_params
                        );
                        argon2id.hash_password_into(
                            task.as_bytes(),
                            &[0u8;8],
                            out.as_mut_slice()
                        )?;
                    }
                    feature = "openssl" => {
                        openssl::kdf::argon2id(
                            None,
                            task.as_bytes(),
                            &[0u8;8],
                            None,
                            None,
                            params.iters as u32,
                            params.parallel as u32,
                            params.mem_size,
                            out.as_mut_slice()
                        )?;
                    }
                }

                {
                    let mut zeros = 0u32;
                    for b in out {
                        if zeros > self.task.bits { break; }
                        if b == 0 { zeros += 8; }
                        else { zeros += b.leading_zeros(); break; }
                    };
                    Ok(zeros)
                }
            }
        }
    }
    /// Solves the PoW challenge in brute-force manner,
    /// using defined stop conditions.
    /// Uses multiple threads based on parallelism of
    /// the current platform and falls back to single-threaded
    /// execution if MT is not supported.
    pub fn solve(&self, cond: ChallengeSolverStopCond) -> Result<ChallengeSolution,ApiError> {
        let thread_num = usize::from(thread::available_parallelism()?);
        let (max_iter, timeout) = match cond {
            ChallengeSolverStopCond::Iterations(iters) => (iters,None),
            ChallengeSolverStopCond::Timeout(timeout) => (usize::MAX,Some(timeout)),
            ChallengeSolverStopCond::Both(timeout, iters) => (iters, Some(timeout))
        };
        enum Msg<T> {
            Alive,
            NotFound,
            Found(T)
        }
        let (prod,con) = mpsc::channel::<Msg<ChallengeSolution>>();
        // FIXME: If I won't find any similar MT task management design,
        //        make it a cool library with multiple backends and optional
        //        scoping :)
        for thr_id in 0..thread_num {
            let prod = prod.clone();
            let challenge = self.clone();
            let task = move || {
                let timer = std::time::Instant::now();
                for nonce in (thr_id..max_iter).step_by(thread_num) {
                    if prod.send(Msg::Alive).is_err() {
                        return None;
                    }
                    if challenge.check_nonce(nonce).ok()? >= challenge.task.bits {
                        let _ = prod.send(Msg::Found(ChallengeSolution {
                            nonce,
                            solve_time: timer.elapsed()
                        }));
                        return Some(());
                    }
                }
                let _ = prod.send(Msg::NotFound);
                None
            };
            if thread_num > 1 { thread::spawn(task); }
            else { task(); }
        }
        drop(prod);
        let recv:Box<dyn Fn() -> Msg<ChallengeSolution>> = match timeout {
            Some(timeout) => Box::new(move || con.recv_timeout(timeout).unwrap()),
            None => Box::new(move || con.recv().unwrap() ),
        };
        // Message consumption
        {
            let mut threads_left = thread_num;
            let mut cres = recv();
            while threads_left > 1 { match cres {
                Msg::Found(nonce) => { return Ok(nonce); },
                Msg::NotFound => threads_left-=1,
                Msg::Alive => {}
            }; cres = recv();}
        }
        Err(ApiError::NotFound)
    }
}
