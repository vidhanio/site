function H1({
  className,
  children,
}: {
  className?: string;
  children: React.ReactNode;
}): JSX.Element {
  return <h1 className={className && "text-8xl font-black"}>{children}</h1>;
}

export default H1;
