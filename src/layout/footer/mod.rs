mod link;
mod ring;

use maud::{html, Markup};

pub use self::link::FooterLink;
use self::ring::ring;

pub fn footer<'a>(links: impl IntoIterator<Item = FooterLink<'a>>) -> Markup {
    html! {
        footer.text-center.w-full."p-8".flex.flex-col.items-center."gap-4"."border-t-2"."border-stone-300"."dark:border-stone-700" {
            p.font-bold.text-lg."text-stone-400"."dark:text-stone-600" {
                "made with <3 by vidhan."
            }
            ul.flex.flex-row.flex-wrap.items-center.justify-center."gap-4"."text-stone-400"."dark:text-stone-600" {
                @for link in links {
                    (link)
                }
            }
            a.text-xs.font-thin."text-stone-400"."dark:text-stone-600"."hover:text-stone-500" href="/static/LICENSE.txt" {
                "site licensed under agpl-3.0."
            }

            (ring())
        }
    }
}
