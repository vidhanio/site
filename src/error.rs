use std::error::Error;

use axum::http::{Method, StatusCode, Uri};
use hypertext::{Buffer, prelude::*};
use thiserror::Error;

use crate::document::DocumentDetails;

#[derive(Error, Debug)]
pub enum SiteError {
    #[error("axum error")]
    Axum(#[from] axum::Error),

    #[error("post not found: \"{0}\"")]
    PostNotFound(String),

    #[error("media not found: \"{0}\"")]
    MediaNotFound(String),

    #[error("font not found: \"{0}\"")]
    FontNotFound(String),

    #[error("page not found: \"{0}\"")]
    PageNotFound(Uri),

    #[error("method not allowed for page \"{0}\": \"{1}\"")]
    MethodNotAllowed(Uri, Method),
}

impl SiteError {
    const fn status_code(&self) -> StatusCode {
        match self {
            Self::Axum(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PostNotFound(_)
            | Self::MediaNotFound(_)
            | Self::FontNotFound(_)
            | Self::PageNotFound(_) => StatusCode::NOT_FOUND,
            Self::MethodNotAllowed(_, _) => StatusCode::METHOD_NOT_ALLOWED,
        }
    }
}

impl Renderable for SiteError {
    fn render_to(&self, output: &mut Buffer) {
        maud! {
            pre {
                code {
                    %(self)

                    @for (i, e) in ErrorSourceIter::new(self)
                        .skip(1)
                        .enumerate()
                    {
                        '\n'
                        @for _ in 0..i { "    " }
                        "└── " %(e)
                    }
                }
            }
        }
        .render_to(output);
    }
}

impl From<SiteError> for DocumentDetails<SiteError> {
    fn from(error: SiteError) -> Self {
        let status = error.status_code();
        Self {
            title: Some("error".into()),
            og_image: None,
            content: error,
            status,
        }
    }
}

#[derive(Clone, Debug)]
struct ErrorSourceIter<'a> {
    current: Option<&'a dyn Error>,
}

impl<'a> ErrorSourceIter<'a> {
    fn new(error: &'a dyn Error) -> Self {
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
