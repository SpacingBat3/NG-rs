// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::borrow::Cow;
use ng_rs_common::types::ApiCtx;

pub(crate) mod submission;
pub(crate) mod search;
pub(crate) mod list;

pub struct Api<'p> {
    pub(crate) ctx: Cow<'p,ApiCtx>
}

impl Default for Api<'static> {
    fn default() -> Self {
        Self {
            ctx: Default::default()
        }
    }
}

impl<'p> From<&'p ApiCtx> for Api<'p> {
    fn from(ctx: &'p ApiCtx) -> Self {
        Self { ctx: Cow::Borrowed(ctx) }
    }
}

impl<'p> From<ApiCtx> for Api<'p> {
    fn from(ctx: ApiCtx) -> Self {
        Self { ctx: Cow::Owned(ctx) }
    }
}
