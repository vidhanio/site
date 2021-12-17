function Error() {
  return (
    <header className="flex flex-col gap-2 justify-center items-center w-full h-full grow">
      <h1 className="text-8xl font-black text-indigo-500">500</h1>
      <p className="text-2xl font-semibold text-indigo-700 dark:text-indigo-300">
        shoot an email to{" "}
        <a
          className="underline transition-colors hover:text-emerald-700 dark:hover:text-emerald-300"
          href="mailto:me@vidhan.io"
        >
          me@vidhan.io
        </a>
        .
      </p>
    </header>
  );
}

export default Error;
