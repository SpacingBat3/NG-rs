// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! # Submission API
//!
//! This implements methods related to browsing data
//! and metadata of single submission.
use reqwest::header;
use scraper::{Html, Selector};

use ng_rs_common::prelude::*;

use crate::types::api::submission::{
    AudioDetails,
    AudioDetailsFetchError
};

use super::*;

impl<'p> MusicApi<'p> {
    pub async fn submission_get_details<'m>(&'m self, id: usize)->Result<AudioDetails,AudioDetailsFetchError<'m>> {
        let mus_path = format!("listen/{}",id);
        let endpoint = self
            .get_route(mus_path.as_str())
            .expect("Failed to get API route");
        let req = self.ctx.session.get(endpoint)
            .header(header::ACCEPT, "text/html")
            .send();
        let [head_select, meta_select] = {
            use selectors::*;
            let [s1, s2] = [GENERIC_HEAD, META]
                .map(|v| Selector::parse(v)
                    .or_else( |e| Err(AudioDetailsFetchError::Selector(e)) )
                );
            [s1?, s2?]
        };
        let mut builder = AudioDetails::builder();
        let html = Html::parse_document(
            req.await?.text().await?.as_str()
        );
        for element in html
                .select(&head_select)
                .next()
                .ok_or(AudioDetailsFetchError::HtmlNoHead)?
                .select(&meta_select) {
            if let (Some(property),Some(content)) = (element.attr("property"),element.attr("content")) {
                match property {
                    "og:title"        => {builder.set_title(content.into());},
                    "og:description"  => {builder.set_description(content.to_string());},
                    "og:image"        => {builder.set_image_src(content.to_string());},
                    "og:image:alt"    => {builder.set_image_alt(content.to_string());},
                    "og:image:width"  => {builder.set_image_width(content.parse().unwrap());},
                    "og:image:height" => {builder.set_image_height(content.parse().unwrap());},
                    "og:image:type"   => {builder.set_image_mime(content.into());},
                    "og:audio:type"   => {builder.set_audio_mime(content.into());},
                    "og:audio"        => {builder.set_audio_src(content.into());},
                    _                 => ()
                };
            }
        }
        builder
            .build()
            .or_else(|e| Err(AudioDetailsFetchError::Build(e)))
    }
}
