use maud::{html, Markup, Render};

use crate::{blog_post::BlogPostMetadata, icon::CHEVRON};

pub struct Link<'a> {
    slug: &'a str,
    metadata: &'a BlogPostMetadata,
}

impl<'a> Link<'a> {
    pub const fn new(slug: &'a str, metadata: &'a BlogPostMetadata) -> Self {
        Self { slug, metadata }
    }
}

impl Render for Link<'_> {
    fn render(&self) -> Markup {
        html! {
            li {
                a.group.w-full.flex.flex-row.justify-between."bg-stone-200"."dark:bg-stone-800" href={"/blog/" (self.slug)} {
                    div."p-4" {
                        h2.text-lg."text-stone-700"."dark:text-stone-300" {
                            (&self.metadata.title)
                        }
                        time."text-stone-600"."dark:text-stone-400" datetime=(self.metadata.date) {
                            (self.metadata.date_text())
                        }
                        p."text-stone-600"."dark:text-stone-400" {
                            (&self.metadata.description)
                        }
                    }
                    div.grid.place-items-center."p-4"."group-hover:translate-x-1".transition-transform."fill-stone-600"."dark:fill-stone-400" {
                        (CHEVRON)
                    }
                }
            }
        }
    }
}
