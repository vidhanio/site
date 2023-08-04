use html_node::{html, Node, Text};

#[derive(Copy, Clone, Debug)]
pub struct NavLink<'a> {
    pub name: &'a str,
    pub link: &'a str,
}

impl<'a> From<NavLink<'a>> for Node {
    fn from(item: NavLink<'a>) -> Self {
        html! {
            <li>
                <a
                    href={item.link}
                    class="text-xl font-extrabold italic text-indigo-500 transition-colors hover:text-emerald-500"
                >
                    {Text::from(item.name)}
                </a>
            </li>
        }
    }
}
