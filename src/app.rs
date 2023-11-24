use std::{
    cmp::Reverse,
    collections::HashMap,
    ffi::OsStr,
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

use axum::{extract::FromRef, Router};
use include_dir::{include_dir, Dir};
use tracing::instrument;

use crate::{
    blog_post::{BlogPost, BlogPostMetadata},
    config::Config,
    highlighter_configs::HighlighterConfigurations,
    pages,
    project::Project,
    r#static, Error,
};

const PROJECTS_YAML: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/content/projects.yml"));
static BLOG_POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content/blog");

/// The application.
#[derive(Clone, FromRef)]
pub struct App {
    pub(crate) config: Arc<Config>,
    pub(crate) projects: Arc<[Project]>,
    pub(crate) blog_posts: Arc<HashMap<&'static str, BlogPost<'static>>>,
}

impl Debug for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl App {
    /// Create a new instance of [`App`] from the given [`Config`].
    #[instrument(level = "debug", ret, err(Debug))]
    pub fn new(config: Config) -> crate::Result<Self> {
        let highlighter_configs = HighlighterConfigurations::new()?;

        let app = Self {
            config: Arc::new(config),
            projects: serde_yaml::from_str::<Vec<_>>(PROJECTS_YAML)
                .map_err(Error::DeserializeProjects)?
                .into(),
            blog_posts: BLOG_POSTS_DIR
                .files()
                .filter_map(|file| {
                    let path = file.path();

                    let slug = if path.extension() == Some(OsStr::new("md")) {
                        path.file_stem()?.to_str()
                    } else {
                        None
                    }?;

                    let markdown = file.contents_utf8()?;

                    let blog_post = BlogPost::new(&highlighter_configs, slug, markdown);

                    Some(blog_post.map(|blog_post| (slug, blog_post)))
                })
                .collect::<crate::Result<HashMap<_, _>>>()?
                .into(),
        };

        Ok(app)
    }

    /// Serve the application.
    #[instrument(level = "debug", ret, err(Debug))]
    pub async fn serve(self) -> crate::Result<()> {
        let tcp_listener = self.config.tcp_listener().await?;

        let router = Router::new()
            .nest("/", pages::router())
            .nest("/static", r#static::router())
            .with_state(self);

        axum::Server::from_tcp(tcp_listener)?
            .serve(router.into_make_service())
            .await
            .map_err(Error::Serve)?;

        Ok(())
    }
}

impl App {
    pub(crate) fn get_blog_post(&self, slug: &str) -> crate::Result<&BlogPost> {
        self.blog_posts
            .get(slug)
            .ok_or_else(|| Error::BlogPostNotFound(slug.into()))
    }

    pub(crate) fn blog_posts_metadatas(&self) -> Box<[(&str, &BlogPostMetadata)]> {
        let mut metadatas = self
            .blog_posts
            .iter()
            .map(|(&slug, blog_post)| (slug, &blog_post.metadata))
            .collect::<Vec<_>>();

        metadatas.sort_by_key(|(_, metadata)| Reverse(metadata.date));

        metadatas.into_boxed_slice()
    }
}
