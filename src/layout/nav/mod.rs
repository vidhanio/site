mod link;

use maud::{html, Markup};

pub use self::link::NavLink;

pub fn nav<'a>(links: impl IntoIterator<Item = NavLink<'a>>) -> Markup {
    // html! {
    //     <nav class="w-full p-8 border-b-2 border-stone-300
    // dark:border-stone-700">         <ul class="flex text-center gap-4
    // flex-col sm:flex-row sm:gap-16 justify-center">             { links }
    //         </ul>
    //     </nav>
    // }
    html! {
        nav.w-full."p-8"."border-b-2"."border-stone-300"."dark:border-stone-700" {
            ul.flex.text-center."gap-4".flex-col."sm:flex-row"."sm:gap-16".justify-center {
                @for link in links {
                    (link)
                }
            }
        }
    }
}
