import NavItem, { NavItemProps } from "./item";

interface Props {
  navItems: NavItemProps[];
}

function Nav({ navItems }: Props): JSX.Element {
  return (
    <nav className="fixed p-8 w-full bg-gray-100 border-b-4 border-gray-900 dark:border-gray-100 dark:bg-gray-900">
      <ul className="flex flex-row gap-4 justify-center items-center text-center">
        {navItems.map((item) => (
          <NavItem {...item} key={item.name} />
        ))}
      </ul>
    </nav>
  );
}

export default Nav;
