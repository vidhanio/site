use axum::extract::Path;
use tracing::instrument;

use crate::{
    ResponseResult,
    document::{Document, DocumentDetails, DocumentRequest},
    post::Post,
};

#[instrument(level = "debug", skip(doc), err(Debug))]
pub async fn get(doc: DocumentRequest, Path(slug): Path<String>) -> ResponseResult<Document<Post>> {
    doc.try_build(|| Post::get(slug).map(DocumentDetails::from))
}
