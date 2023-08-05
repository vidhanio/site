mod link;

use html_node::{html, Node};

pub use self::link::NavLink;

pub fn nav<'a>(links: impl IntoIterator<Item = NavLink<'a>>) -> Node {
    html! {
        <nav class="w-full p-8 border-b-2 border-indigo-300 dark:border-indigo-700">
            <ul class="flex text-center gap-4 flex-col sm:flex-row sm:gap-16 justify-center">
                {links}
            </ul>
        </nav>
    }
}
