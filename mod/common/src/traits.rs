// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

/// This is minimalistic runtime-driven path router, for defining
/// relationship between each types when defining common route paths.
/// FIXME: consider working on library for abstractions at
/// compile time via const/macro.
pub trait Router {
    fn router_path(&self)->impl AsRef<str>;
    fn router_parent(&self)->Option<&impl Router>;
    #[inline] fn route(&self, endpoint:&str)->String {
        let upper = if let Some(parent) = self.router_parent() {
            parent.route(self.router_path().as_ref())
        } else { self.router_path().as_ref().to_string() };
        let endpoint = endpoint.trim_start_matches('/');
        upper + "/" + endpoint
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
