function BlogMainLayout({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <main className="flex flex-col p-16 w-full md:px-32 md:pb-32 xl:px-96">
      <article
        className={
          "prose prose-violet dark:prose-invert prose-code:before:content-none prose-code:after:content-none"
        }
      >
        {children}
      </article>
    </main>
  );
}

export { BlogMainLayout };
