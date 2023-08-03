use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};

use crate::{error::Error, App};

pub struct SyntaxHighlighterStream<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    app: &'a App,
    iter: I,
}

impl<'a, I> SyntaxHighlighterStream<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub const fn new(app: &'a App, iter: I) -> Self {
        Self { app, iter }
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
            Event::Start(Tag::CodeBlock(block_kind)) => {
                let result = self
                    .iter
                    .by_ref()
                    .map_while(|event| match event {
                        Event::Text(text) => Some(Ok(text.into_string())),
                        Event::End(TagEnd::CodeBlock) => None,
                        _ => Some(Err(Error::UnexpectedMarkdownTag)),
                    })
                    .collect::<crate::Result<String>>();

                let code = match result {
                    Ok(text) => text,
                    Err(error) => return Some(Err(error)),
                };

                let highlighted_code_result = match block_kind {
                    CodeBlockKind::Fenced(lang) => self.app.highlight_code(&lang, &code),
                    CodeBlockKind::Indented => Ok(code),
                };

                let highlighted_code = match highlighted_code_result {
                    Ok(highlighted_code) => highlighted_code,
                    Err(error) => return Some(Err(error)),
                };

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

                Some(Ok(event))
            }
            event => Some(Ok(event)),
        }
    }
}
