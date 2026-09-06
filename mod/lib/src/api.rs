// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub use ng_rs_common::types::Context as Ctx;
#[cfg(feature = "aud")]
pub use ng_rs_aud::api::MusicApi as Music;
#[cfg(feature = "guard")]
pub use ng_rs_guard::api::GuardApi as Guard;
