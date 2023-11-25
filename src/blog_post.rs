use maud::{Markup, PreEscaped, Render};
use pulldown_cmark::{CodeBlockKind, Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use serde::Deserialize;
use time::{format_description::FormatItem, macros::format_description, Date};

use crate::{highlighter_configs::HighlighterConfigurations, Error};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BlogPostMetadata {
    pub title: String,
    pub date: Date,
    pub description: String,
}

impl BlogPostMetadata {
    pub fn date_text(&self) -> String {
        static FORMAT_DESCRIPTION: &[FormatItem<'static>] =
            format_description!("[month repr:long] [day padding:none], [year]");

        self.date
            .format(FORMAT_DESCRIPTION)
            .expect("date formatting should not fail")
    }
}

#[derive(Clone, Debug)]
pub struct BlogPost<'a> {
    pub metadata: BlogPostMetadata,
    pub events: Box<[Event<'a>]>,
}

impl<'a> BlogPost<'a> {
    pub fn new(
        highlighter_configs: &HighlighterConfigurations,
        slug: &'a str,
        markdown: &'a str,
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

                    let event = Event::Html(
                        format!(
                            "\
                            <pre>\
                                <code class=\"highlighted-code\">\
                                    {highlighted_code}\
                                </code>\
                            </pre>\
                            "
                        )
                        .into(),
                    );

                    events.push(event);
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

                    let parsed_metadata = serde_yaml::from_str(&metadata_string)
                        .map_err(|e| Error::DeserializePostMetadata(slug.into(), e))?;

                    metadata = Some(parsed_metadata);
                }
                event => events.push(event),
            }
        }

        let metadata = metadata.ok_or_else(|| Error::NoPostMetadata(slug.into()))?;
        let events = events.into();

        Ok(Self { metadata, events })
    }
}

impl Render for BlogPost<'_> {
    fn render(&self) -> Markup {
        let mut buf = String::new();
        pulldown_cmark::html::push_html(&mut buf, self.events.iter().cloned());
        PreEscaped(buf)
    }
}
