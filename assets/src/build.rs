use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use prettyplease::unparse;
use quote::quote;
use syn::parse2;

use crate::LoadedAssets;

/// Processes project assets and writes the generated build-time files.
///
/// This is intended to be called from the consuming crate's build script.
///
/// # Errors
///
/// Returns an error when an input cannot be read or processed, or generated
/// output cannot be written.
pub fn build() -> Result<(), Box<dyn Error>> {
    let project_root = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .ok_or("expected CARGO_MANIFEST_DIR")?;
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
        rerun_path(&project_root.join(path));
    }

    let assets = LoadedAssets::load(&project_root)?;
    fs::create_dir_all(out_dir.join("post-og"))?;

    fs::write(out_dir.join("style.css"), &assets.style)?;
    fs::write(out_dir.join("logo.svg"), &assets.logo_svg)?;
    fs::write(out_dir.join("favicon.ico"), &assets.favicon)?;
    fs::write(out_dir.join("og.png"), &assets.og_image)?;
    fs::write(out_dir.join("resume.pdf"), &assets.resume)?;

    for post in &assets.posts {
        fs::write(
            out_dir.join("post-og").join(format!("{}.png", post.slug)),
            &post.image,
        )?;
    }

    write_fonts(&project_root, &out_dir)?;
    write_media(&project_root, &out_dir)?;
    write_posts(&out_dir, &assets)?;

    Ok(())
}

fn rerun_path(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
}

fn write_fonts(project_root: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let routes = fs::read_dir(project_root.join("assets/static/fonts"))?
        .map(|entry| {
            let path = entry?.path();
            let name = path.file_name().and_then(|value| value.to_str()).ok_or("invalid font filename")?;
            let extension = path.extension().and_then(|value| value.to_str()).unwrap_or_default();
            let mime = match extension {
                "woff" => quote!(mime::FONT_WOFF),
                "woff2" => quote!(mime::FONT_WOFF2),
                _ => return Ok(quote!()),
            };
            Ok(quote! {
                #name => Some((
                    axum_extra::TypedHeader(#mime.into()),
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/static/fonts/", #name))
                )),
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let tokens = quote! {
        mod fonts {
            pub fn get(font: &str) -> Option<(axum_extra::TypedHeader<axum_extra::headers::ContentType>, &'static [u8])> {
                match font { #(#routes)* _ => None }
            }
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
            let mime = match extension {
                "png" => quote!(mime::IMAGE_PNG),
                "jpg" | "jpeg" => quote!(mime::IMAGE_JPEG),
                _ => return Err(format!("unsupported content type: {extension}").into()),
            };
            Ok(quote! {
                #name => Some((
                    axum_extra::TypedHeader(axum_extra::headers::ContentType::from(#mime)),
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/media/", #name))
                )),
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let tokens = quote! {
        mod media {
            pub fn get(media: &str) -> Option<(axum_extra::TypedHeader<axum_extra::headers::ContentType>, &'static [u8])> {
                match media { #(#routes)* _ => None }
            }
        }
    };
    fs::write(out_dir.join("media.rs"), unparse(&parse2(tokens)?))?;
    Ok(())
}

fn write_posts(out_dir: &Path, assets: &LoadedAssets) -> Result<(), Box<dyn Error>> {
    let posts = assets.posts.iter().map(|post| {
        let slug = &post.slug;
        let title = &post.title;
        let (year, month, day) = post.date;
        let content = &post.content;
        let footnotes = post.footnotes.iter().map(|(name, content)| {
            quote! {(#name, hypertext::Raw::dangerously_create(#content))}
        });
        quote! {
            crate::post::Post {
                slug: #slug,
                title: #title,
                date: (#year, #month, #day),
                image: include_bytes!(concat!(env!("OUT_DIR"), "/post-og/", #slug, ".png")),
                footnotes: &[#(#footnotes,)*],
                content: hypertext::Raw::dangerously_create(#content),
            }
        }
    });
    let tokens = quote! {
        mod posts {
            pub const ALL: &[crate::post::Post] = &[#(#posts,)*];
        }
    };
    fs::write(out_dir.join("posts.rs"), unparse(&parse2(tokens)?))?;
    Ok(())
}
