use axum::extract::State;
use html_node::{html, Node};

use crate::{
    components::{document, ProjectLink},
    App,
};

pub async fn get(State(app): State<App>) -> crate::Result<Node> {
    Ok(document(
        Some("projects"),
        html! {
            <h1>projects</h1>
            <ul class="flex flex-col gap-4">
                {app.projects().await?.iter().map(ProjectLink::from)}
            </ul>
        },
    ))
}
