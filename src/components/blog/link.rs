use html_node::{html, Node, Text};

use super::{BlogPostMetadata, BlogSlug};
use crate::components::icons;

#[derive(Clone, Debug)]
pub struct BlogLink {
    pub slug: BlogSlug,
    pub metadata: BlogPostMetadata,
}

impl From<BlogLink> for Node {
    fn from(link: BlogLink) -> Self {
        let href = format!("/blog/{}", link.slug);

        html! {
            <li>
                <a href=href class="group w-full flex flex-row justify-between rounded bg-slate-200 dark:bg-slate-800">
                    <div class="p-4">
                        <h2 class="text-lg text-slate-700 dark:text-slate-300">
                            {Text::from(&link.metadata.title)}
                        </h2>
                        <time class="text-slate-600 dark:text-slate-400" datetime=link.metadata.date>
                            {Text::from(link.metadata.date_text())}
                        </time>
                        <p class="text-slate-600 dark:text-slate-400">
                            {Text::from(link.metadata.description)}
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
                        {icons::chevron_right(Some("h-8"))}
                    </div>
                </a>
            </li>
        }
    }
}
