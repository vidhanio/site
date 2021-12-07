import Link from "next/link";

interface Props {
  name: string;
  url: string;
  className?: string;
}

function NavItem({ name, url, className }: Props): JSX.Element {
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

export type { Props as NavItemProps };
export default NavItem;
