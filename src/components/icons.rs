//! icons sourced from heroicons.

use html_node::{html, Node, Text};

macro_rules! icons {
    ($($name:ident { $($tt:tt)* })*) => {
        $(
            pub fn $name(class: Option<&'static str>) -> Node {
                html! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24" fill="currentColor"
                        class=class as &str
                        role="img"
                    >
                        <title>{Text::from(concat!(stringify!($name), " icon"))}</title>
                        $($tt)*
                    </svg>
                }
            }
        )*
    };
}

icons! {
    date {
        <path
            fill-rule="evenodd"
            d="M6.75 2.25A.75.75 0 017.5 3v1.5h9V3A.75.75 0 0118 3v1.5h.75a3 3 0 013 3v11.25a3 3 0 01-3 3H5.25a3 3 0 01-3-3V7.5a3 3 0 013-3H6V3a.75.75 0 01.75-.75zm13.5 9a1.5 1.5 0 00-1.5-1.5H5.25a1.5 1.5 0 00-1.5 1.5v7.5a1.5 1.5 0 001.5 1.5h13.5a1.5 1.5 0 001.5-1.5v-7.5z"
            clip-rule="evenodd"
        />
    }

    chevron_right {
        <path
            fill-rule="evenodd"
            d="M16.28 11.47a.75.75 0 010 1.06l-7.5 7.5a.75.75 0 01-1.06-1.06L14.69 12 7.72 5.03a.75.75 0 011.06-1.06l7.5 7.5z"
            clip-rule="evenodd"
        />
    }
}
