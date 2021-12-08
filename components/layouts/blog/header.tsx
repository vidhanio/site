function BlogHeaderLayout({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <main className="flex flex-col gap-2 justify-center items-center p-16 w-full text-center min-h-1/2-screen">
      {children}
    </main>
  );
}

export default BlogHeaderLayout;
