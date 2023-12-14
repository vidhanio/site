use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

use axum::extract::FromRef;

use crate::{post::Post, Error};

/// The application.
#[derive(Clone, FromRef)]
pub struct App {
    pub(crate) posts: Arc<[Post<'static>]>,
}

impl Debug for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("App").finish_non_exhaustive()
    }
}

impl App {
    pub(crate) fn get_post(&self, slug: &str) -> crate::Result<&Post> {
        self.posts
            .iter()
            .find(|post| post.slug == slug)
            .ok_or_else(|| Error::PostNotFound(slug.into()))
    }
}
