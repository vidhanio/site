use maud::{html, Markup, PreEscaped, Render};
use pulldown_cmark::{CodeBlockKind, Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use serde::Deserialize;
use time::{format_description::FormatItem, macros::format_description, Date};

use crate::{highlighter_configs::HighlighterConfigurations, Error};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    pub title: String,
    pub date: Date,
}

impl Metadata {
    pub fn date_text(&self) -> String {
        static FORMAT_DESCRIPTION: &[FormatItem<'static>] =
            format_description!("[year]/[month]/[day]");

        self.date
            .format(FORMAT_DESCRIPTION)
            .expect("date formatting should not fail")
            .to_lowercase()
    }
}

#[derive(Clone, Debug)]
pub struct Post {
    pub slug: String,
    pub metadata: Metadata,
    pub content: Markup,
}

impl Post {
    pub fn new(
        highlighter_configs: &HighlighterConfigurations,
        slug: String,
        markdown: &str,
    ) -> crate::Result<Self> {
        let mut parser = Parser::new_ext(markdown, Options::all());

        let mut metadata = None;
        let mut events = Vec::new();

        while let Some(event) = parser.next() {
            match event {
                Event::Start(Tag::CodeBlock(block_kind)) => {
                    let code = parser
                        .by_ref()
                        .map_while(|event| match event {
                            Event::Text(text) => Some(Ok(text.into_string())),
                            Event::End(TagEnd::CodeBlock) => None,
                            _ => Some(Err(Error::UnexpectedMarkdownTag)),
                        })
                        .collect::<crate::Result<String>>()?;

                    let lang = match &block_kind {
                        CodeBlockKind::Fenced(lang) => lang,
                        CodeBlockKind::Indented => "",
                    };

                    let highlighted_code = highlighter_configs.highlight(lang, &code)?;

                    events.push(Event::Html(
                        html! {
                            pre {
                                code.highlighted { (PreEscaped(highlighted_code)) }
                            }
                        }
                        .into_string()
                        .into(),
                    ));
                }
                Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                    let metadata_string = parser
                        .by_ref()
                        .map_while(|event| match event {
                            Event::Text(text) => Some(Ok(text.into_string())),
                            Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => None,
                            _ => Some(Err(Error::UnexpectedMarkdownTag)),
                        })
                        .collect::<crate::Result<String>>()?;

                    let parsed_metadata = serde_yaml::from_str(&metadata_string).map_err(|e| {
                        Error::DeserializePostMetadata {
                            slug: slug.clone(),
                            source: e,
                        }
                    })?;

                    metadata = Some(parsed_metadata);
                }
                event => events.push(event),
            }
        }

        let mut content = String::new();
        pulldown_cmark::html::push_html(&mut content, events.into_iter());

        Ok(Self {
            slug: slug.clone(),
            metadata: metadata.ok_or_else(|| Error::NoPostMetadata(slug))?,
            content: PreEscaped(content),
        })
    }
}

impl Render for Post {
    fn render_to(&self, buffer: &mut String) {
        self.content.render_to(buffer);
    }
}
