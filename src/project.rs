use maud::{html, Markup};
use serde::Deserialize;

use crate::icon::CHEVRON;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Project {
    pub name: String,
    pub description: String,
    pub href: String,
}

impl Project {
    pub fn link(&self) -> Markup {
        html! {
            li {
                a.group.w-full.flex.flex-row.justify-between.rounded."bg-stone-200"."dark:bg-stone-800" href=(self.href) {
                    div."p-4" {
                        h2.text-lg."text-stone-700"."dark:text-stone-300" {
                            (self.name)
                        }
                        p."text-stone-600"."dark:text-stone-400" {
                            (self.description)
                        }
                    }
                    div.grid.place-items-center."p-4"."group-hover:translate-x-1".transition-transform."fill-stone-600"."dark:fill-stone-400" {
                        (CHEVRON)
                    }
                }
            }
        }
    }
}
