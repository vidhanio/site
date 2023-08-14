mod link;
mod post;

use std::fmt::{self, Display, Formatter};

use once_cell::sync::Lazy;
use pulldown_cmark::{Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use regex::Regex;
use serde::Deserialize;
use time::{format_description::FormatItem, macros::format_description, Date};

pub use self::{link::BlogLink, post::BlogPost};
use crate::error::Error;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BlogPostMetadata {
    pub title: String,
    pub date: Date,
    pub description: String,
}

impl BlogPostMetadata {
    pub fn date_text(&self) -> String {
        static FORMAT_DESCRIPTION: &[FormatItem<'static>] =
            format_description!("[month repr:long] [day padding:none], [year]");

        self.date
            .format(FORMAT_DESCRIPTION)
            .expect("date formatting should not fail")
    }

    pub fn from_markdown(slug: &BlogSlug, md: &str) -> crate::Result<Self> {
        let metadata_str = Parser::new_ext(md, Options::all())
            .skip_while(|event| {
                *event != Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle))
            })
            .take_while(|event| {
                *event != Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle))
            })
            .map(|event| match event {
                Event::Text(text) => Ok(text),
                _ => Err(Error::UnexpectedMarkdownTag),
            })
            .skip(1) // skip the start tag
            .try_fold(None::<String>, |acc, text| {
                let mut acc = acc.unwrap_or_default();
                acc.push_str(&text?);
                Ok::<_, Error>(Some(acc))
            })?
            .ok_or_else(|| Error::NoPostMetadata(slug.clone()))?;

        serde_yaml::from_str(&metadata_str)
            .map_err(|e| Error::DeserializePostMetadata(slug.clone(), e))
    }
}

#[derive(Clone, Debug)]
pub struct BlogSlug(String);

static SLUG_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z0-9-]+$").expect("static regex should be valid"));

impl BlogSlug {
    pub fn new(slug: String) -> crate::Result<Self> {
        if SLUG_REGEX.is_match(&slug) {
            Ok(Self(slug))
        } else {
            Err(Error::InvalidBlogSlug(slug))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<BlogSlug> for String {
    fn from(slug: BlogSlug) -> Self {
        slug.0
    }
}

impl Display for BlogSlug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'de> Deserialize<'de> for BlogSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let slug = String::deserialize(deserializer)?;
        Self::new(slug).map_err(serde::de::Error::custom)
    }
}
