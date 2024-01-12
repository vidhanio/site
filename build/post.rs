use std::error::Error;

use heck::ToKebabCase;
use hypertext::{html_elements, maud, GlobalAttributes, Raw};
use pulldown_cmark::{
    CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, MetadataBlockKind, Options, Parser, Tag,
    TagEnd,
};
use serde::{de, Deserialize};

use crate::highlighter_configs::HighlighterConfigurations;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
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
    pub title: String,
    pub date: (u16, u8, u8),
    pub content: String,
    pub footnotes: Vec<(String, Raw<String>)>,
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
        let mut title = None;
        let mut footnotes = Vec::new();

        let mut events = Vec::new();

        while let Some(event) = parser.next() {
            match event {
                Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                    if metadata.is_some() {
                        return Err(format!("multiple metadata blocks for {slug}").into());
                    }

                    metadata = Some(parse_metadata(slug, &mut parser)?);
                }
                Event::Start(Tag::Heading {
                    level: HeadingLevel::H1,
                    ..
                }) => {
                    if title.is_some() {
                        return Err(format!("multiple titles for {slug}").into());
                    }

                    let title_string =
                        collect_text_until(slug, &mut parser, TagEnd::Heading(HeadingLevel::H1))?;

                    title = Some(title_string);
                }
                Event::Start(Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                }) => {
                    events.extend(linkify_heading(
                        slug,
                        &mut parser,
                        level,
                        id,
                        classes,
                        attrs,
                    )?);
                }
                Event::Start(Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                }) if dest_url.starts_with('/') => {
                    events.push(Event::Start(Tag::Image {
                        link_type,
                        dest_url: format!("{dest_url}?v={commit_id}").into(),
                        title,
                        id,
                    }));
                }
                Event::Start(Tag::CodeBlock(block_kind)) => {
                    events.push(Event::Html(
                        highlight_code(highlighter_configs, slug, &mut parser, block_kind)?.into(),
                    ));
                }
                Event::Start(Tag::FootnoteDefinition(name)) => {
                    eat(slug, &mut parser, Event::Start(Tag::Paragraph))?;

                    let text = collect_html_until(&mut parser, TagEnd::Paragraph);

                    eat(slug, &mut parser, Event::End(TagEnd::FootnoteDefinition))?;

                    footnotes.push((name.into_string(), text));
                }
                Event::FootnoteReference(name) => {
                    events.push(Event::Html(
                        maud! {
                            sup {
                                a.footnote href={ "#footnote-" (&*name) } {
                                    "[" (&*name) "]"
                                }
                            }
                        }
                        .render()
                        .into_string()
                        .into(),
                    ));
                }
                event => events.push(event),
            }
        }

        let mut content = String::new();
        pulldown_cmark::html::push_html(&mut content, events.into_iter());

        Ok(Self {
            slug: slug.into(),
            title: title.ok_or_else(|| format!("missing post title for {slug}"))?,
            date: metadata
                .as_ref()
                .map(|metadata| metadata.date)
                .ok_or_else(|| format!("missing post metadata for {slug}"))?,
            footnotes,
            content,
        })
    }
}

fn parse_metadata<'a>(
    slug: &str,
    parser: &mut impl Iterator<Item = Event<'a>>,
) -> Result<Metadata, Box<dyn Error>> {
    let metadata_string = collect_text_until(
        slug,
        parser,
        TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle),
    )?;

    serde_yaml::from_str(&metadata_string)
        .map_err(|e| format!("failed to parse metadata for {slug}: {e}").into())
}

#[allow(clippy::needless_pass_by_value)]
fn linkify_heading<'a>(
    slug: &str,
    parser: &mut impl Iterator<Item = Event<'a>>,
    level: HeadingLevel,
    id: Option<CowStr<'a>>,
    classes: Vec<CowStr<'a>>,
    attrs: Vec<(CowStr<'a>, Option<CowStr<'a>>)>,
) -> Result<[Event<'a>; 5], Box<dyn Error>> {
    let text = collect_text_until(slug, parser, TagEnd::Heading(level))?;

    if id.is_some() {
        return Err(format!("unexpected id for {slug} heading").into());
    }

    let id = text.to_kebab_case();

    Ok([
        Event::Start(Tag::Heading {
            level,
            id: Some(id.clone().into()),
            classes,
            attrs,
        }),
        Event::Start(Tag::Link {
            link_type: LinkType::Reference,
            dest_url: format!("#{id}").into(),
            title: "".into(),
            id: "".into(),
        }),
        Event::Text(text.into()),
        Event::End(TagEnd::Link),
        Event::End(TagEnd::Heading(level)),
    ])
}

#[allow(clippy::needless_pass_by_value)]
fn highlight_code<'a>(
    highlighter_configs: &HighlighterConfigurations,
    slug: &str,
    parser: &mut impl Iterator<Item = Event<'a>>,
    block_kind: CodeBlockKind,
) -> Result<String, Box<dyn Error>> {
    let code = collect_text_until(slug, parser, TagEnd::CodeBlock)?;

    let lang = match &block_kind {
        CodeBlockKind::Fenced(lang) => lang,
        CodeBlockKind::Indented => "",
    };

    let highlighted_code = highlighter_configs.highlight(lang, &code)?;

    Ok(maud! {
        pre {
            code.highlighted { (Raw(highlighted_code)) }
        }
    }
    .render_once()
    .into_string())
}

fn collect_text_until<'a>(
    slug: &str,
    i: &mut impl Iterator<Item = Event<'a>>,
    tag_end: TagEnd,
) -> Result<String, Box<dyn Error>> {
    i.map_while(|event| match event {
        Event::Text(text) => Some(Ok(text.into_string())),
        Event::End(end) if end == tag_end => None,
        _ => Some(Err(format!(
            "unexpected markdown tag for {slug}: expected text or {tag_end:?}, got {event:?}",
        )
        .into())),
    })
    .collect()
}

fn collect_html_until<'a>(i: &mut impl Iterator<Item = Event<'a>>, tag_end: TagEnd) -> Raw<String> {
    let mut buf = String::new();

    pulldown_cmark::html::push_html(
        &mut buf,
        i.take_while(|event| event != &Event::End(tag_end)),
    );

    Raw(buf)
}

#[allow(clippy::needless_pass_by_value)]
fn eat<'a>(
    slug: &str,
    i: &mut impl Iterator<Item = Event<'a>>,
    event: Event<'_>,
) -> Result<(), Box<dyn Error>> {
    i.next()
        .ok_or_else(|| format!("missing markdown event for {slug}").into())
        .and_then(|e| {
            if e == event {
                Ok(())
            } else {
                Err(
                    format!("unexpected markdown event for {slug}: expected {event:?}, got {e:?}",)
                        .into(),
                )
            }
        })
}
