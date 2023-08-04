mod blog;
mod footer_link;
mod nav_link;
mod project_card;
mod section;
mod seo;

use html_node::{html, Node};

pub use self::{
    blog::{BlogCard, BlogPost, BlogPostMetadata, BlogSlug},
    project_card::ProjectCard,
    section::section,
};
use self::{footer_link::FooterLink, nav_link::NavLink, seo::seo};

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
                <link rel="stylesheet" href="/public/css/styles.css" />
            </head>

            <body class="bg-slate-100 font-mono text-slate-900 [font-feature-settings:'ss05'] dark:bg-slate-900 dark:text-slate-100">
                {layout(content)}
            </body>
        </html>
    }
}

fn layout(content: Node) -> Node {
    html! {
        <nav class="border-b-2 border-indigo-200 p-8 dark:border-indigo-800 sm:p-16">
            <ul class="flex flex-row justify-center gap-4">
                {NAV_LINKS}
            </ul>
        </nav>

        <main class="flex flex-col items-center gap-16 px-8 pb-8 pt-16 sm:p-16">
            {content}
        </main>

        <footer class="flex flex-col items-center gap-4 border-t-2 border-indigo-200 p-8 dark:border-indigo-800 sm:p-16">
            <p class="text-lg text-slate-400 dark:text-slate-600">
                "made with <3 by vidhan."
            </p>
            <ul class="flex flex-row justify-center gap-4">
                {FOOTER_LINKS}
            </ul>
            <a
                href="https://github.com/vidhanio/site/blob/main/LICENSE"
                class="text-xs font-thin text-slate-400 underline transition-colors hover:text-slate-600 dark:text-slate-600 dark:hover:text-slate-400"
            >
                "site licensed under agpl-3.0."
            </a>
        </footer>
    }
}
