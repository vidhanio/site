use std::{error::Error, io};

use axum::{
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use hypertext::{DisplayExt, prelude::*};
use thiserror::Error;

use crate::document::Document;

/// An enum encompassing all possible errors from this site.
#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum SiteError {
    /// An axum error occurred.
    #[error("axum error")]
    Axum(#[from] axum::Error),

    /// An IO error occurred.
    #[error("io error")]
    Io(#[from] io::Error),

    /// Post not found.
    #[error("post not found: \"{0}\"")]
    PostNotFound(String),

    /// Content not found.
    #[error("content not found: \"{0}\"")]
    ContentNotFound(String),

    /// Font not found.
    #[error("font not found: \"{0}\"")]
    FontNotFound(String),

    /// Page not found.
    #[error("page not found: \"{0}\"")]
    PageNotFound(Uri),

    /// Method not allowed.
    #[error("method not allowed for page \"{0}\": \"{1}\"")]
    MethodNotAllowed(Uri, Method),
}

impl SiteError {
    /// The status code of this error.
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Axum(_) | Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PostNotFound(_)
            | Self::ContentNotFound(_)
            | Self::FontNotFound(_)
            | Self::PageNotFound(_) => StatusCode::NOT_FOUND,
            Self::MethodNotAllowed(_, _) => StatusCode::METHOD_NOT_ALLOWED,
        }
    }
}

impl Renderable for SiteError {
    fn render_to(&self, output: &mut String) {
        maud! {
            pre {
                code {
                    (self.renderable())

                    @for (i, e) in ErrorSourceIter::new(self)
                        .skip(1)
                        .enumerate()
                    {
                        '\n'
                        @for _ in 0..i { "    " }
                        "└── " (e.renderable())
                    }
                }
            }
        }
        .render_to(output);
    }
}

impl IntoResponse for SiteError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();

        (
            status_code,
            Document::new(
                "error",
                maud! {
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
struct ErrorSourceIter<'a> {
    current: Option<&'a dyn Error>,
}

impl<'a> ErrorSourceIter<'a> {
    pub fn new(error: &'a dyn Error) -> Self {
        Self {
            current: Some(error),
        }
    }
}

impl<'a> Iterator for ErrorSourceIter<'a> {
    type Item = &'a dyn Error;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            self.current = current.source();

            // skip error if it just prints its source
            if let Some(source) = self.current
                && current.to_string() == source.to_string()
            {
                self.next()
            } else {
                Some(current)
            }
        } else {
            self.current = None;

            None
        }
    }
}
