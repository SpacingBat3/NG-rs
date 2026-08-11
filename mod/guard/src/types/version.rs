// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug,Clone,Copy,Default)]
pub enum GuardApiVersion {
    #[default]
    V1
}

impl TryFrom<u8> for GuardApiVersion {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1u8=>Ok(Self::V1),
            _=>Err(format!("Unsupported API version 'v{}'",value)),
        }
    }
}

impl Into<u8> for GuardApiVersion {
    fn into(self) -> u8 {
        match self {
            Self::V1 => 1u8,
        }
    }
}

impl Into<&'static str> for GuardApiVersion {
    fn into(self) -> &'static str {
        match self {
            Self::V1 => "v1"
        }
    }
}
