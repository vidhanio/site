use std::{sync::LazyLock, time::Duration};

use axum::{
    Router,
    extract::{Path, Query},
    response::{IntoResponse, Response},
    routing,
};
use axum_extra::{
    TypedHeader,
    headers::{CacheControl, ContentDisposition, ContentType},
    response::Css,
};
use hypertext::{AttributeRenderable, Renderable, attribute};
use mime::Mime;
use serde::{Deserialize, de::IgnoredAny};
use tracing::instrument;

use crate::{SiteError, SiteResult, post::Post};

pub fn router() -> Router {
    #[derive(Deserialize)]
    struct CacheParams {
        v: Option<IgnoredAny>,
    }

    Router::new()
        .route("/favicon.svg", routing::get(favicon_svg))
        .route("/favicon.ico", routing::get(favicon_ico))
        .route("/style.css", routing::get(style))
        .route("/og.png", routing::get(og_image))
        .route("/post/{slug}/og.png", routing::get(post_og_image))
        .route("/content/{content}", routing::get(content))
        .route("/fonts/{font}", routing::get(fonts))
        .route("/resume.pdf", routing::get(resume))
        .route("/LICENSE.txt", routing::get(license))
        .layer(axum::middleware::map_response(
            |Query(params): Query<CacheParams>, res: Response| async move {
                if params.v.is_some() {
                    let cc = CacheControl::new()
                        .with_public()
                        .with_immutable()
                        .with_max_age(Duration::from_secs(60 * 60 * 24 * 365));

                    (TypedHeader(cc), res).into_response()
                } else {
                    res
                }
            },
        ))
}

#[derive(Debug, Clone, Copy)]
pub struct Cached<T>(pub T);

impl<T: AttributeRenderable> Renderable for Cached<T> {
    fn render_to(&self, output: &mut String) {
        self.render_attribute_to(output);
    }
}

impl<T: AttributeRenderable> AttributeRenderable for Cached<T> {
    fn render_attribute_to(&self, output: &mut String) {
        if cfg!(cache_static) {
            attribute!((self.0) "?v=" (env!("GIT_COMMIT_HASH"))).render_attribute_to(output);
        } else {
            self.0.render_attribute_to(output);
        }
    }
}

#[instrument(level = "trace")]
async fn favicon_svg() -> (TypedHeader<ContentType>, &'static str) {
    const FAVICON_SVG: &str = include_str!(concat!(env!("OUT_DIR"), "/favicon.svg"));

    (TypedHeader(ContentType::from(mime::IMAGE_SVG)), FAVICON_SVG)
}

#[instrument(level = "trace")]
async fn favicon_ico() -> (TypedHeader<ContentType>, &'static [u8]) {
    static X_ICON: LazyLock<Mime> = LazyLock::new(|| {
        "image/x-icon"
            .parse()
            .expect("image/x-icon should be a valid MIME type")
    });

    const FAVICON_ICO: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/favicon.ico"));

    (TypedHeader(X_ICON.clone().into()), FAVICON_ICO)
}

#[instrument(level = "trace")]
async fn style() -> Css<&'static str> {
    const STYLE_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

    Css(STYLE_CSS)
}

#[instrument(level = "trace")]
async fn og_image() -> SiteResult<(TypedHeader<ContentType>, &'static [u8])> {
    const OG_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/og.png"));

    Ok((TypedHeader(mime::IMAGE_PNG.into()), OG_IMAGE))
}

#[instrument(level = "trace")]
async fn post_og_image(
    Path(slug): Path<String>,
) -> SiteResult<(TypedHeader<ContentType>, &'static [u8])> {
    let post = Post::get(&slug).ok_or(SiteError::PostNotFound(slug))?;

    Ok((TypedHeader(mime::IMAGE_PNG.into()), post.image))
}

include!(concat!(env!("OUT_DIR"), "/content.rs"));

#[instrument(level = "trace")]
async fn content(
    Path(content): Path<String>,
) -> SiteResult<(TypedHeader<ContentType>, &'static [u8])> {
    content::get(&content).ok_or(SiteError::ContentNotFound(content))
}

include!(concat!(env!("OUT_DIR"), "/fonts.rs"));

#[instrument(level = "trace")]
async fn fonts(Path(font): Path<String>) -> SiteResult<(TypedHeader<ContentType>, &'static [u8])> {
    fonts::get(&font).ok_or(SiteError::PostNotFound(font))
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
    const LICENSE: &[u8] = include_bytes!("../LICENSE.txt");

    (
        TypedHeader(ContentDisposition::inline()),
        TypedHeader(mime::TEXT_PLAIN.into()),
        LICENSE,
    )
}
