function BlogHeaderLayout({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <main className="flex flex-col gap-2 justify-center items-center p-16 w-full text-center min-h-3/4-screen">
      {children}
    </main>
  );
}

function BlogMainLayout({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <main className="flex flex-col gap-2 px-16 pb-16 w-full md:px-32 md:pb-32 xl:px-96">
      {children}
    </main>
  );
}

export { BlogHeaderLayout, BlogMainLayout };
