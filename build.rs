use std::{env, error::Error, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    build_tailwind()?;
    set_git_hash()?;

    Ok(())
}

fn build_tailwind() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let tailwind_config_file = manifest_dir.join("tailwind.config.js");
    let input_file = manifest_dir.join("static").join("styles.input.css");
    let src_dir = manifest_dir.join("src");

    for path in [&tailwind_config_file, &input_file, &src_dir] {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let output_file = out_dir.join("styles.css");

    let mut command = Command::new("tailwindcss");

    command.args([
        "build",
        "-c",
        &tailwind_config_file.to_string_lossy(),
        "-i",
        &input_file.to_string_lossy(),
        "-o",
        &output_file.to_string_lossy(),
    ]);

    if env::var("PROFILE")? == "release" {
        command.arg("--minify");
    }

    let status = command.spawn()?.wait()?;

    if !status.success() {
        return Err("tailwindcss failed".into());
    }

    Ok(())
}

fn set_git_hash() -> Result<(), Box<dyn Error>> {
    let repo = gix::open(".")?;

    let commit_id = repo.head()?.id().ok_or("head commit should have id")?;

    println!("cargo:rustc-env=GIT_COMMIT_HASH={commit_id}");

    Ok(())
}
