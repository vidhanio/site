//! vidhan's site.

mod config;
mod document;
mod error;
mod pages;
mod post;
mod r#static;

use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub use self::{config::Config, error::SiteError};

type SiteResult<T> = std::result::Result<T, SiteError>;

/// Serve the application.
///
/// # Errors
///
/// Returns an error if the application fails to start.
pub async fn serve(config: Config) -> SiteResult<()> {
    let tcp_listener = TcpListener::bind(config.socket_addr()).await?;
    let router = pages::router()
        .merge(r#static::router())
        .layer(TraceLayer::new_for_http());

    axum::serve(tcp_listener, router).await.map_err(Into::into)
}

macro_rules! public {
    ($path:literal) => {
        concat!($path, "?v=", env!("GIT_COMMIT_HASH"))
    };
}
use public;
