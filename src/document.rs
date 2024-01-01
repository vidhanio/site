use std::{
    borrow::Cow,
    fmt::{self, Debug, Formatter},
};

use axum::{
    extract::{FromRequestParts, OriginalUri},
    http::Uri,
    response::{IntoResponse, Response},
};
use maud::{html, Markup, Render, DOCTYPE};

use crate::public;

#[derive(Debug, FromRequestParts)]
#[allow(clippy::module_name_repetitions)]
pub struct DocumentParts {
    path: OriginalUri,
}

impl DocumentParts {
    pub fn build(
        self,
        title: impl Into<String>,
        image_path: impl Into<String>,
        content: impl Render,
    ) -> Document {
        Document {
            path: Some(self.path.0),
            title: Some(title.into()),
            image_path: Some(image_path.into()),
            content: content.render(),
        }
    }

    pub fn build_simple(self, content: impl Render) -> Document {
        Document {
            path: Some(self.path.0),
            title: None,
            image_path: None,
            content: content.render(),
        }
    }
}

pub struct Document {
    path: Option<Uri>,
    title: Option<String>,
    image_path: Option<String>,
    content: Markup,
}

impl Document {
    pub fn new(title: impl Into<String>, content: impl Render) -> Self {
        Self {
            path: None,
            title: Some(title.into()),
            image_path: None,
            content: content.render(),
        }
    }
}

impl Render for Document {
    fn render(&self) -> Markup {
        const DESCRIPTION: &str = "vidhan's home on the internet.";

        let title = self.title.as_ref().map_or_else(
            || Cow::Borrowed("vidhan.io"),
            |title| Cow::Owned(format!("{title} | vidhan.io")),
        );
        let url = self.path.as_ref().map_or_else(
            || Cow::Borrowed("https://vidhan.io"),
            |path| Cow::Owned(format!("https://vidhan.io{path}")),
        );

        let image_path = self.image_path.as_ref().map_or_else(
            || Cow::Borrowed("https://vidhan.io/og.png"),
            |image_path| Cow::Owned(format!("https://vidhan.io{image_path}")),
        );

        html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta charset="utf-8";

                    title { (title) }
                    meta name="description" content=(DESCRIPTION);
                    meta name="theme-color" content="#00ff80";

                    meta name="og:title" content=(title);
                    meta name="og:description" content=(DESCRIPTION);
                    meta name="og:url" content=(url);
                    meta name="og:type" content="website";
                    meta name="og:image" content=(image_path);

                    meta name="twitter:card" content="summary_large_image";
                    meta name="twitter:site" content="@vidhanio";
                    meta name="twitter:creator" content="@vidhanio";
                    meta name="twitter:title" content=(title);
                    meta name="twitter:description" content=(DESCRIPTION);
                    meta name="twitter:image" content=(image_path);

                    link rel="stylesheet" href=(public!("/styles.css"));
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

                        a href=(public!("/LICENSE.txt")) { "site licensed under agpl-3.0." }

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
        }
    }
}

impl IntoResponse for Document {
    fn into_response(self) -> Response {
        self.render().into_response()
    }
}

impl Debug for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Document")
            .field("path", &self.path)
            .field("title", &self.title)
            .field("image_path", &self.image_path)
            .field("content", &format_args!(".."))
            .finish()
    }
}
