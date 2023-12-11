use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter, Write},
};

use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent, Highlighter};

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

pub struct HighlighterConfigurations(HashMap<&'static str, HighlightConfiguration>);

macro_rules! tree_sitter_query {
    ($path:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tree-sitter/queries/",
            $path,
            ".scm"
        ))
    };
}

impl HighlighterConfigurations {
    pub fn new() -> crate::Result<Self> {
        [
            (
                "rust",
                HighlightConfiguration::new(
                    tree_sitter_rust::language(),
                    tree_sitter_query!("rust/highlights"),
                    tree_sitter_query!("rust/injections"),
                    "",
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
                    tree_sitter_query!("html/highlights"),
                    tree_sitter_query!("html/injections"),
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
        .collect::<crate::Result<_>>()
        .map(Self)
    }

    pub fn highlight(&self, language: &str, code: &str) -> crate::Result<String> {
        let Some(config) = self.0.get(language) else {
            return Ok(html_escape::encode_text_minimal(code).into());
        };

        let mut highlighter = Highlighter::new();

        let mut highlights =
            highlighter.highlight(config, code.as_bytes(), None, |lang| self.0.get(lang))?;

        highlights.try_fold(String::new(), |mut buf, event| {
            match event? {
                HighlightEvent::Source { start, end } => {
                    html_escape::encode_text_minimal_to_string(&code[start..end], &mut buf);
                }
                HighlightEvent::HighlightStart(Highlight(idx)) => {
                    write!(
                        buf,
                        "<span class=\"{}\">",
                        HIGHLIGHT_NAMES[idx].replace('.', " ")
                    )
                    .expect("writing to a string should be infallible");
                }
                HighlightEvent::HighlightEnd => {
                    buf.push_str("</span>");
                }
            }

            Ok(buf)
        })
    }
}

impl Debug for HighlighterConfigurations {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(
                self.0
                    .keys()
                    .map(|&k| (k, format_args!("HighlightConfiguration"))),
            )
            .finish()
    }
}
