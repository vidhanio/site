mod r#static;

use axum::{
    extract::{Path, State},
    Router,
};
use axum_extra::{
    headers::{ContentDisposition, ContentType},
    TypedHeader,
};
use maud::{html, Render};
use tracing::instrument;

use crate::{
    document::{Document, DocumentParts},
    App,
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/", axum::routing::get(home))
        .route("/post/:slug", axum::routing::get(post))
        .route("/resume.pdf", axum::routing::get(resume))
        .route("/LICENSE.txt", axum::routing::get(license))
        .nest("/static", r#static::router())
}

struct Link<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub href: &'a str,
}

impl Render for Link<'_> {
    fn render(&self) -> maud::Markup {
        html! {
            a href=(self.href) {
                b {
                    (self.name)
                }
                " - "
                (self.description)
            }
        }
    }
}

const PROJECTS: &[Link<'static>] = &[
    Link {
        name: "site",
        description: "this website!",
        href: "https://github.com/vidhanio/site",
    },
    Link {
        name: "html-node",
        description: "an html macro for rust.",
        href: "https://github.com/vidhanio/html-node",
    },
    Link {
        name: "fncli",
        description: "an attribute macro to simplify writing simple clis in rust.",
        href: "https://github.com/vidhanio/fncli",
    },
    Link {
        name: "diswordle",
        description: "a discord bot to play wordle right in your discord server.",
        href: "https://github.com/vidhanio/diswordle",
    },
    Link {
        name: "checkpoint",
        description:
            "a discord bot to provide easy verification for discord servers in my school board.",
        href: "https://github.com/vidhanio/checkpoint",
    },
    Link {
        name: "serenity-commands",
        description: "a library for creating/parsing serenity slash commands.",
        href: "https://github.com/vidhanio/serenity-commands",
    },
];

const CONTACTS: &[Link<'static>] = &[
    Link {
        name: "github",
        description: "vidhanio",
        href: "https://github.com/vidhanio",
    },
    Link {
        name: "twitter",
        description: "@vidhanio",
        href: "https://twitter.com/vidhanio",
    },
    Link {
        name: "linkedin",
        description: "/in/vidhanio",
        href: "https://www.linkedin.com/in/vidhanio",
    },
    Link {
        name: "email",
        description: "me@vidhan.io",
        href: "mailto:me@vidhan.io",
    },
];

#[instrument(level = "debug")]
pub async fn home(State(app): State<App>, doc: DocumentParts) -> Document {
    doc.build_without_title(html! {
        header {
            h1 { "👋 hi, i'm vidhan! " }
            hr;
        }

        section #about {
            p {
                "welcome to my personal website!"
                br;
                br;
                "i'm a software engineer and a computer science student at mcmaster university. \
                my favourite programming language is rust, but i also enjoy writing python. \
                i also love basketball! \
                my favourite player is lebron james and i'm a huge fan of the los angeles lakers."
            }

            a #resume href="/resume.pdf" {
                b { "📄 resume.pdf" }
            }
        }

        section #posts {
            h2 { "📝 posts" }
            hr;
            ul {
                @for post in &*app.posts {
                    li {
                        a href={"/post/" (post.slug)} {
                            time datetime=(post.metadata.date) {
                                (post.metadata.date_text())
                            }
                            " - "
                            b {
                                (post.metadata.title)
                            }
                        }
                    }
                }
            }
        }

        section #projects {
            h2 { "🛠️ projects" }
            hr;
            ul {
                @for project in PROJECTS {
                    li { (project) }
                }
            }
        }

        section #contact {
            h2 { "💬 contact" }
            hr;
            ul {
                @for contact in CONTACTS {
                    li { (contact) }
                }
            }
        }
    })
}

#[instrument(level = "debug", err(Debug))]
pub async fn post(
    State(app): State<App>,
    doc: DocumentParts,
    Path(slug): Path<String>,
) -> crate::Result<Document> {
    let post = app.get_post(&slug)?;

    Ok(doc.build(
        &post.metadata.title,
        html! {
            header {
                h1 {
                    (post.metadata.title)
                }
                time datetime=(post.metadata.date) {
                    (post.metadata.date_text())
                }
                hr;
            }

            article {
                (post)
            }
        },
    ))
}

#[instrument(level = "debug")]
async fn resume() -> (
    TypedHeader<ContentDisposition>,
    TypedHeader<ContentType>,
    &'static [u8],
) {
    const RESUME_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/resume.pdf"));

    (
        TypedHeader(ContentDisposition::inline()),
        TypedHeader(mime::APPLICATION_PDF.into()),
        RESUME_BYTES,
    )
}

#[instrument(level = "debug")]
async fn license() -> (
    TypedHeader<ContentDisposition>,
    TypedHeader<ContentType>,
    &'static [u8],
) {
    const LICENSE: &[u8] = include_bytes!("../../LICENSE.txt");

    (
        TypedHeader(ContentDisposition::inline()),
        TypedHeader(mime::TEXT_PLAIN.into()),
        LICENSE,
    )
}
