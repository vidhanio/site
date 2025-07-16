//! vidhan's site.

mod config;
mod document;
mod error;
mod pages;
mod post;
mod r#static;
mod wozeify;

use std::{io, sync::Arc};

use axum::{
    http::{Method, Uri},
    middleware,
    response::{IntoResponse, Response},
};
use rspotify::AuthCodeSpotify;
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

#[derive(Debug)]
struct SiteStateInner {
    spotify: AuthCodeSpotify,
}

type SiteState = Arc<SiteStateInner>;

/// Serve the application.
///
/// # Errors
///
/// Returns an error if the application fails to start.
pub async fn serve(config: Config) -> io::Result<()> {
    let tcp_listener = TcpListener::bind(config.socket_addr()).await?;
    let state = Arc::new(SiteStateInner {
        spotify: config.spotify_client(),
    });

    let router = pages::router()
        .merge(r#static::router())
        .fallback(async |doc: DocumentRequest, uri: Uri| {
            doc.build(SiteError::PageNotFound(uri).into())
        })
        .method_not_allowed_fallback(async |doc: DocumentRequest, uri: Uri, method: Method| {
            doc.build(SiteError::MethodNotAllowed(uri, method).into())
        })
        .layer(middleware::map_response_with_state(
            Arc::clone(&state),
            wozeify::wozeify,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::serve(tcp_listener, router).await
}
