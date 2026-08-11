// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

cfg_select! {
    feature = "rustcrypto" => {
        pub const CRYPTO_PROVIDER:&'static str = "rustcrypto";
    }
    feature = "openssl" => {
        pub const CRYPTO_PROVIDER:&'static str = "openssl";
    }
}
