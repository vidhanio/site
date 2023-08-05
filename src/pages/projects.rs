use html_node::{html, Node};

use crate::components::{document, ProjectLink};

#[allow(clippy::unused_async)]
pub async fn get() -> Node {
    const PROJECTS: [ProjectLink<'static>; 3] = [
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
