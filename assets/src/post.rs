use std::path::Path;

use heck::ToKebabCase;
use hypertext::{Raw, prelude::*};
use pulldown_cmark::{
    CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, MetadataBlockKind, Options, Parser, Tag,
    TagEnd,
};
use serde::{Deserialize, de};
use typst::foundations::IntoValue;

use crate::{
    COLORS, Error, highlighter_configs::HighlighterConfigurations, typst_world::SiteWorld,
};

#[derive(Clone, Debug)]
pub struct Post {
    slug: String,
    title: String,
    date: (u16, u8, u8),
    content: Raw<String>,
    footnotes: Vec<(String, Raw<String>)>,
}

impl Post {
    pub(super) fn new(
        slug: &str,
        markdown: &str,
        commit_hash: &str,
        highlighters: &HighlighterConfigurations,
    ) -> Result<Self, Error> {
        let mut parser = ParserWrapper::new(slug, markdown, highlighters);
        let mut footnotes = Vec::new();
        let mut events = Vec::new();

        let Some(Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle))) = parser.next()
        else {
            return Err(Error::invalid_post(slug, "missing metadata block"));
        };

        let metadata = parser.parse_metadata()?;

        let Some(Event::Start(Tag::Heading {
            level: HeadingLevel::H1,
            ..
        })) = parser.next()
        else {
            return Err(Error::invalid_post(slug, "missing title"));
        };

        let title = parser.collect_text_until(TagEnd::Heading(HeadingLevel::H1))?;

        while let Some(event) = parser.next() {
            match event {
                Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                    return Err(Error::invalid_post(slug, "multiple metadata blocks"));
                }
                Event::Start(Tag::Heading {
                    level: HeadingLevel::H1,
                    ..
                }) => {
                    return Err(Error::invalid_post(slug, "multiple titles"));
                }
                Event::Start(Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                }) => events.extend(parser.linkify_heading(level, id.as_ref(), classes, attrs)?),
                Event::Start(Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                }) if dest_url.starts_with('/') => {
                    events.push(Event::Start(Tag::Image {
                        link_type,
                        dest_url: format!("{dest_url}?v={commit_hash}").into(),
                        title,
                        id,
                    }));
                }
                Event::Start(Tag::CodeBlock(block_kind)) => {
                    events.push(Event::Html(
                        parser.highlight_code(&block_kind)?.into_inner().into(),
                    ));
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
                        .into_inner()
                        .into(),
                    ));
                }
                Event::Start(Tag::FootnoteDefinition(name)) => {
                    parser.eat(&Event::Start(Tag::Paragraph))?;
                    let text = parser.collect_html_until(TagEnd::Paragraph);
                    parser.eat(&Event::End(TagEnd::FootnoteDefinition))?;
                    footnotes.push((name.into_string(), text));
                }
                event => events.push(event),
            }
        }

        let mut content = String::new();
        pulldown_cmark::html::push_html(&mut content, events.into_iter());

        Ok(Self {
            slug: slug.into(),
            title,
            date: metadata.date,
            content: Raw::dangerously_create(content),
            footnotes,
        })
    }

    fn generate_og_image(&self, project_root: &Path) -> Result<Vec<u8>, Error> {
        let document = SiteWorld::new(
            project_root,
            project_root.join("typst/og/post.typ"),
            [
                ("colors", COLORS.default_palette().typst_dict()),
                ("post-title", self.title.as_str().into_value()),
            ],
        )?
        .compile_document()?;

        let page = document.pages().first().ok_or(Error::PageCount {
            document: "post open graph image",
            expected: 1,
            actual: 0,
        })?;
        if document.pages().len() != 1 {
            return Err(Error::PageCount {
                document: "post open graph image",
                expected: 1,
                actual: document.pages().len(),
            });
        }

        let options = typst_render::RenderOptions {
            pixel_per_pt: typst::utils::Scalar::new(4.),
            ..Default::default()
        };

        Ok(typst_render::render(page, &options).encode_png()?)
    }

    pub(super) fn process(self, project_root: &Path) -> Result<ProcessedPost, Error> {
        let og_image = self.generate_og_image(project_root)?;
        Ok(ProcessedPost {
            slug: self.slug,
            title: self.title,
            date: self.date,
            content: self.content.into_inner(),
            footnotes: self
                .footnotes
                .into_iter()
                .map(|(name, content)| (name, content.into_inner()))
                .collect(),
            og_image,
        })
    }
}

/// A fully processed blog post with owned content and Open Graph image bytes.
#[derive(Clone, Debug)]
pub struct ProcessedPost {
    /// URL slug.
    pub slug: String,
    /// The post title.
    pub title: String,
    /// Publication date as year, month, day.
    pub date: (u16, u8, u8),
    /// Rendered post body HTML.
    pub content: String,
    /// Footnote names and rendered HTML.
    pub footnotes: Vec<(String, String)>,
    /// Generated Open Graph image bytes.
    pub og_image: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Metadata {
    #[serde(deserialize_with = "deserialize_date")]
    date: (u16, u8, u8),
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

    Ok((
        year.parse().map_err(de::Error::custom)?,
        month.parse().map_err(de::Error::custom)?,
        day.parse().map_err(de::Error::custom)?,
    ))
}

struct ParserWrapper<'a, 'input> {
    slug: &'a str,
    parser: Parser<'input>,
    highlighters: &'a HighlighterConfigurations,
}

impl<'a, 'input> ParserWrapper<'a, 'input> {
    fn new(slug: &'a str, input: &'input str, highlighters: &'a HighlighterConfigurations) -> Self {
        Self {
            slug,
            parser: Parser::new_ext(input, Options::all()),
            highlighters,
        }
    }

    fn parse_metadata(&mut self) -> Result<Metadata, Error> {
        let metadata_string =
            self.collect_text_until(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle))?;
        serde_yaml::from_str(&metadata_string).map_err(|source| Error::PostMetadata {
            slug: self.slug.to_owned(),
            source,
        })
    }

    fn collect_text_until(&mut self, tag_end: TagEnd) -> Result<String, Error> {
        self.parser
            .by_ref()
            .map_while(|event| match event {
                Event::Text(text) => Some(Ok(text.into_string())),
                Event::End(end) if end == tag_end => None,
                event => Some(Err(Error::invalid_post(
                    self.slug,
                    format!("expected text or {tag_end:?}, got {event:?}"),
                ))),
            })
            .collect()
    }

    fn collect_html_until(&mut self, tag_end: TagEnd) -> Raw<String> {
        let mut output = String::new();
        pulldown_cmark::html::push_html(
            &mut output,
            self.parser
                .by_ref()
                .take_while(|event| event != &Event::End(tag_end)),
        );
        Raw::dangerously_create(output)
    }

    fn highlight_code(&mut self, block_kind: &CodeBlockKind) -> Result<Rendered<String>, Error> {
        let code = self.collect_text_until(TagEnd::CodeBlock)?;
        let language = match block_kind {
            CodeBlockKind::Fenced(language) => language,
            CodeBlockKind::Indented => "",
        };
        let highlighted_code = self.highlighters.highlight(language, &code)?;

        Ok(maud! {
            pre {
                code.highlighted { (&highlighted_code) }
            }
        }
        .render())
    }

    fn linkify_heading(
        &mut self,
        level: HeadingLevel,
        id: Option<&CowStr<'input>>,
        classes: Vec<CowStr<'input>>,
        attrs: Vec<(CowStr<'input>, Option<CowStr<'input>>)>,
    ) -> Result<[Event<'input>; 5], Error> {
        let text = self.collect_text_until(TagEnd::Heading(level))?;
        if id.is_some() {
            return Err(Error::invalid_post(
                self.slug,
                "heading has an unexpected id",
            ));
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

    fn eat(&mut self, expected: &Event<'_>) -> Result<(), Error> {
        let event = self
            .parser
            .next()
            .ok_or_else(|| Error::invalid_post(self.slug, "missing markdown event"))?;
        if &event == expected {
            Ok(())
        } else {
            Err(Error::invalid_post(
                self.slug,
                format!("expected markdown event {expected:?}, got {event:?}"),
            ))
        }
    }
}

impl<'input> Iterator for ParserWrapper<'_, 'input> {
    type Item = Event<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next()
    }
}
