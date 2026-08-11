// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Constant CSS selector `&'static str`s

/// `head` subtree classifier
pub(super) const GENERIC_HEAD:&'static str = "head";

/// Metadata details extraction
pub(super) const META:&'static str = r#"meta[property^="og:"]"#;
/// Additional metadata: CC license url
//pub(super) const META_LEGAL_CC:&'static str = "#creative_commons a";
/// Additional metadata: license text
//pub(super) const META_LEGAL_OTHER:&'static str = "#creative_commons .pod-body p";

/// Audio submission listing
pub(super) const ROOT_AUD:&'static str = "a.item-audiosubmission";

/// Audio submission details: title
pub(super) const TITLE_AUD:&'static str = ".item-details .detail-title h4";
/// Audio submission details: author
pub(super) const AUTHOR_AUD:&'static str = ".item-details .detail-title strong";
