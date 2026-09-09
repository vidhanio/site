use hypertext::prelude::*;
use tracing::instrument;

use crate::{
    document::{Document, DocumentDetails, DocumentRequest},
    markdown_link::MarkdownLink,
    post::Post,
};

#[instrument(level = "debug")]
pub async fn get(doc: DocumentRequest) -> Document<Home> {
    doc.clone().build(DocumentDetails::from_content(Home))
}

#[renderable(pub)]
fn home() -> impl Renderable {
    maud! {
        header #greeting {
            h1 { "vidhan bhatt" }
        }

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
                (MarkdownLink::new("email", "mailto:me@vidhan.io"))
                " or "
                (MarkdownLink::new("@vidhanio", "https://x.com/vidhanio"))
                "."
            }

            span #resume {
                (MarkdownLink::new("resume", "/resume.pdf"))
            }
        }

        section #posts {
            h2 { "posts" }

            ul {
                @for post in Post::all() {
                    li {
                        (MarkdownLink::new(
                            post.title(),
                            format!("/post/{}", post.slug()),
                        ))
                    }
                }
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
}

#[renderable]
fn project(name: &'static str) -> impl Renderable {
    maud! {
        (MarkdownLink::new(
            name,
            format!("https://github.com/vidhanio/{name}"),
        ))
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
