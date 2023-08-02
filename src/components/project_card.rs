use html_node::{html, text, Node};

#[derive(Copy, Clone, Debug)]
pub struct ProjectCard<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub href: &'a str,
    pub image_src: Option<&'a str>,
}

impl<'a> From<ProjectCard<'a>> for Node {
    fn from(card: ProjectCard<'a>) -> Self {
        html! {
            <a href=card.href>
                <div class="flex w-full flex-col items-center justify-center rounded-xl bg-slate-200 dark:bg-slate-800 sm:w-96">
                    {card.image_src.map_or_else(
                        || html! {
                            <div class="flex aspect-square w-full flex-col items-center justify-center sm:h-96 sm:w-96">
                                {project_placeholder()}
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

fn project_placeholder() -> Node {
    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24" fill="currentColor"
            class="w-16 h-16 fill-slate-300 dark:fill-slate-700"
        >
            <path
                fill-rule="evenodd"
                d="M12 6.75a5.25 5.25 0 016.775-5.025.75.75 0 01.313 1.248l-3.32 3.319c.063.475.276.934.641 1.299.365.365.824.578 1.3.64l3.318-3.319a.75.75 0 011.248.313 5.25 5.25 0 01-5.472 6.756c-1.018-.086-1.87.1-2.309.634L7.344 21.3A3.298 3.298 0 112.7 16.657l8.684-7.151c.533-.44.72-1.291.634-2.309A5.342 5.342 0 0112 6.75zM4.117 19.125a.75.75 0 01.75-.75h.008a.75.75 0 01.75.75v.008a.75.75 0 01-.75.75h-.008a.75.75 0 01-.75-.75v-.008z"
                clip-rule="evenodd"
            />
            <path
                d="M10.076 8.64l-2.201-2.2V4.874a.75.75 0 00-.364-.643l-3.75-2.25a.75.75 0 00-.916.113l-.75.75a.75.75 0 00-.113.916l2.25 3.75a.75.75 0 00.643.364h1.564l2.062 2.062 1.575-1.297z"
            />
            <path
                fill-rule="evenodd"
                d="M12.556 17.329l4.183 4.182a3.375 3.375 0 004.773-4.773l-3.306-3.305a6.803 6.803 0 01-1.53.043c-.394-.034-.682-.006-.867.042a.589.589 0 00-.167.063l-3.086 3.748zm3.414-1.36a.75.75 0 011.06 0l1.875 1.876a.75.75 0 11-1.06 1.06L15.97 17.03a.75.75 0 010-1.06z"
                clip-rule="evenodd"
            />
        </svg>
    }
}
