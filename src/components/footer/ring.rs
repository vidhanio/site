use html_node::{html, Node};

use crate::components::icons::{self, Icon};

const LINKS: [(Icon, &str); 3] = [
    (icons::left_arrow, "https://ring.simonwu.dev/prev/vidhan"),
    (
        icons::question_mark,
        "https://ring.simonwu.dev/random/vidhan",
    ),
    (icons::right_arrow, "https://ring.simonwu.dev/next/vidhan"),
];

pub fn ring() -> Node {
    html! {
        <div class="w-3/4 flex flex-col pt-4 px-4 gap-4 border-t-2 border-indigo-300 dark:border-indigo-700">
            <a
                href="https://ring.simonwu.dev"
                class="text-slate-400 dark:text-slate-600 hover:text-slate-500 transition-colors"
            >
                "cozyring"
            </a>
            <div class="flex flex-row flex-wrap items-center justify-center gap-4 text-slate-400 dark:text-slate-600">
                { LINKS.into_iter().map(|(icon, href)| {
                    html! {
                        <a
                            href=href
                            class="text-slate-400 dark:text-slate-600 hover:text-slate-500 transition-colors"
                        >
                            {icon(Some("h-6"))}
                        </a>
                    }
                }) }
            </div>
        </div>
    }
}
