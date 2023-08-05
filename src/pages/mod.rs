mod blog;
mod projects;

use axum::Router;
use html_node::{html, Node};

use crate::{components::document, App};

pub fn router() -> Router<App> {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/projects", axum::routing::get(projects::get))
        .nest("/blog", blog::router())
}

#[allow(clippy::unused_async)]
pub async fn get() -> Node {
    document(
        None,
        html! {
            <h1>vidhan.io</h1>
            <p>
                "hey! i'm vidhan. i'm a software engineer, fullstack developer, discord \
                bot developer, and a cs student at mcmaster. i'm currently working \
                on a ton of cool projects, which you can find on "
                <a class="underline" href="https://github.com/vidhanio">"my github"</a>
                "."
            </p>
        },
    )
}
