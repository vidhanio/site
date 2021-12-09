import { useState, useEffect } from "react";
import innerText from "react-innertext";

function H1({ children }: { children?: React.ReactNode }): JSX.Element {
  return <h1 className="my-4 font-sans text-4xl font-black">{children}</h1>;
}

function H2({ children }: { children?: React.ReactNode }): JSX.Element {
  return <h2 className="my-2 font-sans text-2xl font-bold">{children}</h2>;
}

function H3({ children }: { children?: React.ReactNode }): JSX.Element {
  return <h3 className="my-1 font-sans text-xl font-bold">{children}</h3>;
}

function A({
  children,
  href,
}: {
  children?: React.ReactNode;
  href?: string;
}): JSX.Element {
  return (
    <a
      className="text-purple-600 underline transition-colors dark:text-purple-400 hover:text-green-500"
      href={href}
    >
      {children}
    </a>
  );
}

function Code({
  children,
  className,
}: {
  children?: React.ReactNode;
  className?: string;
}): JSX.Element {
  return (
    <code
      className={
        className
          ? className
          : "px-1 font-mono bg-gray-300 rounded-md dark:bg-gray-700"
      }
    >
      {children}
    </code>
  );
}

function Pre({ children }: { children?: React.ReactNode }): JSX.Element {
  const text = innerText(children);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (copied) {
      setTimeout(() => {
        setCopied(false);
      }, 1000);
    }
  }, [copied]);

  // get the className of the child
  const childClassName = (children as React.ReactElement).props
    .className as string;

  // remove language- prefix
  const language = childClassName
    .replace("language-", "")
    .replace("hljs", "")
    .trim()
    .toUpperCase();

  return (
    <div className="flex flex-col my-4">
      <div className="flex justify-between items-center p-4 bg-gray-300 rounded-t-md dark:bg-gray-700">
        <span className="text-lg font-bold text-gray-900 dark:text-gray-100">
          {language}
        </span>
        <button
          className="p-2 text-sm font-semibold text-white bg-purple-500 rounded-md transition-colors hover:bg-green-500"
          onClick={() => {
            navigator.clipboard.writeText(text);
            setCopied(true);
          }}
        >
          {copied ? "Copied!" : "Copy"}
        </button>
      </div>
      <pre className="overflow-auto p-4 font-mono text-left text-gray-700 bg-gray-200 rounded-b-md dark:text-gray-300 line-numbers dark:bg-gray-800 hyphens-none">
        {children}
      </pre>
    </div>
  );
}

function Ul({ children }: { children?: React.ReactNode }): JSX.Element {
  return <ul className="pl-4 my-4 list-disc">{children}</ul>;
}

const components = {
  h1: H1,
  h2: H2,
  h3: H3,
  a: A,
  code: Code,
  pre: Pre,
  ul: Ul,
};

export { components as mdxComponents };
