// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub use ng_rs_common::types::Context as MusicApiCtx;
pub use crate::types::api::Api as MusicApi;

use ng_rs_common::traits::Router;

impl<'p> Router for MusicApi<'p> {
    #[inline] fn router_path(&self)->impl AsRef<str> {"/audio"}
    #[inline] fn router_parent(&self)->Option<&impl Router> { Some(self.ctx.as_ref()) }
}

mod selectors;
pub mod submission;
pub mod list;
//mod search; <- TBD
