export function WrapperLayout({
  children,
}: {
  children: React.ReactNode;
}): JSX.Element {
  return (
    <section className="flex flex-col items-center gap-16 p-16">
      {children}
    </section>
  );
}
