use axum::extract::State;
use maud::{html, Markup};
use tracing::instrument;

use crate::{layout::document, App};

#[instrument(level = "debug")]
pub async fn get(State(app): State<App>) -> Markup {
    document(
        Some("/projects"),
        &html! {
            h1 { "projects" }
            ul.flex.flex-col."gap-4" {
                @for project in &*app.projects {
                    (project.link())
                }
            }
        },
    )
}
