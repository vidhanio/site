use std::error::Error;

use maud::{html, PreEscaped};
use pulldown_cmark::{CodeBlockKind, Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use serde::{de, Deserialize};

use crate::highlighter_configs::HighlighterConfigurations;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    pub title: String,

    #[serde(deserialize_with = "deserialize_date")]
    pub date: (u16, u8, u8),
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<(u16, u8, u8), D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date = String::deserialize(deserializer)?;

    let (year, monthday) = date
        .split_once('-')
        .ok_or_else(|| de::Error::custom("missing hyphen in date"))?;

    let (month, day) = monthday
        .split_once('-')
        .ok_or_else(|| de::Error::custom("missing hyphen in date"))?;

    let year = year.parse().map_err(de::Error::custom)?;
    let month = month.parse().map_err(de::Error::custom)?;
    let day = day.parse().map_err(de::Error::custom)?;

    Ok((year, month, day))
}

#[derive(Clone, Debug)]
pub struct Post {
    pub slug: String,
    pub metadata: Metadata,
    pub content: String,
}

impl Post {
    pub fn new(
        highlighter_configs: &HighlighterConfigurations,
        slug: &str,
        markdown: &str,
        commit_id: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let mut parser = Parser::new_ext(markdown, Options::all());

        let mut metadata = None;
        let mut events = Vec::new();

        while let Some(event) = parser.next() {
            match event {
                Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                    let metadata_string = parser
                        .by_ref()
                        .map_while(|event| match event {
                            Event::Text(text) => Some(Ok(text.into_string())),
                            Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => None,
                            _ => Some(Err(
                                "unexpected markdown tag: expected text or yaml metadata block end",
                            )),
                        })
                        .collect::<Result<String, _>>()?;

                    let parsed_metadata = serde_yaml::from_str(&metadata_string)
                        .map_err(|e| format!("failed to parse metadata for {slug}: {e}"))?;

                    metadata = Some(parsed_metadata);
                }
                Event::Start(Tag::CodeBlock(block_kind)) => {
                    let code = parser
                        .by_ref()
                        .map_while(|event| match event {
                            Event::Text(text) => Some(Ok(text.into_string())),
                            Event::End(TagEnd::CodeBlock) => None,
                            _ => Some(Err(
                                "unexpected markdown tag: expected text or code block end",
                            )),
                        })
                        .collect::<Result<String, _>>()?;

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
                Event::Start(Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                }) => {
                    events.push(Event::Start(Tag::Image {
                        link_type,
                        dest_url: format!("/assets/{dest_url}?v={commit_id}").into(),
                        title,
                        id,
                    }));
                }
                event => events.push(event),
            }
        }

        let mut content = String::new();
        pulldown_cmark::html::push_html(&mut content, events.into_iter());

        Ok(Self {
            slug: slug.into(),
            metadata: metadata.ok_or_else(|| format!("missing post metadata for {slug}"))?,
            content,
        })
    }
}
