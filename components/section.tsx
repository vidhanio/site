type Props = {
  title: string;
  children: React.ReactNode;
};

export function Section({ title, children }: Props): JSX.Element {
  return (
    <div
      id="introduction"
      className="flex max-w-2xl flex-col items-center gap-8 text-center"
    >
      <h2 className="text-4xl font-bold text-indigo-600 dark:text-indigo-400">
        {title}
      </h2>
      {children}
    </div>
  );
}
