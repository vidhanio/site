//! vidhan's site.

#[cfg(feature = "reload")]
mod assets;
mod config;
mod document;
mod error;
mod markdown_link;
mod pages;
mod post;
mod r#static;
mod wozeify;

use std::io;

use axum::{
    http::{Method, Uri},
    middleware,
    response::{IntoResponse, Response},
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub use crate::config::Config;
use crate::{
    document::{Document, DocumentRequest},
    error::SiteError,
};

type SiteResult<T> = std::result::Result<T, SiteError>;
type ResponseResult<T> = std::result::Result<T, DocumentError>;

#[derive(Debug)]
struct DocumentError(Box<Document<SiteError>>);

impl From<Document<SiteError>> for DocumentError {
    fn from(document: Document<SiteError>) -> Self {
        Self(Box::new(document))
    }
}

impl IntoResponse for DocumentError {
    fn into_response(self) -> Response {
        (*self.0).into_response()
    }
}

type SiteState = ();

/// Serve the application.
///
/// # Errors
///
/// Returns an error if the application fails to start.
pub async fn serve(config: Config) -> io::Result<()> {
    #[cfg(feature = "reload")]
    assets::initialize()?;

    let tcp_listener = TcpListener::bind(config.socket_addr()).await?;
    let router = pages::router()
        .merge(r#static::router())
        .fallback(async |doc: DocumentRequest, uri: Uri| {
            doc.build(SiteError::PageNotFound(uri).into())
        })
        .method_not_allowed_fallback(async |doc: DocumentRequest, uri: Uri, method: Method| {
            doc.build(SiteError::MethodNotAllowed(uri, method).into())
        })
        .layer(middleware::map_response(wozeify::wozeify))
        .layer(TraceLayer::new_for_http());

    axum::serve(tcp_listener, router).await
}
