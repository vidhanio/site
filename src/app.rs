use std::sync::Arc;

use axum::Server;
use request_id::MakeRequestUlid;
use syntect::parsing::SyntaxSet;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};

use crate::{config::Config, pages};

/// The application.
#[derive(Debug, Clone)]
pub struct App {
    pub(crate) syntax_set: Arc<SyntaxSet>,
    pub(crate) config: Arc<Config>,
}

impl App {
    /// Create a new instance of [`App`] from the given [`Config`].
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            syntax_set: Arc::new(SyntaxSet::load_defaults_newlines()),
            config: Arc::new(config),
        }
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
                    .nest_service("/public", ServeDir::new(&self.config.public_dir))
                    .layer(request_id_layer)
                    .with_state(self)
                    .into_make_service(),
            )
            .await?;

        Ok(())
    }
}
