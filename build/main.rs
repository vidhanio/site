#![allow(missing_docs)]

mod colors;
mod highlighter_configs;
mod post;
mod typst_world;

use std::{
    cmp::Reverse,
    env,
    error::Error,
    fs::{self, File},
    path::PathBuf,
    sync::LazyLock,
};

use hypertext::Raw;
use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use quote::quote;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};
use typst::foundations::Dict;
use typst_pdf::PdfOptions;

use self::{highlighter_configs::HighlighterConfigurations, post::Post, typst_world::SiteWorld};
use crate::{colors::COLORS, post::POST_OG_DIR, typst_world::diagnostic_error};

static CARGO_MANIFEST_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .expect("expected env var `CARGO_MANIFEST_DIR` to be set")
});

static OUT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .expect("expected env var `OUT_DIR` to be set")
});

static STATIC_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| rerun_path(CARGO_MANIFEST_DIR.join("static")));

static FONTS_DIR: LazyLock<PathBuf> = LazyLock::new(|| STATIC_DIR.join("fonts"));

static OPEN_GRAPH_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| rerun_path(CARGO_MANIFEST_DIR.join("open-graph")));

static CACHE_STATIC: LazyLock<bool> = LazyLock::new(|| {
    let cache_static =
        env::var("PROFILE").expect("expected env var `PROFILE` to be set") == "debug";
    println!("cargo:rerun-if-env-changed=PROFILE");

    println!("cargo:rustc-check-cfg=cfg(cache_static)");
    if cache_static {
        println!("cargo:rustc-cfg=cache_static");
    }

    cache_static
});

static GIT_COMMIT_HASH: LazyLock<String> = LazyLock::new(|| {
    const DEFAULT_HEAD: &str = "ref: refs/heads/main";

    let head = rerun_path(CARGO_MANIFEST_DIR.join(".git/HEAD"));

    let head_contents = fs::read_to_string(head).expect("reading .git/HEAD should succeed");

    let hash = if head_contents.trim() == DEFAULT_HEAD {
        fs::read_to_string(CARGO_MANIFEST_DIR.join(".git/refs/heads/main"))
            .expect("reading .git/refs/heads/main should succeed")
    } else {
        head_contents
    }
    .trim()
    .to_owned();

    println!("cargo:rustc-env=GIT_COMMIT_HASH={hash}");

    hash
});

fn main() -> Result<(), Box<dyn Error>> {
    LazyLock::force(&GIT_COMMIT_HASH);

    include_fonts()?;
    include_posts()?;
    include_content()?;
    include_opengraph()?;
    include_resume()?;
    include_favicons()?;
    include_css()?;

    Ok(())
}

fn rerun_path(dir: PathBuf) -> PathBuf {
    println!("cargo:rerun-if-changed={}", dir.display());
    dir
}

fn include_fonts() -> Result<(), Box<dyn Error>> {
    let font_routes = FONTS_DIR.read_dir()?.map(|entry| {
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
                TypedHeader(#mime.into()),
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/fonts/", #font_name))
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

    let fonts_path = OUT_DIR.join("fonts.rs");

    fs::write(fonts_path, prettyplease::unparse(&syn::parse2(tokens)?))?;

    Ok(())
}

fn include_posts() -> Result<(), Box<dyn Error>> {
    let posts_dir = rerun_path(CARGO_MANIFEST_DIR.join("posts"));

    fs::create_dir_all(&*POST_OG_DIR)?;

    let highlighter_configs = HighlighterConfigurations::new()?;

    let mut posts = posts_dir
        .read_dir()?
        .map(|entry| {
            let path = entry?.path();

            if !path.is_file() {
                return Err(format!(
                    "posts directory should only contain files, found: {}",
                    path.display()
                )
                .into());
            }

            let post_name = path.file_stem().unwrap().to_str().unwrap();
            let ext = path.extension().unwrap();

            if ext != "md" {
                return Err(format!("unsupported post extension: {}", ext.display()).into());
            }

            let contents = fs::read_to_string(&path)?;

            let post = Post::new(&highlighter_configs, post_name, &contents)?;

            post.generate_image()?;

            Ok(post)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    posts.sort_unstable_by_key(|post| Reverse(post.date));

    let post_tokens = posts.into_iter().map(
        |Post {
             slug,
             title,
             date: (year, month, day),
             footnotes,
             content,
         }| {
            let footnotes = footnotes.iter().map(|(name, Raw(content))| {
                quote! {
                    (#name, hypertext::Raw(#content))
                }
            });

            quote! {
                crate::post::Post {
                    slug: #slug,
                    title: #title,
                    date: (#year, #month, #day),
                    image: include_bytes!(concat!(env!("OUT_DIR"), "/post-og/", #slug, ".png")),
                    footnotes: &[#(#footnotes,)*],
                    content: hypertext::Raw(#content),
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

    let posts_path = OUT_DIR.join("posts.rs");

    fs::write(posts_path, prettyplease::unparse(&syn::parse2(tokens)?))?;

    Ok(())
}

fn include_content() -> Result<(), Box<dyn Error>> {
    let content_dir = rerun_path(CARGO_MANIFEST_DIR.join("content"));

    let content_routes = content_dir
        .read_dir()?
        .map(|entry| {
            let path = entry.unwrap().path();

            if !path.is_file() {
                return Err(format!(
                    "content directory should only contain files, found: {}",
                    path.display()
                )
                .into());
            }

            let content_name = path.file_name().unwrap().to_str().unwrap();
            let ext = path.extension().unwrap().to_str().unwrap();

            let mime = match ext {
                "png" => quote!(mime::IMAGE_PNG),
                "jpg" => quote!(mime::IMAGE_JPEG),
                _ => return Err(format!("Unsupported content type: {ext}").into()),
            };

            Ok(quote! {
                #content_name => Some((
                    TypedHeader(ContentType::from(#mime)),
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/content/", #content_name))
                ))
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    let tokens = quote!(
        mod content {
            use axum_extra::{
                headers::ContentType,
                TypedHeader
            };

            pub fn get(content: &str) -> Option<(TypedHeader<ContentType>, &'static [u8])> {
                match content {
                    #(#content_routes,)*
                    _ => None,
                }
            }
        }
    );

    let content_path = OUT_DIR.join("content.rs");

    fs::write(content_path, prettyplease::unparse(&syn::parse2(tokens)?))?;

    Ok(())
}

fn include_opengraph() -> Result<(), Box<dyn Error>> {
    let og_file = OPEN_GRAPH_DIR.join("global.typ");

    let document = SiteWorld::new(
        &og_file,
        [("colors", COLORS.default_palette().typst_dict())],
    )?
    .compile_document()?;

    let [page] = &*document.pages else {
        return Err("expected exactly one page in open graph document".into());
    };

    let png = typst_render::render(page, 2.).encode_png()?;

    let path = OUT_DIR.join("og.png");

    fs::write(path, png)?;

    Ok(())
}

fn include_resume() -> Result<(), Box<dyn Error>> {
    let resume_dir = rerun_path(CARGO_MANIFEST_DIR.join("resume"));

    let resume_path = resume_dir.join("resume.typ");

    let document = SiteWorld::new(&resume_path, Dict::new())?.compile_document()?;

    let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).map_err(diagnostic_error)?;

    fs::write(OUT_DIR.join("resume.pdf"), pdf)?;

    Ok(())
}

fn include_favicons() -> Result<(), Box<dyn Error>> {
    let icon_svg_path = STATIC_DIR.join("icon.svg");

    let svg_data = fs::read_to_string(&icon_svg_path)?;
    let single_color_data = COLORS.default_palette().apply_to_css(&svg_data);
    let svg = Tree::from_str(&single_color_data, &Options::default())?;
    let svg_size = svg.size().width();

    let mut ico = IconDir::new(ResourceType::Icon);

    for size in [16, 32, 64, 128] {
        let mut pixmap = Pixmap::new(size, size).expect("creating pixmap should succeed");
        resvg::render(
            &svg,
            #[allow(clippy::cast_precision_loss)]
            Transform::from_scale(size as f32 / svg_size, size as f32 / svg_size),
            &mut pixmap.as_mut(),
        );
        let png_data = pixmap.encode_png()?;
        let icon_image = IconImage::read_png(png_data.as_slice())?;
        ico.add_entry(IconDirEntry::encode(&icon_image)?);
    }

    let ico_path = OUT_DIR.join("favicon.ico");
    ico.write(File::create(ico_path)?)?;

    let multi_color_data = COLORS.apply_to_css(&svg_data);
    let svg_path = OUT_DIR.join("favicon.svg");
    fs::write(svg_path, multi_color_data)?;

    Ok(())
}

fn include_css() -> Result<(), Box<dyn Error>> {
    println!(
        "cargo:rustc-env=THEME_COLOR={}",
        COLORS.default_palette().fg
    );

    let css_path = STATIC_DIR.join("style.css");

    let css_data = fs::read_to_string(&css_path)?;

    let css_data = COLORS.apply_to_css(&css_data);

    let mut stylesheet = StyleSheet::parse(
        &css_data,
        ParserOptions {
            filename: css_path.to_string_lossy().into(),
            ..Default::default()
        },
    )
    .map_err(|e| e.to_string())?;

    let targets = Targets::from(Browsers::from_browserslist(["defaults"])?);

    stylesheet
        .minify(MinifyOptions {
            targets,
            ..Default::default()
        })
        .map_err(|e| e.to_string())?;

    let css_output = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            targets,
            ..Default::default()
        })?
        .code;

    let css_path = OUT_DIR.join("style.css");

    fs::write(css_path, css_output)?;

    Ok(())
}
