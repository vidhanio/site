use std::path::Path;

use axum::Router;
use axum_extra::response::Css;
use tower_http::services::ServeDir;
use tracing::instrument;

use crate::App;

const STYLES_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/styles.css"));

pub fn router(static_dir: &Path) -> Router<App> {
    Router::new()
        .route("/css/styles.css", axum::routing::get(styles))
        .nest_service("/", ServeDir::new(static_dir))
}

#[instrument(level = "debug")]
async fn styles() -> Css<&'static str> {
    Css(STYLES_CSS)
}
