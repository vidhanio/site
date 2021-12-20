export function WrapperLayout({ children }: { children: React.ReactNode }) {
  return <div className="flex flex-col gap-16 p-16">{children}</div>;
}
