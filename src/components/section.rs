use html_node::{html, Node, Text};

pub fn section(title: &str, content: Node) -> Node {
    html! {
        <div
            id="introduction"
            class="flex max-w-3xl flex-col items-center gap-4 text-center sm:gap-8"
        >
            <h2 class="text-3xl font-bold text-indigo-600 dark:text-indigo-400">
                {Text::from(title)}
            </h2>
            {content}
        </div>
    }
}
