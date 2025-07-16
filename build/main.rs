#![expect(missing_docs)]

mod colors;
mod highlighter_configs;
mod post;
mod tailwind;
mod typst_world;

use std::{
    cmp::Reverse,
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    sync::LazyLock,
    thread,
};

use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use quote::quote;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};
use typst::foundations::Dict;
use typst_pdf::PdfOptions;

use self::{post::Post, typst_world::SiteWorld};
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
    LazyLock::new(|| rerun_path(CARGO_MANIFEST_DIR.join("assets/static")));

static FONTS_DIR: LazyLock<PathBuf> = LazyLock::new(|| STATIC_DIR.join("fonts"));

static TYPST_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| rerun_path(CARGO_MANIFEST_DIR.join("typst")));

static GIT_COMMIT_HASH: LazyLock<String> = LazyLock::new(|| {
    println!("cargo:rerun-if-env-changed=GIT_COMMIT_HASH");

    let hash = env::var("GIT_COMMIT_HASH").ok().or_else(|| {
        let head_path = rerun_path(CARGO_MANIFEST_DIR.join(".git/HEAD"));
        let head = fs::read_to_string(head_path).ok()?;
        let head = head.trim();
        head.strip_prefix("ref: ")
            .and_then(|reference| {
                fs::read_to_string(CARGO_MANIFEST_DIR.join(".git").join(reference)).ok()
            })
            .or_else(|| Some(head.to_owned()))
    });

    let hash = hash.unwrap_or_else(|| "unknown".to_owned());
    let hash = hash.get(..7).unwrap_or(&hash).to_owned();

    println!("cargo:rustc-env=GIT_COMMIT_HASH={hash}");

    hash
});

fn main() -> Result<(), Box<dyn Error>> {
    LazyLock::force(&GIT_COMMIT_HASH);

    include_fonts()?;
    include_posts()?;
    include_media()?;
    include_opengraph()?;
    include_resume()?;
    include_favicons()?;
    include_tailwind()?;

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
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/static/fonts/", #font_name))
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
    let posts_dir = rerun_path(CARGO_MANIFEST_DIR.join("assets/posts"));

    fs::create_dir_all(&*POST_OG_DIR)?;

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

            let post = Post::new(post_name, &contents)?;

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
            let content = content.into_inner();
            let footnotes = footnotes.into_iter().map(|(name, content)| {
                let content = content.into_inner();
                quote! {
                    (#name, hypertext::Raw::dangerously_create(#content))
                }
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

fn include_media() -> Result<(), Box<dyn Error>> {
    let media_dir = rerun_path(CARGO_MANIFEST_DIR.join("assets/media"));

    let media_names = media_dir
        .read_dir()?
        .map(|entry| {
            let path = entry.unwrap().path();

            if !path.is_file() {
                return Err(format!(
                    "media directory should only contain files, found: {}",
                    path.display()
                )
                .into());
            }

            let media_name = path.file_name().unwrap().to_str().unwrap();
            let ext = path.extension().unwrap().to_str().unwrap();

            let mime = match ext {
                "png" => quote!(mime::IMAGE_PNG),
                "jpg" => quote!(mime::IMAGE_JPEG),
                _ => return Err(format!("unsupported content type: {ext}").into()),
            };

            Ok(quote! {
                #media_name => Some((
                    TypedHeader(ContentType::from(#mime)),
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/media/", #media_name))
                ))
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    let tokens = quote!(
        mod media {
            use axum_extra::{
                headers::ContentType,
                TypedHeader
            };

            pub fn get(media: &str) -> Option<(TypedHeader<ContentType>, &'static [u8])> {
                match media {
                    #(#media_names,)*
                    _ => None,
                }
            }
        }
    );

    let media_path = OUT_DIR.join("media.rs");

    fs::write(media_path, prettyplease::unparse(&syn::parse2(tokens)?))?;

    Ok(())
}

fn include_opengraph() -> Result<(), Box<dyn Error>> {
    let og_file = TYPST_DIR.join("og/global.typ");

    let document = SiteWorld::new(
        &og_file,
        [("colors", COLORS.default_palette().typst_dict())],
    )?
    .compile_document()?;

    let [page] = document.pages() else {
        return Err("expected exactly one page in open graph document".into());
    };

    let options = typst_render::RenderOptions {
        pixel_per_pt: typst::utils::Scalar::new(4.),
        ..Default::default()
    };
    let png = typst_render::render(page, &options).encode_png()?;

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
    let logo_svg_path = STATIC_DIR.join("logo.svg");

    let svg_data = fs::read_to_string(&logo_svg_path)?;
    let single_color_data = COLORS.default_palette().apply_to_css(&svg_data);
    let svg = Tree::from_str(&single_color_data, &Options::default())?;
    let svg_size = svg.size().width();

    let mut ico = IconDir::new(ResourceType::Icon);

    for size in [16, 32, 64, 128] {
        let mut pixmap = Pixmap::new(size, size).expect("creating pixmap should succeed");
        resvg::render(
            &svg,
            #[expect(clippy::cast_precision_loss)]
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
    let svg_path = OUT_DIR.join("logo.svg");
    fs::write(svg_path, multi_color_data)?;

    Ok(())
}

fn include_tailwind() -> Result<(), Box<dyn std::error::Error>> {
    let tailwind_path = tailwind::download()?;

    println!(
        "cargo:rustc-env=THEME_COLOR={}",
        COLORS.default_palette().fg
    );

    let css_path = STATIC_DIR.join("style.css");

    let css_data = fs::read_to_string(&css_path)?;

    let css_data = COLORS.apply_to_css(&css_data);

    let mut child = Command::new(tailwind_path)
        .args([
            "-i",
            "-",
            "-o",
            &OUT_DIR.join("style.css").to_string_lossy(),
            "-m",
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    thread::spawn(move || {
        stdin
            .write_all(css_data.as_bytes())
            .expect("writing to stdin should succeed");
    });
    let output = child.wait_with_output()?;

    if !output.status.success() {
        return Err(format!(
            "failed to execute `tailwindcss`:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
