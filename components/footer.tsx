import Link from "next/link";
import { AnchorHTMLAttributes } from "react";

type FooterItemProps = {
  name: string;
  url: string;
  attributes?: AnchorHTMLAttributes<HTMLAnchorElement>;
};

type Props = {
  footerItems: FooterItemProps[];
};

function FooterItem({ name, url, attributes }: FooterItemProps): JSX.Element {
  return (
    <li>
      <Link
        {...attributes}
        href={url}
        className="text-sm font-light italic text-slate-400 underline transition-colors hover:text-slate-600 dark:text-slate-600 dark:hover:text-slate-400"
      >
        {name}
      </Link>
    </li>
  );
}

export default function Footer({ footerItems }: Props): JSX.Element {
  return (
    <footer className="flex flex-col items-center gap-4 border-t-2 border-indigo-200 p-8 dark:border-indigo-800 sm:p-16">
      <p className="text-lg text-slate-400 dark:text-slate-600">
        {"made with <3 by vidhan."}
      </p>
      <ul className="flex flex-row justify-center gap-4">
        {footerItems.map((item) => (
          <FooterItem {...item} key={item.name} />
        ))}
      </ul>
      <a
        href="https://github.com/vidhanio/site/blob/main/LICENSE"
        className="text-xs font-thin text-slate-400 underline transition-colors hover:text-slate-600 dark:text-slate-600 dark:hover:text-slate-400"
      >
        {"site licensed under agpl-3.0."}
      </a>
    </footer>
  );
}
