mod blog;
mod projects;

use axum::Router;
use maud::{html, Markup};
use tracing::instrument;

use crate::{layout::document, App};

pub fn router() -> Router<App> {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/projects", axum::routing::get(projects::get))
        .nest("/blog", blog::router())
}

#[instrument(level = "debug")]
pub async fn get() -> Markup {
    document(
        None,
        html! {
            h1 { "hey, i'm vidhan!" }
            p."text-2xl" {
                "i'm a software engineer, fullstack developer, discord \
                bot developer, and a cs student at mcmaster. i'm currently working \
                on a ton of cool projects, which you can find on "
                a.underline href="https://github.com/vidhanio" { "my github" } "."
            }
        },
    )
}
