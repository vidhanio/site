use std::time::Duration;

use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Response},
    Router,
};
use axum_extra::{
    headers::{CacheControl, ContentDisposition, ContentType},
    response::Css,
    TypedHeader,
};
use serde::{de::IgnoredAny, Deserialize};
use tracing::instrument;

use crate::{post::Post, SiteError, SiteResult};

#[derive(Debug, Clone, Deserialize)]
struct CacheParams {
    v: Option<IgnoredAny>,
}

pub fn router() -> Router {
    Router::new()
        // .route("/favicon.ico", axum::routing::get(favicon))
        .route("/styles.css", axum::routing::get(styles))
        .route("/og.png", axum::routing::get(og_image))
        .route("/post/:slug/og.png", axum::routing::get(post_og_image))
        .route("/assets/:asset", axum::routing::get(assets))
        .route("/fonts/:font", axum::routing::get(fonts))
        .route("/resume.pdf", axum::routing::get(resume))
        .route("/LICENSE.txt", axum::routing::get(license))
        .layer(axum::middleware::map_response(
            |Query(params): Query<CacheParams>, res: Response| async move {
                if params.v.is_some() {
                    let cc = CacheControl::new()
                        .with_max_age(Duration::from_secs(60 * 60 * 24 * 365))
                        .with_immutable();

                    (TypedHeader(cc), res).into_response()
                } else {
                    res
                }
            },
        ))
}

// #[instrument(level = "trace")]
// async fn favicon() -> &'static [u8] {
//     todo!()
// }

#[instrument(level = "trace")]
async fn styles() -> Css<&'static str> {
    const STYLES_CSS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/styles.css"));

    Css(STYLES_CSS)
}

#[instrument(level = "trace")]
async fn og_image() -> SiteResult<(TypedHeader<ContentType>, &'static [u8])> {
    const OG_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/og.png"));

    Ok((TypedHeader(ContentType::from(mime::IMAGE_PNG)), OG_IMAGE))
}

#[instrument(level = "trace")]
async fn post_og_image(
    Path(slug): Path<String>,
) -> SiteResult<(TypedHeader<ContentType>, &'static [u8])> {
    let post = Post::get(&slug).ok_or_else(|| SiteError::PostNotFound(slug))?;

    Ok((TypedHeader(ContentType::from(mime::IMAGE_PNG)), post.image))
}

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

#[instrument(level = "trace")]
async fn assets(
    Path(asset): Path<String>,
) -> SiteResult<(TypedHeader<ContentType>, &'static [u8])> {
    assets::get(&asset).ok_or_else(|| SiteError::AssetNotFound(asset))
}

include!(concat!(env!("OUT_DIR"), "/fonts.rs"));

#[instrument(level = "trace")]
async fn fonts(Path(font): Path<String>) -> SiteResult<(TypedHeader<ContentType>, &'static [u8])> {
    fonts::get(&font).ok_or_else(|| SiteError::PostNotFound(font))
}

#[instrument(level = "debug")]
async fn resume() -> (
    TypedHeader<ContentDisposition>,
    TypedHeader<ContentType>,
    &'static [u8],
) {
    const RESUME_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/resume.pdf"));

    (
        TypedHeader(ContentDisposition::inline()),
        TypedHeader(mime::APPLICATION_PDF.into()),
        RESUME_BYTES,
    )
}

#[instrument(level = "debug")]
async fn license() -> (
    TypedHeader<ContentDisposition>,
    TypedHeader<ContentType>,
    &'static [u8],
) {
    const LICENSE: &[u8] = include_bytes!("../../LICENSE.txt");

    (
        TypedHeader(ContentDisposition::inline()),
        TypedHeader(mime::TEXT_PLAIN.into()),
        LICENSE,
    )
}
