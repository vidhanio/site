export function MainLayout({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <main className="flex flex-col gap-16 justify-center items-center w-full text-center">
      {children}
    </main>
  );
}

export function SectionLayout({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <section className="flex flex-col gap-8 justify-center items-center w-full text-center">
      {children}
    </section>
  );
}
