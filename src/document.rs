use std::fmt::Debug;

use axum::{
    extract::{FromRequestParts, OriginalUri},
    http::Uri,
    response::{IntoResponse, Response},
};
use hypertext::prelude::*;

use crate::r#static::Cached;

#[derive(Debug, FromRequestParts)]
#[allow(clippy::module_name_repetitions)]
pub struct DocumentParts {
    path: OriginalUri,
}

impl DocumentParts {
    pub fn build<R: Renderable>(
        self,
        title: impl Into<String>,
        og_image: impl Into<String>,
        content: R,
    ) -> Document<R> {
        Document {
            path: Some(self.path.0),
            title: Some(title.into()),
            og_image: Some(og_image.into()),
            content,
        }
    }

    pub fn build_simple<R: Renderable>(self, content: R) -> Document<R> {
        Document {
            path: Some(self.path.0),
            title: None,
            og_image: None,
            content,
        }
    }
}

pub struct Document<R> {
    path: Option<Uri>,
    title: Option<String>,
    og_image: Option<String>,
    content: R,
}

impl<R: Renderable> Document<R> {
    pub fn new(title: impl Into<String>, content: R) -> Self {
        Self {
            path: None,
            title: Some(title.into()),
            og_image: None,
            content,
        }
    }
}

impl<R: Renderable> Renderable for Document<R> {
    fn render_to(&self, output: &mut String) {
        const DESCRIPTION: &str = "vidhan's home on the internet.";

        maud! {
            !DOCTYPE
            html lang="en" id {
                head {
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta charset="utf-8";

                    title { "vidhan.io / " (self.title) }
                    meta name="description" content=(DESCRIPTION);
                    meta name="theme-color" content="#753bbd";

                    meta name="og:title" content={
                        @if let Some(title) = &self.title {
                            title
                        } @else {
                            "vidhan.io"
                        }
                    };
                    meta name="og:description" content=(DESCRIPTION);
                    meta name="og:url" content={ "https://vidhan.io" (self.path) };
                    meta name="og:type" content="website";
                    meta name="og:image" content={
                        @if let Some(path) = &self.og_image {
                            "https://vidhan.io" (Cached(path))
                        } @else {
                            (Cached("https://vidhan.io/og.png"))
                        }
                    };
                    meta name="twitter:card" content="summary_large_image";
                    meta name="twitter:site" content="@vidhanio";
                    meta name="twitter:creator" content="@vidhanio";

                    link rel="stylesheet" href=(Cached("/style.css"));

                    link rel="icon" type="image/svg+xml" href=(Cached("/favicon.svg"));
                    link rel="icon" type="image/x-icon" href=(Cached("/favicon.ico"));
                }

                body {
                    nav {
                        a href="/" { "🏠" }
                    }

                    main { (self.content) }

                    footer {
                        a #repository href="https://github.com/vidhanio/site" {
                            "made with with rust and ❤️ by vidhan."
                        }

                        br;
                        br;

                        a #license href=(Cached("/LICENSE.txt")) {
                            "site licensed under agpl-3.0."
                        }

                        br;
                        br;

                        span #ring {
                            a href="https://ring.simonwu.dev/prev/vidhan" {
                                "←"
                            }
                            " "
                            a href="https://ring.simonwu.dev/random/vidhan" {
                                "🎲"
                            }
                            " "
                            a href="https://ring.simonwu.dev/next/vidhan" {
                                "→"
                            }
                        }
                    }
                }
            }
        }
        .render_to(output);
    }
}

impl<R: Renderable> IntoResponse for Document<R> {
    fn into_response(self) -> Response {
        self.render().into_response()
    }
}
