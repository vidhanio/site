use axum::response::{IntoResponse, Response};
use maud::{html, Markup, Render, DOCTYPE};

const NAV_LINKS: &[(&str, &str)] = &[
    ("home", "/"),
    ("contact", "/contact"),
    ("resume", "/resume.pdf"),
];

pub struct Document {
    pub path: Option<String>,
    pub title: String,
    pub subheader: Option<Markup>,
    pub content: Markup,
}

impl Render for Document {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    (seo(self.path.as_deref(), &self.title))
                    link rel="stylesheet" href=(concat!("/static/styles.css?v=", env!("GIT_COMMIT_HASH")));
                }

                body {
                    nav {
                        @for (text, href) in NAV_LINKS {
                            a href=(href) { (text) }
                        }
                    }

                    main {
                        header {
                            h1 { (self.title) }
                            @if let Some(subheader) = &self.subheader {
                                (subheader)
                            }
                            hr;
                        }

                        (self.content)
                    }

                    footer {
                        div {
                            a href="https://github.com/vidhanio/site" { "made with with rust" } " and <3 by vidhan."
                        }

                        a href="/static/LICENSE.txt" { "site licensed under agpl-3.0." }

                        div #ring {
                            @for (icon, action) in [('←', "prev"), ('🎲', "random"), ('→', "next")] {
                                a href={"https://ring.simonwu.dev/" (action) "/vidhan"} { (icon) }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl IntoResponse for Document {
    fn into_response(self) -> Response {
        self.render().into_response()
    }
}

fn seo(path: Option<&str>, title: &str) -> Markup {
    let title = format!("{title} | vidhan.io");
    let url = path.map_or_else(
        || "https://vidhan.io".into(),
        |path| format!("https://vidhan.io/{path}"),
    );

    html! {
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        meta charset="utf-8";

        title { (title) }
        meta name="description" content="vidhan's home on the internet.";
        meta name="theme-color" content="#1b1917";
        // link rel="icon" href="/static/favicon.ico";

        meta name="og:title" content=(title);
        meta name="og:description" content="vidhan's home on the internet.";
        meta name="og:url" content=(url);
        meta name="og:type" content="website";

        meta name="twitter:card" content="summary_large_image";
        meta name="twitter:site" content="@vidhanio";
        meta name="twitter:creator" content="@vidhanio";
        meta name="twitter:title" content=(title);
        meta name="twitter:description" content="vidhan's home on the internet.";
    }
}
