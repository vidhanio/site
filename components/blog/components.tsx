import A from "components/elements/a";
import Image from "next/image";
import { MDXComponents } from "mdx/types";

export function MDXPre({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <pre className="overflow-x-auto rounded-md bg-slate-200 px-0 py-4 font-mono text-slate-800 [font-feature-settings:normal]  dark:bg-slate-800 dark:text-slate-200">
      {children}
    </pre>
  );
}

export function MDXImg({
  src,
  alt,
}: {
  src?: string;
  alt?: string;
}): JSX.Element {
  return (
    <div className="rounded-md">
      <Image
        src={src ?? ""}
        alt={alt ?? ""}
        width={16}
        height={9}
        className="rounded-md"
      />
      <figcaption>{alt}</figcaption>
    </div>
  );
}

export function MDXA({
  href,
  children,
}: {
  href?: string;
  children?: React.ReactNode;
}): JSX.Element {
  return <A href={href}>{children}</A>;
}

const mdxComponents: MDXComponents = {
  pre: MDXPre,
  img: MDXImg,
  a: MDXA,
};

export default mdxComponents;
