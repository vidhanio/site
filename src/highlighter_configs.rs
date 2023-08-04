use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
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

impl HighlighterConfigurations {
    pub fn new() -> crate::Result<Self> {
        [
            (
                "rust",
                HighlightConfiguration::new(
                    tree_sitter_rust::language(),
                    include_str!("tree-sitter/queries/rust/highlights.scm"),
                    include_str!("tree-sitter/queries/rust/injections.scm"),
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
                    include_str!("tree-sitter/queries/html/highlights.scm"),
                    include_str!("tree-sitter/queries/html/injections.scm"),
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
            return Ok(encoded_with_line_starts(code, true, true));
        };

        let mut highlighter = Highlighter::new();

        let mut highlights =
            highlighter.highlight(config, code.as_bytes(), None, |lang| self.0.get(lang))?;

        highlights.try_fold(String::new(), |mut html, event| {
            let event = event?;

            match event {
                HighlightEvent::Source { start, end } => {
                    html.push_str(&encoded_with_line_starts(
                        &code[start..end],
                        start == 0,
                        end == code.len(),
                    ));
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

impl Debug for HighlighterConfigurations {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(
                self.0
                    .keys()
                    .map(|&k| (k, "HighlightConfiguration { ... }")),
            )
            .finish()
    }
}

fn encoded_with_line_starts(s: &str, is_true_start: bool, is_true_end: bool) -> String {
    let escaped = if is_true_end {
        html_escape::encode_text_minimal(s.strip_suffix('\n').unwrap_or(s))
    } else {
        html_escape::encode_text_minimal(s)
    };

    let with_line_starts = escaped.replace('\n', "\n<span class=\"line-start\"></span>");

    if is_true_start {
        format!("<span class=\"line-start\"></span>{with_line_starts}")
    } else {
        with_line_starts
    }
}
