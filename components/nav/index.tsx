import NavItem, { NavItemProps } from "./item";

interface Props {
  navItems: NavItemProps[];
}

function Nav({ navItems }: Props): JSX.Element {
  return (
    <nav className="fixed p-16 w-full bg-gradient-to-b from-gray-100 to-transparent dark:from-gray-900">
      <ul className="flex flex-row gap-4 justify-center items-center text-center">
        {navItems.map((item) => (
          <NavItem {...item} key={item.name} />
        ))}
      </ul>
    </nav>
  );
}

export default Nav;
