use html_node::{html, Node};

use crate::components::{document, ProjectCard};

#[allow(clippy::unused_async)]
pub async fn get() -> Node {
    const PROJECTS: [ProjectCard<'static>; 3] = [
        ProjectCard {
            name: "diswordle",
            description: "a discord bot to play wordle right in your discord server.",
            href: "https://github.com/vidhanio/diswordle",
            image_src: None,
        },
        ProjectCard {
            name: "checkpoint",
            description:
                "a discord bot to provide easy verification for discord servers in my school board.",
            href: "https://github.com/vidhanio/checkpoint",
            image_src: None,
        },
        ProjectCard {
            name: "dmux",
            description: "a package for go to make discord command definitions more declarative.",
            href: "https://github.com/vidhanio/dmux",
            image_src: None,
        },
    ];

    document(
        Some("projects"),
        html! {
            <h1>projects</h1>
            <div class="flex flex-row flex-wrap justify-center gap-4 sm:gap-8">
                {PROJECTS}
            </div>
        },
    )
}
