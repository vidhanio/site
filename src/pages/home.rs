use std::borrow::Cow;

use hypertext::{Buffer, prelude::*};
use tracing::instrument;

use crate::{
    assets::Assets,
    document::{Document, DocumentDetails, DocumentRequest},
};

#[instrument(level = "debug")]
pub async fn get(doc: DocumentRequest, assets: Assets) -> Document<Home> {
    doc.build(DocumentDetails::from_content(Home(assets)))
}

#[derive(Debug)]
pub struct Home(Assets);

impl Renderable for Home {
    fn render_to(&self, output: &mut Buffer) {
        maud! {
        section #about {
            p {
                "hi, i'm vidhan. welcome to my personal website."
            }
            p {
                "i'm studying computer science @ mcmaster university.
                i am currently captivated by nixos and rust, and you'll usually find me tinkering with my nix config
                or working on interesting ways to interact with agents."
            }
            p {
                "i'm also a huge fan of the toronto raptors."
            }
            p {
                "reach out via "
                (markdown_link("email", "mailto:me@vidhan.io"))
                " or "
                (markdown_link("@vidhanio", "https://x.com/vidhanio"))
                "."
            }

            span #resume {
                (markdown_link("resume", "/resume.pdf"))
            }
        }

        section #posts {
            h2 { "posts" }

            ul {
                (post_links(&self.0))
            }
        }

        section #projects {
            h2 { "projects" }

            ul {
                @for project in Project::ALL {
                    li { (project) }
                }
            }
        }

        }
        .render_to(output);
    }
}
fn post_links(assets: &Assets) -> impl Renderable + '_ {
    maud! {
        @for (slug, title) in assets.posts() {
            li {
                (markdown_link(
                    title,
                    format!("/post/{slug}"),
                ))
            }
        }
    }
}

#[renderable]
fn project(name: &'static str) -> impl Renderable {
    maud! {
        (markdown_link(
            name,
            format!("https://github.com/vidhanio/{name}"),
        ))
    }
}

fn markdown_link<'a>(
    label: impl Into<Cow<'a, str>>,
    href: impl Into<Cow<'a, str>>,
) -> impl Renderable + 'a {
    let label = label.into();
    let href = href.into();

    maud! {
        a .markdown-link href=(href) {
            strong { "[" (label) "]" }
            "("
            span .markdown-link-destination { (href) }
            ")"
        }
    }
}

impl Project {
    const ALL: [Self; 7] = [
        Self { name: "site" },
        Self { name: "hypertext" },
        Self { name: "html-node" },
        Self { name: "fncli" },
        Self { name: "diswordle" },
        Self { name: "checkpoint" },
        Self {
            name: "serenity-commands",
        },
    ];
}
