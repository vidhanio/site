#![expect(missing_docs)]

mod colors;
mod highlighter_configs;
mod post;
mod typst_world;

pub mod build;

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

pub use colors::{COLORS, Colors, Palette};
use highlighter_configs::HighlighterConfigurations;
use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use post::Post;
pub use post::ProcessedPost;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};
use typst::foundations::Dict;
use typst_pdf::PdfOptions;
use typst_world::{SiteWorld, diagnostic_error};

/// A binary asset together with the content type used to serve it.
#[derive(Clone, Debug)]
pub struct BinaryAsset {
    /// The MIME type of the asset.
    pub content_type: &'static str,
    /// The owned asset bytes.
    pub bytes: Vec<u8>,
}

/// All site assets loaded and processed from a project directory.
#[derive(Clone, Debug)]
pub struct LoadedAssets {
    /// Processed stylesheet.
    pub style: String,
    /// Processed SVG logo.
    pub logo_svg: String,
    /// Generated favicon bytes.
    pub favicon: Vec<u8>,
    /// Generated site Open Graph image.
    pub og_image: Vec<u8>,
    /// Generated resume PDF.
    pub resume: Vec<u8>,
    /// Loaded media files by filename.
    pub media: BTreeMap<String, BinaryAsset>,
    /// Loaded font files by filename.
    pub fonts: BTreeMap<String, BinaryAsset>,
    /// Processed blog posts, newest first.
    pub posts: Vec<ProcessedPost>,
}

/// Short name for the loaded asset collection.
pub type Assets = LoadedAssets;

/// Loads and processes every site asset below `project_root`.
///
/// # Errors
///
/// Returns an error when an asset cannot be read, parsed, or generated.
pub fn load(project_root: impl AsRef<Path>) -> Result<LoadedAssets, Box<dyn std::error::Error>> {
    LoadedAssets::load(project_root)
}

impl LoadedAssets {
    /// Loads and processes every asset below `project_root`.
    ///
    /// # Errors
    ///
    /// Returns an error when an asset cannot be read, parsed, or generated.
    pub fn load(project_root: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let project_root = project_root.as_ref();
        let commit_hash = git_commit_hash(project_root);
        let highlighters = HighlighterConfigurations::new(project_root)?;

        let style = COLORS.apply_to_css(&fs::read_to_string(
            project_root.join("assets/static/style.css"),
        )?);
        let logo_source = fs::read_to_string(project_root.join("assets/static/logo.svg"))?;
        let logo_svg = COLORS.apply_to_css(&logo_source);
        let favicon = favicon_bytes(&logo_svg)?;
        let og_image = open_graph_image(project_root)?;
        let resume = resume_bytes(project_root)?;
        let media = read_media(project_root)?;
        let fonts = read_fonts(project_root)?;
        let posts = read_posts(project_root, &commit_hash, &highlighters)?;

        Ok(Self {
            style,
            logo_svg,
            favicon,
            og_image,
            resume,
            media,
            fonts,
            posts,
        })
    }

    /// Finds a media asset by its URL filename.
    #[must_use]
    pub fn media(&self, name: &str) -> Option<&BinaryAsset> {
        self.media.get(name)
    }

    /// Finds a font asset by its URL filename.
    #[must_use]
    pub fn font(&self, name: &str) -> Option<&BinaryAsset> {
        self.fonts.get(name)
    }
}

fn read_posts(
    project_root: &Path,
    commit_hash: &str,
    highlighters: &HighlighterConfigurations,
) -> Result<Vec<ProcessedPost>, Box<dyn std::error::Error>> {
    let mut posts = fs::read_dir(project_root.join("assets/posts"))?
        .map(|entry| {
            let path = entry?.path();
            if !path.is_file() {
                return Err(format!(
                    "posts directory should only contain files, found: {}",
                    path.display()
                )
                .into());
            }
            if path.extension().and_then(std::ffi::OsStr::to_str) != Some("md") {
                return Err(format!("unsupported post extension: {}", path.display()).into());
            }
            let slug = path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .ok_or_else(|| format!("invalid post filename: {}", path.display()))?
                .to_owned();
            let markdown = fs::read_to_string(&path)?;
            Post::new(&slug, &markdown, commit_hash, highlighters)
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?
        .into_iter()
        .map(|post| post.process(project_root))
        .collect::<Result<Vec<_>, _>>()?;

    posts.sort_unstable_by_key(|post| std::cmp::Reverse(post.date));
    Ok(posts)
}

fn read_media(
    project_root: &Path,
) -> Result<BTreeMap<String, BinaryAsset>, Box<dyn std::error::Error>> {
    read_binary_assets(
        project_root.join("assets/media"),
        |extension| match extension {
            "png" => Some("image/png"),
            "jpg" | "jpeg" => Some("image/jpeg"),
            _ => None,
        },
    )
}

fn read_fonts(
    project_root: &Path,
) -> Result<BTreeMap<String, BinaryAsset>, Box<dyn std::error::Error>> {
    read_binary_assets(
        project_root.join("assets/static/fonts"),
        |extension| match extension {
            "woff" => Some("font/woff"),
            "woff2" => Some("font/woff2"),
            _ => None,
        },
    )
}

fn read_binary_assets(
    directory: PathBuf,
    content_type: impl Fn(&str) -> Option<&'static str>,
) -> Result<BTreeMap<String, BinaryAsset>, Box<dyn std::error::Error>> {
    fs::read_dir(directory)?
        .map(|entry| {
            let path = entry?.path();
            if !path.is_file() {
                return Err(format!(
                    "asset directory should only contain files: {}",
                    path.display()
                )
                .into());
            }
            let name = path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .ok_or_else(|| format!("invalid asset filename: {}", path.display()))?;
            let extension = path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default();
            let Some(content_type) = content_type(extension) else {
                return Ok(None);
            };
            Ok(Some((
                name.to_owned(),
                BinaryAsset {
                    content_type,
                    bytes: fs::read(path)?,
                },
            )))
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()
        .map(|assets| assets.into_iter().flatten().collect())
}

fn open_graph_image(project_root: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let document = SiteWorld::new(
        project_root,
        project_root.join("typst/og/global.typ"),
        [("colors", COLORS.default_palette().typst_dict())],
    )?
    .compile_document()?;
    let [page] = document.pages() else {
        return Err("expected exactly one page in open graph document".into());
    };
    Ok(typst_render::render(
        page,
        &typst_render::RenderOptions {
            pixel_per_pt: typst::utils::Scalar::new(4.),
            ..Default::default()
        },
    )
    .encode_png()?)
}

fn resume_bytes(project_root: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let document = SiteWorld::new(
        project_root,
        project_root.join("resume/resume.typ"),
        Dict::new(),
    )?
    .compile_document()?;
    typst_pdf::pdf(&document, &PdfOptions::default()).map_err(diagnostic_error)
}

fn favicon_bytes(svg_data: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let svg = Tree::from_str(svg_data, &Options::default())?;
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
        let icon_image = IconImage::read_png(pixmap.encode_png()?.as_slice())?;
        ico.add_entry(IconDirEntry::encode(&icon_image)?);
    }

    let mut data = Vec::new();
    ico.write(std::io::Cursor::new(&mut data))?;
    Ok(data)
}

pub(crate) fn git_commit_hash(project_root: &Path) -> String {
    let hash = env::var("GIT_COMMIT_HASH").ok().or_else(|| {
        let head = fs::read_to_string(project_root.join(".git/HEAD")).ok()?;
        let head = head.trim();
        head.strip_prefix("ref: ")
            .and_then(|reference| {
                fs::read_to_string(project_root.join(".git").join(reference)).ok()
            })
            .or_else(|| Some(head.to_owned()))
    });
    let hash = hash.unwrap_or_else(|| "unknown".to_owned());
    hash.get(..7).unwrap_or(&hash).to_owned()
}
