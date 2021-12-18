import Link from "next/link";

type ItemProps = {
  name: string;
  url: string;
  className?: string;
};

type Props = {
  navItems: ItemProps[];
};

function NavItem({ name, url, className }: ItemProps): JSX.Element {
  return (
    <li>
      <Link href={url}>
        <a
          className={`text-xl font-bold text-indigo-500 transition-colors ${className} hover:text-emerald-500`}
        >
          {name}
        </a>
      </Link>
    </li>
  );
}

export default function Nav({ navItems }: Props): JSX.Element {
  return (
    <nav className="sticky top-0 justify-center items-center w-full h-32 bg-gray-100 dark:bg-gray-900">
      <ul className="flex flex-row gap-4 justify-center items-center w-full h-full text-center">
        {navItems.map((item) => (
          <NavItem {...item} key={item.name} />
        ))}
      </ul>
    </nav>
  );
}
