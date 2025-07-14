use axum::{Router, extract::Path};
use hypertext::prelude::*;
use tracing::instrument;

use crate::{
    SiteError, SiteResult,
    document::{Document, DocumentParts},
    post::Post,
    r#static::Cached,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(home))
        .route("/post/{slug}", axum::routing::get(post))
}

#[instrument(level = "debug")]
pub async fn home(doc: DocumentParts) -> Document<impl Renderable> {
    doc.build_simple(maud! {
        header #greeting {
            h1 { "👋🏽 hi, i'm vidhan!" }
        }

        hr;

        section #about {
            p {
                "welcome to my personal website!"
                br;
                br;
                "i'm a software engineer and a computer science student at mcmaster university.
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
                            time datetime=(post.date_dashed()) {
                                (post.date_slashed())
                            }
                            " - "
                            b {
                                (post.title)
                            }
                        }
                    }
                }
            }
        }

        section #projects {
            h2 { "🛠️ projects" }

            ul {
                Project
                    name="site"
                    description="this website!";
                Project
                    name="hypertext"
                    description="a blazing fast type-checked html macro.";
                Project
                    name="html-node"
                    description="an html macro for rust.";
                Project
                    name="fncli"
                    description="an attribute macro to simplify writing simple clis in rust.";
                Project
                    name="diswordle"
                    description="a discord bot to play wordle right in your discord server.";
                Project
                    name="checkpoint"
                    description="a discord bot to provide easy verification for discord servers in my school board.";
                Project
                    name="serenity-commands"
                    description="a library for creating/parsing serenity slash commands.";
            }
        }

        section #contact {
            h2 { "💬 contact" }

            ul {
                Contact
                    kind="email" 
                    name="me@vidhan.io"
                    href="mailto:me@vidhan.io";
                Contact
                    kind="github"
                    name="vidhanio"
                    href="https://github.com/vidhanio";
                Contact
                    kind="twitter"
                    name="@vidhanio"
                    href="https://twitter.com/vidhanio";
                Contact
                    kind="linkedin"
                    name="/in/vidhanio"
                    href="https://www.linkedin.com/in/vidhanio";
            }
        }
    })
}

#[component]
fn project<'a>(name: &'a str, description: &'a str) -> impl Renderable + 'a {
    maud!(
        li {
            a href={ "https://github.com/vidhanio/" (name) } {
                strong { (name) }
                " - "
                (description)
            }
        }
    )
}

#[component]
fn contact<'a>(kind: &'a str, name: &'a str, href: &'a str) -> impl Renderable + 'a {
    maud! {
        li {
            a href=(href) {
                strong { "[" (kind) "]" }
                " "
                (name)
            }
        }
    }
}

#[instrument(level = "debug", err(Debug))]
pub async fn post(
    doc: DocumentParts,
    Path(slug): Path<String>,
) -> SiteResult<Document<impl Renderable>> {
    let post = Post::get(&slug).ok_or(SiteError::PostNotFound(slug))?;

    Ok(doc.build(
        post.title,
        format!("/post/{}/og.png", post.slug),
        maud! {
            article {
                header {
                    h1 {
                        (post.title)
                    }
                    time datetime=(post.date_dashed()) {
                        (post.date_slashed())
                    }
                }

                hr;

                section #content {
                    (post.content)
                }


                @if !post.footnotes.is_empty() {
                    hr;

                    section #footnotes {
                        h2 #footnotes {
                            a href="#footnotes" { "footnotes" }
                        }

                        ul {
                            @for &(name, content) in post.footnotes {
                                li #{ "footnote-" (name) } {
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
        },
    ))
}
