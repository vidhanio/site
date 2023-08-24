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
mod icons;
mod layout;
mod pages;
mod project;
mod r#static;

pub use self::{app::App, config::Config, error::Error};

type Result<T> = std::result::Result<T, Error>;

/// Serve the application.
pub async fn serve(config: Config) -> Result<()> {
    App::new(config)?.serve().await?;

    Ok(())
}
