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

export default BlogHeaderLayout;
