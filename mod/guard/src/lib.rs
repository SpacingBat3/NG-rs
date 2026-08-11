// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! NG-rs "Guard" API
//! ========================================================================
//!
//! A highly experimental client implementation for Newgrounds Guard API,
//! which uses proof-of-work to solve challenges against automatic server
//! requests.
//!
//! Remarks
//! ------------------------------------------------------------------------
//!
//! It is compatible with WASM, althouth a revision to introduce
//! `wasm-bindgen-rayon` (and `rayon` backend) for efficient problem
//! solving on modern MT-focused hardware design is required.
//!
//! It is not guaranteed to be operational for non-interactive
//! applications, and is not pursued towards this goal, therefore
//! using this as "aggresive counter-measure" is not guaranteed to
//! work.
//!
//! This might not be useful in context of this project, albeit service
//! provider (namely, Newgrounds.com) might take an interest in this
//! project and adopt it as official solution, if it'll offer better
//! performance and good compatibility browser-side.
//!
//! License
//! ------------------------------------------------------------------------
//!
//! Copyright (C) 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Compiler tests

cfg_select! {
    all(feature = "openssl", target_family = "wasm") => {
        compile_error!(concat!(
            "'openssl' is incompatible with WASM, ",
            "please use 'rustcrypto'."
        ));
    }
    not(any(feature="openssl", feature="rustcrypto")) => {
        compile_error!("At least one crypto impl is required.");
    }
    _=> {
        pub mod api;
        pub mod types;
        pub mod config;
        #[cfg(feature = "example-demo")]
        pub mod demo;
    }
}
