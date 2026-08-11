// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod op;
mod env;

#[tokio::main]
async fn main() {
    env::parse_cli().await;
}
