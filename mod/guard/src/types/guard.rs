// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::borrow::Cow;
use super::version::GuardApiVersion;
use ng_rs_common::types::ApiCtx;

/// Newgrounds Guard API, based on Proof-of-Work concept.
///
/// This allows clients to pass proof-of-work challenges
/// and verify themselves for legitimate access.
///
/// **Note:** Not yet a complete implementation!
pub struct Api<'p> {
    pub(crate) ctx: Cow<'p,ApiCtx>,
    pub(crate) version: GuardApiVersion,
}

impl<'p> Default for Api<'p> {
    fn default()->Self {
        Self {
            ctx: Cow::Owned(Default::default()),
            version: Default::default(),
        }
    }
}

impl<'p> From<GuardApiVersion> for Api<'p> {
    fn from(version: GuardApiVersion) -> Self {
        Self {
            ctx: Default::default(),
            version,
        }
    }
}

impl<'p> From<&'p ApiCtx> for Api<'p> {
    fn from(ctx: &'p ApiCtx) -> Self {
        Self {
            ctx: Cow::Borrowed(ctx),
            version: Default::default(),
        }
    }
}

impl<'p> From<ApiCtx> for Api<'p> {
    fn from(ctx: ApiCtx) -> Self {
        Self {
            ctx: Cow::Owned(ctx),
            version: Default::default(),
        }
    }
}
