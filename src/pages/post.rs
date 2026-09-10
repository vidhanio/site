use axum::extract::Path;
use tracing::instrument;

use crate::{
    ResponseResult,
    assets::{Assets, Post},
    document::{Document, DocumentDetails, DocumentRequest},
};

#[instrument(level = "debug", skip(doc), err(Debug))]
pub async fn get(
    doc: DocumentRequest,
    assets: Assets,
    Path(slug): Path<String>,
) -> ResponseResult<Document<Post>> {
    doc.try_build(|| assets.post(slug).map(DocumentDetails::from))
}
