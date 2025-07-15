use std::{error::Error, fs, path::PathBuf, sync::LazyLock};

use heck::ToKebabCase;
use hypertext::{Raw, prelude::*};
use pulldown_cmark::{
    CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, MetadataBlockKind, Options, Parser, Tag,
    TagEnd,
};
use serde::{Deserialize, de};
use typst::foundations::IntoValue;

use crate::{
    CACHE_STATIC, GIT_COMMIT_HASH, OPEN_GRAPH_DIR, OUT_DIR, colors::COLORS,
    highlighter_configs::HIGHLIGHTER_CONFIGS, typst_world::SiteWorld,
};

pub static POST_OG_DIR: LazyLock<PathBuf> = LazyLock::new(|| OUT_DIR.join("post-og"));

#[derive(Clone, Debug)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: (u16, u8, u8),
    pub content: Raw<String>,
    pub footnotes: Vec<(String, Raw<String>)>,
}

impl Post {
    pub fn new(slug: &str, markdown: &str) -> Result<Self, Box<dyn Error>> {
        let mut parser = ParserWrapper::new(slug, markdown);

        let mut footnotes = Vec::new();

        let mut events = Vec::new();

        let Some(Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle))) = parser.next()
        else {
            return Err(format!("missing metadata block for {slug}").into());
        };

        let metadata = parser.parse_metadata()?;

        let Some(Event::Start(Tag::Heading {
            level: HeadingLevel::H1,
            ..
        })) = parser.next()
        else {
            return Err(format!("missing title for {slug}").into());
        };

        let title = parser.collect_text_until(TagEnd::Heading(HeadingLevel::H1))?;

        while let Some(event) = parser.next() {
            match event {
                Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                    return Err(format!("multiple metadata blocks for {slug}").into());
                }
                Event::Start(Tag::Heading {
                    level: HeadingLevel::H1,
                    ..
                }) => {
                    return Err(format!("multiple titles for {slug}").into());
                }
                Event::Start(Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                }) => {
                    events.extend(parser.linkify_heading(level, id.as_ref(), classes, attrs)?);
                }
                Event::Start(Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                }) if dest_url.starts_with('/') => {
                    events.push(Event::Start(Tag::Image {
                        link_type,
                        dest_url: if *CACHE_STATIC {
                            format!("{dest_url}?v={}", &*GIT_COMMIT_HASH).into()
                        } else {
                            dest_url
                        },
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
            content: Raw(content),
            footnotes,
        })
    }

    pub fn generate_image(&self) -> Result<(), Box<dyn Error>> {
        let document = SiteWorld::new(
            OPEN_GRAPH_DIR.join("post.typ"),
            [
                ("colors", COLORS.default_palette().typst_dict()),
                ("post-title", self.title.as_str().into_value()),
            ],
        )?
        .compile_document()?;

        let [page] = &*document.pages else {
            return Err("expected exactly one page in open graph document".into());
        };

        let png = typst_render::render(page, 4.).encode_png()?;

        let path = POST_OG_DIR.join(&self.slug).with_extension("png");

        fs::create_dir_all(&*POST_OG_DIR)?;
        fs::write(path, png)?;

        Ok(())
    }
}

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

struct ParserWrapper<'a, 'input> {
    slug: &'a str,
    parser: Parser<'input>,
}

impl<'a, 'input> ParserWrapper<'a, 'input> {
    fn new(slug: &'a str, input: &'input str) -> Self {
        Self {
            slug,
            parser: Parser::new_ext(input, Options::all()),
        }
    }

    fn parse_metadata(&mut self) -> Result<Metadata, Box<dyn Error>> {
        let metadata_string =
            self.collect_text_until(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle))?;

        serde_yaml::from_str(&metadata_string)
            .map_err(|e| format!("failed to parse metadata for {}: {e}", self.slug).into())
    }

    fn collect_text_until(&mut self, tag_end: TagEnd) -> Result<String, Box<dyn Error>> {
        self.parser
            .by_ref()
            .map_while(|event| match event {
                Event::Text(text) => Some(Ok(text.into_string())),
                Event::End(end) if end == tag_end => None,
                _ => Some(Err(format!(
                    "unexpected markdown tag for {}: expected text or {tag_end:?}, got {event:?}",
                    self.slug,
                )
                .into())),
            })
            .collect()
    }

    fn collect_html_until(&mut self, tag_end: TagEnd) -> Raw<String> {
        let mut buf = String::new();

        pulldown_cmark::html::push_html(
            &mut buf,
            self.parser
                .by_ref()
                .take_while(|event| event != &Event::End(tag_end)),
        );

        Raw(buf)
    }

    fn highlight_code(
        &mut self,
        block_kind: &CodeBlockKind,
    ) -> Result<Rendered<String>, Box<dyn Error>> {
        let code = self.collect_text_until(TagEnd::CodeBlock)?;

        let lang = match &block_kind {
            CodeBlockKind::Fenced(lang) => lang,
            CodeBlockKind::Indented => "",
        };

        let highlighted_code = HIGHLIGHTER_CONFIGS.highlight(lang, &code)?;

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
    ) -> Result<[Event<'input>; 5], Box<dyn Error>> {
        let text = self.collect_text_until(TagEnd::Heading(level))?;

        if id.is_some() {
            return Err(format!("unexpected id for {} heading", self.slug).into());
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

    fn eat(&mut self, event: &Event<'_>) -> Result<(), Box<dyn Error>> {
        self.parser
            .next()
            .ok_or_else(|| format!("missing markdown event for {}", self.slug).into())
            .and_then(|e| {
                if &e == event {
                    Ok(())
                } else {
                    Err(format!(
                        "unexpected markdown event for {}: expected {event:?}, got {e:?}",
                        self.slug,
                    )
                    .into())
                }
            })
    }
}

impl<'input> Iterator for ParserWrapper<'_, 'input> {
    type Item = Event<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next()
    }
}
