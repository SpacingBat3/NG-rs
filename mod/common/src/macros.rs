// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

/// A convenient `match`-like macro for `From<...>` syntax sugar for
/// enums wrapping other types.
#[macro_export]
macro_rules! map_err {
    ($e:ty=>$o:ident::$c:ident) => { impl From<$e> for $o {
        fn from(value: $e) -> $o {
            $o::$c(value)
        }
    }};
    ($e:ty=>$o:ident::$c:ident,$($e2:ty=>$o2:ident::$c2:ident),+) => {
        map_err!{ $e=>$o::$c } map_err!{ $($e2=>$o2::$c2),+ }
    };
}
