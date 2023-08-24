use std::{ffi::OsStr, path::PathBuf};

use axum::{extract::Path, Router};
use axum_extra::{headers::ContentType, response::Css, TypedHeader};
use include_dir::{include_dir, Dir};
use tracing::instrument;

use crate::{App, Error};

pub fn router() -> Router<App> {
    Router::new()
        .route("/favicon.ico", axum::routing::get(favicon))
        .route("/LICENSE.txt", axum::routing::get(license))
        .route("/styles.css", axum::routing::get(styles))
        .route("/fonts/:font", axum::routing::get(fonts))
}

macro_rules! static_path {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $path)
    };
}

#[instrument(level = "debug")]
async fn favicon() -> &'static [u8] {
    include_bytes!(static_path!("favicon.ico"))
}

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
    const FONTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static/fonts");

    let font = PathBuf::from(font);

    let mime = match font.extension() {
        Some(ext) if ext == OsStr::new("woff2") => mime::FONT_WOFF2,
        Some(ext) if ext == OsStr::new("woff") => mime::FONT_WOFF,
        _ => return Err(Error::InvalidFontExtension(font)),
    };

    FONTS_DIR
        .get_file(&font)
        .map(|file| (TypedHeader(mime.into()), file.contents()))
        .ok_or_else(|| Error::FontNotFound(font))
}
