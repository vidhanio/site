use std::{
    borrow::Cow,
    convert::Infallible,
    fmt::Debug,
    sync::{Arc, LazyLock},
    time::Duration,
};

use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, OriginalUri},
    http::{StatusCode, Uri, request::Parts},
    response::{IntoResponse, Response},
};
use hypertext::{Buffer, prelude::*};
use moka::future::Cache;
use rspotify::{
    model::{FullTrack, PlayableItem},
    prelude::OAuthClient,
};
use tracing::{error, trace};

use crate::{ResponseResult, SiteResult, SiteState, r#static::Cached};

#[derive(Debug, Clone)]
pub struct DocumentRequest {
    path: Uri,
    now_playing: Option<Arc<FullTrack>>,
}

impl DocumentRequest {
    pub const fn build<T>(self, details: DocumentDetails<T>) -> Document<T> {
        Document {
            request: self,
            details,
        }
    }

    pub fn try_build<T>(
        self,
        f: impl FnOnce() -> SiteResult<DocumentDetails<T>>,
    ) -> ResponseResult<Document<T>> {
        match f() {
            Ok(details) => Ok(self.build(details)),
            Err(error) => Err(self.build(error.into()).into()),
        }
    }

    pub fn try_respond<T>(self, f: impl FnOnce() -> SiteResult<T>) -> ResponseResult<T> {
        match f() {
            Ok(value) => Ok(value),
            Err(error) => Err(self.build(error.into()).into()),
        }
    }

    pub async fn try_respond_async<T>(
        self,
        f: impl AsyncFnOnce() -> SiteResult<T>,
    ) -> ResponseResult<T> {
        match f().await {
            Ok(value) => Ok(value),
            Err(error) => Err(self.build(error.into()).into()),
        }
    }
}

impl FromRequestParts<SiteState> for DocumentRequest {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SiteState,
    ) -> Result<Self, Self::Rejection> {
        static NOW_PLAYING_CACHE: LazyLock<Cache<(), Option<Arc<FullTrack>>>> =
            LazyLock::new(|| {
                Cache::builder()
                    .time_to_live(Duration::from_secs(5))
                    .build()
            });

        let OriginalUri(path) = parts.extract().await?;
        let now_playing = NOW_PLAYING_CACHE
            .get_with((), async {
                trace!("fetching now playing track from spotify");

                state
                    .spotify
                    .current_user_playing_item()
                    .await
                    .inspect_err(|error| {
                        error!(?error, "failed to get now playing track");
                    })
                    .ok()
                    .and_then(|item| {
                        if let PlayableItem::Track(track) = item?.item? {
                            Some(Arc::new(track))
                        } else {
                            None
                        }
                    })
            })
            .await;

        Ok(Self { path, now_playing })
    }
}

#[derive(Debug, Clone)]
pub struct DocumentDetails<T> {
    pub title: Option<Cow<'static, str>>,
    pub og_image: Option<Cow<'static, str>>,
    pub content: T,
    pub status: StatusCode,
}

impl<T> DocumentDetails<T> {
    pub fn new(
        title: impl Into<Cow<'static, str>>,
        og_image: impl Into<Cow<'static, str>>,
        content: T,
    ) -> Self {
        Self {
            title: Some(title.into()),
            og_image: Some(og_image.into()),
            content,
            status: StatusCode::OK,
        }
    }

    pub const fn from_content(content: T) -> Self {
        Self {
            title: None,
            og_image: None,
            content,
            status: StatusCode::OK,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Document<T> {
    request: DocumentRequest,
    details: DocumentDetails<T>,
}

impl<T: Renderable> Renderable for Document<T> {
    fn render_to(&self, output: &mut Buffer) {
        maud! {
            !DOCTYPE
            html lang="en" {
                head {
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta charset="utf-8";

                    title { "vidhan.io / " (self.details.title) }
                    meta name="description" content="vidhan's home on the internet.";
                    meta name="theme-color" content=(env!("THEME_COLOR"));

                    meta name="og:title" content={
                        @if let Some(title) = &self.details.title {
                            title
                        } @else {
                            "vidhan.io"
                        }
                    };
                    meta name="og:url" content={ "https://vidhan.io" %(self.request.path) };
                    meta name="og:type" content="website";
                    meta name="og:image" content={
                        @if let Some(path) = &self.details.og_image {
                            "https://vidhan.io" (Cached(path))
                        } @else {
                            (Cached("https://vidhan.io/og.png"))
                        }
                    };
                    meta name="twitter:card" content="summary_large_image";
                    meta name="twitter:site" content="@vidhanio";
                    meta name="twitter:creator" content="@vidhanio";

                    link rel="stylesheet" href=(Cached("/style.css"));

                    link rel="icon" type="image/svg+xml" href=(Cached("/logo.svg"));
                    link rel="icon" type="image/x-icon" href=(Cached("/favicon.ico"));
                }

                body ."
                    text-vidhan bg-vidhan-white dark:bg-vidhan-black
                    font-berkeley-mono p-body
                " {
                    nav ."flex items-center justify-between" {
                        a #logo href="/" {
                            object ."w-4 pointer-events-none" type="image/svg+xml" data=(Cached("/logo.svg")) {}
                        }

                        @if let Some(now_playing) = &self.request.now_playing {
                            div #now-playing {
                                a href=[now_playing.external_urls.get("spotify")] {
                                    (now_playing.name)
                                }
                                " - "
                                @for (i, artist) in now_playing.artists.iter().enumerate() {
                                    a href=[artist.external_urls.get("spotify")] {
                                        (artist.name)
                                    }
                                    @if i < now_playing.artists.len() - 1 {
                                        ", "
                                    }
                                }
                            }
                        }
                    }

                    hr;

                    main { (self.details.content) }

                    hr;

                    footer ."text-center" {
                        a #repository href="https://github.com/vidhanio/site" {
                            "made with with rust and ❤️ by vidhan."
                        }
                        br;
                        a #license href=(Cached("/LICENSE.txt")) {
                            "site licensed under agpl-3.0."
                        }
                        br;
                        span #ring {
                            a href="https://ring.simonwu.dev/prev/vidhan" {
                                "←"
                            }
                            " "
                            a href="https://ring.simonwu.dev/random/vidhan" {
                                "🎲"
                            }
                            " "
                            a href="https://ring.simonwu.dev/next/vidhan" {
                                "→"
                            }
                        }
                    }
                }
            }
        }
        .render_to(output);
    }
}

impl<T: Renderable> IntoResponse for Document<T> {
    fn into_response(self) -> Response {
        (self.details.status, self.render()).into_response()
    }
}
