use html_node::{html, Node, Text};

use crate::{blog_post::BlogPostMetadata, icons};

pub fn link(slug: &str, metadata: &BlogPostMetadata) -> Node {
    let href = format!("/blog/{slug}");

    html! {
        <li>
            <a href=href class="group w-full flex flex-row justify-between rounded bg-slate-200 dark:bg-slate-800">
                <div class="p-4">
                    <h2 class="text-lg text-slate-700 dark:text-slate-300">
                        { Text::from(&metadata.title) }
                    </h2>
                    <time class="text-slate-600 dark:text-slate-400" datetime=metadata.date>
                        { Text::from(metadata.date_text()) }
                    </time>
                    <p class="text-slate-600 dark:text-slate-400">
                        { Text::from(&metadata.description) }
                    </p>
                </div>
                <div
                    class="\
                        grid place-items-center \
                        p-4 \
                        group-hover:translate-x-1 transition-transform \
                        fill-slate-600 dark:fill-slate-400 \
                    "
                >
                    { icons::chevron_right(Some("h-8")) }
                </div>
            </a>
        </li>
    }
}
