use std::{sync::LazyLock, time::Duration};

use axum::{
    Router,
    extract::{Path, Query},
    middleware,
    response::{IntoResponse, Response},
    routing,
};
use axum_extra::{
    TypedHeader,
    headers::{CacheControl, ContentDisposition, ContentType},
    response::Css,
};
use hypertext::Renderable;
use mime::Mime;
use serde::{Deserialize, de::IgnoredAny};
use tracing::instrument;

use crate::{ResponseResult, SiteError, SiteState, document::DocumentRequest, post::Post};

pub fn router() -> Router<SiteState> {
    #[derive(Deserialize)]
    struct CacheParams {
        v: Option<IgnoredAny>,
    }

    Router::new()
        .route("/logo.svg", routing::get(logo_svg))
        .route("/favicon.ico", routing::get(favicon_ico))
        .route("/style.css", routing::get(style))
        .route("/og.png", routing::get(og_image))
        .route("/post/{slug}/og.png", routing::get(post_og_image))
        .route("/media/{media}", routing::get(media))
        .route("/fonts/{font}", routing::get(fonts))
        .route("/resume.pdf", routing::get(resume))
        .route("/LICENSE.txt", routing::get(license))
        .layer(middleware::map_response(
            async |Query(params): Query<CacheParams>, res: Response| {
                if params.v.is_some() {
                    let cc = CacheControl::new()
                        .with_public()
                        .with_immutable()
                        .with_max_age(Duration::from_hours(8760));

                    (TypedHeader(cc), res).into_response()
                } else {
                    res
                }
            },
        ))
}

#[derive(Debug, Clone, Copy, Renderable)]
#[attribute(
    (self.0)
    "?v=" (env!("GIT_COMMIT_HASH"))
)]
pub struct Cached<'a>(pub &'a str);

#[instrument(level = "trace")]
async fn logo_svg() -> (TypedHeader<ContentType>, &'static str) {
    const LOGO_SVG: &str = include_str!(concat!(env!("OUT_DIR"), "/logo.svg"));

    (TypedHeader(ContentType::from(mime::IMAGE_SVG)), LOGO_SVG)
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
async fn og_image() -> ResponseResult<(TypedHeader<ContentType>, &'static [u8])> {
    const OG_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/og.png"));

    Ok((TypedHeader(mime::IMAGE_PNG.into()), OG_IMAGE))
}

#[instrument(level = "trace")]
async fn post_og_image(
    doc: DocumentRequest,
    Path(slug): Path<String>,
) -> ResponseResult<(TypedHeader<ContentType>, &'static [u8])> {
    doc.try_respond(|| {
        let post = Post::get(slug)?;

        Ok((TypedHeader(mime::IMAGE_PNG.into()), post.image))
    })
}

include!(concat!(env!("OUT_DIR"), "/media.rs"));

#[instrument(level = "trace")]
async fn media(
    doc: DocumentRequest,
    Path(media): Path<String>,
) -> ResponseResult<(TypedHeader<ContentType>, &'static [u8])> {
    doc.try_respond(|| media::get(&media).ok_or(SiteError::MediaNotFound(media)))
}

include!(concat!(env!("OUT_DIR"), "/fonts.rs"));

#[instrument(level = "trace")]
async fn fonts(
    doc: DocumentRequest,
    Path(font): Path<String>,
) -> ResponseResult<(TypedHeader<ContentType>, &'static [u8])> {
    doc.try_respond(|| fonts::get(&font).ok_or(SiteError::FontNotFound(font)))
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
