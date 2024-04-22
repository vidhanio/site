use std::{error::Error, io};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use hypertext::{html_elements, maud, Displayed, Renderable};
use thiserror::Error;

use crate::document::Document;

/// An enum encompassing all possible errors from this site.
#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum SiteError {
    /// An IO error occurred.
    #[error("io error")]
    Io(#[from] io::Error),

    /// Post not found.
    #[error("post not found: {0}")]
    PostNotFound(String),

    /// Asset not found.
    #[error("asset not found: {0}")]
    AssetNotFound(String),

    /// Font not found.
    #[error("font not found: {0}")]
    FontNotFound(String),
}

impl SiteError {
    /// The status code of this error.
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PostNotFound(_) | Self::AssetNotFound(_) | Self::FontNotFound(_) => {
                StatusCode::NOT_FOUND
            }
        }
    }
}

impl Renderable for SiteError {
    fn render_to(self, output: &mut String) {
        maud! {
            pre {
                code {
                    (Displayed(&self))

                    @for (i, e) in ErrorSourceIter::new(&self)
                        // .skip(1)
                        .cycle()
                        .take(10)
                        .enumerate()
                    {
                        "\n" (" ".repeat(i * 2)) "└ " (Displayed(e))
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
pub struct ErrorSourceIter<'a> {
    current: Option<&'a (dyn Error + 'static)>,
}

impl<'a> ErrorSourceIter<'a> {
    pub fn new(error: &'a (dyn Error + 'static)) -> Self {
        Self {
            current: Some(error),
        }
    }
}

impl<'a> Iterator for ErrorSourceIter<'a> {
    type Item = &'a (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(Error::source);
        current
    }
}
