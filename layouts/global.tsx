function WrapperLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex flex-col gap-16 p-16 min-h-[calc(100vh-8rem)]">
      {children}
    </div>
  );
}

export { WrapperLayout };
