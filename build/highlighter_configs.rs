use std::{collections::HashMap, error::Error, fmt::Write, sync::LazyLock};

use hypertext::Raw;
use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent, Highlighter};

pub struct HighlighterConfigurations(HashMap<&'static str, HighlightConfiguration>);

pub static HIGHLIGHTER_CONFIGS: LazyLock<HighlighterConfigurations> = LazyLock::new(|| {
    HighlighterConfigurations::new().expect("should be able to create highlighter configurations")
});

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

    fn new() -> Result<Self, Box<dyn Error>> {
        [
            (
                "rust",
                tree_sitter_rust::LANGUAGE.into(),
                tree_sitter_rust::HIGHLIGHTS_QUERY,
                tree_sitter_rust::INJECTIONS_QUERY,
            ),
            (
                "java",
                tree_sitter_java::LANGUAGE.into(),
                tree_sitter_java::HIGHLIGHTS_QUERY,
                "",
            ),
            (
                "html",
                tree_sitter_html::LANGUAGE.into(),
                tree_sitter_html::HIGHLIGHTS_QUERY,
                tree_sitter_html::INJECTIONS_QUERY,
            ),
            (
                "css",
                tree_sitter_css::LANGUAGE.into(),
                &format!(
                    "{}\n{}",
                    tree_sitter_css::HIGHLIGHTS_QUERY,
                    tree_sitter_query!("css/highlights.ext")
                ),
                "",
            ),
            (
                "dockerfile",
                tree_sitter_dockerfile::language(),
                tree_sitter_query!("dockerfile/highlights"),
                "",
            ),
        ]
        .into_iter()
        .map(|(name, lang, highlights, injections)| {
            let mut config = HighlightConfiguration::new(lang, name, highlights, injections, "")?;

            config.configure(Self::HIGHLIGHT_NAMES);

            Ok((name, config))
        })
        .collect::<Result<_, _>>()
        .map(Self)
    }

    pub fn highlight(&self, language: &str, code: &str) -> Result<Raw<String>, Box<dyn Error>> {
        let Some(config) = self.0.get(language) else {
            return Ok(Raw(html_escape::encode_text_minimal(code).into()));
        };

        let mut highlighter = Highlighter::new();

        let mut highlights =
            highlighter.highlight(config, code.as_bytes(), None, |lang| self.0.get(lang))?;

        highlights
            .try_fold(String::new(), |mut buf, event| {
                match event? {
                    HighlightEvent::HighlightStart(Highlight(idx)) => {
                        _ = write!(
                            buf,
                            r#"<span class="{}">"#,
                            Self::HIGHLIGHT_NAMES[idx].replace('.', " ")
                        );
                    }
                    HighlightEvent::Source { start, end } => {
                        html_escape::encode_text_minimal_to_string(&code[start..end], &mut buf);
                    }
                    HighlightEvent::HighlightEnd => {
                        buf.push_str("</span>");
                    }
                }

                Ok(buf)
            })
            .map(Raw)
    }
}
