use aho_corasick::{AhoCorasick, MatchKind};
use axum::{
    body::Body,
    extract::{Query, Request},
    middleware::Next,
    response::Response,
};
use axum_extra::headers::{ContentType, HeaderMapExt};
use futures_util::{StreamExt, TryStreamExt, stream};
use mime::Mime;
use serde::Deserialize;

use crate::{SiteError, SiteResult};

#[derive(Debug, Deserialize)]
pub struct WozeParams {
    #[serde(default)]
    woze: bool,
}

pub async fn wozeify(
    Query(WozeParams { woze }): Query<WozeParams>,
    request: Request,
    next: Next,
) -> SiteResult<Response> {
    let response = next.run(request).await;

    let ac = AhoCorasick::builder()
        .match_kind(MatchKind::LeftmostFirst)
        .build(["vidhanio", "vidhan"])
        .expect("aho corasick should be valid");

    if woze
        && response
            .headers()
            .typed_get::<ContentType>()
            .is_some_and(|ct| Mime::from(ct) == mime::TEXT_HTML_UTF_8)
    {
        let (parts, body) = response.into_parts();

        let html = body
            .into_data_stream()
            .and_then(async |bytes| Ok(stream::iter(bytes).map(Ok::<_, SiteError>)))
            .try_flatten()
            .try_collect::<Vec<u8>>()
            .await?;

        let replaced_html = ac.replace_all_bytes(&html, &["wozeparrot", "wozeparrot"]);

        Ok(Response::from_parts(parts, Body::from(replaced_html)))
    } else {
        Ok(response)
    }
}
