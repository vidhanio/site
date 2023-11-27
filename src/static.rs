use std::{ffi::OsStr, future, path::PathBuf, time::Duration};

use axum::{extract::Path, Router};
use axum_extra::{
    headers::{CacheControl, ContentType},
    response::Css,
    TypedHeader,
};
use include_dir::{include_dir, Dir};
use tracing::instrument;

use crate::{App, Error};

pub fn router() -> Router<App> {
    Router::new()
        // .route("/favicon.ico", axum::routing::get(favicon))
        .route("/LICENSE.txt", axum::routing::get(license))
        .route("/styles.css", axum::routing::get(styles))
        .route("/fonts/:font", axum::routing::get(fonts))
        .layer(axum::middleware::map_response(|res| {
            let cc = CacheControl::new()
                .with_max_age(Duration::from_secs(60 * 60 * 24 * 365))
                .with_immutable();

            future::ready((TypedHeader(cc), res))
        }))
}

macro_rules! static_path {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $path)
    };
}

// #[instrument(level = "debug")]
// async fn favicon() -> &'static [u8] {
//     include_bytes!(static_path!("favicon.ico"))
// }

#[instrument(level = "debug")]
async fn license() -> &'static [u8] {
    include_bytes!(static_path!("LICENSE.txt"))
}

#[instrument(level = "debug")]
async fn styles() -> Css<&'static str> {
    const STYLES_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/styles.css"));

    Css(STYLES_CSS)
}

#[instrument(level = "debug")]
async fn fonts(
    Path(font): Path<String>,
) -> crate::Result<(TypedHeader<ContentType>, &'static [u8])> {
    static FONTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static/fonts");

    let font = PathBuf::from(font);

    let mime = match font.extension() {
        Some(ext) if ext == OsStr::new("woff2") => mime::FONT_WOFF2,
        Some(ext) if ext == OsStr::new("woff") => mime::FONT_WOFF,
        ext => return Err(Error::InvalidFontExtension(ext.map(Into::into))),
    };

    FONTS_DIR
        .get_file(&font)
        .map(|file| (TypedHeader(mime.into()), file.contents()))
        .ok_or_else(|| Error::FontNotFound(font))
}
