// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use reqwest::header;
use scraper::{Html,Selector};

use super::*;
use crate::types::api::list::AudioList;
pub use crate::types::api::list::ListKind;

impl<'p> MusicApi<'p> {
    // FIXME: category = enum { ... }
    pub async fn list_audio(&self, category:ListKind, offset:usize)->Box<[AudioList]> {
        let mut url = self.get_route(category.into()).unwrap();
        url.set_query(Some(format!("offset={}",offset).as_str()));
        let req = self.ctx.session.get(url)
            .header(header::ACCEPT, "text/html")
            .send();
        let [root,title,author] = {
            use selectors::*;
            let [root,title,author] = [ROOT_AUD,TITLE_AUD,AUTHOR_AUD]
                .map(|s| Selector::parse(s).unwrap()); // FIXME
            [root,title,author]
        };
        let html = Html::parse_document(req
            .await.expect("Audio listing failed on request")
            .text()
            .await.expect("Audio listing failed on body parse")
            .as_str()
        );
        html.select(&root).map(|root| {
            let id:usize = root
                .attr("href").expect("Unexpected partial listing")
                .rsplit_once("/").expect("Unexpected href without delimiter")
                .1.parse().expect("Could not parse 'id' as integer");
            let title = root.select(&title)
                .next().expect("Title not found")
                .inner_html().into_boxed_str();
            let author = root.select(&author)
                .next().expect("Author not found")
                .inner_html().into_boxed_str();
            AudioList { id, title, author }
        }).collect()
    }
}
