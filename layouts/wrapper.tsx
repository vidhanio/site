type Props = {
  children: React.ReactNode;
};

export function WrapperLayout({ children }: Props): JSX.Element {
  return (
    <main className="flex flex-col items-center gap-16 px-8 pb-8 pt-16 sm:p-16">
      {children}
    </main>
  );
}
