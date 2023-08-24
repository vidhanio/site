use std::io;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use html_node::{html, text, Node};
use thiserror::Error;

use crate::layout::document;

/// An enum encompassing all possible errors from this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// A [`tree_sitter::QueryError`]
    #[error("tree-sitter query error")]
    TreeSitterQuery(#[from] tree_sitter::QueryError),

    /// A [`tree_sitter_highlight::Error`]
    #[error("tree-sitter highlight error")]
    TreeSitterHighlight(#[from] tree_sitter_highlight::Error),

    /// An error occurred while trying to serve the application.
    #[error("application serve error")]
    Serve(#[source] io::Error),

    /// A blog post was missing metadata.
    #[error("metadata missing for blog post: `{0}`")]
    NoPostMetadata(String),

    /// A blog post's metadata could not be deserialized.
    #[error("failed to deserialize metadata for blog post: `{0}`")]
    DeserializePostMetadata(String, #[source] serde_yaml::Error),

    /// Unexpected markdown tag.
    #[error("unexpected markdown tag")]
    UnexpectedMarkdownTag,

    /// The blog post was not found
    #[error("blog post not found: `{0}`")]
    BlogPostNotFound(String),

    /// The projects file could not be deserialized.
    #[error("failed to deserialize projects file")]
    DeserializeProjects(#[source] serde_yaml::Error),
}

impl Error {
    /// The status code of this error.
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Serve(_)
            | Self::TreeSitterQuery(_)
            | Self::TreeSitterHighlight(_)
            | Self::NoPostMetadata(_)
            | Self::UnexpectedMarkdownTag
            | Self::DeserializePostMetadata(_, _)
            | Self::DeserializeProjects(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BlogPostNotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<Error> for Node {
    fn from(error: Error) -> Self {
        html! {
            <pre class="bg-slate-200 dark:bg-slate-800 overflow-x-auto rounded-lg p-4 [font-feature-settings:normal]">
                <code>
                {
                    ErrorSourceIter::new(&error)
                        .enumerate()
                        .flat_map(|(i, e)| {
                            let indent = text!(
                                "{}",
                                "  ".repeat(i.saturating_sub(1))
                            );

                            let corner = if i == 0 {
                                Self::EMPTY
                            } else {
                                html! {
                                    <span class="text-violet-500 font-bold">"└ "</span>
                                }
                            };

                            [
                                indent,
                                corner,
                                text!("{e}\n"),
                            ]
                        })
                }
                </code>
            </pre>
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = document(
            None,
            html! {
                <h1>{ text!("{} error", self.status_code().as_u16()) }</h1>
                { self }
            },
        );

        (status_code, body).into_response()
    }
}

#[derive(Clone, Debug)]
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
