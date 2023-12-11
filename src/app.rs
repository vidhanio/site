use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

use axum::extract::FromRef;

use crate::{post::Post, Error};

/// The application.
#[derive(Clone, FromRef)]
pub struct App {
    pub(crate) blog_posts: Arc<[Post<'static>]>,
}

impl Debug for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("App").finish_non_exhaustive()
    }
}

impl App {
    pub(crate) fn get_blog_post(&self, slug: &str) -> crate::Result<&Post> {
        self.blog_posts
            .iter()
            .find(|blog_post| blog_post.slug == slug)
            .ok_or_else(|| Error::BlogPostNotFound(slug.into()))
    }
}
