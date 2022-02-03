type Props = {
  children: React.ReactNode;
};

export default function H1({ children }: Props): JSX.Element {
  return (
    <h1 className="not-prose mb-2 text-8xl font-extrabold italic text-indigo-500">
      {children}
    </h1>
  );
}
