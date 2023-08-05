use std::sync::Arc;

use axum::{extract::State, Router};
use futures_util::TryStreamExt;
use html_node::{html, Node};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;
use tracing::instrument;

use crate::{
    components::{document, BlogLink, BlogPost, BlogSlug},
    App, Error,
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/:slug", axum::routing::get(get_post))
}

#[instrument(err)]
pub async fn get(State(app): State<App>) -> crate::Result<Node> {
    let dir = fs::read_dir(&app.config.content_dir.join("blog")).await?;
    let mut post_links = ReadDirStream::new(dir)
        .map_err(Error::from)
        .try_filter_map(|dir_entry| {
            let highlighter_configs = Arc::clone(&app.highlighter_configs);

            async move {
                let path = dir_entry.path();

                let maybe_slug = path
                    .extension()
                    .filter(|&ext| ext == "md")
                    .and_then(|_| path.file_stem());

                // check if .md and them map file_stem
                if let Some(slug) = maybe_slug {
                    let md = fs::read_to_string(&path).await?;

                    let metadata = BlogPost::new(&highlighter_configs, &md).metadata()?;

                    Ok::<_, Error>(Some(BlogLink::new(
                        slug.to_string_lossy().into_owned(),
                        metadata,
                    )))
                } else {
                    Ok(None)
                }
            }
        })
        .try_collect::<Vec<_>>()
        .await?;

    post_links.sort_by_key(|link| link.metadata.date);

    Ok(document(
        Some("blog"),
        html! {
            <h1>blog</h1>
            <ul class="flex flex-col gap-4">
                {post_links}
            </ul>
        },
    ))
}

#[instrument(err)]
pub async fn get_post(
    axum::extract::Path(slug): axum::extract::Path<BlogSlug>,
    State(app): State<App>,
) -> crate::Result<Node> {
    let path = app
        .config
        .content_dir
        .join("blog")
        .join(slug.as_str())
        .with_extension("md");

    let md = fs::read_to_string(&path).await?;

    let blog_post = BlogPost::new(&app.highlighter_configs, &md);

    Ok(document(Some("blog/hello-world"), blog_post.into_node()?))
}
