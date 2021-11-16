import type { AppProps } from "next/app";
import Head from "next/head";

import "tailwindcss/tailwind.css";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <title>triple tac toe</title>
        <link rel="icon" href="favicon.ico" />

        <meta property="og:title" content="triple tac toe" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="https://triple-tac-toe.vidhan.io" />
        <meta property="og:image" content="https://vidhan.io/og.png" />
        <meta
          property="og:description"
          content="vidhan's home on the internet."
        />
      </Head>
      <Component {...pageProps} />
    </>
  );
}

export default MyApp;
