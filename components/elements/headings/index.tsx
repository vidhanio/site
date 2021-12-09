interface Props {
  className?: string;
  children: React.ReactNode;
}

function H1({ className, children }: Props): JSX.Element {
  return (
    <h1 className={"font-sans text-8xl font-black" + " " + className ?? ""}>
      {children}
    </h1>
  );
}

function H2({ className, children }: Props): JSX.Element {
  return (
    <h2
      className={
        "font-sans text-2xl font-medium text-purple-400 dark:text-purple-600" +
          " " +
          className ?? ""
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
        "font-sans text-xl font-medium text-purple-300 dark:text-purple-700" +
          " " +
          className ?? ""
      }
    >
      {children}
    </h2>
  );
}

export { H1, H2, H3 };
