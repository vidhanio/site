mod link;
mod ring;

use html_node::{html, Node};

pub use self::link::FooterLink;
use self::ring::ring;

pub fn footer<'a>(links: impl IntoIterator<Item = FooterLink<'a>>) -> Node {
    html! {
        <footer class="text-center w-full p-8 flex flex-col items-center gap-4 border-t-2 border-indigo-300 dark:border-indigo-700">
            <p class="italic text-lg text-slate-400 dark:text-slate-600">
                "made with <3 by vidhan."
            </p>
            <ul class="flex flex-row flex-wrap items-center justify-center gap-4 text-slate-400 dark:text-slate-600">
                { links }
            </ul>
            <a
                href="/static/LICENSE.txt"
                class="text-xs font-thin text-slate-400 dark:text-slate-600 hover:text-slate-500"
            >
                "site licensed under agpl-3.0."
            </a>

            { ring() }
        </footer>
    }
}
