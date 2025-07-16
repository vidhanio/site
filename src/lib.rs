//! vidhan's site.

mod config;
mod document;
mod error;
mod pages;
mod post;
mod r#static;
mod wozeify;

use axum::{
    http::{Method, Uri},
    middleware,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub use self::{config::Config, error::SiteError};

/// A convenience type for [`Result`]s returned by the site.
pub type SiteResult<T> = std::result::Result<T, SiteError>;

/// Serve the application.
///
/// # Errors
///
/// Returns an error if the application fails to start.
pub async fn serve(config: Config) -> SiteResult<()> {
    let tcp_listener = TcpListener::bind(config.socket_addr()).await?;
    let router = pages::router()
        .merge(r#static::router())
        .fallback(async |uri: Uri| SiteError::PageNotFound(uri))
        .method_not_allowed_fallback(async |uri: Uri, method: Method| {
            SiteError::MethodNotAllowed(uri, method)
        })
        .layer(middleware::from_fn(wozeify::wozeify))
        .layer(TraceLayer::new_for_http());

    axum::serve(tcp_listener, router).await.map_err(Into::into)
}
