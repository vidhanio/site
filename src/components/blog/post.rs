use html_node::{html, text, Node, UnsafeText};
use pulldown_cmark::{CodeBlockKind, Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use time::macros::format_description;

use super::BlogPostMetadata;
use crate::{error::Error, highlighter_configs::HighlighterConfigurations};

pub struct BlogPost<'configs, 'input, 'callback> {
    highlighter_configs: &'configs HighlighterConfigurations,
    parser: Parser<'input, 'callback>,
}

impl<'configs, 'input> BlogPost<'configs, 'input, '_> {
    pub fn new(
        highlighter_configs: &'configs HighlighterConfigurations,
        markdown: &'input str,
    ) -> Self {
        Self {
            highlighter_configs,
            parser: Parser::new_ext(markdown, Options::all()),
        }
    }

    pub fn events_and_metadata(&mut self) -> crate::Result<(Vec<Event<'_>>, BlogPostMetadata)> {
        let mut events = Vec::new();
        let mut metadata = None;

        while let Some(event) = self.parser.next() {
            match event {
                Event::Start(Tag::CodeBlock(block_kind)) => {
                    let code = self
                        .parser
                        .by_ref()
                        .map_while(|event| match event {
                            Event::Text(text) => Some(Ok(text.into_string())),
                            Event::End(TagEnd::CodeBlock) => None,
                            _ => Some(Err(Error::UnexpectedMarkdownTag)),
                        })
                        .collect::<crate::Result<String>>()?;

                    let highlighted_code = match block_kind {
                        CodeBlockKind::Fenced(lang) => {
                            self.highlighter_configs.highlight(&lang, &code)
                        }
                        CodeBlockKind::Indented => self.highlighter_configs.highlight("", &code),
                    }?;

                    let event = Event::Html(
                        format!(
                            "\
                            <pre \
                                class=\"highlighted-code\"\
                            >\
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
                    let metadata_string = self
                        .parser
                        .by_ref()
                        .map_while(|event| match event {
                            Event::Text(text) => Some(Ok(text.into_string())),
                            Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => None,
                            _ => Some(Err(Error::UnexpectedMarkdownTag)),
                        })
                        .collect::<crate::Result<String>>()?;

                    let parsed_metadata =
                        serde_yaml::from_str::<BlogPostMetadata>(&metadata_string)?;

                    metadata = Some(parsed_metadata);
                }
                event => events.push(event),
            }
        }

        let metadata = metadata.ok_or(Error::MissingMetadata)?;

        Ok((events, metadata))
    }

    pub fn metadata(mut self) -> crate::Result<BlogPostMetadata> {
        while let Some(event) = self.parser.next() {
            if event == Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) {
                let metadata_string = self
                    .parser
                    .by_ref()
                    .map_while(|event| match event {
                        Event::Text(text) => Some(Ok(text.into_string())),
                        Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => None,
                        _ => Some(Err(Error::UnexpectedMarkdownTag)),
                    })
                    .collect::<crate::Result<String>>()?;

                let parsed_metadata = serde_yaml::from_str::<BlogPostMetadata>(&metadata_string)?;

                return Ok(parsed_metadata);
            }
        }

        Err(Error::MissingMetadata)
    }

    pub fn into_node(mut self) -> crate::Result<Node> {
        static FORMAT_DESCRIPTION: &[time::format_description::FormatItem<'static>] =
            format_description!("[month repr:long] [day padding:none], [year]");

        let mut buf = String::new();

        let (events, metadata) = self.events_and_metadata()?;

        pulldown_cmark::html::push_html(&mut buf, events.into_iter());

        Ok(html! {
            <header class="flex flex-col text-center gap-8">
                <h1>{text!("{}", metadata.title)}</h1>
                <p class="text-lg text-slate-700 dark:text-slate-300">{text!("{}", metadata.description)}</p>
                <time class="text-md text-slate-600 dark:text-slate-400" datetime=metadata.date>
                    {text!("{}", metadata.date.format(FORMAT_DESCRIPTION)?)}
                </time>
            </header>

            <article class="w-full max-w-3xl flex flex-col gap-6">
                {UnsafeText::from(buf)}
            </article>
        })
    }
}
