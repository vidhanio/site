type Props = {
  children: React.ReactNode;
};

export default function BlogArticleLayout({ children }: Props): JSX.Element {
  return (
    <article
      className={
        "prose prose-hr:border-none prose-a:transition-colors prose-a:text-indigo-600 hover:prose-a:text-emerald-600 dark:hover:prose-a:text-emerald-400 dark:prose-a:text-indigo-400 dark:prose-invert prose-code:before:content-none prose-code:after:content-none w-full max-w-2xl"
      }
    >
      {children}
    </article>
  );
}
