function BlogMainLayout({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <main className="flex flex-col gap-2 px-32 pb-32 w-full xl:px-96">
      {children}
    </main>
  );
}

export default BlogMainLayout;
