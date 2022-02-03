type Props = {
  children: React.ReactNode;
  className?: string;
};

export default function H1({ children, className }: Props): JSX.Element {
  return (
    <h1
      className={`text-8xl font-extrabold italic text-indigo-500 ${className}`}
    >
      {children}
    </h1>
  );
}
