// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub enum CliOp {
    Help,
    Version,
    Query,
    List,
    Get,
    Search
}

#[derive(Debug)]
pub struct InvalidOpError(String);

// Foward map
impl TryFrom<&str> for CliOp {
    type Error = InvalidOpError;
    fn try_from(value: &str) -> Result<Self,Self::Error> {
        match value {
            "-h"|"--help"    => Ok(Self::Help),
            "-V"|"--version" => Ok(Self::Version),
            "-L"|"--list"    => Ok(Self::List),
            "-Q"|"--query"   => Ok(Self::Query),
            "-G"|"--get"     => Ok(Self::Get),
            "-S"|"--search"  => Ok(Self::Search),
            _    => Err(InvalidOpError(value.to_string()))
        }
    }
}

// Reverse map
impl Into<(&'static str,&'static str)> for CliOp {
    fn into(self) -> (&'static str,&'static str) {
        match self {
            Self::Help    => ("-h","--help"),
            Self::Version => ("-V","--version"),
            Self::List    => ("-L","--list"),
            Self::Query   => ("-Q","--query"),
            Self::Get     => ("-G","--get"),
            Self::Search  => ("-S","--search")
        }
    }
}

// Enum iterator
impl CliOp {
    pub fn list()-> impl Iterator<Item = Self> {
        use CliOp::*;
        [Help,Version,List,Query].into_iter()
    }
    pub fn get_help_params(&self)->&'static str {
        match self {
            Self::List  => " category [offset]",
            Self::Query => " id",
            _ => ""
        }
    }
}

impl TryFrom<String> for CliOp {
    type Error = InvalidOpError;
    fn try_from(value: String) -> Result<Self,Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::fmt::Display for InvalidOpError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "Invalid operaton -- '{}'", self.0)
    }
}

impl std::error::Error for InvalidOpError {}
