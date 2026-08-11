// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! NG-rs
//! ======================================================================
//!
//! A Rust library to help you access the Newgrounds from your code!
//!
//! Remarks
//! -----------------------------------------------------------------------
//!
//! This is mostly implemented by scrapping and is supposed to at most as
//! resource-heavy for servers as just normal browsing via a web browser
//! – albeit that goal requires a complete API first, caching consideration
//! and persistent sessions.
//!
//! The goal of this scrapping library is to also respect as much service
//! provider as possible, in order to make a scrapping attempt easier to
//! control and investigate, as well as make legitimate attempts easier
//! to understand and control by potential website administrator.
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

pub mod api;

pub use ng_rs_common as common;
#[cfg(feature = "aud")]
pub use ng_rs_aud as aud;
#[cfg(feature = "guard_unstable")]
pub use ng_rs_guard as guard;
