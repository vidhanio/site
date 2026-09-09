use std::borrow::Cow;

use hypertext::{Buffer, prelude::*};

#[derive(Clone, Debug)]
pub struct MarkdownLink {
    label: Cow<'static, str>,
    href: Cow<'static, str>,
}

impl MarkdownLink {
    pub fn new(label: impl Into<Cow<'static, str>>, href: impl Into<Cow<'static, str>>) -> Self {
        let href = href.into();
        Self {
            label: label.into(),
            href,
        }
    }
}

impl Renderable for MarkdownLink {
    fn render_to(&self, output: &mut Buffer) {
        maud! {
            a .markdown-link href=(self.href) {
                strong { "[" (self.label) "]" }
                "("
                span .markdown-link-destination { (self.href) }
                ")"
            }
        }
        .render_to(output);
    }
}
