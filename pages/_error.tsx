import { NextPageContext } from "next";

interface Props {
  statusCode?: number;
}

function Error({ statusCode }: Props) {
  return (
    <header className="flex flex-col gap-2 justify-center items-center w-full h-full grow">
      {statusCode ? (
        <h1 className="text-8xl font-black text-indigo-500">{statusCode}</h1>
      ) : (
        <h1 className="text-8xl">Error</h1>
      )}
    </header>
  );
}

Error.getInitialProps = function ({ res, err }: NextPageContext) {
  const statusCode = res ? res.statusCode : err ? err.statusCode : 404;
  return { statusCode };
};

export default Error;
