type Props = {
  children: React.ReactNode;
};

export function WrapperLayout({ children }: Props): JSX.Element {
  return (
    <main className="flex flex-col items-center gap-16 p-16">{children}</main>
  );
}
