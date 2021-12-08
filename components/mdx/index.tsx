function H1({ children }: { children: React.ReactNode }): JSX.Element {
  return (
    <h1 className="font-sans text-4xl font-black">
      <span className="text-purple-200 dark:text-purple-800">#</span> {children}
    </h1>
  );
}

function H2({ children }: { children: React.ReactNode }): JSX.Element {
  return (
    <h2 className="font-sans text-2xl font-bold">
      <span className="text-purple-200 dark:text-purple-800">##</span>{" "}
      {children}
    </h2>
  );
}

function H3({ children }: { children: React.ReactNode }): JSX.Element {
  return (
    <h3 className="font-sans text-xl font-bold">
      <span className="text-purple-200 dark:text-purple-800">###</span>{" "}
      {children}
    </h3>
  );
}

function A({
  children,
  href,
}: {
  children: React.ReactNode;
  href: string;
}): JSX.Element {
  return (
    <a
      className="text-purple-600 transition-colors dark:text-purple-400 hover:text-green-500"
      href={href}
    >
      {children}
    </a>
  );
}

function InlineCode({ children }: { children: React.ReactNode }): JSX.Element {
  return (
    <code className="p-1 font-mono text-sm text-gray-800 bg-gray-200 rounded-md dark:text-gray-200 dark:bg-gray-800">
      {children}
    </code>
  );
}

function Code({ children }: { children: React.ReactNode }): JSX.Element {
  return (
    <p>
      <code className="p-1 font-mono text-sm bg-gray-200 rounded-md dark:bg-gray-800">
        {children}
      </code>
    </p>
  );
}

const components = {
  h1: H1,
  h2: H2,
  h3: H3,
  a: A,
  inlineCode: InlineCode,
  code: Code,
};

export { components as postComponents };
