mod home;
mod post;

use axum::Router;

use crate::SiteState;

pub fn router() -> Router<SiteState> {
    Router::new()
        .route("/", axum::routing::get(home::get))
        .route("/post/{slug}", axum::routing::get(post::get))
}
