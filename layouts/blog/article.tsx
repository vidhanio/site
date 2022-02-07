type Props = {
  children: React.ReactNode;
};

export default function BlogArticleLayout({ children }: Props): JSX.Element {
  return (
    <article
      className={
        "prose-code:px-1 prose-code:rounded-md prose prose-hr:border-none prose-a:transition-colors prose-a:text-indigo-600 hover:prose-a:text-emerald-600 dark:hover:prose-a:text-emerald-400 dark:prose-a:text-indigo-400 dark:prose-invert prose-code:before:content-none prose-code:[font-feature-settings:normal] prose-code:text-violet-700 dark:prose-code:text-violet-300 prose-code:bg-gray-200 prose-code:after:content-none dark:prose-code:bg-gray-800 w-full max-w-3xl"
      }
    >
      {children}
    </article>
  );
}
