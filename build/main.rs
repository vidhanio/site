#![expect(missing_docs)]

use std::{env, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-env-changed=GIT_COMMIT_HASH");
    println!("cargo:rerun-if-changed=build/main.rs");

    let project_root = env::var_os("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .ok_or("expected CARGO_MANIFEST_DIR")?;
    let commit_hash = git_commit_hash(&project_root);
    println!("cargo:rustc-env=GIT_COMMIT_HASH={commit_hash}");

    if env::var_os("CARGO_FEATURE_RELOAD").is_none() {
        vidhan_site_assets::build::build()?;
    }

    Ok(())
}

fn git_commit_hash(project_root: &Path) -> String {
    let hash = env::var("GIT_COMMIT_HASH").ok().or_else(|| {
        let head_path = project_root.join(".git/HEAD");
        println!("cargo:rerun-if-changed={}", head_path.display());
        let head = fs::read_to_string(head_path).ok()?;
        let head = head.trim();
        head.strip_prefix("ref: ")
            .and_then(|reference| {
                let reference_path = project_root.join(".git").join(reference);
                println!("cargo:rerun-if-changed={}", reference_path.display());
                fs::read_to_string(reference_path).ok()
            })
            .or_else(|| Some(head.to_owned()))
    });
    let hash = hash.unwrap_or_else(|| "unknown".to_owned());
    hash.get(..7).unwrap_or(&hash).to_owned()
}
