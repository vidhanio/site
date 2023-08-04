use std::io;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use html_node::{html, text};
use thiserror::Error;

use crate::components::document;

/// An enum encompassing all possible errors from this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// A [`hyper::Error`].
    #[error("hyper error")]
    Hyper(#[from] hyper::Error),

    /// A [`io::Error`].
    #[error("io error")]
    Io(#[from] io::Error),

    /// Unexpected markdown tag.
    #[error("unexpected markdown tag")]
    UnexpectedMarkdownTag,

    /// A blog post was missing metadata.
    #[error("missing blog post metadata")]
    MissingMetadata,

    /// A [`serde_yaml::Error`]
    #[error("serde yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// A [`tree_sitter::QueryError`]
    #[error("tree-sitter query error")]
    TreeSitterQuery(#[from] tree_sitter::QueryError),

    /// A [`tree_sitter_highlight::Error`]
    #[error("tree-sitter highlight error")]
    TreeSitterHighlight(#[from] tree_sitter_highlight::Error),

    /// An invalid blog slug was provided.
    #[error("invalid blog slug")]
    InvalidBlogSlug,

    /// A [`time::error::Format`]
    #[error("time format error")]
    TimeFormat(#[from] time::error::Format),
}

impl Error {
    /// The status code of this error.
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Hyper(_)
            | Self::Io(_)
            | Self::UnexpectedMarkdownTag
            | Self::MissingMetadata
            | Self::Yaml(_)
            | Self::TreeSitterQuery(_)
            | Self::TreeSitterHighlight(_)
            | Self::TimeFormat(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidBlogSlug => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = document(
            None,
            html! {
                <h1>Error</h1>
                <p>{text!("{self}")}</p>
            },
        );

        (status_code, body).into_response()
    }
}
