export function WrapperLayout({
  children,
}: {
  children: React.ReactNode;
}): JSX.Element {
  return (
    <main className="flex flex-col items-center gap-16 p-16">{children}</main>
  );
}
