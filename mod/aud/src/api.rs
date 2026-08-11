// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub use ng_rs_common::types::ApiCtx as MusicApiCtx;
pub use crate::types::api::Api as MusicApi;

use ng_rs_common::traits::Router;

impl<'p> Router for MusicApi<'p> {
    type  RouterRoot = MusicApiCtx;
    const ROUTER_PATH:&'static str = "/audio";
}

mod selectors;
pub mod submission;
pub mod list;
//mod search; <- TBD
