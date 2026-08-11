// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! NG-rs "Audio" API
//! ========================================================================
//!
//! An implementation of client for Newgrounds `/audio` submission browsing,
//! browsing metadata for a given submission and getting information
//! neccesary to preview the submission, for native applications in need of
//! such APIs.
//!
//! Remarks
//! ------------------------------------------------------------------------
//! Implementation of this API is mostly made by the HTML DOM traversal via
//! selectors, which still requires DOM parsing to gather information,
//! however it might be faster
//!
//! License
//! ------------------------------------------------------------------------
//!
//! © 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
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

mod types;
pub mod api;
