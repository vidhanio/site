use axum::{extract::State, Router};
use futures_util::{StreamExt, TryStreamExt};
use html_node::{html, Node, UnsafeText};
use pulldown_cmark::{Options, Parser};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;

use crate::{components::document, syntax_highlighting::SyntaxHighlighterStream, App, Error};

pub fn router() -> Router<App> {
    Router::new().route("/", axum::routing::get(get))
    // .route("/projects", axum::routing::get(projects::get))
}

#[allow(clippy::unused_async)]
pub async fn get(State(app): State<App>) -> crate::Result<Node> {
    let dir = fs::read_dir(&app.config.content_dir.join("blog")).await?;
    let markdowns = ReadDirStream::new(dir)
        .and_then(|entry| fs::read_to_string(entry.path()))
        .map(|markdown| {
            let markdown = markdown?;

            let events = SyntaxHighlighterStream::new(
                &app.syntax_set,
                Parser::new_ext(&markdown, Options::all()),
            )
            .collect::<Result<Vec<_>, _>>()?;

            let mut buf = String::new();

            pulldown_cmark::html::push_html(&mut buf, events.into_iter());

            Ok::<_, Error>(Node::UnsafeText(UnsafeText { text: buf }))
        })
        .try_collect::<Vec<Node>>()
        .await?;

    Ok(document(
        None,
        html! {
            {markdowns}
        },
    ))
}
