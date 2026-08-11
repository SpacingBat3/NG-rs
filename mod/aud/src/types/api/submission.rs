// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Types related to single submission detail previews

use std::ops::{BitOr,BitOrAssign};

use ng_rs_common::types::{
    RemoteFileData as RemoteAudio,
    RemoteFileData as RemoteImageData,
    RemoteImage,
};

// Details

pub struct AudioDetails {
    pub title: Box<str>,
    pub description: Option<Box<str>>,
    pub cover: RemoteImage,
    pub audio: RemoteAudio
}

// DetailsBuilders

#[derive(Default)]
pub(crate) struct AudioDetailsBuilder {
    title: Option<String>,
    description: Option<String>,
    image: AudioCoverBuilder,
    audio: AudioFileBuilder
}

#[derive(Default)]
struct AudioCoverBuilder {
    file: AudioFileBuilder,
    alt: Option<String>,
    dim: PartialDims,
}

#[derive(Default)]
struct AudioFileBuilder {
    src:  Option<String>,
    mime: Option<String>
}

// Helper types
#[derive(Clone,Copy,Default)]
enum PartialDims {
    #[default]
    None,
    Width(u32),
    Height(u32),
    All(u32,u32)
}

#[derive(Debug)]
pub enum AudioDetailsBuildError {
    MissingParamError(&'static str),
    MissingDimWidth,
    MissingDimHeight,
    MissingDims
}

#[derive(Debug)]
pub enum AudioDetailsFetchError<'a> {
    Selector(scraper::error::SelectorErrorKind<'a>),
    Fetch(reqwest::Error),
    Build(AudioDetailsBuildError),
    HtmlNoHead
}

impl From<reqwest::Error> for AudioDetailsFetchError<'_> {
    fn from(value: reqwest::Error) -> Self {
        AudioDetailsFetchError::Fetch(value)
    }
}

macro_rules! derive_optional_str {
    ($self:ident,$($key:ident).+) => {
        $self.$($key).+
            .ok_or(MissingParamError(stringify!($($key).+)))?
            .into_boxed_str()
    };
}

impl TryInto<(u32,u32)> for PartialDims {
    type Error = AudioDetailsBuildError;
    fn try_into(self) -> Result<(u32,u32), Self::Error> {
        use PartialDims::*;
        use AudioDetailsBuildError::*;
        match self {
            None => Err(MissingDims),
            Width(_) => Err(MissingDimHeight),
            Height(_) => Err(MissingDimWidth),
            All(w,h) => Ok((w,h))
        }
    }
}

impl BitOr for PartialDims {
    /// Produces an union of two dims
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        use PartialDims::*;
        match (self,rhs) {
            // 1. Dim union
            (Width(w),Height(h)) | (Height(h),Width(w)) |
            // 2. Rhs partial replace
            (All(_,h),Width(w)) | (All(w,_),Height(h))
                => All(w,h),
            // 3. Null elimination
            (any,None) | (None,any) => any,
            // 4. Rhs full replace
            (_,replace) => replace
        }
    }
}

impl BitOrAssign for PartialDims {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

// Setters
impl AudioDetailsBuilder {
    // Title
    #[inline] pub fn set_title(&mut self,title:String)->&mut Self {
        self.title = Some(title); self
    }
    // Description
    #[inline] pub fn set_description(&mut self,desc:String)->&mut Self {
        self.description = Some(desc); self
    }
    // Image
    #[inline] pub fn set_image_src(&mut self,src:String)->&mut Self {
        self.image.file.src = Some(src); self
    }
    #[inline] pub fn set_image_mime(&mut self,mime:String)->&mut Self {
        self.image.file.mime = Some(mime); self
    }
    #[inline] pub fn set_image_alt(&mut self,alt:String)->&mut Self {
        self.image.alt = Some(alt); self
    }
    #[inline] pub fn set_image_width(&mut self,w:u32)->&mut Self {
        self.image.dim |= PartialDims::Width(w); self
    }
    #[inline] pub fn set_image_height(&mut self,h:u32)->&mut Self {
        self.image.dim |= PartialDims::Height(h); self
    }
    /* #[inline] pub fn set_image_dims(&mut self,w:u32,h:u32)->&mut Self {
        self.image.dim |= PartialDims::All(w,h); self
    } <- unused */
    // Audio
    #[inline] pub fn set_audio_src(&mut self,src:String)->&mut Self {
        self.audio.src = Some(src); self
    }
    #[inline] pub fn set_audio_mime(&mut self,mime:String)->&mut Self {
        self.audio.mime = Some(mime); self
    }
}

// Builder
impl TryFrom<AudioDetailsBuilder> for AudioDetails {
    type Error = AudioDetailsBuildError;
    fn try_from(value:AudioDetailsBuilder) -> Result<AudioDetails,Self::Error> {
        use AudioDetailsBuildError::*;
        Ok(AudioDetails {
            title: derive_optional_str!(value,title),
            description: value.description.map(|str| str.into_boxed_str()),
            cover: RemoteImage {
                image: RemoteImageData {
                    src: derive_optional_str!(value,image.file.src),
                    mime: derive_optional_str!(value,image.file.mime),
                },
                alt: value.image.alt.map(|this| this.into_boxed_str()),
                dim: value.image.dim.try_into()?
            },
            audio: RemoteAudio {
                src: derive_optional_str!(value,audio.src),
                mime: derive_optional_str!(value,audio.mime),
            }
        })
    }
}

impl ng_rs_common::traits::Builder for AudioDetailsBuilder {
    type Output = AudioDetails;
    type Error  = AudioDetailsBuildError;
}

impl ng_rs_common::traits::Buildable<AudioDetailsBuilder> for AudioDetails {}
