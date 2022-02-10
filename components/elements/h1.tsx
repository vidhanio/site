type Props = {
  children: React.ReactNode;
};

export default function H1({ children }: Props): JSX.Element {
  return (
    <h1 className="text-6xl font-extrabold italic text-indigo-500">
      {children}
    </h1>
  );
}
