use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

use axum::Server;
use request_id::MakeRequestUlid;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent, Highlighter};

use crate::{config::Config, pages};

const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "carriage-return",
    "comment",
    "comment.documentation",
    "constant",
    "constant.builtin",
    "constructor",
    "constructor.builtin",
    "embedded",
    "error",
    "escape",
    "function",
    "function.builtin",
    "keyword",
    "markup",
    "markup.bold",
    "markup.heading",
    "markup.italic",
    "markup.link",
    "markup.link.url",
    "markup.list",
    "markup.list.checked",
    "markup.list.numbered",
    "markup.list.unchecked",
    "markup.list.unnumbered",
    "markup.quote",
    "markup.raw",
    "markup.raw.block",
    "markup.raw.inline",
    "markup.strikethrough",
    "module",
    "number",
    "operator",
    "property",
    "property.builtin",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regexp",
    "string.special",
    "string.special.symbol",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
];

/// The application.
#[derive(Clone)]
pub struct App {
    pub(crate) highlighter_configs: Arc<HashMap<&'static str, HighlightConfiguration>>,
    pub(crate) config: Arc<Config>,
}

impl App {
    /// Create a new instance of [`App`] from the given [`Config`].
    pub fn new(config: Config) -> crate::Result<Self> {
        let app = Self {
            highlighter_configs: Arc::new(highlighter_configs()?),
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
                    .nest_service("/public", ServeDir::new(&self.config.public_dir))
                    .layer(request_id_layer)
                    .with_state(self)
                    .into_make_service(),
            )
            .await?;

        Ok(())
    }

    pub(crate) fn highlight_code(&self, language: &str, code: &str) -> crate::Result<String> {
        let Some(config) = self.highlighter_configs.get(language) else {
            return Ok(encoded_with_line_starts(code, true));
        };

        let mut highlighter = Highlighter::new();

        let mut highlights = highlighter.highlight(config, code.as_bytes(), None, |lang| {
            self.highlighter_configs.get(lang)
        })?;

        highlights.try_fold(String::new(), |mut html, event| {
            let event = event?;

            match event {
                HighlightEvent::Source { start, end } => {
                    html.push_str(&encoded_with_line_starts(&code[start..end], start == 0));
                }
                HighlightEvent::HighlightStart(Highlight(idx)) => {
                    use std::fmt::Write;
                    write!(
                        html,
                        "<span class=\"{}\">",
                        HIGHLIGHT_NAMES[idx].replace('.', " ")
                    )
                    .expect("writing to string should not fail");
                }
                HighlightEvent::HighlightEnd => {
                    html.push_str("</span>");
                }
            }

            Ok(html)
        })
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("App").field("config", &self.config).finish()
    }
}

fn encoded_with_line_starts(s: &str, add_to_start: bool) -> String {
    let escaped = html_escape::encode_text_minimal(s);

    let with_line_starts = escaped.replace('\n', "\n<span class=\"line-start\"></span>");

    if add_to_start {
        format!("<span class=\"line-start\"></span>{with_line_starts}")
    } else {
        with_line_starts
    }
}

fn highlighter_configs() -> crate::Result<HashMap<&'static str, HighlightConfiguration>> {
    [
        (
            "rust",
            HighlightConfiguration::new(
                tree_sitter_rust::language(),
                include_str!("tree-sitter/rust/highlights.scm"),
                include_str!("tree-sitter/rust/injections.scm"),
                include_str!("tree-sitter/rust/locals.scm"),
            ),
        ),
        (
            "java",
            HighlightConfiguration::new(
                tree_sitter_java::language(),
                tree_sitter_java::HIGHLIGHT_QUERY,
                "",
                "",
            ),
        ),
        (
            "html",
            HighlightConfiguration::new(
                tree_sitter_html::language(),
                tree_sitter_html::HIGHLIGHT_QUERY,
                tree_sitter_html::INJECTION_QUERY,
                "",
            ),
        ),
    ]
    .into_iter()
    .map(|(name, config)| {
        let mut config = config?;

        config.configure(HIGHLIGHT_NAMES);

        Ok((name, config))
    })
    .collect()
}
