use axum::{
    extract::{Path, State},
    Router,
};
use axum_extra::extract::WithRejection;
use html_node::{html, Node};
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
    let metadatas = app.blog_posts_metadatas().await?;

    Ok(document(
        Some("/blog"),
        html! {
            <h1>blog</h1>
            <ul class="flex flex-col gap-4">
                {
                    metadatas
                        .into_iter()
                        .map(|(slug, metadata)| BlogLink { slug, metadata })
                }
            </ul>
        },
    ))
}

#[instrument(err)]
pub async fn get_post(
    State(app): State<App>,
    WithRejection(Path(slug), _): WithRejection<Path<BlogSlug>, Error>,
) -> crate::Result<Node> {
    let md = app.blog_post_markdown(&slug).await?;

    let blog_post = BlogPost::new(&app.highlighter_configs, &slug, &md);

    Ok(document(
        Some(&format!("/blog/{slug}")),
        blog_post.try_into()?,
    ))
}
