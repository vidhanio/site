#![allow(missing_docs)]

mod highlighter_configs;
mod open_graph;
mod post;

use core::panic;
use std::{
    cmp::Reverse,
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use headless_chrome::{Browser, LaunchOptions};
use maud::{html, PreEscaped};
use open_graph::generate_image;
use quote::quote;

use crate::{
    highlighter_configs::HighlighterConfigurations,
    post::{Metadata, Post},
};

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let browser = Browser::new(
        LaunchOptions::default_builder()
            .sandbox(false)
            .window_size(Some((1024, 512)))
            .build()?,
    )?;

    let commit_id = set_git_commit_id(&manifest_dir)?;
    include_fonts(&manifest_dir, &out_dir)?;
    include_posts(&browser, &manifest_dir, &out_dir, &commit_id)?;
    include_assets(&manifest_dir, &out_dir)?;
    compile_resume(&manifest_dir, &out_dir)?;
    generate_image(
        &browser,
        &out_dir,
        "og.png",
        "vidhan.io",
        None::<&'static str>,
    )?;

    Ok(())
}

fn set_git_commit_id(manifest_dir: &Path) -> Result<String, Box<dyn Error>> {
    const DEFAULT_HEAD: &str = "ref: refs/heads/main";

    let head = manifest_dir.join(".git/HEAD");

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join(".git/HEAD").display()
    );

    let head_contents = fs::read_to_string(head)?;

    let commit_id = if head_contents.trim() == DEFAULT_HEAD {
        fs::read_to_string(manifest_dir.join(".git/refs/heads/main"))?
    } else {
        head_contents
    };

    println!("cargo:rustc-env=GIT_COMMIT_HASH={commit_id}");

    Ok(commit_id)
}

fn include_fonts(manifest_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let fonts_dir = manifest_dir.join("fonts/");
    println!("cargo:rerun-if-changed={}", fonts_dir.display());

    let font_routes = fonts_dir.read_dir()?.map(|entry| {
        let path = entry.unwrap().path();

        assert!(path.is_file(), "fonts directory should only contain files");

        let font_name = path.file_name().unwrap().to_str().unwrap();
        let ext = path.extension().unwrap().to_str().unwrap();

        let mime = match ext {
            "woff" => quote!(mime::FONT_WOFF),
            "woff2" => quote!(mime::FONT_WOFF2),
            _ => return quote!(),
        };

        quote! {
            #font_name => Some((
                TypedHeader(ContentType::from(#mime)),
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/", #font_name))
            )),
        }
    });

    let tokens = quote!(
        mod fonts {
            use axum_extra::{
                headers::ContentType,
                TypedHeader
            };

            pub fn get(font: &str) -> Option<(TypedHeader<ContentType>, &'static [u8])> {
                match font {
                    #(#font_routes)*
                    _ => None,
                }
            }
        }
    );

    let fonts_path = out_dir.join("fonts.rs");

    fs::write(fonts_path, prettyplease::unparse(&syn::parse2(tokens)?))?;

    Ok(())
}

fn include_posts(
    browser: &Browser,
    manifest_dir: &Path,
    out_dir: &Path,
    commit_id: &str,
) -> Result<(), Box<dyn Error>> {
    let posts_dir = manifest_dir.join("posts/");
    println!("cargo:rerun-if-changed={}", posts_dir.display());

    fs::create_dir_all(out_dir.join("post-images"))?;

    let highlighter_configs = HighlighterConfigurations::new()?;

    let mut posts = posts_dir
        .read_dir()?
        .map(|entry| {
            let path = entry?.path();

            assert!(path.is_file(), "posts directory should only contain files");

            let post_name = path.file_stem().unwrap().to_str().unwrap();
            let ext = path.extension().unwrap();

            assert_eq!(ext, "md", "post extension should be md");

            let contents = fs::read_to_string(&path)?;

            let post = Post::new(&highlighter_configs, post_name, &contents, commit_id)?;

            generate_image(
                browser,
                out_dir,
                format!("post-images/{}.png", post.slug),
                &post.title,
                Some(html!("post on " b { "vidhan.io" })),
            )?;

            Ok(post)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    posts.sort_unstable_by_key(|post| Reverse(post.metadata.date));

    let post_tokens = posts.into_iter().map(
        |Post {
             slug,
             title,
             metadata: Metadata {
                 date: (year, month, day),
             },
             footnotes,
             content,
         }| {
            let footnotes = footnotes.iter().map(|(name, PreEscaped(content))| {
                quote! {
                    (#name, maud::PreEscaped(#content))
                }
            });

            quote! {
                crate::post::Post {
                    slug: #slug,
                    title: #title,
                    date: (#year, #month, #day),
                    image: include_bytes!(concat!(env!("OUT_DIR"), "/post-images/", #slug, ".png")),
                    footnotes: &[#(#footnotes,)*],
                    content: maud::PreEscaped(#content),
                }
            }
        },
    );

    let tokens = quote!(
        mod posts {
            pub const ALL: &[crate::post::Post] = &[
                #(#post_tokens,)*
            ];
        }
    );

    let posts_path = out_dir.join("posts.rs");

    fs::write(posts_path, prettyplease::unparse(&syn::parse2(tokens)?))?;

    Ok(())
}

fn include_assets(manifest_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let assets_dir = manifest_dir.join("assets/");
    println!("cargo:rerun-if-changed={}", assets_dir.display());

    let asset_routes = assets_dir.read_dir()?.map(|entry| {
        let path = entry.unwrap().path();

        assert!(path.is_file(), "assets directory should only contain files");

        let asset_name = path.file_name().unwrap().to_str().unwrap();
        let ext = path.extension().unwrap().to_str().unwrap();

        let mime = match ext {
            "png" => quote!(mime::IMAGE_PNG),
            "jpg" => quote!(mime::IMAGE_JPEG),
            _ => panic!("asset extension should be png or jpg"),
        };

        quote! {
            #asset_name => Some((
                TypedHeader(ContentType::from(#mime)),
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", #asset_name))
            ))
        }
    });

    let tokens = quote!(
        mod assets {
            use axum_extra::{
                headers::ContentType,
                TypedHeader
            };

            pub fn get(asset: &str) -> Option<(TypedHeader<ContentType>, &'static [u8])> {
                match asset {
                    #(#asset_routes,)*
                    _ => None,
                }
            }
        }
    );

    let assets_path = out_dir.join("assets.rs");

    fs::write(assets_path, prettyplease::unparse(&syn::parse2(tokens)?))?;

    Ok(())
}

fn compile_resume(manifest_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn Error>> {
    let resume_dir = manifest_dir.join("resume/");
    println!("cargo:rerun-if-changed={}", resume_dir.display());

    let resume_path = resume_dir.join("resume.typ");

    let output = Command::new("typst")
        .arg("compile")
        .arg("--font-path")
        .arg(manifest_dir.join("fonts/"))
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
