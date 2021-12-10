import { useState, useEffect, ReactElement } from "react";
import innerText from "react-innertext";

interface Props {
  className?: string;
  children?: React.ReactNode;
}

function H1({ className, children }: Props): JSX.Element {
  return (
    <h1 className={"text-8xl font-black" + " " + (className ?? "")}>
      {children}
    </h1>
  );
}

function H2({ className, children }: Props): JSX.Element {
  return (
    <h2
      className={
        "text-2xl font-medium text-violet-400 dark:text-violet-600" +
        " " +
        (className ?? "")
      }
    >
      {children}
    </h2>
  );
}

function H3({ className, children }: Props): JSX.Element {
  return (
    <h2
      className={
        "text-xl font-medium text-violet-300 dark:text-violet-700" +
        " " +
        (className ?? "")
      }
    >
      {children}
    </h2>
  );
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
      className="text-violet-600 underline transition-colors dark:text-violet-400 hover:text-green-500"
      href={href}
    >
      {children}
    </a>
  );
}

function Pre({ children }: { children?: React.ReactNode }): JSX.Element {
  return (
    <div className="overflow-x-auto text-gray-700 py-4 bg-gray-200 font-['Fira_Code'] rounded-md not-prose dark:text-gray-300 dark:bg-gray-800">
      <pre>{children}</pre>
    </div>
  );
}

const mdxComponents = {
  pre: Pre,
};

export { H1, H2, H3, A, Pre, mdxComponents };
