use std::{
    borrow::Cow,
    fmt::{self, Debug, Formatter},
};

use axum::{
    extract::{FromRequestParts, OriginalUri},
    http::Uri,
    response::{IntoResponse, Response},
};
use hypertext::{html_elements, maud, Displayed, GlobalAttributes, Renderable};

use crate::cached;

#[derive(Debug, FromRequestParts)]
#[allow(clippy::module_name_repetitions)]
pub struct DocumentParts {
    path: OriginalUri,
}

impl DocumentParts {
    pub fn build<R: Renderable>(
        self,
        title: impl Into<String>,
        image_path: impl Into<String>,
        content: R,
    ) -> Document<R> {
        Document {
            path: Some(self.path.0),
            title: Some(title.into()),
            image_path: Some(image_path.into()),
            content,
        }
    }

    pub fn build_simple<R: Renderable>(self, content: R) -> Document<R> {
        Document {
            path: Some(self.path.0),
            title: None,
            image_path: None,
            content,
        }
    }
}

pub struct Document<R> {
    path: Option<Uri>,
    title: Option<String>,
    image_path: Option<String>,
    content: R,
}

impl<R: Renderable> Document<R> {
    pub fn new(title: impl Into<String>, content: R) -> Self {
        Self {
            path: None,
            title: Some(title.into()),
            image_path: None,
            content,
        }
    }
}

impl<R: Renderable> Renderable for Document<R> {
    fn render_to(self, output: &mut String) {
        const DESCRIPTION: &str = "vidhan's home on the internet.";

        let url = maud! {
            "https://vidhan.io" @if let Some(path) = &self.path { (Displayed(path)) }
        };

        let image_path = self.image_path.as_ref().map_or(
            Cow::Borrowed(cached!("https://vidhan.io/og.png")),
            |image_path| Cow::Owned(format!(cached!("https://vidhan.io{}"), image_path)),
        );

        let meta_title = self.title.as_deref().unwrap_or("vidhan.io");

        maud! {
            !DOCTYPE
            html lang="en" {
                head {
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta charset="utf-8";

                    title { "vidhan.io" @if let Some(title) = &self.title { " / " (title) } }
                    meta name="description" content=(DESCRIPTION);
                    meta name="theme-color" content="#00ff80";

                    meta name="og:title" content=(meta_title);
                    meta name="og:description" content=(DESCRIPTION);
                    meta name="og:url" content=(url);
                    meta name="og:type" content="website";
                    meta name="og:image" content=(&*image_path);

                    meta name="twitter:card" content="summary_large_image";
                    meta name="twitter:site" content="@vidhanio";
                    meta name="twitter:creator" content="@vidhanio";
                    meta name="twitter:title" content=(meta_title);
                    meta name="twitter:description" content=(DESCRIPTION);
                    meta name="twitter:image" content=(&*image_path);

                    link rel="stylesheet" href=(cached!("/styles.css"));
                }

                body {
                    nav {
                        a href="/" { "🏠" }
                    }

                    main { (self.content) }

                    footer {
                        a href="https://github.com/vidhanio/site" { "made with with rust" } " and ❤️ by vidhan."

                        br;
                        br;

                        a href=(cached!("/LICENSE.txt")) { "site licensed under agpl-3.0." }

                        br;
                        br;

                        span #ring {
                            a href="https://ring.simonwu.dev/prev/vidhan" { "←" }
                            " "
                            a href="https://ring.simonwu.dev/random/vidhan" { "🎲" }
                            " "
                            a href="https://ring.simonwu.dev/next/vidhan" { "→" }
                        }
                    }
                }
            }
        }.render_to(output);
    }
}

impl<R: Renderable> IntoResponse for Document<R> {
    fn into_response(self) -> Response {
        self.render().into_response()
    }
}

impl<R> Debug for Document<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Document")
            .field("path", &self.path)
            .field("title", &self.title)
            .field("image_path", &self.image_path)
            .field("content", &format_args!(".."))
            .finish()
    }
}
