use hypertext::{Buffer, prelude::*};

use crate::{assets::Post, document::DocumentDetails};

impl Post {
    fn time_element(&self) -> impl Renderable {
        let (year, month, day) = self.date();
        maud! {
            time datetime={ (format_args!("{year:04}-{month:02}-{day:02}")) } {
                (format_args!("{year:04}-{month:02}-{day:02}"))
            }
        }
    }
}

impl Renderable for Post {
    fn render_to(&self, output: &mut Buffer) {
        maud! {
            article {
                header {
                    h1 { (self.title()) }
                    (self.time_element())
                }

                section #content { (self.content()) }

                @if self.has_footnotes() {
                    hr;
                    section {
                        h2 #footnotes {
                            a href="#footnotes" { "footnotes" }
                        }

                        ul {
                            @for (name, content) in self.footnotes() {
                                li #{ "footnote-" (name) } {
                                    p {
                                        a.footnote href={ "#footnote-" (name) } {
                                            strong { "[" (name) "]" }
                                        }
                                        " "
                                        (content)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        .render_to(output);
    }
}

impl From<Post> for DocumentDetails<Post> {
    fn from(post: Post) -> Self {
        Self::new(
            post.title().to_owned(),
            format!("/post/{}/og.png", post.slug()),
            post,
        )
    }
}
