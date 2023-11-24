use maud::{html, Markup};

use crate::icon::Icon;

#[rustfmt::skip]
const LEFT_ARROW: Icon = Icon {
    d: "M20.25 12a.75.75 0 01-.75.75H6.31l5.47 5.47a.75.75 0 11-1.06 1.06l-6.75-6.75a.75.75 0 010-1.06l6.75-6.75a.75.75 0 111.06 1.06l-5.47 5.47H19.5a.75.75 0 01.75.75z",
    class: "h-6",
};

#[rustfmt::skip]
const QUESTION_MARK: Icon = Icon {
    d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm11.378-3.917c-.89-.777-2.366-.777-3.255 0a.75.75 0 01-.988-1.129c1.454-1.272 3.776-1.272 5.23 0 1.513 1.324 1.513 3.518 0 4.842a3.75 3.75 0 01-.837.552c-.676.328-1.028.774-1.028 1.152v.75a.75.75 0 01-1.5 0v-.75c0-1.279 1.06-2.107 1.875-2.502.182-.088.351-.199.503-.331.83-.727.83-1.857 0-2.584zM12 18a.75.75 0 100-1.5.75.75 0 000 1.5z",
    class: "h-6",
};

#[rustfmt::skip]
const RIGHT_ARROW: Icon = Icon { 
    d: "M3.75 12a.75.75 0 01.75-.75h13.19l-5.47-5.47a.75.75 0 011.06-1.06l6.75 6.75a.75.75 0 010 1.06l-6.75 6.75a.75.75 0 11-1.06-1.06l5.47-5.47H4.5a.75.75 0 01-.75-.75z",
    class: "h-6",
};

const LINKS: [(Icon, &str); 3] = [
    (LEFT_ARROW, "https://ring.simonwu.dev/prev/vidhan"),
    (QUESTION_MARK, "https://ring.simonwu.dev/random/vidhan"),
    (RIGHT_ARROW, "https://ring.simonwu.dev/next/vidhan"),
];

pub fn ring() -> Markup {
    html! {
        div.flex.flex-row.flex-wrap.items-center.justify-center."gap-4"."text-stone-400"."dark:text-stone-600" {
            @for (icon, href) in LINKS {
                a."text-stone-400"."dark:text-stone-600"."hover:text-stone-500".transition-colors href=(href) {
                    (icon)
                }
            }
        }
    }
}
