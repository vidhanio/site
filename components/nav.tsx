import Link from "next/link";

type NavItemProps = {
  name: string;
  url: string;
  className?: string;
};

type Props = {
  navItems: NavItemProps[];
};

function NavItem({ name, url, className }: NavItemProps): JSX.Element {
  return (
    <li>
      <Link href={url}>
        <a
          className={`text-xl font-extrabold italic text-indigo-500 transition-colors ${className} hover:text-emerald-500`}
        >
          {name}
        </a>
      </Link>
    </li>
  );
}

export default function Nav({ navItems }: Props): JSX.Element {
  return (
    <nav className="sticky top-0 w-full items-center justify-center p-16">
      <ul className="flex h-full w-full flex-row items-center justify-center gap-4 text-center">
        {navItems.map((item) => (
          <NavItem {...item} key={item.name} />
        ))}
      </ul>
    </nav>
  );
}
