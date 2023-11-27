mod footer;
mod nav;
mod seo;

use maud::{html, Markup, DOCTYPE};

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

pub fn document(path: Option<&str>, content: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                (seo(path))
                link rel="stylesheet" href=(concat!("/static/styles.css?v=", env!("GIT_COMMIT_HASH")));
            }

            body.min-h-screen."px-[10%]"."lg:px-[25%]".flex.flex-col.items-center.font-mono."bg-stone-100"."dark:bg-stone-900"."dark:text-stone-300"."text-stone-700" {
                (nav(NAV_LINKS))
                main.w-full."py-8"."flex-1"."border-stone-600".flex.flex-col."gap-8" {
                    (content)
                }
                (footer(FOOTER_LINKS))
            }
        }
    }
}
