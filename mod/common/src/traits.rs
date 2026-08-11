// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use url::{Url,ParseError};

/// Express that this structure is an API root, which
/// defines common origin for the API.
pub trait ApiRoot {
    /// Common origin for API.
    ///
    /// Used by API routers as URL base when resolving.
    const ORIGIN:&'static str;
    /// Gets origin from this API router
    #[inline] fn get_origin(&self)->&str { Self::ORIGIN }
}

/// Express that this structure is an API router, that
/// resolves route to endpoints and (usually) operates
/// on them.
pub trait Router {
    /// A root element type, from which this root originates.
    type  RouterRoot:ApiRoot;
    /// A path on which router operates.
    const ROUTER_PATH:&'static str;
    /// Gets route to relative "endpoint" path from this router.
    fn get_route(&self, endpoint: &str) -> Result<Url,ParseError> {
        Url::parse(Self::RouterRoot::ORIGIN)
            ?.join((String::from(Self::ROUTER_PATH.trim_end_matches('/'))+"/").as_str())
            ?.join(endpoint)
    }
}

/// A builder definition to craft `Buildable` structures.
///
/// Immutable instances for this type make no sense.
pub trait Builder: Sized+Default {
    type Output:Buildable<Self>;
    type Error;
    fn build(self)->Result<Self::Output,Self::Error>
    where
        Self::Output: TryFrom<Self>,
        Self::Error: From<<Self::Output as TryFrom<Self>>::Error>

    {
        Ok(Self::Output::try_from(self)?)
    }
}

/// Marks `T` in this structure as associated builder.
pub trait Buildable<T:Builder>:Sized {
    #[inline] fn builder()->T { T::default() }
}
