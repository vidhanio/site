//! vidhan's site.

mod app;
mod config;
mod document;
mod error;
mod highlighter_configs;
mod post;
mod routes;

use std::{cmp::Reverse, ffi::OsStr};

use highlighter_configs::HighlighterConfigurations;
use include_dir::{include_dir, Dir};
use tower_http::trace::TraceLayer;

use self::post::Post;
pub use self::{app::App, config::Config, error::Error};

type Result<T> = std::result::Result<T, Error>;

static POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content/posts");

/// Serve the application.
///
/// # Errors
///
/// Returns an error if the application fails to serve.
pub async fn serve(config: Config) -> crate::Result<()> {
    let highlighter_configs = HighlighterConfigurations::new()?;

    let mut posts = POSTS_DIR
        .files()
        .filter_map(|file| {
            let path = file.path();

            let slug = if path.extension() == Some(OsStr::new("md")) {
                path.file_stem()?.to_str()
            } else {
                None
            }?;

            Some(Post::new(&highlighter_configs, slug, file.contents_utf8()?))
        })
        .collect::<crate::Result<Vec<_>>>()?;

    posts.sort_by_key(|post| Reverse(post.metadata.date));

    let app = App {
        posts: posts.into(),
    };

    let tcp_listener = config.tcp_listener().await?;

    let router = routes::router()
        .layer(TraceLayer::new_for_http())
        .with_state(app);

    axum::serve(tcp_listener, router.into_make_service())
        .await
        .map_err(Error::Serve)?;

    Ok(())
}
