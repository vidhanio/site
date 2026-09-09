use hypertext::{Buffer, Raw, prelude::*};

use crate::{SiteResult, document::DocumentDetails, error::SiteError};

#[cfg(not(feature = "reload"))]
include!(concat!(env!("OUT_DIR"), "/posts.rs"));

#[cfg(not(feature = "reload"))]
#[derive(Debug, Clone, Copy)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    date: (u16, u8, u8),
    pub image: &'static [u8],
    content: Raw<&'static str>,
    footnotes: &'static [(&'static str, Raw<&'static str>)],
}

#[cfg(feature = "reload")]
#[derive(Debug, Clone, Copy)]
pub struct Post(&'static vidhan_site_assets::ProcessedPost);

impl Post {
    pub fn all() -> impl Iterator<Item = Self> {
        #[cfg(not(feature = "reload"))]
        {
            posts::ALL.iter().copied()
        }

        #[cfg(feature = "reload")]
        {
            crate::assets::get().posts.iter().map(Self)
        }
    }

    pub fn get(slug: String) -> SiteResult<Self> {
        Self::all()
            .find(|post| post.slug() == slug)
            .ok_or_else(|| SiteError::PostNotFound(slug))
    }

    pub const fn slug(self) -> &'static str {
        #[cfg(not(feature = "reload"))]
        {
            self.slug
        }

        #[cfg(feature = "reload")]
        {
            self.0.slug.as_str()
        }
    }

    pub const fn title(self) -> &'static str {
        #[cfg(not(feature = "reload"))]
        {
            self.title
        }

        #[cfg(feature = "reload")]
        {
            self.0.title.as_str()
        }
    }

    pub const fn image(self) -> &'static [u8] {
        #[cfg(not(feature = "reload"))]
        {
            self.image
        }

        #[cfg(feature = "reload")]
        {
            self.0.image.as_slice()
        }
    }

    const fn date(self) -> (u16, u8, u8) {
        #[cfg(not(feature = "reload"))]
        {
            self.date
        }

        #[cfg(feature = "reload")]
        {
            self.0.date
        }
    }

    const fn content(self) -> Raw<&'static str> {
        #[cfg(not(feature = "reload"))]
        {
            self.content
        }

        #[cfg(feature = "reload")]
        {
            Raw::dangerously_create(self.0.content.as_str())
        }
    }

    fn footnotes(self) -> impl Iterator<Item = (&'static str, Raw<&'static str>)> {
        #[cfg(not(feature = "reload"))]
        {
            self.footnotes
                .iter()
                .map(|&(name, content)| (name, content))
        }

        #[cfg(feature = "reload")]
        {
            self.0
                .footnotes
                .iter()
                .map(|(name, content)| (name.as_str(), Raw::dangerously_create(content.as_str())))
        }
    }

    const fn has_footnotes(self) -> bool {
        #[cfg(not(feature = "reload"))]
        {
            !self.footnotes.is_empty()
        }

        #[cfg(feature = "reload")]
        {
            !self.0.footnotes.is_empty()
        }
    }

    pub fn time_element(&self) -> impl Renderable {
        let (year, month, day) = self.date();
        maud! {
            time datetime={ (format_args!("{year:04}-{month:02}-{day:02}")) } {
                (format_args!("{year:04}/{month:02}/{day:02}"))
            }
        }
    }
}

impl Renderable for Post {
    fn render_to(&self, output: &mut Buffer) {
        maud! {
            article {
                header {
                    h1 { (self.title()) }
                    (self.time_element())
                }

                hr;
                section #content { (self.content()) }

                @if self.has_footnotes() {
                    hr;
                    section {
                        h2 #footnotes {
                            a href="#footnotes" { "footnotes" }
                        }

                        ul {
                            @for (name, content) in self.footnotes() {
                                li #{ "footnote-" (name) } {
                                    p {
                                        a.footnote href={ "#footnote-" (name) } {
                                            strong { "[" (name) "]" }
                                        }
                                        " "
                                        (content)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        .render_to(output);
    }
}

impl From<Post> for DocumentDetails<Post> {
    fn from(post: Post) -> Self {
        Self::new(
            post.title().to_owned(),
            format!("/post/{}/og.png", post.slug()),
            post,
        )
    }
}
