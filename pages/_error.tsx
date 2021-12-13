import { NextPageContext } from "next";
import { WrapperLayout } from "layouts/global";

interface Props {
  statusCode?: number;
}

function Error({ statusCode }: Props) {
  return (
    <header className="flex flex-col justify-center w-full h-[calc(100vh-24rem)] items-center">
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
