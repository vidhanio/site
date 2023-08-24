mod components;

use axum::extract::State;
use html_node::{html, Node};
use tracing::instrument;

use crate::{layout::document, App};

#[instrument(level = "debug")]
pub async fn get(State(app): State<App>) -> Node {
    document(
        Some("/projects"),
        html! {
            <h1>projects</h1>
            <ul class="flex flex-col gap-4">
                { app.projects.iter().map(components::link) }
            </ul>
        },
    )
}
