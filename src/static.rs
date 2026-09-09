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
                if params.v.is_some() && !cfg!(feature = "reload") {
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

#[cfg(not(feature = "reload"))]
#[derive(Debug, Clone, Copy, Renderable)]
#[attribute(
    (self.0)
    "?v=" (env!("GIT_COMMIT_HASH"))
)]
pub struct Cached<'a>(pub &'a str);

#[cfg(feature = "reload")]
#[derive(Debug, Clone, Copy, Renderable)]
#[attribute((self.0))]
pub struct Cached<'a>(pub &'a str);

#[instrument(level = "trace")]
async fn logo_light_svg() -> (TypedHeader<ContentType>, &'static str) {
    #[cfg(not(feature = "reload"))]
    let logo = include_str!(concat!(env!("OUT_DIR"), "/logo-light.svg"));
    #[cfg(feature = "reload")]
    let logo = crate::assets::get().logo_light_svg.as_str();

    (TypedHeader(ContentType::from(mime::IMAGE_SVG)), logo)
}

#[instrument(level = "trace")]
async fn logo_dark_svg() -> (TypedHeader<ContentType>, &'static str) {
    #[cfg(not(feature = "reload"))]
    let logo = include_str!(concat!(env!("OUT_DIR"), "/logo-dark.svg"));
    #[cfg(feature = "reload")]
    let logo = crate::assets::get().logo_dark_svg.as_str();

    (TypedHeader(ContentType::from(mime::IMAGE_SVG)), logo)
}

#[instrument(level = "trace")]
async fn favicon_ico() -> (TypedHeader<ContentType>, &'static [u8]) {
    static X_ICON: LazyLock<Mime> = LazyLock::new(|| {
        "image/x-icon"
            .parse()
            .expect("image/x-icon should be a valid MIME type")
    });

    #[cfg(not(feature = "reload"))]
    let favicon = include_bytes!(concat!(env!("OUT_DIR"), "/favicon.ico")).as_slice();
    #[cfg(feature = "reload")]
    let favicon = crate::assets::get().favicon.as_slice();

    (TypedHeader(X_ICON.clone().into()), favicon)
}

#[instrument(level = "trace")]
async fn style() -> Css<&'static str> {
    #[cfg(not(feature = "reload"))]
    let css = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
    #[cfg(feature = "reload")]
    let css = crate::assets::get().style.as_str();

    Css(css)
}

#[instrument(level = "trace")]
async fn og_image() -> ResponseResult<(TypedHeader<ContentType>, &'static [u8])> {
    #[cfg(not(feature = "reload"))]
    let image = include_bytes!(concat!(env!("OUT_DIR"), "/og.png")).as_slice();
    #[cfg(feature = "reload")]
    let image = crate::assets::get().og_image.as_slice();

    Ok((TypedHeader(mime::IMAGE_PNG.into()), image))
}

#[instrument(level = "trace")]
async fn post_og_image(
    doc: DocumentRequest,
    Path(slug): Path<String>,
) -> ResponseResult<(TypedHeader<ContentType>, &'static [u8])> {
    doc.try_respond(|| {
        let post = Post::get(slug)?;

        Ok((TypedHeader(mime::IMAGE_PNG.into()), post.image()))
    })
}

#[cfg(not(feature = "reload"))]
include!(concat!(env!("OUT_DIR"), "/media.rs"));

#[instrument(level = "trace")]
async fn media(
    doc: DocumentRequest,
    Path(media): Path<String>,
) -> ResponseResult<(TypedHeader<ContentType>, &'static [u8])> {
    doc.try_respond(|| {
        #[cfg(not(feature = "reload"))]
        {
            self::media::get(&media).ok_or(SiteError::MediaNotFound(media))
        }

        #[cfg(feature = "reload")]
        {
            let asset = crate::assets::get()
                .media(&media)
                .ok_or(SiteError::MediaNotFound(media))?;
            Ok((content_type(asset.content_type), asset.bytes.as_slice()))
        }
    })
}

#[cfg(not(feature = "reload"))]
include!(concat!(env!("OUT_DIR"), "/fonts.rs"));

#[instrument(level = "trace")]
async fn fonts(
    doc: DocumentRequest,
    Path(font): Path<String>,
) -> ResponseResult<(TypedHeader<ContentType>, &'static [u8])> {
    doc.try_respond(|| {
        #[cfg(not(feature = "reload"))]
        {
            self::fonts::get(&font).ok_or(SiteError::FontNotFound(font))
        }

        #[cfg(feature = "reload")]
        {
            let asset = crate::assets::get()
                .font(&font)
                .ok_or(SiteError::FontNotFound(font))?;
            Ok((content_type(asset.content_type), asset.bytes.as_slice()))
        }
    })
}

#[instrument(level = "debug")]
async fn resume() -> (
    TypedHeader<ContentDisposition>,
    TypedHeader<ContentType>,
    &'static [u8],
) {
    #[cfg(not(feature = "reload"))]
    let resume = include_bytes!(concat!(env!("OUT_DIR"), "/resume.pdf")).as_slice();
    #[cfg(feature = "reload")]
    let resume = crate::assets::get().resume.as_slice();

    (
        TypedHeader(ContentDisposition::inline()),
        TypedHeader(mime::APPLICATION_PDF.into()),
        resume,
    )
}

#[cfg(feature = "reload")]
fn content_type(value: &str) -> TypedHeader<ContentType> {
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
    const LICENSE: &[u8] = include_bytes!("../LICENSE.txt");

    (
        TypedHeader(ContentDisposition::inline()),
        TypedHeader(mime::TEXT_PLAIN.into()),
        LICENSE,
    )
}
