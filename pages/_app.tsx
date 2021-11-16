import type { AppProps } from "next/app";
import Head from "next/head";

import "tailwindcss/tailwind.css";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <title>vidhan</title>
        <link rel="icon" href="favicon.ico" />

        <meta property="og:title" content="vidhan" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="https://vidhan.io" />
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
