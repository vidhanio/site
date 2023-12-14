use std::{borrow::Cow, ffi::OsString, io, path::PathBuf};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{html, Markup, Render};
use thiserror::Error;

use crate::document::Document;

/// An enum encompassing all possible errors from this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// A [`tree_sitter::QueryError`]
    #[error("tree-sitter query error")]
    TreeSitterQuery(#[from] tree_sitter::QueryError),

    /// A [`tree_sitter_highlight::Error`]
    #[error("tree-sitter highlight error")]
    TreeSitterHighlight(#[from] tree_sitter_highlight::Error),

    /// An error occurred while trying to bind to the socket address.
    #[error("tcp listener bind error")]
    Bind(#[source] io::Error),

    /// An error occurred while trying to serve the application.
    #[error("application serve error")]
    Serve(#[source] io::Error),

    /// A post was missing metadata.
    #[error("metadata missing for post: `{0}`")]
    NoPostMetadata(String),

    /// A post's metadata could not be deserialized.
    #[error("failed to deserialize metadata for post: `{slug}`")]
    DeserializePostMetadata {
        /// The slug of the post.
        slug: String,

        /// The source of the error.
        source: serde_yaml::Error,
    },

    /// Unexpected markdown tag.
    #[error("unexpected markdown tag")]
    UnexpectedMarkdownTag,

    /// The post was not found
    #[error("post not found: `{0}`")]
    PostNotFound(String),

    /// Invalid font path.
    #[error(
        "invalid font extension (must be `woff` or `woff2`): `{}`",
        .0.as_ref().map_or(Cow::Borrowed("<none>"), |s| s.to_string_lossy())
    )]
    InvalidFontExtension(Option<OsString>),

    /// Font not found.
    #[error("font not found: `{0}`")]
    FontNotFound(PathBuf),
}

impl Error {
    /// The status code of this error.
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Bind(_)
            | Self::Serve(_)
            | Self::TreeSitterQuery(_)
            | Self::TreeSitterHighlight(_)
            | Self::NoPostMetadata(_)
            | Self::UnexpectedMarkdownTag
            | Self::DeserializePostMetadata { slug: _, source: _ } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::PostNotFound(_) | Self::FontNotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidFontExtension(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl Render for Error {
    fn render(&self) -> Markup {
        html! {
            pre {
                code {
                    (maud::display(self));

                    @for (i, e) in ErrorSourceIter::new(self)
                        .skip(1)
                        .enumerate()
                    {
                        "\n" (" ".repeat(i * 2)) "└ " (e)
                    }
                }
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = self.status_code();

        (
            status_code,
            Document::new(
                "error",
                html! {
                    header {
                        h1 {
                            (status_code.to_string().to_lowercase())
                        }
                        hr;
                    }

                    (self)
                },
            ),
        )
            .into_response()
    }
}

#[derive(Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct ErrorSourceIter<'a> {
    current: Option<&'a (dyn std::error::Error + 'static)>,
}

impl<'a> ErrorSourceIter<'a> {
    pub fn new(error: &'a (dyn std::error::Error + 'static)) -> Self {
        Self {
            current: Some(error),
        }
    }
}

impl<'a> Iterator for ErrorSourceIter<'a> {
    type Item = &'a (dyn std::error::Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(std::error::Error::source);
        current
    }
}
