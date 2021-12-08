function H1({
  className,
  children,
}: {
  className?: string;
  children: React.ReactNode;
}): JSX.Element {
  return (
    <h1 className={className ? className : "font-sans text-8xl font-black"}>
      {children}
    </h1>
  );
}

function H2({
  className,
  children,
}: {
  className?: string;
  children: React.ReactNode;
}): JSX.Element {
  return (
    <h2 className={className ? className : "font-sans text-2xl font-medium"}>
      {children}
    </h2>
  );
}

function H3({
  className,
  children,
}: {
  className?: string;
  children: React.ReactNode;
}): JSX.Element {
  return (
    <h2 className={className ? className : "font-sans text-xl font-medium"}>
      {children}
    </h2>
  );
}

export { H1, H2, H3 };
