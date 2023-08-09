mod blog;
mod footer;
mod icons;
mod nav;
mod project_link;
mod seo;

use html_node::{html, Node};

pub use self::{
    blog::{BlogLink, BlogPost, BlogPostMetadata, BlogSlug},
    project_link::ProjectLink,
};
use self::{
    footer::{footer, FooterLink},
    nav::{nav, NavLink},
    seo::seo,
};

const NAV_LINKS: [NavLink<'static>; 3] = [
    NavLink {
        name: "home",
        link: "/",
    },
    NavLink {
        name: "projects",
        link: "/projects",
    },
    NavLink {
        name: "blog",
        link: "/blog",
    },
];

const FOOTER_LINKS: [FooterLink<'static>; 6] = [
    FooterLink {
        name: "source code",
        url: "https://github.com/vidhanio/site",
        rel: None,
    },
    FooterLink {
        name: "github",
        url: "https://github.com/vidhanio",
        rel: None,
    },
    FooterLink {
        name: "twitter",
        url: "https://twitter.com/vidhanio",
        rel: None,
    },
    FooterLink {
        name: "linkedin",
        url: "https://linkedin.com/in/vidhanio",
        rel: None,
    },
    FooterLink {
        name: "mastodon",
        url: "https://fosstodon.org/@vidhan",
        rel: Some("me"),
    },
    FooterLink {
        name: "email",
        url: "mailto:me@vidhan.io",
        rel: None,
    },
];

pub fn document(path: Option<&str>, content: Node) -> Node {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                {seo(path)}
                <link rel="stylesheet" href="/static/css/styles.css" />
            </head>

            <body
                class="\
                    min-h-screen \
                    px-[10%] lg:px-[25%] \
                    flex flex-col items-center \
                    font-mono [font-feature-settings:'ss05'] \
                    bg-slate-100 dark:bg-slate-900 dark:text-slate-300 text-slate-700\
                "
            >
                {nav(NAV_LINKS)}
                <main class="w-full py-8 flex-1 border-indigo-600 flex flex-col gap-8">
                    {content}
                </main>
                {footer(FOOTER_LINKS)}
            </body>
        </html>
    }
}
