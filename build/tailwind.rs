#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{fs::File, io, path::PathBuf, sync::LazyLock};

use which::which;

use crate::{CARGO_MANIFEST_DIR, OUT_DIR, rerun_path};

pub const TAILWIND_FILENAME: &str = if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
    if cfg!(target_env = "musl") {
        "tailwindcss-linux-x64-musl"
    } else {
        "tailwindcss-linux-x64"
    }
} else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
    if cfg!(target_env = "musl") {
        "tailwindcss-linux-arm64-musl"
    } else {
        "tailwindcss-linux-arm64"
    }
} else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
    "tailwindcss-macos-arm64"
} else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
    "tailwindcss-macos-x64"
} else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
    "tailwindcss-windows-x64.exe"
} else {
    panic!("unsupported target os/arch combination");
};

pub static TAILWIND_EXECUTABLE: LazyLock<PathBuf> =
    LazyLock::new(|| OUT_DIR.join(TAILWIND_FILENAME));

pub fn download() -> Result<PathBuf, Box<dyn std::error::Error>> {
    rerun_path(CARGO_MANIFEST_DIR.join("src"));

    match which("tailwindcss") {
        Ok(path) => return Ok(path),
        Err(which::Error::CannotFindBinaryPath) => {}
        Err(e) => return Err(e.into()),
    }
    if TAILWIND_EXECUTABLE.exists() {
        return Ok(TAILWIND_EXECUTABLE.clone());
    }

    let mut download = ureq::get(format!(
        "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/{TAILWIND_FILENAME}",
    ))
    .call()?
    .into_body()
    .into_reader();

    let mut file = File::create(&*TAILWIND_EXECUTABLE)?;
    io::copy(&mut download, &mut file)?;
    if cfg!(unix) {
        // make executable
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o755);
        file.set_permissions(perms)?;
    }

    Ok(TAILWIND_EXECUTABLE.clone())
}
