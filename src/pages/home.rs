use hypertext::prelude::*;
use tracing::instrument;

use crate::{
    document::{Document, DocumentDetails, DocumentRequest},
    post::Post,
    r#static::Cached,
};

#[instrument(level = "debug")]
pub async fn get(doc: DocumentRequest) -> Document<Home> {
    doc.clone().build(DocumentDetails::from_content(Home))
}

#[renderable(pub)]
fn home() -> impl Renderable {
    maud! {
        header #greeting {
            h1."text-3xl font-bold" { "👋🏽 hi, i'm vidhan!" }
        }

        hr;

        section #about {
            p {
                "welcome to my personal website!"
                br;
                br;
                "i'm a software engineer working @ flipp and studying computer science @ mcmaster university.
                my favourite programming language is rust, but i also enjoy writing python.
                i also love basketball! i'm a huge fan of the toronto raptors 🦖."
            }

            a #resume href=(Cached("/resume.pdf")) {
                b { "📄 resume" }
            }
        }

        section #posts {
            h2 { "📝 posts" }

            ul {
                @for post in Post::ALL {
                    li {
                        a href={ "/post/" (post.slug) } {
                            (post.time_element())
                            " - "
                            b { (post.title) }
                        }
                    }
                }
            }
        }

        section #projects {
            h2 { "🛠️ projects" }

            ul {
                @for project in Project::ALL {
                    li { (project) }
                }
            }
        }

        section #contact {
            h2 { "💬 contact" }

            ul {
                @for contact in Contact::ALL {
                    li { (contact) }
                }
            }
        }
    }
}

#[renderable]
fn project(name: &'static str, description: &'static str) -> impl Renderable {
    maud! {
        a href={ "https://github.com/vidhanio/" (name) } {
            strong { (name) } ": " (description)
        }
    }
}

impl Project {
    const ALL: [Self; 7] = [
        Self {
            name: "site",
            description: "this website!",
        },
        Self {
            name: "hypertext",
            description: "a blazing fast type-checked html macro.",
        },
        Self {
            name: "html-node",
            description: "an html macro for rust.",
        },
        Self {
            name: "fncli",
            description: "an attribute macro to simplify writing simple clis in rust.",
        },
        Self {
            name: "diswordle",
            description: "a discord bot to play wordle right in your discord server.",
        },
        Self {
            name: "checkpoint",
            description: "a discord bot to provide easy verification for discord servers in my school board.",
        },
        Self {
            name: "serenity-commands",
            description: "a library for creating/parsing serenity slash commands.",
        },
    ];
}

#[renderable]
fn contact(kind: &'static str, name: &'static str, href: &'static str) -> impl Renderable {
    maud! {
        a href=(href) {
            strong { (kind) } ": " (name)
        }
    }
}

impl Contact {
    const ALL: [Self; 4] = [
        Self {
            kind: "email",
            name: "me@vidhan.io",
            href: "mailto:me@vidhan.io",
        },
        Self {
            kind: "github",
            name: "vidhanio",
            href: "https://github.com/vidhanio",
        },
        Self {
            kind: "x",
            name: "@vidhanio",
            href: "https://x.com/vidhanio",
        },
        Self {
            kind: "linkedin",
            name: "/in/vidhanio",
            href: "https://www.linkedin.com/in/vidhanio",
        },
    ];
}
