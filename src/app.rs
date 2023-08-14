use std::{cmp::Reverse, io, path::PathBuf, sync::Arc};

use axum::{extract::FromRef, Server};
use futures_util::TryStreamExt;
use request_id::MakeRequestUlid;
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};

use crate::{
    components::{BlogPostMetadata, BlogSlug},
    config::Config,
    highlighter_configs::HighlighterConfigurations,
    pages,
    project::Project,
    Error,
};

/// The application.
#[derive(Debug, Clone, FromRef)]
pub struct App {
    pub(crate) highlighter_configs: Arc<HighlighterConfigurations>,
    pub(crate) config: Arc<Config>,
}

impl App {
    /// Create a new instance of [`App`] from the given [`Config`].
    pub fn new(config: Config) -> crate::Result<Self> {
        let app = Self {
            highlighter_configs: Arc::new(HighlighterConfigurations::new()?),
            config: Arc::new(config),
        };

        Ok(app)
    }

    /// Serve the application.
    pub async fn serve(self) -> crate::Result<()> {
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_response(DefaultOnResponse::new().include_headers(true));

        let cors_layer = CorsLayer::very_permissive();

        let request_id_layer = ServiceBuilder::new()
            .set_x_request_id(MakeRequestUlid)
            .layer(trace_layer)
            .propagate_x_request_id()
            .layer(cors_layer);

        Server::bind(&self.config.socket_address())
            .serve(
                pages::router()
                    .nest_service("/static", ServeDir::new(&self.config.static_dir))
                    .layer(request_id_layer)
                    .with_state(self)
                    .into_make_service(),
            )
            .await?;

        Ok(())
    }
}

impl App {
    pub(crate) fn blog_post_dir(&self) -> PathBuf {
        self.config.content_dir.join("blog")
    }

    pub(crate) fn blog_post_path(&self, slug: &BlogSlug) -> PathBuf {
        self.blog_post_dir()
            .join(slug.as_str())
            .with_extension("md")
    }

    pub(crate) async fn blog_post_markdown(&self, slug: &BlogSlug) -> crate::Result<String> {
        let path = self.blog_post_path(slug);

        fs::read_to_string(&path).await.map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Error::PostNotFound(slug.clone())
            } else {
                Error::ReadPost(slug.clone(), e)
            }
        })
    }

    pub(crate) async fn blog_posts_metadatas(
        &self,
    ) -> crate::Result<Vec<(BlogSlug, BlogPostMetadata)>> {
        let dir = fs::read_dir(&self.blog_post_dir())
            .await
            .map_err(Error::ReadPostDirectory)?;
        let mut metadatas = ReadDirStream::new(dir)
            .map_err(Error::ReadPostDirectory)
            .try_filter_map(|dir_entry| async move {
                let path = dir_entry.path();

                let maybe_slug = path
                    .extension()
                    .filter(|&ext| ext == "md")
                    .and_then(|_| path.file_stem());

                if let Some(slug) = maybe_slug {
                    let slug = BlogSlug::new(slug.to_string_lossy().into())?;

                    let md = fs::read_to_string(&path)
                        .await
                        .map_err(|e| Error::ReadPost(slug.clone(), e))?;

                    let metadata = BlogPostMetadata::from_markdown(&slug, &md)?;

                    Ok(Some((slug, metadata)))
                } else {
                    Ok(None)
                }
            })
            .try_collect::<Vec<_>>()
            .await?;

        metadatas.sort_by_key(|link| Reverse(link.1.date));

        Ok(metadatas)
    }

    pub(crate) fn projects_path(&self) -> PathBuf {
        self.config.content_dir.join("projects.yml")
    }

    pub(crate) async fn projects(&self) -> crate::Result<Vec<Project>> {
        let path = self.projects_path();
        let yaml = fs::read_to_string(&path)
            .await
            .map_err(Error::ReadProjects)?;

        serde_yaml::from_str(&yaml).map_err(Error::DeserializeProjects)
    }
}
