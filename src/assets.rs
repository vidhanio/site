use std::{io, sync::OnceLock};

use vidhan_site_assets::Assets;

static ASSETS: OnceLock<Assets> = OnceLock::new();

pub fn initialize() -> io::Result<()> {
    let assets = vidhan_site_assets::load(env!("CARGO_MANIFEST_DIR"))
        .map_err(|error| io::Error::other(error.to_string()))?;
    ASSETS
        .set(assets)
        .map_err(|_| io::Error::other("site assets were initialized twice"))
}

pub fn get() -> &'static Assets {
    ASSETS.get().expect("site assets should be initialized")
}
