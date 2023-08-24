mod components;

use axum::{
    extract::{Path, State},
    Router,
};
use html_node::{html, Node, Text};
use tracing::instrument;

use crate::{icons, layout::document, App};

pub fn router() -> Router<App> {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/:slug", axum::routing::get(get_post))
}

#[instrument(level = "debug")]
pub async fn get(State(app): State<App>) -> Node {
    document(
        Some("/blog"),
        html! {
            <h1>blog</h1>
            <ul class="flex flex-col gap-4">
                {
                    app.blog_posts_metadatas()
                        .iter()
                        .map(|(slug, metadata)| components::link(slug, metadata))
                }
            </ul>
        },
    )
}

#[instrument(level = "debug", err(Debug))]
pub async fn get_post(State(app): State<App>, Path(slug): Path<String>) -> crate::Result<Node> {
    let blog_post = app.get_blog_post(&slug)?;

    Ok(document(
        Some(&format!("/blog/{slug}")),
        html! {
            <header>
                <h1>{ Text::from(&blog_post.metadata.title) }</h1>
                <time
                    class="flex flex-row gap-2 items-center text-slate-600 dark:text-slate-400 mt-2"
                    datetime=blog_post.metadata.date
                >
                    { icons::date(Some("h-4")) }
                    { Text::from(blog_post.metadata.date_text()) }
                </time>
                <p class="text-lg text-slate-500 mt-4">
                    { Text::from(&blog_post.metadata.description) }
                </p>
            </header>

            <hr class="w-3/4 border-indigo-500">

            <article
                class="\
                    prose prose-slate dark:prose-invert \
                    prose-pre:bg-slate-200 dark:prose-pre:bg-slate-800 \
                        prose-code:font-normal \
                        prose-code:before:content-none prose-code:after:content-none \
                        prose-code:bg-slate-200 dark:prose-code:bg-slate-800 \
                        prose-code:[font-feature-settings:normal] \
                    max-w-none \
                "
            >
                { blog_post.to_post_html() }
            </article>
        },
    ))
}
