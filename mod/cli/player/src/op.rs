// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! CLI operations groups

pub(crate) mod types;

pub fn help(argv0:&str) {
    println!(concat!(
        "usage:  {} <operation> [...]\n",
        "operations:"
    ),argv0);
    for op in types::CliOp::list() {
        let args = op.get_help_params();
        let flags:(&str,&str) = op.into();
        println!(
            "  {me} {{{short} {long}}}{args}",
            me=argv0,
            short=flags.0,
            long=flags.1,
            args=args
        );
    }
    // FIXME: API?
    println!("categories: same as in NG (browse/latest,featured,popular)");
    println!(concat!(
        "\nYou may chain ops and use '${{N}}' to reffer to {{N}}th id,",
        "\n e.g. `-L latest -Q '$0'` will query latest submission."
    ))
}


/// Display current application version
pub fn version() {
    println!("\
\u{0020}_____   _____   \n\
       |  _  | |  ___|  NG-rs AUD (cli) v{ver}\n\
       | | | | | / ___  Copyright (C) 2026 SpacingBat3\n\
       | | | | | |_| |  \n\
       |_| |_| |_____|  Licensed under terms of GPLv3+ license\n\
\u{0020}                (SPDX: {license})
    ",ver=env!("CARGO_PKG_VERSION"),license=env!("CARGO_PKG_LICENSE"))
}
