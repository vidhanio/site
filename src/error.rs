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

    /// A [`tree_sitter::QueryError`]
    #[error("tree-sitter query error")]
    TreeSitterQuery(#[from] tree_sitter::QueryError),

    /// A [`tree_sitter_highlight::Error`]
    #[error("tree-sitter highlight error")]
    TreeSitterHighlight(#[from] tree_sitter_highlight::Error),
}

impl Error {
    /// The status code of this error.
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Hyper(_)
            | Self::Io(_)
            | Self::UnexpectedMarkdownTag
            | Self::TreeSitterQuery(_)
            | Self::TreeSitterHighlight(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
