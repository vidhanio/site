function MainLayout({ children }: { children?: React.ReactNode }): JSX.Element {
  return (
    <main className="flex flex-col gap-8 justify-center items-center p-16 w-full text-center">
      {children}
    </main>
  );
}

export default MainLayout;
