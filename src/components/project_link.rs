use html_node::{html, Node, Text};

use super::icons;
use crate::project::Project;

#[derive(Copy, Clone, Debug)]
pub struct ProjectLink<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub href: &'a str,
}

impl<'a> From<&'a Project> for ProjectLink<'a> {
    fn from(project: &'a Project) -> Self {
        Self {
            name: &project.name,
            description: &project.description,
            href: &project.href,
        }
    }
}

impl<'a> From<ProjectLink<'a>> for Node {
    fn from(link: ProjectLink<'a>) -> Self {
        html! {
            <li>
                <a href=link.href class="group w-full flex flex-row justify-between rounded bg-slate-200 dark:bg-slate-800">
                    <div class="p-4">
                        <h2 class="text-lg text-slate-700 dark:text-slate-300">
                            {Text::from(link.name)}
                        </h2>
                        <p class="text-slate-600 dark:text-slate-400">
                            {Text::from(link.description)}
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
                        {icons::chevron_right(Some("h-8 w-8"))}
                    </div>
                </a>
            </li>
        }
    }
}
