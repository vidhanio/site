use std::{collections::HashMap, fmt::Write, fs, path::Path};

use hypertext::Raw;
use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent, Highlighter};

use crate::Error;

pub struct HighlighterConfigurations(HashMap<&'static str, HighlightConfiguration>);

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

    pub(super) fn new(project_root: &Path) -> Result<Self, Error> {
        let css_extensions = fs::read_to_string(
            project_root.join("assets/tree-sitter/queries/css/highlights.ext.scm"),
        )?;

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
                &format!("{}\n{}", tree_sitter_css::HIGHLIGHTS_QUERY, css_extensions),
                "",
            ),
        ]
        .into_iter()
        .map(|(name, language, highlights, injections)| {
            let mut config =
                HighlightConfiguration::new(language, name, highlights, injections, "")?;
            config.configure(Self::HIGHLIGHT_NAMES);
            Ok((name, config))
        })
        .collect::<Result<_, _>>()
        .map(Self)
    }

    pub fn highlight(&self, language: &str, code: &str) -> Result<Raw<String>, Error> {
        let Some(config) = self.0.get(language) else {
            return Ok(Raw::dangerously_create(
                html_escape::encode_text_minimal(code).into_owned(),
            ));
        };

        let mut highlighter = Highlighter::new();
        let mut highlights =
            highlighter.highlight(config, code.as_bytes(), None, None, |lang| self.0.get(lang))?;

        highlights
            .try_fold(String::new(), |mut output, event| {
                match event? {
                    HighlightEvent::HighlightStart(Highlight(index)) => {
                        write!(
                            output,
                            r#"<span class="{}">"#,
                            Self::HIGHLIGHT_NAMES[index].replace('.', " ")
                        )?;
                    }
                    HighlightEvent::Source { start, end } => {
                        html_escape::encode_text_minimal_to_string(&code[start..end], &mut output);
                    }
                    HighlightEvent::HighlightEnd => output.push_str("</span>"),
                }

                Ok(output)
            })
            .map(Raw::dangerously_create)
    }
}
