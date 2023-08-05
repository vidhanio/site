use html_node::{html, Node};

use crate::components::{document, ProjectLink};

const PROJECTS: [ProjectLink<'static>; 6] = [
    ProjectLink {
        name: "site",
        description: "this website!",
        href: "https://github.com/vidhanio/site",
    },
    ProjectLink {
        name: "html-node",
        description: "an html macro for rust (used to build this website!)",
        href: "https://github.com/vidhanio/html-node",
    },
    ProjectLink {
        name: "fncli",
        description: "an attribute macro to simplify writing simple clis in rust.",
        href: "https://github.com/vidhanio/fncli",
    },
    ProjectLink {
        name: "diswordle",
        description: "a discord bot to play wordle right in your discord server.",
        href: "https://github.com/vidhanio/diswordle",
    },
    ProjectLink {
        name: "checkpoint",
        description:
            "a discord bot to provide easy verification for discord servers in my school board.",
        href: "https://github.com/vidhanio/checkpoint",
    },
    ProjectLink {
        name: "dmux",
        description: "a package for go to make discord command definitions more declarative.",
        href: "https://github.com/vidhanio/dmux",
    },
];

#[allow(clippy::unused_async)]
pub async fn get() -> Node {
    document(
        Some("projects"),
        html! {
            <h1>projects</h1>
            <ul class="flex flex-col gap-4">
                {PROJECTS}
            </ul>
        },
    )
}
