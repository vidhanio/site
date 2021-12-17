function Error() {
  return (
    <header className="flex flex-col gap-2 justify-center items-center w-full h-full grow">
      <h1 className="text-8xl font-black text-indigo-500">404</h1>
      <p className="text-2xl font-semibold text-indigo-700 dark:text-indigo-300">
        my bad.
      </p>
    </header>
  );
}

export default Error;
