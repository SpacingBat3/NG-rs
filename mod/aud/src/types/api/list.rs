// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub use super::search::AudioSearch as AudioList;

pub enum ListKind {
    Featured,
    Latest,
    Popular
}



impl Into<&'static str> for ListKind {
    fn into(self) -> &'static str {
        match self {
            Self::Featured => "featured",
            Self::Latest   => "browse",
            Self::Popular  => "popular"
        }
    }
}

impl From<&str> for ListKind {
    fn from(value: &str) -> Self {
        match value {
            "popular" => Self::Popular,
            "browse"|"latest" => Self::Latest,
            _ => Self::Featured
        }
    }
}
