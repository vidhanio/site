use std::{
    cmp::Reverse,
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

use axum::extract::FromRef;

use crate::{
    blog_post::{BlogPost, BlogPostMetadata},
    project::Project,
    Error,
};

/// The application.
#[derive(Clone, FromRef)]
pub struct App {
    pub(crate) projects: Arc<[Project]>,
    pub(crate) blog_posts: Arc<HashMap<&'static str, BlogPost<'static>>>,
}

impl Debug for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("App").finish_non_exhaustive()
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
