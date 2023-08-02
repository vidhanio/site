use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};
use syntect::{parsing::SyntaxSet, util::LinesWithEndings};

use self::html_generator::HighlightedHtmlGenerator;
use crate::error::Error;

pub struct SyntaxHighlighterStream<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    syntax_set: &'a SyntaxSet,
    iter: I,
}

impl<'a, I> SyntaxHighlighterStream<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub const fn new(syntax_set: &'a SyntaxSet, iter: I) -> Self {
        Self { syntax_set, iter }
    }
}

impl<'a, I> Iterator for SyntaxHighlighterStream<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.iter.next()?;

        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                let syntax = match kind {
                    CodeBlockKind::Fenced(lang) => self
                        .syntax_set
                        .find_syntax_by_token(&lang)
                        .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text()),
                    CodeBlockKind::Indented => self.syntax_set.find_syntax_plain_text(),
                };

                let event = self
                    .iter
                    .by_ref()
                    .map_while(|event| match event {
                        Event::Text(text) => Some(Ok(text)),
                        Event::End(TagEnd::CodeBlock) => None,
                        _ => Some(Err(Error::UnexpectedMarkdownTag)),
                    })
                    .try_fold(
                        HighlightedHtmlGenerator::new_with_class_style(syntax, self.syntax_set),
                        |mut generator, lines| {
                            let lines = lines?;

                            for line in LinesWithEndings::from(&lines) {
                                generator.parse_html_for_line_which_includes_newline(line)?;
                            }

                            Ok::<_, Error>(generator)
                        },
                    )
                    .map(|generator| {
                        let html = generator.finalize();

                        Event::Html(
                            format!(
                                "\
                            <pre \
                                class=\"highlighted-code\"\
                            >\
                                <code class=\"highlighted-code\">\
                                    {html}\
                                </code>\
                            </pre>\
                            "
                            )
                            .into(),
                        )
                    });

                Some(event)
            }
            event => Some(Ok(event)),
        }
    }
}

mod html_generator;
