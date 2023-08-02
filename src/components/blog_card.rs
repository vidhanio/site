use html_node::{html, text, Node};

#[derive(Copy, Clone, Debug)]
pub struct BlogCard<'a> {
    pub slug: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub image_src: Option<&'a str>,
}

impl<'a> From<BlogCard<'a>> for Node {
    fn from(card: BlogCard<'a>) -> Self {
        html! {
            <a href=format!("/blog/{}", card.slug)>
                <div class="flex w-full flex-col items-center justify-center rounded-xl bg-slate-200 dark:bg-slate-800 sm:w-96">
                    {card.image_src.map_or_else(
                        || html! {
                            <div class="flex aspect-square w-full flex-col items-center justify-center sm:h-96 sm:w-96">
                                {blog_placeholder()}
                            </div>
                        },
                        |src| html! {
                            <img
                                src=src
                                alt=card.name
                                width=384
                                height=384
                                class="w-full rounded-t-xl"
                            />
                        }
                    )}
                    <div class="flex flex-col items-center justify-center p-4 text-center sm:p-8">
                        <h3 class="text-xl font-bold text-indigo-600 dark:text-indigo-400">
                            {text!("{}", card.name)}
                        </h3>
                        <p class="text-slate-600 font-normal dark:text-slate-400">
                            {text!("{}", card.description)}
                        </p>
                    </div>
                </div>
            </a>
        }
    }
}

fn blog_placeholder() -> Node {
    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            class="w-16 h-16 fill-slate-300 dark:fill-slate-700"
        >
            <path
                fill-rule="evenodd"
                d="M4.125 3C3.089 3 2.25 3.84 2.25 4.875V18a3 3 0 003 3h15a3 3 0 01-3-3V4.875C17.25 3.839 16.41 3 15.375 3H4.125zM12 9.75a.75.75 0 000 1.5h1.5a.75.75 0 000-1.5H12zm-.75-2.25a.75.75 0 01.75-.75h1.5a.75.75 0 010 1.5H12a.75.75 0 01-.75-.75zM6 12.75a.75.75 0 000 1.5h7.5a.75.75 0 000-1.5H6zm-.75 3.75a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5H6a.75.75 0 01-.75-.75zM6 6.75a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h3a.75.75 0 00.75-.75v-3A.75.75 0 009 6.75H6z"
                clip-rule="evenodd"
            />
            <path
                d="M18.75 6.75h1.875c.621 0 1.125.504 1.125 1.125V18a1.5 1.5 0 01-3 0V6.75z"
            />
        </svg>
    }
}
