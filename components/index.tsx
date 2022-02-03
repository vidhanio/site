export function Section({
  children,
}: {
  children: React.ReactNode;
}): JSX.Element {
  return (
    <div
      id="introduction"
      className="flex max-w-2xl flex-col items-center gap-8 text-center"
    >
      {children}
    </div>
  );
}
