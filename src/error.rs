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

    /// A [`syntect::Error`].
    #[error("syntect error")]
    Syntect(#[from] syntect::Error),

    /// Unexpected markdown tag.
    #[error("unexpected markdown tag")]
    UnexpectedMarkdownTag,
}

impl Error {
    /// The status code of this error.
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Hyper(_) | Self::Io(_) | Self::Syntect(_) | Self::UnexpectedMarkdownTag => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
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
