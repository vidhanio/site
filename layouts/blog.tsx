function BlogMainLayout({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <main className="flex flex-col p-8 w-full sm:p-16 sm:pb-16 md:px-32 md:pb-32 xl:px-96">
      <article
        className={
          "w-full max-w-none prose prose-hr:border-none prose-a:transition-colors prose-a:text-indigo-600 hover:prose-a:text-emerald-600 dark:hover:prose-a:text-emerald-400 dark:prose-a:text-indigo-400 dark:prose-invert prose-code:before:content-none prose-code:after:content-none"
        }
      >
        {children}
      </article>
    </main>
  );
}

export { BlogMainLayout };
