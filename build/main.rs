#![expect(missing_docs)]

use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use prettyplease::unparse;
use quote::quote;
use syn::parse2;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-env-changed=GIT_COMMIT_HASH");
    println!("cargo:rerun-if-changed=build/main.rs");

    let project_root = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .ok_or("expected CARGO_MANIFEST_DIR")?;
    let commit_hash = git_commit_hash(&project_root);
    println!("cargo:rustc-env=GIT_COMMIT_HASH={commit_hash}");

    if env::var_os("CARGO_FEATURE_RELOAD").is_none() {
        build_assets(&project_root)?;
    }

    Ok(())
}

fn build_assets(project_root: &Path) -> Result<(), Box<dyn Error>> {
    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .ok_or("expected OUT_DIR")?;

    for path in [
        "assets/static",
        "assets/media",
        "assets/posts",
        "assets/tree-sitter",
        "typst",
        "resume",
    ] {
        println!(
            "cargo:rerun-if-changed={}",
            project_root.join(path).display()
        );
    }

    let assets = vidhan_site_assets::load(project_root)?;
    fs::create_dir_all(out_dir.join("post-og"))?;

    fs::write(out_dir.join("style.css"), &assets.style)?;
    fs::write(out_dir.join("logo-light.svg"), &assets.logo_light_svg)?;
    fs::write(out_dir.join("logo-dark.svg"), &assets.logo_dark_svg)?;
    fs::write(out_dir.join("favicon.ico"), &assets.favicon)?;
    fs::write(out_dir.join("og.png"), &assets.og_image)?;
    fs::write(out_dir.join("resume.pdf"), &assets.resume)?;

    for post in &assets.posts {
        fs::write(
            out_dir.join("post-og").join(format!("{}.png", post.slug)),
            &post.og_image,
        )?;
    }

    write_fonts(project_root, &out_dir)?;
    write_media(project_root, &out_dir)?;
    write_posts(&out_dir, &assets)?;

    Ok(())
}

fn write_fonts(project_root: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let routes = fs::read_dir(project_root.join("assets/static/fonts"))?
        .map(|entry| {
            let path = entry?.path();
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or("invalid font filename")?;
            let extension = path
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let content_type = match extension {
                "woff" => "font/woff",
                "woff2" => "font/woff2",
                _ => return Ok(quote!()),
            };
            Ok(quote! {
                #name => Some((
                    #content_type,
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/static/fonts/", #name)).as_slice()
                )),
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let tokens = quote! {
        pub fn get(font: &str) -> Option<(&'static str, &'static [u8])> {
            match font { #(#routes)* _ => None }
        }
    };
    fs::write(out_dir.join("fonts.rs"), unparse(&parse2(tokens)?))?;
    Ok(())
}

fn write_media(project_root: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let routes = fs::read_dir(project_root.join("assets/media"))?
        .map(|entry| {
            let path = entry?.path();
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or("invalid media filename")?;
            let extension = path
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let content_type = match extension {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                _ => return Err(format!("unsupported content type: {extension}").into()),
            };
            Ok(quote! {
                #name => Some((
                    #content_type,
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/media/", #name)).as_slice()
                )),
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let tokens = quote! {
        pub fn get(media: &str) -> Option<(&'static str, &'static [u8])> {
            match media { #(#routes)* _ => None }
        }
    };
    fs::write(out_dir.join("media.rs"), unparse(&parse2(tokens)?))?;
    Ok(())
}

fn write_posts(
    out_dir: &Path,
    assets: &vidhan_site_assets::LoadedAssets,
) -> Result<(), Box<dyn Error>> {
    let posts = assets.posts.iter().map(|post| {
        let slug = &post.slug;
        let title = &post.title;
        let (year, month, day) = post.date;
        let content = &post.content;
        let footnotes = post.footnotes.iter().map(|(name, content)| {
            quote! {(#name, hypertext::Raw::dangerously_create(#content))}
        });
        quote! {
            super::EmbeddedPost {
                slug: #slug,
                title: #title,
                date: (#year, #month, #day),
                og_image: include_bytes!(concat!(env!("OUT_DIR"), "/post-og/", #slug, ".png")),
                footnotes: &[#(#footnotes,)*],
                content: hypertext::Raw::dangerously_create(#content),
            }
        }
    });
    let tokens = quote! {
        pub const ALL: &[super::EmbeddedPost] = &[#(#posts,)*];
    };
    fs::write(out_dir.join("posts.rs"), unparse(&parse2(tokens)?))?;
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
