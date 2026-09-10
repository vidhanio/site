use std::{fmt, io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error while processing assets")]
    Io(#[from] io::Error),

    #[error("missing required build environment variable {0}")]
    MissingEnvironment(&'static str),

    #[error("invalid asset path: {}", .0.display())]
    InvalidPath(PathBuf),

    #[error("unsupported asset content type for {}", .0.display())]
    UnsupportedContentType(PathBuf),

    #[error("invalid post {slug}: {message}")]
    InvalidPost { slug: String, message: String },

    #[error("failed to parse metadata for post {slug}")]
    PostMetadata {
        slug: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("failed to configure syntax highlighting")]
    HighlightQuery(#[from] tree_sitter::QueryError),

    #[error("syntax highlighting failed")]
    Highlight(#[from] tree_sitter_highlight::Error),

    #[error("failed to render highlighted HTML")]
    Format(#[from] fmt::Error),

    #[error("Typst failed: {diagnostics}")]
    Typst { diagnostics: String },

    #[error("{document} produced {actual} pages; expected {expected}")]
    PageCount {
        document: &'static str,
        expected: usize,
        actual: usize,
    },

    #[error("failed to parse generated SVG")]
    Svg(#[from] resvg::usvg::Error),

    #[error("failed to encode PNG")]
    Png(#[from] png::EncodingError),
}

impl Error {
    pub(crate) fn invalid_post(slug: &str, message: impl Into<String>) -> Self {
        Self::InvalidPost {
            slug: slug.to_owned(),
            message: message.into(),
        }
    }
}
