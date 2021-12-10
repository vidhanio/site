import Link from "next/link";

interface ItemProps {
  name: string;
  url: string;
  className?: string;
}

interface Props {
  navItems: ItemProps[];
}

function NavItem({ name, url, className }: ItemProps): JSX.Element {
  return (
    <li>
      <Link href={url}>
        <a className={`transition-colors ${className} hover:text-green-500`}>
          {name}
        </a>
      </Link>
    </li>
  );
}

function Nav({ navItems }: Props): JSX.Element {
  return (
    <nav className="sticky p-8 w-full bg-gray-100 border-b-4 border-gray-900 dark:border-gray-100 dark:bg-gray-900">
      <ul className="flex flex-row gap-4 justify-center items-center text-center">
        {navItems.map((item) => (
          <NavItem {...item} key={item.name} />
        ))}
      </ul>
    </nav>
  );
}

export default Nav;
