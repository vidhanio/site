use std::borrow::Cow;

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use hypertext::{Raw, prelude::*};

use crate::{SiteResult, error::SiteError};

std::cfg_select! {
    feature = "reload" => {
        #[derive(Debug)]
        pub struct Assets(vidhan_site_assets::Assets);

        #[derive(Debug, Clone)]
        pub struct Post(vidhan_site_assets::ProcessedPost);
    }
    _ => {
        mod fonts {
            include!(concat!(env!("OUT_DIR"), "/fonts.rs"));
        }
        mod media {
            include!(concat!(env!("OUT_DIR"), "/media.rs"));
        }
        mod posts {
            include!(concat!(env!("OUT_DIR"), "/posts.rs"));
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Assets;

        #[derive(Debug, Clone, Copy)]
        pub struct EmbeddedPost {
            pub slug: &'static str,
            pub title: &'static str,
            pub date: (u16, u8, u8),
            pub og_image: &'static [u8],
            pub content: Raw<&'static str>,
            pub footnotes: &'static [(&'static str, Raw<&'static str>)],
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Post(&'static EmbeddedPost);
    }
}

#[cfg_attr(
    not(feature = "reload"),
    expect(
        clippy::missing_const_for_fn,
        clippy::unused_self,
        reason = "embedded and reload assets share one API"
    )
)]
impl Assets {
    pub fn logo_light(self) -> Cow<'static, str> {
        std::cfg_select! {
            feature = "reload" => Cow::Owned(self.0.logo_light_svg),
            _ => Cow::Borrowed(include_str!(concat!(env!("OUT_DIR"), "/logo-light.svg"))),
        }
    }

    pub fn logo_dark(self) -> Cow<'static, str> {
        std::cfg_select! {
            feature = "reload" => Cow::Owned(self.0.logo_dark_svg),
            _ => Cow::Borrowed(include_str!(concat!(env!("OUT_DIR"), "/logo-dark.svg"))),
        }
    }

    pub fn favicon(self) -> Cow<'static, [u8]> {
        std::cfg_select! {
            feature = "reload" => Cow::Owned(self.0.favicon),
            _ => Cow::Borrowed(include_bytes!(concat!(env!("OUT_DIR"), "/favicon.ico"))),
        }
    }

    pub fn style(self) -> Cow<'static, str> {
        std::cfg_select! {
            feature = "reload" => Cow::Owned(self.0.style),
            _ => Cow::Borrowed(include_str!(concat!(env!("OUT_DIR"), "/style.css"))),
        }
    }

    pub fn open_graph_image(self) -> Cow<'static, [u8]> {
        std::cfg_select! {
            feature = "reload" => Cow::Owned(self.0.og_image),
            _ => Cow::Borrowed(include_bytes!(concat!(env!("OUT_DIR"), "/og.png"))),
        }
    }

    pub fn resume(self) -> Cow<'static, [u8]> {
        std::cfg_select! {
            feature = "reload" => Cow::Owned(self.0.resume),
            _ => Cow::Borrowed(include_bytes!(concat!(env!("OUT_DIR"), "/resume.pdf"))),
        }
    }

    pub fn media(self, name: &str) -> Option<(&'static str, Cow<'static, [u8]>)> {
        std::cfg_select! {
            feature = "reload" => {
                let mut media = self.0.media;
                media
                    .remove(name)
                    .map(|asset| (asset.content_type, Cow::Owned(asset.bytes)))
            }
            _ => media::get(name).map(|(content_type, bytes)| (content_type, Cow::Borrowed(bytes))),
        }
    }

    pub fn font(self, name: &str) -> Option<(&'static str, Cow<'static, [u8]>)> {
        std::cfg_select! {
            feature = "reload" => {
                let mut fonts = self.0.fonts;
                fonts
                    .remove(name)
                    .map(|asset| (asset.content_type, Cow::Owned(asset.bytes)))
            }
            _ => fonts::get(name).map(|(content_type, bytes)| (content_type, Cow::Borrowed(bytes))),
        }
    }

    pub fn posts(&self) -> impl Iterator<Item = (&str, &str)> {
        std::cfg_select! {
            feature = "reload" => self
                .0
                .posts
                .iter()
                .map(|post| (post.slug.as_str(), post.title.as_str())),
            _ => posts::ALL.iter().map(|post| (post.slug, post.title)),
        }
    }

    pub fn post(self, slug: String) -> SiteResult<Post> {
        std::cfg_select! {
            feature = "reload" => self
                .0
                .posts
                .into_iter()
                .find(|post| post.slug == slug)
                .map(Post)
                .ok_or_else(|| SiteError::PostNotFound(slug)),
            _ => posts::ALL
                .iter()
                .find(|post| post.slug == slug)
                .map(Post)
                .ok_or_else(|| SiteError::PostNotFound(slug)),
        }
    }
}

impl Post {
    #[cfg_attr(
        not(feature = "reload"),
        expect(clippy::missing_const_for_fn, reason = "reload posts own strings")
    )]
    pub fn slug(&self) -> &str {
        std::cfg_select! {
            feature = "reload" => &self.0.slug,
            _ => self.0.slug,
        }
    }

    #[cfg_attr(
        not(feature = "reload"),
        expect(clippy::missing_const_for_fn, reason = "reload posts own strings")
    )]
    pub fn title(&self) -> &str {
        std::cfg_select! {
            feature = "reload" => &self.0.title,
            _ => self.0.title,
        }
    }

    #[cfg_attr(
        not(feature = "reload"),
        expect(
            clippy::trivially_copy_pass_by_ref,
            reason = "post representations share one accessor API"
        )
    )]
    pub const fn date(&self) -> (u16, u8, u8) {
        self.0.date
    }

    #[cfg_attr(
        not(feature = "reload"),
        expect(clippy::missing_const_for_fn, reason = "reload posts own strings")
    )]
    pub fn content(&self) -> Raw<&str> {
        std::cfg_select! {
            feature = "reload" => Raw::dangerously_create(&self.0.content),
            _ => self.0.content,
        }
    }

    pub const fn footnotes(&self) -> Footnotes<'_> {
        Footnotes {
            post: self,
            index: 0,
        }
    }

    #[cfg_attr(
        not(feature = "reload"),
        expect(
            clippy::trivially_copy_pass_by_ref,
            reason = "post representations share one accessor API"
        )
    )]
    pub const fn has_footnotes(&self) -> bool {
        !self.0.footnotes.is_empty()
    }

    #[cfg_attr(
        not(feature = "reload"),
        expect(clippy::missing_const_for_fn, reason = "reload posts own images")
    )]
    pub fn og_image(self) -> Cow<'static, [u8]> {
        std::cfg_select! {
            feature = "reload" => Cow::Owned(self.0.og_image),
            _ => Cow::Borrowed(self.0.og_image),
        }
    }
}

#[derive(Debug)]
pub struct Footnotes<'a> {
    post: &'a Post,
    index: usize,
}

impl<'a> Iterator for Footnotes<'a> {
    type Item = (&'a str, Raw<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        let item = std::cfg_select! {
            feature = "reload" => {
                self.post
                    .0
                    .footnotes
                    .get(self.index)
                    .map(|(name, content)| {
                        (name.as_str(), Raw::dangerously_create(content.as_str()))
                    })
            }
            _ => self
                .post
                .0
                .footnotes
                .get(self.index)
                .map(|(name, content)| (*name, *content)),
        };

        self.index += usize::from(item.is_some());
        item
    }
}

impl<S> FromRequestParts<S> for Assets
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    fn from_request_parts(
        _parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        std::future::ready(std::cfg_select! {
            feature = "reload" => {
                vidhan_site_assets::load(env!("CARGO_MANIFEST_DIR"))
                    .map(Self)
                    .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
            }
            _ => Ok(Self),
        })
    }
}

#[derive(Debug, Clone, Copy, Renderable)]
#[attribute((self.0) (cache_buster()))]
pub struct Cached<'a>(pub &'a str);

const fn cache_buster() -> &'static str {
    std::cfg_select! {
        feature = "reload" => "",
        _ => concat!("?v=", env!("GIT_COMMIT_HASH")),
    }
}
