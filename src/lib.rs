//! vidhan's site.

#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

mod app;
mod blog_post;
mod config;
mod error;
mod highlighter_configs;
mod icon;
mod layout;
mod pages;
mod project;
mod r#static;

use std::{collections::HashMap, ffi::OsStr};

use axum::Router;
use blog_post::BlogPost;
use highlighter_configs::HighlighterConfigurations;
use include_dir::{include_dir, Dir};

pub use self::{app::App, config::Config, error::Error};

type Result<T> = std::result::Result<T, Error>;

const PROJECTS_YAML: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/content/projects.yml"));
static BLOG_POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content/blog");

/// Serve the application.
pub async fn serve(config: Config) -> crate::Result<()> {
    let highlighter_configs = HighlighterConfigurations::new()?;

    let app = App {
        projects: serde_yaml::from_str::<Vec<_>>(PROJECTS_YAML)
            .map_err(Error::DeserializeProjects)?
            .into(),
        blog_posts: BLOG_POSTS_DIR
            .files()
            .filter_map(|file| {
                let path = file.path();

                let slug = if path.extension() == Some(OsStr::new("md")) {
                    path.file_stem()?.to_str()
                } else {
                    None
                }?;

                let markdown = file.contents_utf8()?;

                let blog_post = BlogPost::new(&highlighter_configs, slug, markdown);

                Some(blog_post.map(|blog_post| (slug, blog_post)))
            })
            .collect::<crate::Result<HashMap<_, _>>>()?
            .into(),
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
