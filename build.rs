use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> Result<(), Box<dyn Error>> {
    let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    compile_resume(&cargo_manifest_dir, &out_dir)?;
    set_git_hash(&cargo_manifest_dir)?;

    Ok(())
}

fn compile_resume(manifest_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let resume_dir = manifest_dir.join("content/resume");
    println!("cargo:rerun-if-changed={}", resume_dir.display());

    let resume_path = resume_dir.join("resume.typ");

    let output = Command::new("typst")
        .arg("compile")
        .arg(resume_path)
        .arg(out_dir.join("resume.pdf"))
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "typst failed with status code {}\n {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}

fn set_git_hash(manifest_dir: &Path) -> Result<(), Box<dyn Error>> {
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join(".git/HEAD").display()
    );

    let repo = gix::open(manifest_dir)?;

    let commit_id = repo.head()?.id().ok_or("head commit should have id")?;

    println!("cargo:rustc-env=GIT_COMMIT_HASH={commit_id}");

    Ok(())
}
