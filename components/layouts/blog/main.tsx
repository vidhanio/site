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

export default BlogMainLayout;
