// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Interpret CLI arguments

use std::{cell::LazyCell, env::args, ops::Index, process::exit};
use super::op::{*,types::*};

use ng_rs::aud::api::{
    MusicApi,
    list::ListKind
};

/*enum CliArg {
    Op(CliOp),
    Flag // TBD
}*/


/// # `parse_cli`
/// 
/// Parses CLI and executes its functionality
/// 
/// **Important**: This is a prototype implementation to test API,
/// in the future it might be decided to use some CLI framework
/// to handle more granually the terminal dimansions, ANSI escape
/// sequences (mainly, colors and text modifiers) and overall
/// text formatting.
/// 
pub async fn parse_cli() {
    let mut argv:Vec<String> = args().collect();
    argv.reverse();
    let argv0 = argv.pop().unwrap_or(env!("CARGO_PKG_NAME").to_string());
    if argv.len() < 1 {
        eprintln!("{}: No operation specified",argv0);
        exit(1);
    }
    let api = LazyCell::new(|| MusicApi::default());
    let mut last = Option::<Box<str>>::None;
    let mut last_ids = Vec::<usize>::with_capacity(30usize);
    while let Some(op) = argv.pop() { use CliOp::*; match CliOp::try_from(op) {
        Ok(op) => match op {
            Help => return help(&argv0),
            Version => return version(),
            List => {
                if let Some(cat) = argv.pop() {
                    let list = ListKind::from(cat.as_str());
                    let offset_raw = argv.pop();
                    let offset = offset_raw.map_or(0, |v| {
                        v.parse().unwrap_or_else(|_| {
                            argv.push(v);
                            0
                        })
                    });
                    let mut this = String::new();
                    last_ids.clear();
                    for res in api.list_audio(list, offset).await {
                        last_ids.push(res.id);
                        this += (format!(
                            "\x1b[92m[{id}]\x1b[0m \x1b[96m{title}\x1b[0m\n          by \x1b[0;1m{author}\x1b[0m\n",
                            author=res.author,
                            title=res.title,
                            id=res.id
                        )+"\n").as_str();
                    };
                    last = Some(this.into_boxed_str());
                } else {
                    eprintln!("{}: No category specified",argv0);
                    exit(1);
                }
            }
            Query => {
                if let Some(id) = argv.pop().map(|v| {
                            let num = v.strip_prefix('$')
                                .unwrap_or(v.as_str())
                                .parse::<usize>()
                                .unwrap();
                            if v.starts_with("$") { last_ids.index(num).to_owned() }
                            else { num }
                        }) {
                    let aud = api.submission_get_details(id).await.unwrap();
                    let mut this;
                    this = format!("\
                        Id          : {id}\n\
                        Title       : {title}\n\
                    ",
                        id = id,
                        title = aud.title
                    );
                    if let Some(desc) = aud.description {
                        this += format!("Description : {}\n",desc).as_str();
                    }
                    this += format!("\
                        Audio       : {src}\n\
                        Audio.Mime  : {mime}\n\
                    ",
                        src = aud.audio.src,
                        mime = aud.audio.mime
                    ).as_str();
                    last = Some(this.into_boxed_str());
                    last_ids.clear();
                }
            }
            op => {
                let (short,long) = op.into();
                todo!("{} {{{} {}}}",&argv0,short,long)
            }
        }
        Err(e) => {
            eprintln!("{}: {}",argv0,e);
            exit(1);
        }
    }}
    if let Some(last) = last { print!("{}",last); }
}
