use hypertext::{Buffer, Raw, prelude::*};

use crate::{SiteResult, document::DocumentDetails, error::SiteError};

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

#[derive(Debug, Clone, Copy)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    date: (u16, u8, u8),
    pub image: &'static [u8],
    content: Raw<&'static str>,
    footnotes: &'static [(&'static str, Raw<&'static str>)],
}

impl Post {
    pub const ALL: &'static [Self] = posts::ALL;

    pub fn get(slug: String) -> SiteResult<Self> {
        Self::ALL
            .iter()
            .find(|post| post.slug == slug)
            .copied()
            .ok_or_else(|| SiteError::PostNotFound(slug))
    }

    pub fn time_element(&self) -> impl Renderable {
        maud! {
            time datetime={
                (format_args!(
                    "{:04}-{:02}-{:02}",
                    self.date.0, self.date.1, self.date.2
                ))
            } {
                (format_args!("{:04}/{:02}/{:02}", self.date.0, self.date.1, self.date.2))
            }
        }
    }
}

impl Renderable for Post {
    fn render_to(&self, output: &mut Buffer) {
        maud! {
            article ."prose" {
                header {
                    h1 {
                        (self.title)
                    }
                    (self.time_element())
                }

                hr;

                section #content {
                    (self.content)
                }


                @if !self.footnotes.is_empty() {
                    hr;

                    section #footnotes {
                        h2 #footnotes {
                            a href="#footnotes" { "footnotes" }
                        }

                        ul {
                            @for &(name, content) in self.footnotes {
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
        Self::new(post.title, format!("/post/{}/og.png", post.slug), post)
    }
}
