// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Types related search queries

pub struct AudioSearch {
    pub id:     usize,
    pub title:  Box<str>,
    pub author: Box<str>,
    // image:  Option<!>
}
