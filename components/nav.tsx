import Link from "next/link";

interface NavItemProps {
  name: string;
  url: string;
  className?: string;
}

interface NavProps {
  navItems: NavItemProps[];
}

function NavItem({ name, url, className }: NavItemProps) {
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

function Nav({ navItems }: NavProps) {
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
