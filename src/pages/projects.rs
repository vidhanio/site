use axum::extract::State;
use html_node::{html, Node};
use tracing::instrument;

use crate::{
    components::{document, ProjectLink},
    App,
};

#[instrument(err)]
pub async fn get(State(app): State<App>) -> crate::Result<Node> {
    Ok(document(
        Some("/projects"),
        html! {
            <h1>projects</h1>
            <ul class="flex flex-col gap-4">
                { app.projects().await?.iter().map(ProjectLink::from) }
            </ul>
        },
    ))
}
