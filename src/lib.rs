//! vidhan's site.

mod app;
mod config;
mod error;
mod highlighter_configs;
mod layout;
mod pages;
mod post;
mod r#static;

use std::{cmp::Reverse, ffi::OsStr};

use axum::Router;
use highlighter_configs::HighlighterConfigurations;
use include_dir::{include_dir, Dir};

use self::post::Post;
pub use self::{app::App, config::Config, error::Error};

type Result<T> = std::result::Result<T, Error>;

static BLOG_POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content/posts");

/// Serve the application.
///
/// # Errors
///
/// Returns an error if the application fails to serve.
pub async fn serve(config: Config) -> crate::Result<()> {
    let highlighter_configs = HighlighterConfigurations::new()?;

    let mut blog_posts = BLOG_POSTS_DIR
        .files()
        .filter_map(|file| {
            let path = file.path();

            let slug = if path.extension() == Some(OsStr::new("md")) {
                path.file_stem()?.to_str()
            } else {
                None
            }?;

            let markdown = file.contents_utf8()?;

            let blog_post = Post::new(&highlighter_configs, slug, markdown);

            Some(blog_post)
        })
        .collect::<crate::Result<Vec<_>>>()?;

    blog_posts.sort_by_key(|blog_post| Reverse(blog_post.metadata.date));

    let app = App {
        blog_posts: blog_posts.into(),
    };

    let tcp_listener = config.tcp_listener().await?;

    let router = Router::new()
        .nest("/", pages::router())
        .nest("/static", r#static::router())
        .with_state(app);

    axum::serve(tcp_listener, router.into_make_service())
        .await
        .map_err(Error::Serve)?;

    Ok(())
}
