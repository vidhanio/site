function Layout({ children }: { children?: React.ReactNode }) {
  return (
    <main className="flex flex-col gap-2 justify-center items-center p-16 w-full min-h-screen text-center">
      {children}
    </main>
  );
}

export default Layout;
