// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod version;
pub(crate) mod error;
pub(crate) mod challenge;
pub(crate) mod verify;
pub(crate) mod guard;

pub use challenge::{
    Challenge,
    ChallengeSolverStopCond
};

pub use error::ApiError;
