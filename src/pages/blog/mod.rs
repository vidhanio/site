mod components;

use axum::{
    extract::{Path, State},
    Router,
};
use maud::{html, Markup};
use tracing::instrument;

use self::components::Link;
use crate::{icon::Icon, layout::document, App};

pub fn router() -> Router<App> {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/:slug", axum::routing::get(get_post))
}

#[instrument(level = "debug")]
pub async fn get(State(app): State<App>) -> Markup {
    document(
        Some("/blog"),
        html! {
            h1 { "blog" }
            ul.flex.flex-col."gap-4" {
                @for (slug, metadata) in &*app.blog_posts_metadatas() {
                    (Link::new(slug, metadata))
                }
            }
        },
    )
}

#[instrument(level = "debug", err(Debug))]
pub async fn get_post(State(app): State<App>, Path(slug): Path<String>) -> crate::Result<Markup> {
    const CALENDAR_ICON: Icon = Icon {
        d: "M6.75 2.25A.75.75 0 017.5 3v1.5h9V3A.75.75 0 0118 3v1.5h.75a3 3 0 013 3v11.25a3 3 0 01-3 3H5.25a3 3 0 01-3-3V7.5a3 3 0 013-3H6V3a.75.75 0 01.75-.75zm13.5 9a1.5 1.5 0 00-1.5-1.5H5.25a1.5 1.5 0 00-1.5 1.5v7.5a1.5 1.5 0 001.5 1.5h13.5a1.5 1.5 0 001.5-1.5v-7.5z",
        class: "h-4",
    };

    let blog_post = app.get_blog_post(&slug)?;

    Ok(document(
        Some(&format!("/blog/{slug}")),
        html! {
            header {
                h1 { (blog_post.metadata.title) }
                time.flex.flex-row."gap-2".items-center."text-stone-600"."dark:text-stone-400"."mt-2" datetime=(blog_post.metadata.date) {
                    (CALENDAR_ICON)
                    (blog_post.metadata.date_text())
                }
                p.text-lg."text-stone-500"."mt-4" {
                    (blog_post.metadata.description)
                }
            }

            hr."w-3/4"."border-stone-500";

            article.prose.prose-slate."dark:prose-invert"
                ."prose-pre:bg-stone-200"."dark:prose-pre:bg-stone-800"
                ."prose-code:font-normal"."prose-code:bg-stone-200"."dark:prose-code:bg-stone-800"
                    ."prose-code:before:content-none"."prose-code:after:content-none"
                ."prose-code:[font-feature-settings:normal]"
                .max-w-none {
                (blog_post)
            }
        },
    ))
}
