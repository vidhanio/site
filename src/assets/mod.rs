mod internal;

use std::{borrow::Cow, sync::LazyLock, time::Duration};

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
};
pub use internal::{Assets, Cached, Post};
use mime::Mime;
use serde::{Deserialize, de::IgnoredAny};
use tracing::instrument;

use crate::{ResponseResult, SiteState, document::DocumentRequest, error::SiteError};

pub fn router() -> Router<SiteState> {
    #[derive(Deserialize)]
    struct CacheParams {
        v: Option<IgnoredAny>,
    }

    Router::new()
        .route("/logo-light.svg", routing::get(logo_light_svg))
        .route("/logo-dark.svg", routing::get(logo_dark_svg))
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
                if params.v.is_some() && immutable_cache_enabled() {
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

const fn immutable_cache_enabled() -> bool {
    !cfg!(feature = "reload")
}

#[instrument(level = "trace")]
async fn logo_light_svg(assets: Assets) -> (TypedHeader<ContentType>, Cow<'static, str>) {
    (
        TypedHeader(ContentType::from(mime::IMAGE_SVG)),
        assets.logo_light(),
    )
}

#[instrument(level = "trace")]
async fn logo_dark_svg(assets: Assets) -> (TypedHeader<ContentType>, Cow<'static, str>) {
    (
        TypedHeader(ContentType::from(mime::IMAGE_SVG)),
        assets.logo_dark(),
    )
}

#[instrument(level = "trace")]
async fn favicon_ico(assets: Assets) -> (TypedHeader<ContentType>, Cow<'static, [u8]>) {
    static X_ICON: LazyLock<Mime> = LazyLock::new(|| {
        "image/x-icon"
            .parse()
            .expect("image/x-icon should be a valid MIME type")
    });

    (TypedHeader(X_ICON.clone().into()), assets.favicon())
}

#[instrument(level = "trace")]
async fn style(assets: Assets) -> (TypedHeader<ContentType>, Cow<'static, str>) {
    (
        TypedHeader(ContentType::from(mime::TEXT_CSS)),
        assets.style(),
    )
}

#[instrument(level = "trace")]
async fn og_image(assets: Assets) -> (TypedHeader<ContentType>, Cow<'static, [u8]>) {
    (
        TypedHeader(mime::IMAGE_PNG.into()),
        assets.open_graph_image(),
    )
}

#[instrument(level = "trace")]
async fn post_og_image(
    doc: DocumentRequest,
    assets: Assets,
    Path(slug): Path<String>,
) -> ResponseResult<(TypedHeader<ContentType>, Cow<'static, [u8]>)> {
    doc.try_respond(|| {
        let post = assets.post(slug)?;
        Ok((TypedHeader(mime::IMAGE_PNG.into()), post.og_image()))
    })
}

#[instrument(level = "trace")]
async fn media(
    doc: DocumentRequest,
    assets: Assets,
    Path(media): Path<String>,
) -> ResponseResult<(TypedHeader<ContentType>, Cow<'static, [u8]>)> {
    doc.try_respond(|| {
        let (content_type, bytes) = assets
            .media(&media)
            .ok_or(SiteError::MediaNotFound(media))?;
        Ok((typed_content_type(content_type), bytes))
    })
}

#[instrument(level = "trace")]
async fn fonts(
    doc: DocumentRequest,
    assets: Assets,
    Path(font): Path<String>,
) -> ResponseResult<(TypedHeader<ContentType>, Cow<'static, [u8]>)> {
    doc.try_respond(|| {
        let (content_type, bytes) = assets.font(&font).ok_or(SiteError::FontNotFound(font))?;
        Ok((typed_content_type(content_type), bytes))
    })
}

#[instrument(level = "debug")]
async fn resume(
    assets: Assets,
) -> (
    TypedHeader<ContentDisposition>,
    TypedHeader<ContentType>,
    Cow<'static, [u8]>,
) {
    (
        TypedHeader(ContentDisposition::inline()),
        TypedHeader(mime::APPLICATION_PDF.into()),
        assets.resume(),
    )
}

fn typed_content_type(value: &str) -> TypedHeader<ContentType> {
    let mime = value
        .parse::<Mime>()
        .expect("asset MIME type should be valid");
    TypedHeader(ContentType::from(mime))
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
